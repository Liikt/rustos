#![no_main]
#![no_std]

use core::panic::PanicInfo;

use cpu;

/// System-wide panic handler
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern fn entry() -> ! {
    cpu::hlt();
}