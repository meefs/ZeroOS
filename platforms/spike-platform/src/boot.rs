#[allow(unused_imports)]
pub use htif::{fromhost, tohost};

extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
    static __stack_top: u8;
    static __stack_bottom: u8;
}

#[no_mangle]
pub extern "C" fn __platform_bootstrap() {
    debug::writeln!("[BOOT] __platform_bootstrap");

    zeroos::initialize();

    unsafe {
        #[cfg(feature = "memory")]
        {
            let heap_start = core::ptr::addr_of!(__heap_start) as usize;
            let heap_end = core::ptr::addr_of!(__heap_end) as usize;
            debug::writeln!("[BOOT] Heap start=0x{:x}, end=0x{:x}", heap_start, heap_end);
            let heap_size = heap_end - heap_start;

            foundation::kfn::memory::kinit(heap_start, heap_size);
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                core::arch::asm!("la      t0, _trap_handler", "csrw    mtvec, t0",);
                debug::writeln!("[BOOT] Trap handler installed");


                #[cfg(feature = "vfs")]
                {

                    #[cfg(feature = "vfs-device-console")]
                    {
                        debug::writeln!("[BOOT] Registering console file descriptors");
                        register_console_fd(1, &STDOUT_FOPS);
                        register_console_fd(2, &STDERR_FOPS);
                    }
                }

            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vfs-device-console")] {
        use zeroos::vfs::{self};

        fn htif_console_write(_file: *mut u8, buf: *const u8, count: usize) -> isize {
            debug::writeln!("[HTIF] htif_console_write called with count={}", count);
            unsafe {
                let slice = core::slice::from_raw_parts(buf, count);
                for &byte in slice {
                    htif::putchar(byte);
                }
            }
            count as isize
        }

        fn register_console_fd(fd: i32, ops: &'static vfs::FileOps) {
            debug::writeln!("[HTIF] register_console_fd fd={}", fd);
            let _ = vfs::register_fd(
                fd,
                vfs::FdEntry {
                    ops,
                    private_data: core::ptr::null_mut(),
                },
            );
        }

        static STDOUT_FOPS: vfs::FileOps =
        vfs::devices::console::stdout_fops(htif_console_write);
        static STDERR_FOPS: vfs::FileOps =
        vfs::devices::console::stderr_fops(htif_console_write);
    }
}
