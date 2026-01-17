// 🔥 RECURSIVE RUSTC NIGHTLY WRAPPER: Build rustc with telemetry, then wrap it
use std::process::Command;
use std::env;
use telemetry_macros::*;

// Include transparent telemetry
include!(concat!(env!("OUT_DIR"), "/telemetry_macros.rs"));

fn main() {
    println!("🔥 RECURSIVE RUSTC NIGHTLY WRAPPER");
    println!("===================================");
    
    preload_telemetry!();
    
    // Step 1: Build rustc nightly with nix overlay
    let rustc_path = build_rustc_nightly();
    
    // Step 2: Wrap the built rustc with telemetry
    if let Some(path) = rustc_path {
        wrap_rustc_recursively(&path);
    }
}

fn build_rustc_nightly() -> Option<String> {
    println!("🚀 Building rustc nightly...");
    
    // Simple approach - just get rustc from nixpkgs
    let output = Command::new("nix-build")
        .args(&["-E", "with import <nixpkgs> {}; rustc"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("✅ Built rustc: {}", path);
            Some(path)
        }
        Ok(output) => {
            println!("❌ Build failed: {}", String::from_utf8_lossy(&output.stderr));
            None
        }
        Err(e) => {
            println!("❌ Command failed: {}", e);
            None
        }
    }
}

fn wrap_rustc_recursively(rustc_path: &str) {
    println!("\n🔧 WRAPPING RUSTC RECURSIVELY");
    println!("=============================");
    
    // Find rustc binary in the nix store path
    let rustc_bin = format!("{}/bin/rustc", rustc_path);
    
    if std::path::Path::new(&rustc_bin).exists() {
        println!("🎯 Found rustc binary: {}", rustc_bin);
        
        // Use our nix telemetry integration directly on the rustc binary
        let output = Command::new("cargo")
            .args(&["run", "--bin", "nix_telemetry_integration", "--", "-E", 
                   &format!("\"{}\"", rustc_path)])
            .output();
        
        match output {
            Ok(output) => {
                println!("📊 Rustc telemetry analysis:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                println!("❌ Failed to analyze rustc: {}", e);
            }
        }
    } else {
        println!("⚠️  No rustc binary found at {}", rustc_bin);
        println!("🔍 Listing contents of {}:", rustc_path);
        
        if let Ok(entries) = std::fs::read_dir(rustc_path) {
            for entry in entries.flatten() {
                println!("  📁 {}", entry.path().display());
            }
        }
    }
    
    println!("\n🎯 Rustc recursive analysis complete!");
}
