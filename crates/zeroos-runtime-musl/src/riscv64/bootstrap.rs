use crate::build_musl_stack;
use core::arch::naked_asm;

use foundation::__main_entry;

extern "C" {
    fn __libc_start_main(
        main_fn: extern "C" fn(i32, *const *const u8, *const *const u8) -> i32,
        argc: i32,
        argv: *const *const u8,
        init: extern "C" fn(),
        fini: extern "C" fn(),
        ldso_dummy: Option<extern "C" fn()>,
    ) -> i32;

    static __ehdr_start: u8;
}

#[no_mangle]
pub extern "C" fn _init() {}

#[no_mangle]
pub extern "C" fn _fini() {}

static PROGRAM_NAME: &[u8] = b"zerokernel\0";

#[no_mangle]
extern "C" fn __boot_trace_runtime() {
    debug::writeln!("[BOOT] __runtime_bootstrap");
}

#[no_mangle]
extern "C" fn __boot_trace_switched(sp: usize) {
    debug::writeln!(
        "[BOOT] Switched to musl stack, SP=0x{:x} aligned={}",
        sp,
        sp % 16 == 0
    );
}

#[no_mangle]
extern "C" fn __boot_trace_argc(argc: usize) {
    debug::writeln!("[BOOT] argc={}", argc);
}

#[no_mangle]
extern "C" fn __boot_trace_before_musl() {
    debug::writeln!("[BOOT] __libc_start_main");
}

const MUSL_BUFFER_SIZE: usize = 512;
const MUSL_BUFFER_BYTES: usize = MUSL_BUFFER_SIZE * core::mem::size_of::<usize>();

static mut MUSL_BUILD_BUFFER: [usize; MUSL_BUFFER_SIZE] = [0; MUSL_BUFFER_SIZE];

unsafe fn build_musl_in_buffer() -> usize {
    let buffer_ptr = core::ptr::addr_of_mut!(MUSL_BUILD_BUFFER) as *mut usize;
    let buffer_bottom = buffer_ptr as usize;
    let buffer_top = buffer_ptr.add(MUSL_BUFFER_SIZE) as usize;
    let ehdr_start = core::ptr::addr_of!(__ehdr_start) as usize;

    let size = build_musl_stack(buffer_top, buffer_bottom, ehdr_start, PROGRAM_NAME);

    if size > MUSL_BUFFER_BYTES {
        panic!(
            "Musl stack overflow! Used {} bytes, buffer is {} bytes",
            size, MUSL_BUFFER_BYTES
        );
    }

    size
}
/// - Never returns: Tail-calls into musl's __libc_start_main
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn __runtime_bootstrap() -> ! {
    naked_asm!(
        "   call    {trace_runtime}",

        "   call    {build_impl}",

        "   mv      t2, a0",             // t2 = size to copy

        "   la      t0, {buffer}",       // t0 = buffer start
        "   li      t1, {buffer_bytes}", // t1 = buffer size in bytes
        "   add     t0, t0, t1",         // t0 = buffer_top
        "   sub     t6, t0, t2",         // t6 = buffer_top - size = src

        "   addi    sp, sp, -512",         // HACK: sp = sp - 512
        "   sub     t3, sp, t2",         // t3 = sp - size = dst (new SP)
        "   mv      t5, t3",             // t5 = save new SP for later

        "1:",
        "   beqz    t2, 2f",             // if size == 0, done
        "   ld      t1, 0(t6)",          // load 8 bytes from src
        "   sd      t1, 0(t3)",          // store 8 bytes to dst
        "   addi    t6, t6, 8",          // src += 8
        "   addi    t3, t3, 8",          // dst += 8
        "   addi    t2, t2, -8",         // size -= 8
        "   j       1b",                 // loop
        "2:",

        "   mv      sp, t5",             // sp = new SP (start of copied stack)

        "   la      a0, {main}",         // a0 = main function
        "   ld      a1, 0(sp)",          // a1 = argc (int, 64-bit)
        "   addi    a2, sp, 8",          // a2 = argv (char**)
        "   la      a3, {init}",         // a3 = _init
        "   la      a4, {fini}",         // a4 = _fini
        "   li      a5, 0",              // a5 = NULL (ldso_dummy)

        // Tail call __libc_start_main (never returns)
        "   tail    {libc_start_main}",

        trace_runtime = sym __boot_trace_runtime,
        build_impl = sym build_musl_in_buffer,
        buffer = sym MUSL_BUILD_BUFFER,
        buffer_bytes = const MUSL_BUFFER_BYTES,
        main = sym __main_entry,
        init = sym _init,
        fini = sym _fini,
        libc_start_main = sym __libc_start_main,
    )
}
