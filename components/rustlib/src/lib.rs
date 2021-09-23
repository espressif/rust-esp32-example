#![cfg_attr(not(feature = "std"), no_std)]

#![feature(asm)]

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;


/// Create aliases for FFI types for esp32c3, which doesn't have std.
#[cfg(not(feature = "std"))]
mod ffi {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    pub type c_char = u8;
    pub type c_int = i32;
}

pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[no_mangle]
pub extern "C" fn add_in_rust(x: i32, y: i32) -> i32 {
    unsafe {
        sys::validate_param_in_c(0, x);
        sys::validate_param_in_c(1, y);
    }
    x + y
}

#[no_mangle]
pub extern "C" fn add_in_rust_inline_asm(mut x: i32, y: i32) -> i32 {
    let fma: f32 = 1.0;
    println!("{}", fma);
    let a = lerp(fma, 1.0, 2.0);
    println!("{}", a);
    

    unsafe {
        sys::validate_param_in_c(0, x);
        sys::validate_param_in_c(1, y);
    }
    unsafe {
        // more detail available: https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html
        asm!("add {0}, {0}, {1}", inout(reg) x, in(reg) y);
    }
    x
}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// this function emits a fma intrinsics as of 1.55 rust
pub fn lerp(s: f32, start: f32, end: f32) -> f32 {
    // consistent
    if start == end {
        start

    // exact/monotonic
    } else {
        s.mul_add(end, (-s).mul_add(start, start))
    }
}
