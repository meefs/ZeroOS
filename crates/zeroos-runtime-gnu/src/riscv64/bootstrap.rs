use core::arch::naked_asm;

#[no_mangle]
pub extern "C" fn _init() {}

#[no_mangle]
pub extern "C" fn _fini() {}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn __runtime_bootstrap() -> ! {
    naked_asm!(
        // For now, just infinite loop to avoid undefined behavior
        "   j       .",
    )
}
