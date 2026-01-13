use redhook::{hook, real};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::sync::atomic::{AtomicUsize, Ordering};

// Include simplified telemetry
include!("simple_telemetry.rs");

// Global counters
static EXECVE_COUNT: AtomicUsize = AtomicUsize::new(0);
static INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

hook! {
    unsafe fn execve(
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char
    ) -> c_int => my_execve {
        let count = EXECVE_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        
        if !pathname.is_null() {
            let path_str = CStr::from_ptr(pathname).to_string_lossy();
            eprintln!("EXECVE[{}]: {}", count, path_str);
        }
        
        real!(execve)(pathname, argv, envp)
    }
}

#[ctor::ctor]
fn init() {
    let count = INIT_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    eprintln!("INIT[{}]:rust_preload_loaded", count);
    init_all_call_wrappers!();
}

#[ctor::dtor]
fn cleanup() {
    let execve_total = EXECVE_COUNT.load(Ordering::SeqCst);
    let init_total = INIT_COUNT.load(Ordering::SeqCst);
    eprintln!("📊 USAGE SUMMARY: init={}, execve={}", init_total, execve_total);
}
