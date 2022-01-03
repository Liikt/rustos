use std::{path::Path, error::Error, process::Command};

use elf_parser::{ElfParser, PTYPE_LOAD};

// The name of the bootloader file
const BOOTFILE: &str = "rustos.boot";

// Flatten a given elf into it's loaded bytes and additionally return the entry
// point and image base
fn flatten_img<P: AsRef<Path>>(path: P) -> Option<(u32, u32, Vec<u8>)> {
    let img = std::fs::read(path).ok()?;
    let elf = ElfParser::parse(&img)?;
    let img_size: usize = elf.image_end.checked_sub(elf.image_base)?
        .checked_add(1)?.try_into().ok()?;
    let mut flat = std::vec![0u8; img_size];

    // Go through all sections and add the LOAD sections to the flat image
    elf.sections(|base, vsize, ptype, raw, _, _, _| {
        if ptype != PTYPE_LOAD { return Some(()); }

        let size: usize = std::cmp::min(vsize.try_into().ok()?, raw.len());
        let flat_off: usize = (base - elf.image_base).try_into().ok()?;

        flat[flat_off..flat_off.checked_add(size)?].copy_from_slice(raw);

        Some(())
    })?;

    // Check that the entry point falls within the image
    if elf.entry_point < elf.image_base || elf.entry_point > elf.image_end {
        return None;
    }

    Some((
        elf.entry_point.try_into().ok()?,
        elf.image_base.try_into().ok()?,
        flat
    ))
}

// Build and deploy the bootloader
fn main() -> Result<(), Box<dyn Error>> {
    // Build the bootloader
    let boot_args = ["build", "--release", "-Zbuild-std", "--target",
        "x86-64-bootloader.json"];
    if !Command::new("cargo").current_dir("bootloader").args(&boot_args)
            .status()?.success() {
        panic!("Couldn't build bootloader!");
    }

    // Flatten the rust portion of the bootloader
    let (entry, _, flat) = 
        flatten_img("bootloader/target/x86-64-bootloader/release/bootloader")
        .ok_or("Couldn't flatten bootloader!")?;

    // Write the flat image out to a file that can be included by the stage0.S
    // file
    std::fs::write("bin/boot.flat", flat)?;

    // Args for nasm
    let m = format!("-Dentry_point={:#x}", entry);
    let mut args = vec!["-f", "bin", &m, "-o",
        BOOTFILE, "bootloader/src/stage0.S"];

    // Build a debug version of the bootloader if needed
    if std::env::args().any(|x| x == "debug") {
        args.push("-Ddbg");
    }
    
    // Build stage0
    if !Command::new("nasm").args(args).status()?.success() {
        panic!("Couldn't assemble stage0");
    }

    // Check that the bootloader falls within the given restrictions
    let md = std::fs::metadata(BOOTFILE)
        .expect("Couldn't get metadata from bootfile");
    assert!(md.is_file(), "Bootfile is not a file");
    assert!(md.len() <= (32 * 1024));

    // Print the size of the bootloader
    println!("Bootloader size is {} bytes ({:8.4}%)",
        md.len(), md.len() as f64 / (32. * 1024.) * 1024.);

    // Deploy the bootloader to the tftp folder
    std::fs::rename(BOOTFILE,format!("bin/{}", BOOTFILE))?;

    Ok(())
}
