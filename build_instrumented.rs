// build.rs - Compile-time function instrumentation using LMFDB decoder
// Inspect function bytes, decode arguments, generate wrappers

use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("🔬 Compile-time function instrumentation");
    
    // Load LMFDB catalog
    let catalog = load_lmfdb_catalog();
    println!("✅ Loaded {} function patterns", catalog.len());
    
    // Find target libraries
    let libs = vec![
        "/lib/x86_64-linux-gnu/libc.so.6",
        "/lib/x86_64-linux-gnu/libpthread.so.0",
    ];
    
    let mut wrappers = String::new();
    wrappers.push_str("// Auto-generated instrumented wrappers\n\n");
    wrappers.push_str("use libc;\n");
    wrappers.push_str("use std::sync::atomic::{AtomicU64, Ordering};\n\n");
    wrappers.push_str("static CALL_COUNT: AtomicU64 = AtomicU64::new(0);\n\n");
    
    let mut total_wrapped = 0;
    
    for lib_path in &libs {
        if let Ok(functions) = inspect_library(lib_path) {
            println!("📦 {} - {} functions", lib_path, functions.len());
            
            for (name, pattern, args) in functions.iter().take(20) {
                if is_wrappable(name) {
                    let wrapper = generate_wrapper(name, pattern, args);
                    wrappers.push_str(&wrapper);
                    total_wrapped += 1;
                }
            }
        }
    }
    
    println!("✅ Generated {} instrumented wrappers", total_wrapped);
    
    // Write to output
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("instrumented_wrappers.rs");
    fs::write(&dest_path, wrappers).unwrap();
    
    println!("💾 Saved to: {:?}", dest_path);
}

fn load_lmfdb_catalog() -> HashMap<String, String> {
    // Load from our Parquet catalog
    // For now, return empty - full implementation would query Parquet
    HashMap::new()
}

fn inspect_library(path: &str) -> Result<Vec<(String, Vec<u8>, Vec<String>)>, Box<dyn std::error::Error>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut functions = Vec::new();
    
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let func_bytes = buffer[offset..offset + size.min(32)].to_vec();
                    let args = decode_arguments(&func_bytes);
                    functions.push((name.to_string(), func_bytes, args));
                }
            }
        }
    }
    
    Ok(functions)
}

fn decode_arguments(bytes: &[u8]) -> Vec<String> {
    let mut args = Vec::new();
    
    // Decode based on pattern
    if bytes.starts_with(&[0x48, 0x89]) {
        // mov instruction - 2 register args
        let dst = decode_register((bytes.get(2).unwrap_or(&0) >> 3) & 0x7);
        let src = decode_register(bytes.get(2).unwrap_or(&0) & 0x7);
        args.push(format!("reg_{}", dst));
        args.push(format!("reg_{}", src));
    } else if bytes.starts_with(&[0x48, 0x8b]) {
        // mov load - register + memory
        let dst = decode_register((bytes.get(2).unwrap_or(&0) >> 3) & 0x7);
        let src = decode_register(bytes.get(2).unwrap_or(&0) & 0x7);
        args.push(format!("reg_{}", dst));
        args.push(format!("mem_{}", src));
    } else if bytes.starts_with(&[0x53]) {
        // push rbx
        args.push("reg_rbx".to_string());
    }
    
    args
}

fn decode_register(reg_bits: u8) -> &'static str {
    match reg_bits & 0x7 {
        0 => "rax",
        1 => "rcx",
        2 => "rdx",
        3 => "rbx",
        4 => "rsp",
        5 => "rbp",
        6 => "rsi",
        7 => "rdi",
        _ => "unknown",
    }
}

fn is_wrappable(name: &str) -> bool {
    // Only wrap common functions
    matches!(name, "malloc" | "free" | "open" | "close" | "read" | "write" | "getpid" | "getuid")
}

fn generate_wrapper(name: &str, pattern: &[u8], args: &[String]) -> String {
    let mut wrapper = String::new();
    
    // Generate wrapper based on function signature
    let (ret_type, params) = match name {
        "malloc" => ("*mut libc::c_void", vec![("size", "libc::size_t")]),
        "free" => ("()", vec![("ptr", "*mut libc::c_void")]),
        "open" => ("libc::c_int", vec![("path", "*const libc::c_char"), ("flags", "libc::c_int")]),
        "close" => ("libc::c_int", vec![("fd", "libc::c_int")]),
        "read" => ("libc::ssize_t", vec![("fd", "libc::c_int"), ("buf", "*mut libc::c_void"), ("count", "libc::size_t")]),
        "write" => ("libc::ssize_t", vec![("fd", "libc::c_int"), ("buf", "*const libc::c_void"), ("count", "libc::size_t")]),
        "getpid" => ("libc::pid_t", vec![]),
        "getuid" => ("libc::uid_t", vec![]),
        _ => return String::new(),
    };
    
    // Function signature
    wrapper.push_str(&format!("#[no_mangle]\npub unsafe extern \"C\" fn {}(", name));
    for (i, (pname, ptype)) in params.iter().enumerate() {
        if i > 0 { wrapper.push_str(", "); }
        wrapper.push_str(&format!("{}: {}", pname, ptype));
    }
    wrapper.push_str(&format!(") -> {} {{\n", ret_type));
    
    // Instrumentation
    wrapper.push_str("    CALL_COUNT.fetch_add(1, Ordering::Relaxed);\n");
    
    // Log pattern info
    let pattern_hex: String = pattern.iter().take(8)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ");
    
    wrapper.push_str(&format!("    // LMFDB pattern: {}\n", pattern_hex));
    wrapper.push_str(&format!("    // Decoded args: {:?}\n", args));
    
    // Call real function via dlsym
    wrapper.push_str(&format!("    let real_fn = libc::dlsym(libc::RTLD_NEXT, b\"{}\\0\".as_ptr() as *const _);\n", name));
    wrapper.push_str("    if real_fn.is_null() {\n");
    wrapper.push_str(&format!("        panic!(\"Failed to find real {}\");\n", name));
    wrapper.push_str("    }\n");
    
    // Cast and call
    wrapper.push_str(&format!("    let real_fn: unsafe extern \"C\" fn("));
    for (i, (_, ptype)) in params.iter().enumerate() {
        if i > 0 { wrapper.push_str(", "); }
        wrapper.push_str(ptype);
    }
    wrapper.push_str(&format!(") -> {} = std::mem::transmute(real_fn);\n", ret_type));
    
    wrapper.push_str("    real_fn(");
    for (i, (pname, _)) in params.iter().enumerate() {
        if i > 0 { wrapper.push_str(", "); }
        wrapper.push_str(pname);
    }
    wrapper.push_str(")\n");
    
    wrapper.push_str("}\n\n");
    
    wrapper
}
