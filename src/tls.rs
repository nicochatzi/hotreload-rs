pub const fn is_active() -> bool {
    true
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::*;
    use crate::once_cell::sync::Lazy;
    use cstr::cstr;
    use std::ffi::c_void;

    /// Signature of the desctrutor register function
    type DtorRegFn = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);

    static SYSTEM_THREAD_ATEXIT: Lazy<Option<DtorRegFn>> = Lazy::new(|| unsafe {
        std::mem::transmute(libc::dlsym(
            libc::RTLD_NEXT,
            #[allow(clippy::transmute_ptr_to_ref)]
            cstr!("__cxa_thread_atexit_impl").as_ptr(),
        ))
    });

    /// Turns glibc's TLS destructor register function, `__cxa_thread_atexit_impl`,
    /// into a no-op if hot reloading is enabled.
    ///
    /// # Safety
    /// This needs to be public for symbol visibility reasons, but you should
    /// never need to call this yourself
    pub unsafe fn thread_atexit(func: *mut c_void, obj: *mut c_void, dso_symbol: *mut c_void) {
        if super::is_active() {
            // avoid registering TLS destructors on purpose, to avoid
            // double-frees and general crashiness
        } else if let Some(system_thread_atexit) = *SYSTEM_THREAD_ATEXIT {
            // hot reloading is disabled, and system provides `__cxa_thread_atexit_impl`,
            // so forward the call to it.
            system_thread_atexit(func, obj, dso_symbol);
        } else {
            // hot reloading is disabled *and* we don't have `__cxa_thread_atexit_impl`,
            // throw hands up in the air and leak memory.
        }
    }
}
