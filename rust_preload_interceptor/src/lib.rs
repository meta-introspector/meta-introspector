use redhook::{hook, real};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);

fn get_log_file() -> std::fs::File {
    let mut file_opt = LOG_FILE.lock().unwrap();
    if file_opt.is_none() {
        let base_dir = "/mnt/data1/meta-introspector/data/telemetry";
        std::fs::create_dir_all(base_dir).ok();
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let project = std::env::var("PROJECT_NAME").unwrap_or_else(|_| "unknown".to_string());
        let filename = format!("{}/{}_{}.jsonl", base_dir, project, timestamp);
        
        *file_opt = Some(OpenOptions::new().create(true).append(true).open(filename).unwrap());
    }
    file_opt.as_ref().unwrap().try_clone().unwrap()
}

hook! {
    unsafe fn execve(
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char
    ) -> c_int => my_execve {
        if !pathname.is_null() {
            let path_str = CStr::from_ptr(pathname).to_string_lossy();
            let mut file = get_log_file();
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            writeln!(file, r#"{{"type":"execve","path":"{}","timestamp":{},"pid":{}}}"#, 
                    path_str, timestamp, std::process::id()).ok();
        }
        real!(execve)(pathname, argv, envp)
    }
}

hook! {
    unsafe fn malloc(size: libc::size_t) -> *mut libc::c_void => my_malloc {
        let result = real!(malloc)(size);
        let mut file = get_log_file();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        writeln!(file, r#"{{"type":"malloc","size":{},"ptr":"0x{:x}","timestamp":{},"pid":{}}}"#, 
                size, result as usize, timestamp, std::process::id()).ok();
        result
    }
}

hook! {
    unsafe fn fopen(
        pathname: *const c_char,
        mode: *const c_char
    ) -> *mut libc::FILE => my_fopen {
        let result = real!(fopen)(pathname, mode);
        if !pathname.is_null() {
            let path_str = CStr::from_ptr(pathname).to_string_lossy();
            let mut file = get_log_file();
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            writeln!(file, r#"{{"type":"fopen","filename":"{}","timestamp":{},"pid":{}}}"#, 
                    path_str, timestamp, std::process::id()).ok();
        }
        result
    }
}

#[ctor::ctor]
fn init() {
    let mut file = get_log_file();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    writeln!(file, r#"{{"type":"init","message":"Rust LD_PRELOAD loaded","timestamp":{},"pid":{}}}"#, 
            timestamp, std::process::id()).ok();
}
