use core::convert::TryInto;

// Magic number for a 32-bit compiled elf
const X86_MACHINE: u16 = 0x03;
// Magic number for a 64-bit compiled elf 
const AMD64_MACHINE: u16 = 0x3e;

// Bit that marks the section executable
const SECTION_EXEC: u32 = 0x01;
// Bit that marks the section writeable
const SECTION_WRITE: u32 = 0x02;
// Bit that marks the section readable
const SECTION_READ: u32 = 0x04;

/// The magic number for a LOAD section
pub const PTYPE_LOAD: u32 = 0x1;

/// Struct to hold the important information of an elf to flatten it later
pub struct ElfParser<'a> {
    // The raw bytes of the elf
    raw: &'a [u8],

    // The bitness of the elf
    machine: u16,

    /// The base virtual address of the elf
    pub image_base: u64,

    /// The end virtual address of the elf
    pub image_end: u64,

    // The number of physical sections
    num_sections: usize,

    // The offset of the physical section headers in the elf
    section_off: usize,

    // The size of the section headers
    section_size: u16,

    /// The entry point of the elf
    pub entry_point: u64
} 

impl<'a> ElfParser<'a> {
    /// Parse a blob of memory as an elf file and carve out the interesting 
    /// information
    pub fn parse(raw: &'a [u8]) -> Option<Self> {
        let raw: &[u8] = raw.as_ref();

        // Return None if the magic bytes are missing
        if raw.get(0x00..0x04) != Some(b"\x7fELF") { return None; }

        // Return None if the binary is big endian
        if raw.get(0x05) != Some(&1) { return None; }

        // Check if the elf is either x86 or x86-64
        let machine = 
            u16::from_le_bytes(raw.get(0x12..0x14)?.try_into().ok()?);
        if machine != X86_MACHINE && machine != AMD64_MACHINE {
            return None;
        }

        // Get the entry point of the elf
        let entry = if machine == X86_MACHINE {
            u32::from_le_bytes(raw.get(0x18..0x1c)?.try_into()
                .ok()?) as u64
        } else {
            u64::from_le_bytes(raw.get(0x18..0x20)?.try_into().ok()?)
        };

        // Get the number of physical headers
        let num_sections = if machine == X86_MACHINE {
            u16::from_le_bytes(raw.get(0x2c..0x2e)?.try_into()
                .ok()?) as usize
        } else {
            u16::from_le_bytes(raw.get(0x38..0x3a)?.try_into()
                .ok()?) as usize
        };

        // Get the size of a physical header
        let phentsize = if machine == X86_MACHINE {
            u16::from_le_bytes(raw.get(0x2a..0x2c)?.try_into().ok()?)
        } else {
            u16::from_le_bytes(raw.get(0x36..0x38)?.try_into().ok()?)
        };

        // Get the offset to the physical headers
        let phoff = if machine == X86_MACHINE {
            u32::from_le_bytes(raw.get(0x1c..0x20)?.try_into()
                .ok()?) as usize
        } else {
            u64::from_le_bytes(raw.get(0x20..0x28)?.try_into()
                .ok()?) as usize
        };

        let mut base: u64 = !0;
        let mut end: u64 = 0;

        // Calculate the base and the end of the image
        for i in 0..num_sections {
            let phbase = (i*phentsize as usize)+phoff;

            // Get the type of the current section
            let t = u32::from_le_bytes(
                raw.get(phbase..phbase+0x04)?.try_into().ok()?);

            // We are only interested in LOAD sections
            if t != PTYPE_LOAD { continue; }

            // Get the virtual address of the section
            let vaddr = if machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    phbase+0x08..phbase+0x0c)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    phbase+0x10..phbase+0x18)?.try_into().ok()?)
            };

            // Get the size of the section after loading
            let pmemsize = if machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    phbase+0x14..phbase+0x18)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    phbase+0x28..phbase+0x30)?.try_into().ok()?)
            };

            // Update base and end
            base = core::cmp::min(vaddr, base);
            end = core::cmp::max(end, vaddr.checked_add(pmemsize)?);
        }

        Some(ElfParser {
            raw: raw,
            machine: machine,
            image_base: base,
            image_end: end,
            num_sections: num_sections,
            section_off: phoff,
            section_size: phentsize,
            entry_point: entry
        })
    }

    /// Apply a closure to every section in the parsed elf. The closure is of
    /// type: `|base, size, type, raw, exec, write, read|`
    pub fn sections<F>(&self, mut func: F) -> Option<()>
            where F: FnMut(u64, u64, u32, &[u8], bool, bool, bool) -> Option<()>
    {
        let raw = self.raw;

        for section in 0..self.num_sections {
            let base = self.section_off + section*self.section_size as usize;

            // Get the virtual bas address of the section
            let vaddr = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x08..base+0x0c)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x10..base+0x18)?.try_into().ok()?)
            };

            // Get the size of the section
            let vsize = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x14..base+0x18)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x28..base+0x30)?.try_into().ok()?)
            };

            // Get the type of the section
            let ptype = u32::from_le_bytes(raw.get(base..base+0x4)?
                .try_into().ok()?);

            // Get the offset into the elf of the section
            let roff = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x04..base+0x08)?.try_into().ok()?) as usize
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x08..base+0x10)?.try_into().ok()?) as usize
            };

            // Get the size of the section in the elf
            let rsize = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x10..base+0x14)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x20..base+0x28)?.try_into().ok()?)
            };

            // Because the size of the section can be larger than the actual 
            // memory, we want to get the smaller of the two.
            let rsize = core::cmp::min(rsize, vsize);

            // Get the execution flags of the elf
            let flags = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x18..base+0x1c)?.try_into().ok()?)
            } else {
                u32::from_le_bytes(raw.get(
                    base+0x04..base+0x08)?.try_into().ok()?)
            };

            // Apply the closure
            func(
                self.image_base.checked_add(vaddr.checked_sub(self.image_base)
                    .map_or(0, |x| x))?,
                vsize,
                ptype,
                raw.get(roff..roff.checked_add(rsize.try_into().ok()?)?)?,
                flags & SECTION_EXEC  != 0,
                flags & SECTION_WRITE != 0,
                flags & SECTION_READ  != 0
            )?;
        }

        Some(())
    }
}