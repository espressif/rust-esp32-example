// #![no_std]

use core::panic::PanicInfo;

pub mod ffi {
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

    use crate::ffi;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use crate::sys::*;


// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

#[no_mangle]
pub extern "C" fn add_in_rust(x: i32, y: i32) -> i32 {

    extern "C" {
        fn validate_param_in_c(param: i32, value: i32) -> i32;
    }

    unsafe {
        validate_param_in_c(0, x);
        validate_param_in_c(2, y);
    }
    x + y
}
