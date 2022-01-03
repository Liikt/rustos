#![no_main]
#![no_std]

use core::panic::PanicInfo;

/// System-wide panic handler
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The main entry point for the rust bootloader
#[no_mangle]
extern fn entry() -> ! {
    cpu::hlt();
}