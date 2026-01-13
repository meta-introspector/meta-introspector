use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::sync::atomic::{AtomicU64, Ordering};
use std::fs::OpenOptions;
use std::io::Write;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

// Global counters and state
lazy_static! {
    static ref CALL_COUNTER: AtomicU64 = AtomicU64::new(0);
    static ref LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
    static ref SESSION_ID: String = uuid::Uuid::new_v4().to_string();
}

// Telemetry event structure
#[derive(serde::Serialize)]
struct LibcEvent {
    event_id: u64,
    session_id: String,
    timestamp: f64,
    pid: u32,
    tid: u64,
    function: String,
    args: serde_json::Value,
    result: Option<serde_json::Value>,
    errno: Option<i32>,
    duration_ns: Option<u64>,
}

// Initialize logging
fn init_logging() {
    let mut log_file = LOG_FILE.lock();
    if log_file.is_none() {
        let log_path = std::env::var("LIBC_TELEMETRY_LOG")
            .unwrap_or_else(|_| format!("/tmp/libc_telemetry_{}.jsonl", 
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
        
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(&log_path) {
            *log_file = Some(file);
            eprintln!("🔍 LibC Interceptor Active - Session: {}", *SESSION_ID);
            eprintln!("📊 Logging to: {}", log_path);
        }
    }
}

// Log a libc event
fn log_event(event: &LibcEvent) {
    let mut log_file = LOG_FILE.lock();
    if let Some(ref mut file) = *log_file {
        if let Ok(json_str) = serde_json::to_string(event) {
            let _ = writeln!(file, "{}", json_str);
            let _ = file.flush();
        }
    }
}

// Get original function pointer
macro_rules! get_original {
    ($name:expr) => {{
        static mut ORIGINAL: Option<*const c_void> = None;
        static INIT: std::sync::Once = std::sync::Once::new();
        
        INIT.call_once(|| unsafe {
            ORIGINAL = Some(libc::dlsym(libc::RTLD_NEXT, concat!($name, "\0").as_ptr() as *const c_char));
        });
        
        unsafe { ORIGINAL.unwrap() }
    }};
}

// Macro to create interceptor functions
macro_rules! intercept_function {
    ($name:ident, $ret_type:ty, $($arg:ident: $arg_type:ty),*) => {
        #[no_mangle]
        pub extern "C" fn $name($($arg: $arg_type),*) -> $ret_type {
            init_logging();
            
            let event_id = CALL_COUNTER.fetch_add(1, Ordering::SeqCst);
            let start_time = SystemTime::now();
            let timestamp = start_time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            
            // Capture arguments
            let args = json!({
                $(stringify!($arg): format!("{:?}", $arg)),*
            });
            
            // Call original function
            let original: extern "C" fn($($arg_type),*) -> $ret_type = unsafe {
                std::mem::transmute(get_original!(stringify!($name)))
            };
            
            let result = original($($arg),*);
            let duration = start_time.elapsed().unwrap().as_nanos() as u64;
            
            // Capture result and errno
            let errno = unsafe { *libc::__errno_location() };
            
            let event = LibcEvent {
                event_id,
                session_id: SESSION_ID.clone(),
                timestamp,
                pid: unsafe { libc::getpid() as u32 },
                tid: unsafe { libc::pthread_self() },
                function: stringify!($name).to_string(),
                args,
                result: Some(json!(format!("{:?}", result))),
                errno: if errno != 0 { Some(errno) } else { None },
                duration_ns: Some(duration),
            };
            
            log_event(&event);
            result
        }
    };
}

// File I/O interceptors
intercept_function!(open, c_int, pathname: *const c_char, flags: c_int);
intercept_function!(close, c_int, fd: c_int);
intercept_function!(read, isize, fd: c_int, buf: *mut c_void, count: usize);
intercept_function!(write, isize, fd: c_int, buf: *const c_void, count: usize);

// Process management interceptors
intercept_function!(fork, libc::pid_t);
intercept_function!(execve, c_int, pathname: *const c_char, argv: *const *const c_char, envp: *const *const c_char);
intercept_function!(waitpid, libc::pid_t, pid: libc::pid_t, status: *mut c_int, options: c_int);

// Memory management interceptors
intercept_function!(malloc, *mut c_void, size: usize);
intercept_function!(free, (), ptr: *mut c_void);
intercept_function!(mmap, *mut c_void, addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: libc::off_t);

// Network interceptors
intercept_function!(socket, c_int, domain: c_int, type_: c_int, protocol: c_int);
intercept_function!(connect, c_int, sockfd: c_int, addr: *const libc::sockaddr, addrlen: libc::socklen_t);

// Special interceptors with custom logic
#[no_mangle]
pub extern "C" fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int {
    init_logging();
    
    let event_id = CALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let start_time = SystemTime::now();
    let timestamp = start_time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    
    // Safely convert pathname to string
    let path_str = unsafe {
        if pathname.is_null() {
            "NULL".to_string()
        } else {
            CStr::from_ptr(pathname).to_string_lossy().to_string()
        }
    };
    
    let args = json!({
        "dirfd": dirfd,
        "pathname": path_str,
        "flags": flags
    });
    
    // Call original openat
    let original: extern "C" fn(c_int, *const c_char, c_int) -> c_int = unsafe {
        std::mem::transmute(get_original!("openat"))
    };
    
    let result = original(dirfd, pathname, flags);
    let duration = start_time.elapsed().unwrap().as_nanos() as u64;
    let errno = unsafe { *libc::__errno_location() };
    
    let event = LibcEvent {
        event_id,
        session_id: SESSION_ID.clone(),
        timestamp,
        pid: unsafe { libc::getpid() as u32 },
        tid: unsafe { libc::pthread_self() },
        function: "openat".to_string(),
        args,
        result: Some(json!(result)),
        errno: if errno != 0 { Some(errno) } else { None },
        duration_ns: Some(duration),
    };
    
    log_event(&event);
    result
}

// System call interceptor using ptrace-like approach
#[no_mangle]
pub extern "C" fn syscall(number: libc::c_long, mut args: ...) -> libc::c_long {
    init_logging();
    
    let event_id = CALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let start_time = SystemTime::now();
    let timestamp = start_time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    
    // Capture syscall number and basic info
    let syscall_args = json!({
        "syscall_number": number,
        "syscall_name": get_syscall_name(number)
    });
    
    // Call original syscall
    let original: extern "C" fn(libc::c_long, ...) -> libc::c_long = unsafe {
        std::mem::transmute(get_original!("syscall"))
    };
    
    let result = unsafe { original(number, args) };
    let duration = start_time.elapsed().unwrap().as_nanos() as u64;
    let errno = unsafe { *libc::__errno_location() };
    
    let event = LibcEvent {
        event_id,
        session_id: SESSION_ID.clone(),
        timestamp,
        pid: unsafe { libc::getpid() as u32 },
        tid: unsafe { libc::pthread_self() },
        function: "syscall".to_string(),
        args: syscall_args,
        result: Some(json!(result)),
        errno: if errno != 0 { Some(errno) } else { None },
        duration_ns: Some(duration),
    };
    
    log_event(&event);
    result
}

// Helper function to get syscall name
fn get_syscall_name(number: libc::c_long) -> &'static str {
    match number {
        libc::SYS_read => "read",
        libc::SYS_write => "write",
        libc::SYS_open => "open",
        libc::SYS_close => "close",
        libc::SYS_fork => "fork",
        libc::SYS_execve => "execve",
        libc::SYS_mmap => "mmap",
        libc::SYS_socket => "socket",
        _ => "unknown",
    }
}

// Constructor to initialize when library is loaded
#[no_mangle]
pub extern "C" fn _init() {
    init_logging();
    
    let event = LibcEvent {
        event_id: 0,
        session_id: SESSION_ID.clone(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
        pid: unsafe { libc::getpid() as u32 },
        tid: unsafe { libc::pthread_self() },
        function: "library_init".to_string(),
        args: json!({"message": "LibC interceptor loaded"}),
        result: None,
        errno: None,
        duration_ns: None,
    };
    
    log_event(&event);
}
