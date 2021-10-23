#![no_main]
#![no_std]

#![feature(rustc_private, lang_items)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use core_reqs;

/// System-wide panic handler
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn entry() -> ! {
    let _x = 42;
    panic!();
}