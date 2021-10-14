#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// System-wide panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn entry() {
    loop {}
}