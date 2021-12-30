#![feature(asm)]
#![no_std]

use core::arch::asm;

#[inline]
pub fn hlt() -> !{
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}