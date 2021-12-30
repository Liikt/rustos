use elf_parser::ElfParser;

fn main() {
    let file = std::fs::read("target/debug/rustos").ok().unwrap();
    let parser = ElfParser::parse(&file);
    if parser.is_some() {
        println!("Parsed correctly");
    } else {
        println!("foop not a valid elf");
        panic!("blub");
    }
    let parser = parser.unwrap();
    parser.sections(|vaddr, vsize, raw, exec, write, read| {
        println!("Vaddr: {:x}", vaddr);
        println!("Vsize: {:x}", vsize);
        println!("Raw:   {:x?}", raw.get(0..10));
        println!("Exec:  {}", exec);
        println!("Write: {}", write);
        println!("Read:  {}", read);
        println!("===================");
        Some(())
    });
}
