use redhook::{hook, real};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

// Include the generated macro wrappers
include!("latest_dev.rs");

hook! {
    unsafe fn execve(
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char
    ) -> c_int => my_execve {
        if !pathname.is_null() {
            let path_str = CStr::from_ptr(pathname).to_string_lossy();
            eprintln!("EXECVE:{}", path_str);
        }
        real!(execve)(pathname, argv, envp)
    }
}

#[ctor::ctor]
fn init() {
    eprintln!("INIT:rust_preload_loaded");
    // Initialize all the generated call wrappers
    init_all_call_wrappers!();
}
