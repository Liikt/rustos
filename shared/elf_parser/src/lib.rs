use core::convert::TryInto;

const X86_MACHINE: u16 = 0x03;
const AME64_MACHINE: u16 = 0x3e;

const SECTION_EXEC: u32 = 0x01;
const SECTION_WRITE: u32 = 0x02;
const SECTION_READ: u32 = 0x04;

pub struct ElfParser<'a> {
    raw: &'a [u8],
    machine: u16,
    image_base: u64,
    num_sections: usize,
    section_off: usize,
    section_size: u16,
    pub entry_point: u64
} 

impl<'a> ElfParser<'a> {
    pub fn parse(raw: &'a [u8]) -> Option<Self> {
        let raw: &[u8] = raw.as_ref();

        // Return None if the magic bytes are missing
        if !(raw.get(0x00..0x04) == Some(b"\x7fELF")) { return None; }

        // Return None if the binary is big endian
        if !(raw.get(0x05) != Some(&1)) { return None; }

        let machine = 
            u16::from_le_bytes(raw.get(0x12..0x14)?.try_into().ok()?);
        if machine != X86_MACHINE && machine != AME64_MACHINE {
            return None;
        }

        let entry = if machine == X86_MACHINE {
            u32::from_le_bytes(raw.get(0x18..0x1c)?.try_into().ok()?) as u64
        } else {
            u64::from_le_bytes(raw.get(0x18..0x20)?.try_into().ok()?)
        };

        let num_sections = if machine == X86_MACHINE {
            u16::from_le_bytes(raw.get(0x2c..0x2e)?.try_into().ok()?) as usize
        } else {
            u64::from_le_bytes(raw.get(0x38..0x3a)?.try_into().ok()?) as usize
        };

        let phentsize = if machine == X86_MACHINE {
            u16::from_le_bytes(raw.get(0x2a..0x2c)?.try_into().ok()?)
        } else {
            u16::from_le_bytes(raw.get(0x36..0x38)?.try_into().ok()?)
        };

        let phoff = if machine == X86_MACHINE {
            u32::from_le_bytes(raw.get(0x1c..0x20)?.try_into().ok()?) as usize
        } else {
            u64::from_le_bytes(raw.get(0x20..0x28)?.try_into().ok()?) as usize
        };

        let mut base: u64 = !0;
        for i in 0..num_sections {
            let phbase = (i*phentsize as usize)+phoff;

            let t = u32::from_le_bytes(
                raw.get(phbase..phbase+0x04)?.try_into().ok()?);
            if t != 1 { continue; }

            let vaddr = if machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    phbase+0x08..phbase+0x0c)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    phbase+0x10..phbase+0x18)?.try_into().ok()?)
            };

            base = core::cmp::min(vaddr, base);
        }

        Some(ElfParser {
            raw: raw,
            machine: machine,
            image_base: base,
            num_sections: num_sections,
            section_off: phoff,
            section_size: phentsize,
            entry_point: entry
        })
    }

    pub fn sections<F>(&self, mut func: F) -> Option<()>
            where F: FnMut(u64, u64, &[u8], bool, bool, bool) -> Option<()> {
        let raw = self.raw;

        for section in 0..self.num_sections {
            let base = self.section_off + section*self.section_size as usize;

            let vaddr = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x08..base+0x0c)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x10..base+0x18)?.try_into().ok()?)
            };

            let vsize = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x14..base+0x18)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x28..base+0x30)?.try_into().ok()?)
            };

            let roff = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x04..base+0x08)?.try_into().ok()?) as usize
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x08..base+0x10)?.try_into().ok()?) as usize
            };

            let rsize = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x10..base+0x14)?.try_into().ok()?) as u64
            } else {
                u64::from_le_bytes(raw.get(
                    base+0x20..base+0x28)?.try_into().ok()?)
            };
            let rsize = core::cmp::min(rsize, vsize);

            let flags = if self.machine == X86_MACHINE {
                u32::from_le_bytes(raw.get(
                    base+0x18..base+0x1c)?.try_into().ok()?)
            } else {
                u32::from_le_bytes(raw.get(
                    base+0x04..base+0x08)?.try_into().ok()?)
            };

            func(
                self.image_base.checked_add(vaddr)?,
                vsize,
                raw.get(roff..roff.checked_add(rsize.try_into().ok()?)?)?,
                flags & SECTION_EXEC  != 0,
                flags & SECTION_WRITE != 0,
                flags & SECTION_READ  != 0
            )?;
        }

        Some(())
    }
}