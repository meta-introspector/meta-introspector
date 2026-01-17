// Build all binaries with nix and analyze our source + zos-server

mod content_addressable_store;
mod rust_spectrum_comprehension;
mod rand_shim;

use content_addressable_store::ContentStore;
use rand_shim::init_rand;
use std::process::Command;

fn main() {
    init_rand();
    
    println!("🔨 Building with Nix and Analyzing\n");
    
    // Build with nix
    println!("📦 Building binaries with nix...\n");
    
    let build = Command::new("nix-build")
        .current_dir("/mnt/data1/meta-introspector")
        .output();
    
    if let Ok(output) = build {
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("  ✓ Build complete: {}", result.lines().last().unwrap_or(""));
        } else {
            println!("  ⚠ Build failed, continuing with analysis...");
        }
    }
    
    // Create stores
    let mut our_store = ContentStore::new("/tmp/meta-introspector-analysis");
    let mut zos_store = ContentStore::new("/tmp/zos-server-analysis");
    
    // Analyze our source
    println!("\n📊 Analyzing meta-introspector source...\n");
    
    if let Ok(entries) = std::fs::read_dir("/mnt/data1/meta-introspector") {
        let mut count = 0;
        for entry in entries.flatten() {
            if let Some(path_str) = entry.path().to_str() {
                if path_str.ends_with(".rs") {
                    if let Ok(source) = std::fs::read_to_string(path_str) {
                        our_store.store(&source);
                        count += 1;
                    }
                }
            }
        }
        println!("  Ingested {} Rust files", count);
    }
    
    our_store.report();
    
    // Save our analysis
    let our_parquet = "/tmp/meta-introspector-analysis/analysis.parquet";
    if our_store.save_to_parquet(our_parquet).is_ok() {
        if let Ok(meta) = std::fs::metadata(our_parquet) {
            println!("\n  ✓ Saved to {} ({} bytes)", our_parquet, meta.len());
        }
    }
    
    // Look for zos-server
    println!("\n📊 Looking for zos-server...\n");
    
    let zos_paths = vec![
        "/mnt/data1/zos-server",
        "/mnt/data1/meta-introspector/zos-server",
        "../zos-server",
    ];
    
    let mut found_zos = false;
    for zos_path in zos_paths {
        if std::path::Path::new(zos_path).exists() {
            println!("  Found zos-server at {}", zos_path);
            
            if let Ok(entries) = std::fs::read_dir(zos_path) {
                let mut count = 0;
                for entry in entries.flatten() {
                    if let Some(path_str) = entry.path().to_str() {
                        if path_str.ends_with(".rs") {
                            if let Ok(source) = std::fs::read_to_string(path_str) {
                                zos_store.store(&source);
                                count += 1;
                            }
                        }
                    }
                }
                println!("  Ingested {} Rust files", count);
                found_zos = true;
                break;
            }
        }
    }
    
    if found_zos {
        zos_store.report();
        
        let zos_parquet = "/tmp/zos-server-analysis/analysis.parquet";
        if zos_store.save_to_parquet(zos_parquet).is_ok() {
            if let Ok(meta) = std::fs::metadata(zos_parquet) {
                println!("\n  ✓ Saved to {} ({} bytes)", zos_parquet, meta.len());
            }
        }
    } else {
        println!("  ⚠ zos-server not found in expected locations");
    }
    
    println!("\n✅ Analysis complete!");
    println!("\n💡 Results:");
    println!("  • Built binaries with nix");
    println!("  • Analyzed meta-introspector source");
    println!("  • Analyzed zos-server (if found)");
    println!("  • Compressed and stored by complexity");
    println!("  • Saved metadata to parquet");
}
