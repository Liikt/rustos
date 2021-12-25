#![no_main]
#![no_std]
#![feature(global_asm)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use core_reqs;

global_asm!(include_str!("stage0.S"));

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