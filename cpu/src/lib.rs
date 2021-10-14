#![feature(asm)]
#![no_std]

#[inline]
pub fn hlt() -> !{
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}