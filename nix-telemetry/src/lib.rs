// Feature-gated telemetry - enable wrappers one at a time
use std::sync::atomic::{AtomicU64, Ordering};

mod safe_print;
use safe_print::{safe_eprintln};

static CALL_COUNT: AtomicU64 = AtomicU64::new(0);

fn log_call(name: &str) {
    let count = CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    
    // Safe logging using raw syscalls
    if count % 1000 == 0 {
        safe_eprintln(&format!("📊 {} calls: {}", name, count));
    }
}

// Feature: wrap_getpid (safest - no I/O, no allocation)
#[cfg(feature = "wrap_getpid")]
#[no_mangle]
pub extern "C" fn getpid() -> i32 {
    log_call("getpid");
    unsafe { libc::getpid() }
}

// Feature: wrap_getuid
#[cfg(feature = "wrap_getuid")]
#[no_mangle]
pub extern "C" fn getuid() -> u32 {
    log_call("getuid");
    unsafe { libc::getuid() }
}

// Feature: wrap_open
#[cfg(feature = "wrap_open")]
#[no_mangle]
pub extern "C" fn open(path: *const i8, flags: i32, mode: u32) -> i32 {
    log_call("open");
    unsafe { libc::open(path, flags, mode as libc::mode_t) }
}

// Feature: wrap_close
#[cfg(feature = "wrap_close")]
#[no_mangle]
pub extern "C" fn close(fd: i32) -> i32 {
    log_call("close");
    unsafe { libc::close(fd) }
}

// Feature: wrap_read
#[cfg(feature = "wrap_read")]
#[no_mangle]
pub extern "C" fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    log_call("read");
    unsafe { libc::read(fd, buf as *mut libc::c_void, count) }
}

// Feature: wrap_write
#[cfg(feature = "wrap_write")]
#[no_mangle]
pub extern "C" fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    log_call("write");
    unsafe { libc::write(fd, buf as *const libc::c_void, count) }
}

// Feature: wrap_malloc (DANGEROUS - causes recursion)
#[cfg(feature = "wrap_malloc")]
#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    log_call("malloc");
    unsafe { libc::malloc(size) as *mut u8 }
}

// Feature: wrap_free (DANGEROUS - causes recursion)
#[cfg(feature = "wrap_free")]
#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    log_call("free");
    unsafe { libc::free(ptr as *mut libc::c_void) };
}

// Export call count
#[no_mangle]
pub extern "C" fn get_telemetry_count() -> u64 {
    CALL_COUNT.load(Ordering::Relaxed)
}
