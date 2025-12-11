#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "debug")] {
        extern "C" {
            fn __debug_write(msg: *const u8, len: usize);
        }

        pub struct DebugWriter;

        impl core::fmt::Write for DebugWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                unsafe {
                    __debug_write(s.as_ptr(), s.len());
                }
                Ok(())
            }
        }
    }
}

#[cfg(feature = "debug")]
mod macros {
    #[macro_export]
    macro_rules! write {
        ($($arg:tt)*) => {{
            use core::fmt::Write;
            let _ = core::write!($crate::DebugWriter, $($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! writeln {
        ($($arg:tt)*) => {{
            use core::fmt::Write;
            let _ = core::writeln!($crate::DebugWriter, $($arg)*);
        }};
    }
}

#[cfg(not(feature = "debug"))]
mod macros {
    #[macro_export]
    macro_rules! write {
        ($($arg:tt)*) => {{}};
    }

    #[macro_export]
    macro_rules! writeln {
        ($($arg:tt)*) => {{}};
    }
}
