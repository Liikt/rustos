#![no_main]
#![no_std]

use core::panic::PanicInfo;

/// System-wide panic handler
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern fn entry() -> ! {
    panic!();
}