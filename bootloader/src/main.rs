#![no_main]
#![no_std]
#![feature(global_asm)]

global_asm!(include_str!("stage0.S"));

use core::panic::PanicInfo;

// #[allow(unused_imports)]
// use core_reqs;

/// System-wide panic handler
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn entry() {
    panic!();
}

global_asm!(
    ".org 510",
    ".word 0xaa55"
);