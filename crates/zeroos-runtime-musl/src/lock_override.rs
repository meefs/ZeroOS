#[no_mangle]
#[inline(never)]
pub extern "C" fn __lock(_l: *mut i32) {}

#[no_mangle]
#[inline(never)]
pub extern "C" fn __unlock(_l: *mut i32) {}

#[no_mangle]
#[inline(never)]
pub extern "C" fn __lockfile(_f: *mut core::ffi::c_void) -> i32 {
    1
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn __unlockfile(_f: *mut core::ffi::c_void) {}

#[no_mangle]
pub extern "C" fn __wrap___lock(_l: *mut i32) {}

#[no_mangle]
pub extern "C" fn __wrap___unlock(_l: *mut i32) {}

#[no_mangle]
pub extern "C" fn __wrap___lockfile(_f: *mut core::ffi::c_void) -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn __wrap___unlockfile(_f: *mut core::ffi::c_void) {}
