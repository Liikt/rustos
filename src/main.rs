use std::{path::Path, error::Error, process::Command};

use elf_parser::ElfParser;

const PTYPE_LOAD: u32 = 0x1;
const BOOTFILE: &str = "rustos.boot";

fn flatten_img<P: AsRef<Path>>(path: P) -> Option<(u32, u32, Vec<u8>)> {
    let img = std::fs::read(path).ok()?;
    let elf = ElfParser::parse(&img)?;
    let img_size: usize = elf.image_end.checked_sub(elf.image_base)?
        .checked_add(1)?.try_into().ok()?;
    let mut flat = std::vec![0u8; img_size];

    elf.sections(|base, vsize, ptype, raw, _, _, _| {
        if ptype != PTYPE_LOAD { return Some(()); }

        let size: usize = std::cmp::min(vsize.try_into().ok()?, raw.len());
        let flat_off: usize = (base - elf.image_base).try_into().ok()?;

        flat[flat_off..flat_off.checked_add(size)?].copy_from_slice(raw);

        Some(())
    })?;

    if elf.entry_point < elf.image_base || elf.entry_point > elf.image_end {
        return None;
    }

    println!("Entrypoint is sane");

    Some((
        elf.entry_point.try_into().ok()?,
        elf.image_base.try_into().ok()?,
        flat
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    if !Command::new("cargo").current_dir("bootloader")
            .args(&["build", "--release", "-Zbuild-std", "--target", "x86-64-bootloader.json"])
            .status()?.success() {
        panic!("Couldn't build bootloader!"); 
    }

    let (entry, _, flat) = 
        flatten_img("bootloader/target/x86-64-bootloader/release/bootloader")
        .ok_or("Couldn't flatten bootloader!")?;

    std::fs::write("bin/boot.flat", flat)?;

    let m = format!("-Dentry_point={:#x}", entry);
    let mut args = vec!["-f", "bin", &m, "-o",
        BOOTFILE, "bootloader/src/stage0.S"];

    if std::env::args().find(|x| x == "debug").is_some() {
        args.push("-Ddbg");
    }

    if !Command::new("nasm").args(args).status()?.success() {
        panic!("Couldn't assemble stage0");
    }

    let md = std::fs::metadata(BOOTFILE).expect("Couldn't get metadata from bootfile");
    assert!(md.is_file(), "Bootfile is not a file");

    println!("Bootloader size is {} bytes ({:8.4}%)",
        md.len(), md.len() as f64 / (32. * 1024.) * 1024.);

    assert!(md.len() <= (32 * 1024));

    std::fs::rename(BOOTFILE,format!("bin/{}", BOOTFILE))?;

    Ok(())
}
