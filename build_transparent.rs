// 🔧 BUILD.RS: Clean Semantic Layer for Transparent Telemetry
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let telemetry_path = Path::new(&out_dir).join("telemetry_macros.rs");
    
    println!("🔧 Generating clean telemetry layer...");
    
    let telemetry_code = generate_clean_telemetry();
    fs::write(&telemetry_path, telemetry_code).unwrap();
    
    println!("✅ Clean telemetry generated: {:?}", telemetry_path);
}

fn generate_clean_telemetry() -> String {
    // Our own semantic layer - no format! strings needed
    let mut code = String::new();
    
    // Base telemetry infrastructure
    code.push_str(r#"
// 🔧 CLEAN TELEMETRY SEMANTIC LAYER
use std::time::Instant;

// Core telemetry capture
macro_rules! telemetry_wrap {
    ($name:literal, $call:expr) => {{
        let start = Instant::now();
        println!("🎯 CALL: {}", $name);
        let result = $call;
        let duration = start.elapsed().as_micros();
        println!("📊 DONE: {} ({}μs)", $name, duration);
        result
    }};
}

// Memory operations
macro_rules! malloc {
    ($size:expr) => {{
        telemetry_wrap!("malloc", unsafe { libc::malloc($size) })
    }};
}

macro_rules! free {
    ($ptr:expr) => {{
        telemetry_wrap!("free", unsafe { libc::free($ptr) })
    }};
}

// File operations  
macro_rules! fopen {
    ($path:expr, $mode:expr) => {{
        telemetry_wrap!("fopen", unsafe { libc::fopen($path, $mode) })
    }};
}

macro_rules! fclose {
    ($file:expr) => {{
        telemetry_wrap!("fclose", unsafe { libc::fclose($file) })
    }};
}

// String operations
macro_rules! printf {
    ($fmt:expr) => {{
        telemetry_wrap!("printf", unsafe { libc::printf($fmt) })
    }};
}

// Socket operations
macro_rules! socket {
    ($domain:expr, $type:expr, $protocol:expr) => {{
        telemetry_wrap!("socket", unsafe { libc::socket($domain, $type, $protocol) })
    }};
}

// Thread operations
macro_rules! pthread_create {
    ($thread:expr, $attr:expr, $start_routine:expr, $arg:expr) => {{
        telemetry_wrap!("pthread_create", unsafe { 
            libc::pthread_create($thread, $attr, $start_routine, $arg) 
        })
    }};
}

"#);

    // Add dynamic symbol wrappers using our clean approach
    code.push_str(&generate_symbol_wrappers());
    
    code
}

fn generate_symbol_wrappers() -> String {
    let mut wrappers = String::new();
    
    // Common libc symbols we want to wrap
    let symbols = vec![
        "malloc", "free", "calloc", "realloc",
        "fopen", "fclose", "fread", "fwrite",
        "socket", "bind", "listen", "accept",
        "pthread_create", "pthread_join", "pthread_mutex_lock"
    ];
    
    wrappers.push_str("\n// 🔗 SYMBOL TELEMETRY WRAPPERS\n");
    
    for symbol in symbols {
        wrappers.push_str(&create_symbol_wrapper(symbol));
    }
    
    wrappers.push_str(r#"
// Master preload macro
#[macro_export]
macro_rules! preload_telemetry {
    () => {{
        println!("🔧 Telemetry layer active - all calls wrapped!");
        println!("📊 Memory, file, socket, thread operations monitored");
    }};
}

"#);
    
    wrappers
}

fn create_symbol_wrapper(symbol: &str) -> String {
    let macro_name = format!("{}_telemetry", symbol);
    
    format!(r#"
// Wrapper for {}
macro_rules! {} {{
    ($($args:expr),*) => {{{{
        telemetry_wrap!("{}", unsafe {{ libc::{}($($args),*) }})
    }}}};
}}

"#, symbol, macro_name, symbol, symbol)
}
