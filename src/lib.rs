#![deny(missing_debug_implementations, rust_2018_idioms, unused_imports)]

mod err;
mod file;

pub mod hot;
pub mod tls;

pub use lib;
pub use once_cell;

pub use macros::*;

#[macro_export]
macro_rules! register {
    ($($name: expr => $path: expr), +) => {
        (pub(crate) static HOTRELOAD_LIB_$name: $crate::once_cell::sync::Lazy<$crate::hot::HotLibrary> =
            $crate::once_cell::sync::Lazy::new(|| {
                $crate::hot::HotLibrary::new($path, stringify!($name))
            });), +

        #[cfg(target_os = "linux")]
        #[no_mangle]
        pub unsafe extern "C" fn __cxa_thread_atexit_impl(
            func: *mut std::ffi::c_void,
            obj: *mut std::ffi::c_void,
            dso_symbol: *mut std::ffi::c_void,
        ) {
            $crate::tls::linux::thread_atexit(func, obj, dso_symbol);
        }
    };
}
