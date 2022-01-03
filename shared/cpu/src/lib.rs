#![no_std]

use core::arch::asm;

/// Halt the CPU indefinitely
#[inline]
pub fn hlt() -> !{
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}