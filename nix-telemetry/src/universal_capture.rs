// Universal Data Capture - Stream all function calls to Parquet with type info
// Uses safe print + structured logging for post-processing to Parquet

use std::sync::atomic::{AtomicU64, Ordering};
use std::fs::OpenOptions;
use std::io::Write;

mod safe_print;

static CALL_ID: AtomicU64 = AtomicU64::new(0);

// Structured log entry (CSV format, convert to Parquet later)
fn log_structured(
    function: &str,
    arg_types: &str,
    arg_values: &str,
    return_type: &str,
    return_value: &str,
) {
    let id = CALL_ID.fetch_add(1, Ordering::Relaxed);
    let timestamp = unsafe { libc::time(std::ptr::null_mut()) };
    
    // Write to structured log file
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/universal_capture.csv")
    {
        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            id, timestamp, function, arg_types, arg_values, return_type, return_value
        ).ok();
    }
}

// Macro to generate typed wrappers
macro_rules! wrap_typed {
    ($name:ident() -> $ret:ty) => {
        #[no_mangle]
        pub extern "C" fn $name() -> $ret {
            let result = unsafe { libc::$name() };
            log_structured(
                stringify!($name),
                "",
                "",
                stringify!($ret),
                &format!("{:?}", result),
            );
            result
        }
    };
    
    ($name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty) => {
        #[no_mangle]
        pub extern "C" fn $name($($arg: $arg_ty),*) -> $ret {
            let arg_types = stringify!($($arg_ty),*);
            let arg_values = format!($("{:?}",)* $($arg),*);
            
            let result = unsafe { libc::$name($($arg),*) };
            
            log_structured(
                stringify!($name),
                arg_types,
                &arg_values,
                stringify!($ret),
                &format!("{:?}", result),
            );
            
            result
        }
    };
}

// Universal wrappers with full type information
wrap_typed!(getpid() -> i32);
wrap_typed!(getuid() -> u32);
wrap_typed!(geteuid() -> u32);
wrap_typed!(getgid() -> u32);
wrap_typed!(getegid() -> u32);

// Typed I/O wrappers
#[no_mangle]
pub extern "C" fn open(path: *const i8, flags: i32, mode: u32) -> i32 {
    let path_str = if !path.is_null() {
        unsafe { std::ffi::CStr::from_ptr(path).to_string_lossy().into_owned() }
    } else {
        "NULL".to_string()
    };
    
    let result = unsafe { libc::open(path, flags, mode as libc::mode_t) };
    
    log_structured(
        "open",
        "*const i8, i32, u32",
        &format!("{}, {}, {}", path_str, flags, mode),
        "i32",
        &format!("{}", result),
    );
    
    result
}

#[no_mangle]
pub extern "C" fn close(fd: i32) -> i32 {
    let result = unsafe { libc::close(fd) };
    log_structured("close", "i32", &format!("{}", fd), "i32", &format!("{}", result));
    result
}

#[no_mangle]
pub extern "C" fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let result = unsafe { libc::read(fd, buf as *mut libc::c_void, count) };
    log_structured(
        "read",
        "i32, *mut u8, usize",
        &format!("{}, {:p}, {}", fd, buf, count),
        "isize",
        &format!("{}", result),
    );
    result
}

#[no_mangle]
pub extern "C" fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let result = unsafe { libc::write(fd, buf as *const libc::c_void, count) };
    log_structured(
        "write",
        "i32, *const u8, usize",
        &format!("{}, {:p}, {}", fd, buf, count),
        "isize",
        &format!("{}", result),
    );
    result
}

// Export stats
#[no_mangle]
pub extern "C" fn get_capture_count() -> u64 {
    CALL_ID.load(Ordering::Relaxed)
}
