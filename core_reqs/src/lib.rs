#![no_std]
#![feature(asm)]
#![feature(rustc_private)]

extern crate compiler_builtins;

#[inline(always)]
#[cfg(target_arch = "x86")]
unsafe fn memcpy_int(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!(
        "mov esi, {0}",
        "rep movsb",
        inout(reg)   src  => _,
        inout("ecx") n    => _,
        inout("edi") dest => _,
    );

    dest
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn memcpy_int(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!("rep movsb",
        inout("rcx") n    => _,
        inout("rdi") dest => _,
        inout("rsi") src  => _,
    );

    dest
}

#[inline(always)]
#[cfg(target_arch = "x86")]
unsafe fn memset_int(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!("rep stosb",
        inout("ecx") n => _,
        inout("edi") s => _,
        in("eax")    c as u32,
    );

    s
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn memset_int(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!("rep stosb",
        inout("rcx") n => _,
        inout("rdi") s => _,
        in("eax")    c as u32,
    );

    s
}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    memcpy_int(dest, src, n)
}

#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    memset_int(s, c, n)
}

#[no_mangle]
pub unsafe extern fn memcmp(l: *const u8, r: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = l.offset(i as isize);
        let b = r.offset(i as isize);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
    }

    0
}

/// Perform n % d
#[export_name="\x01__aullrem"]
pub extern "stdcall" fn __aullrem(n: u64, d: u64) -> u64 {
    compiler_builtins::int::udiv::__umoddi3(n, d)
}

/// Perform n / d
#[export_name="\x01__aulldiv"]
pub extern "stdcall" fn __aulldiv(n: u64, d: u64) -> u64 {
    compiler_builtins::int::udiv::__udivdi3(n, d)
}

/// Whether or not floats are used. This is used by the MSVC calling convention
/// and it just has to exist.
#[export_name="_fltused"]
pub static FLTUSED: usize = 0;