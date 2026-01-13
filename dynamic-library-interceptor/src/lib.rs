use std::collections::HashMap;
use std::ffi::{CStr, CString, OsStr};
use std::fs;
use std::os::raw::{c_char, c_void, c_int};
use std::path::Path;
use std::sync::{Mutex, Once};
use libc::{dlopen, dlsym, dlclose, RTLD_LAZY, RTLD_NOW};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicCall {
    pub library: String,
    pub symbol: String,
    pub timestamp: u64,
    pub thread_id: u64,
    pub args_raw: Vec<u64>,
    pub return_value: Option<u64>,
    pub call_depth: u32,
    pub duration_ns: Option<u64>,
}

#[derive(Debug)]
pub struct LibraryWrapper {
    pub name: String,
    pub handle: *mut c_void,
    pub symbols: HashMap<String, *mut c_void>,
    pub call_log: Vec<DynamicCall>,
}

static mut WRAPPED_LIBRARIES: Option<Mutex<HashMap<String, LibraryWrapper>>> = None;
static INIT: Once = Once::new();

fn init_wrapper_system() {
    INIT.call_once(|| unsafe {
        WRAPPED_LIBRARIES = Some(Mutex::new(HashMap::new()));
    });
}

pub fn wrap_library(library_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    init_wrapper_system();
    
    let lib_name = Path::new(library_path)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("unknown")
        .to_string();
    
    eprintln!("🔗 Wrapping library: {}", lib_name);
    
    // Load the library
    let c_path = CString::new(library_path)?;
    let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW) };
    
    if handle.is_null() {
        return Err(format!("Failed to load library: {}", library_path).into());
    }
    
    // Extract symbols using nm or objdump
    let symbols = extract_library_symbols(library_path)?;
    
    let mut symbol_map = HashMap::new();
    for symbol in &symbols {
        let c_symbol = CString::new(symbol.as_str())?;
        let sym_ptr = unsafe { dlsym(handle, c_symbol.as_ptr()) };
        if !sym_ptr.is_null() {
            symbol_map.insert(symbol.clone(), sym_ptr);
            eprintln!("  📍 Found symbol: {}", symbol);
        }
    }
    
    let wrapper = LibraryWrapper {
        name: lib_name.clone(),
        handle,
        symbols: symbol_map,
        call_log: Vec::new(),
    };
    
    unsafe {
        if let Some(ref libs) = WRAPPED_LIBRARIES {
            if let Ok(mut libs_guard) = libs.lock() {
                libs_guard.insert(lib_name, wrapper);
            }
        }
    }
    
    Ok(())
}

fn extract_library_symbols(library_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // Try nm first, then objdump as fallback
    let output = Command::new("nm")
        .args(&["-D", "--defined-only", library_path])
        .output();
    
    let symbols = if let Ok(nm_output) = output {
        String::from_utf8_lossy(&nm_output.stdout)
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && (parts[1] == "T" || parts[1] == "t") {
                    Some(parts[2].to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        // Fallback to objdump
        let objdump_output = Command::new("objdump")
            .args(&["-T", library_path])
            .output()?;
        
        String::from_utf8_lossy(&objdump_output.stdout)
            .lines()
            .filter_map(|line| {
                if line.contains("DF") && line.contains(".text") {
                    line.split_whitespace().last().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect()
    };
    
    Ok(symbols)
}

// Macro to generate wrapper functions dynamically
macro_rules! generate_wrapper {
    ($lib_name:expr, $symbol_name:expr, $fn_type:ty) => {{
        let wrapper_fn = move |args: &[u64]| -> u64 {
            let start_time = get_timestamp();
            
            // Log the call
            let call = DynamicCall {
                library: $lib_name.to_string(),
                symbol: $symbol_name.to_string(),
                timestamp: start_time,
                thread_id: get_thread_id(),
                args_raw: args.to_vec(),
                return_value: None,
                call_depth: get_call_depth(),
                duration_ns: None,
            };
            
            // Call original function (this is simplified - real implementation would need proper type handling)
            let result = 0u64; // Placeholder
            
            let end_time = get_timestamp();
            let duration = end_time - start_time;
            
            // Update call log
            log_dynamic_call(call, Some(result), Some(duration));
            
            result
        };
        
        wrapper_fn
    }};
}

pub fn intercept_rustc_driver() -> Result<(), Box<dyn std::error::Error>> {
    // Find rustc_driver.so in the system
    let possible_paths = vec![
        "/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver.so",
        "/usr/local/lib/librustc_driver.so",
        "./target/debug/deps/librustc_driver.so",
        "./librustc_driver.so",
    ];
    
    for path in possible_paths {
        if Path::new(path).exists() {
            eprintln!("🎯 Found rustc_driver at: {}", path);
            return wrap_library(path);
        }
    }
    
    // Try to find it dynamically
    let find_output = std::process::Command::new("find")
        .args(&["/", "-name", "librustc_driver*.so", "-type", "f"])
        .output();
    
    if let Ok(output) = find_output {
        let paths = String::from_utf8_lossy(&output.stdout);
        for path in paths.lines().take(1) {
            if !path.is_empty() {
                eprintln!("🔍 Found rustc_driver via find: {}", path);
                return wrap_library(path);
            }
        }
    }
    
    Err("Could not find rustc_driver.so".into())
}

fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn get_thread_id() -> u64 {
    unsafe { libc::pthread_self() as u64 }
}

fn get_call_depth() -> u32 {
    // Simplified - could use backtrace crate for real depth
    0
}

fn log_dynamic_call(mut call: DynamicCall, return_value: Option<u64>, duration: Option<u64>) {
    call.return_value = return_value;
    call.duration_ns = duration;
    
    // Write to telemetry log
    let log_entry = serde_json::to_string(&call).unwrap_or_default();
    
    let log_file = std::env::var("DYNAMIC_INTERCEPT_LOG")
        .unwrap_or_else(|_| "/tmp/dynamic_intercept.jsonl".to_string());
    
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file) {
        use std::io::Write;
        let _ = writeln!(file, "{}", log_entry);
    }
}

// Export C-compatible functions for LD_PRELOAD
#[no_mangle]
pub extern "C" fn init_dynamic_wrapper() {
    eprintln!("🚀 Dynamic Library Wrapper Initialized");
    
    // Auto-detect and wrap common Rust libraries
    let _ = intercept_rustc_driver();
    
    // Try to wrap other common libraries
    let common_libs = vec![
        "libstd.so",
        "libcore.so", 
        "libproc_macro.so",
        "libsyn.so",
    ];
    
    for lib in common_libs {
        if let Ok(_) = wrap_library(lib) {
            eprintln!("✅ Wrapped: {}", lib);
        }
    }
}

// Constructor to auto-initialize when loaded
#[no_mangle]
pub extern "C" fn _init() {
    init_dynamic_wrapper();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_extraction() {
        // Test with a known library
        if let Ok(symbols) = extract_library_symbols("/lib/x86_64-linux-gnu/libc.so.6") {
            assert!(!symbols.is_empty());
            println!("Found {} symbols in libc", symbols.len());
        }
    }
}
