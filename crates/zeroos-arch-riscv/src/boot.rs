//! RISC-V boot sequence (RV32/RV64)
//!
//! ## Boot Modes
//!
//! ### With `std` feature (Linux/musl targets):
//! - Builds musl-compatible stack with argc/argv/envp/auxv
//! - Calls `__libc_start_main` which then calls user's `main`
//!
//! ### Without `std` feature (bare-metal/zkVM targets):
//! - Uses boot stack directly (no musl stack building)
//! - Calls user's `main` directly with zero argc/argv
//!
//! ## Entry Point
//!
//! By default, uses simple `main()` with no arguments:
//! ```rust
//! #[no_mangle]
//! fn main() {
//!     println!("Hello!");
//! }
//! ```
//!
//! With `libc-main` feature, uses C-style `main(argc, argv)`:
//! ```c
//! int main(int argc, char **argv) {
//!     return 0;
//! }
//! ```

use core::arch::{global_asm, naked_asm};

#[unsafe(naked)]
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        ".weak __global_pointer$",
        ".hidden __global_pointer$",
        ".option push",
        ".option norelax",
        "   lla     gp, __global_pointer$",
        ".option pop",

        ".weak __stack_top",
        ".hidden __stack_top",
        "   lla     sp, __stack_top",
        "   andi    sp, sp, -16",

        "   call    {trace_start}",

        "   tail    {bootstrap}",

        trace_start = sym __boot_trace_start,
        bootstrap = sym __bootstrap,
    )
}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn __bootstrap() -> ! {
    naked_asm!(
        "   call    {trace_bootstrap}",

        "   call    {platform_bootstrap}",

        // Runtime environment initialization (never returns)
        "   tail    {runtime_bootstrap}",

        // Safety: If main() returns, halt forever.
        // User should call exit() to terminate properly.
        // Using inline asm instead of `loop {}` or `spin_loop()` because:
        // - `loop {}` may be optimized away as undefined behavior
        // - `spin_loop()` generates `pause` only with Zihintpause extension,
        //   otherwise no instruction on RISC-V
        // - `j .` guarantees a single-instruction infinite loop
        "   j       .",

        trace_bootstrap = sym __boot_trace_bootstrap,
        platform_bootstrap = sym crate::__platform_bootstrap,
        runtime_bootstrap = sym crate::__runtime_bootstrap,
    )
}

// Weak default platform bootstrap for no_std targets
// Platforms can override this with their own implementation
#[cfg(not(feature = "std"))]
global_asm!(
    ".weak __platform_bootstrap",
    "__platform_bootstrap:",
    "   ret",
);

#[no_mangle]
extern "C" fn __boot_trace_start() {
    debug::writeln!("[BOOT] _start");
}

#[no_mangle]
extern "C" fn __boot_trace_bootstrap() {
    debug::writeln!("[BOOT] __bootstrap");
}
