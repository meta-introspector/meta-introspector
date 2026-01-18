// Safe print macros - raw syscalls, no libc, no recursion
use std::sync::atomic::{AtomicBool, Ordering};

// Recursion guard
static IN_SAFE_PRINT: AtomicBool = AtomicBool::new(false);

// Raw write syscall - bypasses all libc
fn raw_write(fd: i32, buf: &[u8]) {
    unsafe {
        libc::syscall(libc::SYS_write, fd, buf.as_ptr(), buf.len());
    }
}

// Safe print to stderr - no recursion
pub fn safe_eprint(msg: &str) {
    // Guard against recursion
    if IN_SAFE_PRINT.swap(true, Ordering::Relaxed) {
        return; // Already printing, skip
    }
    
    raw_write(2, msg.as_bytes());
    
    IN_SAFE_PRINT.store(false, Ordering::Relaxed);
}

// Safe print with newline
pub fn safe_eprintln(msg: &str) {
    safe_eprint(msg);
    raw_write(2, b"\n");
}

// Macro for formatted safe printing
#[macro_export]
macro_rules! safe_print {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::safe_eprint(&msg);
    }};
}

#[macro_export]
macro_rules! safe_println {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::safe_eprintln(&msg);
    }};
}
