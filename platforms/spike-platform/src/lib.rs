#![cfg_attr(not(feature = "std"), no_std)]

mod arch;
mod boot;
pub mod runtime;

extern crate zeroos;

pub use arch::trap_handler;

#[no_mangle]
pub unsafe extern "C" fn __debug_write(msg: *const u8, len: usize) {
    if !msg.is_null() && len > 0 {
        let slice = core::slice::from_raw_parts(msg, len);
        for &byte in slice {
            htif::putchar(byte);
        }
    }
}

#[no_mangle]
pub extern "Rust" fn platform_exit(code: i32) -> ! {
    htif::exit(code as u32)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::{eprintln, println};

        pub fn exit(code: i32) -> ! {
            std::process::exit(code)
        }
    } else {
        pub use htif::{eprintln, println, putchar};

        pub fn exit(code: i32) -> ! {
            platform_exit(code)
        }

        #[cfg(feature = "memory")]
        #[global_allocator]
        static ALLOCATOR: zeroos::alloc::System = zeroos::alloc::System;

        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            eprintln!("PANIC: {}", info);
            exit(1)
        }
    }
}
