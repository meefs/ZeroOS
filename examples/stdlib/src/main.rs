#![cfg_attr(target_os = "none", no_std)]
#![no_main] // Always use bolt's custom entry point (both std and no_std)

extern crate alloc;
use alloc::string::{String, ToString};

cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use platform::println;
    } else {
        use std::println;

        #[allow(unused_imports)]
        use platform;
    }
}

fn int_to_string(n: i32) -> String {
    n.to_string()
}

/// Must call exit() to terminate.
#[no_mangle]
fn main() -> ! {
    let num = 42;
    let s = int_to_string(num);
    println!("int_to_string({}) = {}", num, s);
    platform::exit(0)
}
