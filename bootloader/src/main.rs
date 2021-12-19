#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use cpu;

global_asm!(include_str!("stage0.S"));

/// System-wide panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::hlt();
}

#[no_mangle]
fn entry() {
    panic!();
}