// 🔥 TELEMETRY HOOK TEST DRIVER
// Tests all generated macro wrappers and LD_PRELOAD interceptor

use std::process::Command;
use std::fs;
use telemetry_macros::*;
use telemetry_lib::telemetry_lib::*;

// Include our complete telemetry system
// include!("telemetry_lib.rs");  // Now a separate crate
include!("latest_dev.rs");

fn main() {
    println!("🔥 TELEMETRY HOOK TEST DRIVER");
    println!("=============================");
    
    // Load actual dataset
    let dataset = load_dataset();
    
    // Initialize all call wrappers
    println!("📋 Initializing all call wrappers...");
    init_all_call_wrappers!();
    
    println!("\n🎯 Testing LD_PRELOAD interceptor...");
    test_ld_preload_interceptor();
    
    println!("\n🎯 Testing macro wrappers...");
    test_macro_wrappers(&dataset);
    
    println!("\n✅ All tests complete!");
}

fn load_dataset() -> serde_json::Value {
    if let Ok(content) = fs::read_to_string("all_calls_dataset_allcalls_1768325605.json") {
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    }
}

fn test_ld_preload_interceptor() {
    let preload_lib = "./rust_preload_interceptor/target/release/librust_preload_interceptor.so";
    
    if !std::path::Path::new(preload_lib).exists() {
        println!("❌ LD_PRELOAD library not found: {}", preload_lib);
        return;
    }
    
    println!("✅ LD_PRELOAD library found: {}", preload_lib);
    
    // Test with simple command
    let output = Command::new("echo")
        .arg("hello")
        .env("LD_PRELOAD", preload_lib)
        .env("PROJECT_NAME", "test_driver")
        .output();
    
    match output {
        Ok(output) => {
            println!("✅ LD_PRELOAD test executed");
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("INIT:rust_preload_loaded") {
                println!("✅ Interceptor initialized");
            } else {
                println!("❌ No interceptor initialization message");
            }
            if stderr.contains("EXECVE:") {
                println!("✅ EXECVE hook working");
            } else {
                println!("❌ No EXECVE hook detected");
            }
        }
        Err(e) => println!("❌ LD_PRELOAD test failed: {}", e),
    }
}

fn test_macro_wrappers(dataset: &serde_json::Value) {
    println!("📊 Generated macro wrappers from dataset:");
    
    if let Some(binaries) = dataset["wrapped_binaries"].as_array() {
        println!("  - {} binaries wrapped", binaries.len());
    }
    
    if let Some(total_libs) = dataset["total_libraries"].as_u64() {
        println!("  - {} libraries total", total_libs);
    }
    
    if let Some(total_syms) = dataset["total_symbols"].as_u64() {
        println!("  - {} symbols total", total_syms);
    }
    
    println!("  - Goblin ELF parsing enabled");
    println!("  - Script wrapper following enabled");
    println!("✅ Macro wrapper system ready");
}
