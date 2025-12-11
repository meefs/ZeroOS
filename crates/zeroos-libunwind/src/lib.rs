//! These symbols are required by Rust's std::backtrace module even with panic=abort.
//! Required by musl-based runtimes using std.

#![no_std]

#[no_mangle]
pub extern "C" fn _Unwind_Resume(_exception: *mut u8) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _Unwind_Backtrace(
    _trace_fn: extern "C" fn(*mut u8, *mut u8) -> i32,
    _trace_argument: *mut u8,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetIP(_context: *mut u8) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetIPInfo(_context: *mut u8, _ip_before_insn: *mut i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetCFA(_context: *mut u8) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetLanguageSpecificData(_context: *mut u8) -> *mut u8 {
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn _Unwind_GetRegionStart(_context: *mut u8) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetTextRelBase(_context: *mut u8) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetDataRelBase(_context: *mut u8) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_SetGR(_context: *mut u8, _index: i32, _value: usize) {}

#[no_mangle]
pub extern "C" fn _Unwind_SetIP(_context: *mut u8, _value: usize) {}

#[no_mangle]
pub extern "C" fn _Unwind_FindEnclosingFunction(_pc: *mut u8) -> *mut u8 {
    core::ptr::null_mut()
}
