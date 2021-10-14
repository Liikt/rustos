#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cpu;

/// System-wide panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::hlt();
}

#[no_mangle]
fn entry() {
    panic!();
}