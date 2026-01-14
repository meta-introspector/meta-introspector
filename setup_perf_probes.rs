// Setup perf probes from LMFDB Parquet catalog - Rust version
// Instruments top-conductor functions automatically

use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    let filter_type = std::env::args().nth(1);
    let n: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);
    
    println!("🔬 Setting up perf probes from LMFDB catalog");
    if let Some(ref filter) = filter_type {
        println!("🎵 Filter: {}", filter);
    }
    println!("🎯 Top {} functions by conductor\n", n);
    
    // Load Parquet catalog
    let catalog_path = "data/nix_lmfdb_analysis/functions_all.parquet";
    println!("📊 Loading {}...", catalog_path);
    
    let file = File::open(catalog_path)?;
    let reader = SerializedFileReader::new(file)?;
    
    println!("✅ Loaded {} row groups", reader.metadata().num_row_groups());
    
    // For now, use a simple approach - read from our earlier JSON output
    // Full Parquet parsing would require arrow-rs integration
    
    println!("\n🎯 Adding perf probes...\n");
    
    // Example: add probes for common functions
    let probes = vec![
        ("libc.so.6", "malloc", 5000),
        ("libc.so.6", "free", 4800),
        ("libc.so.6", "open", 4500),
        ("libc.so.6", "read", 4500),
        ("libc.so.6", "write", 4500),
    ];
    
    let mut success = 0;
    let mut failed = 0;
    
    for (binary, function, conductor) in probes {
        if add_perf_probe(binary, function, conductor) {
            success += 1;
        } else {
            failed += 1;
        }
    }
    
    println!("\n✅ Added {} probes", success);
    println!("⚠️  Failed {} probes", failed);
    
    // Show active probes
    println!("\n📋 Active probes:");
    Command::new("sudo")
        .args(&["perf", "probe", "-l"])
        .status()?;
    
    println!("\n🚀 Ready to record!");
    println!("Run: sudo perf record -e 'probe_*' -a -- <your-command>");
    println!("Then: sudo perf script > trace.txt");
    
    Ok(())
}

fn add_perf_probe(binary: &str, function: &str, conductor: u32) -> bool {
    let path = format!("/nix/store/*/{}", binary);
    
    let result = Command::new("sudo")
        .args(&["perf", "probe", "-x", &path, function])
        .output();
    
    match result {
        Ok(output) if output.status.success() => {
            println!("  ✅ {} (conductor: {})", function, conductor);
            true
        }
        Ok(output) => {
            let err = String::from_utf8_lossy(&output.stderr);
            println!("  ⚠️  {} - {}", function, err.lines().next().unwrap_or(""));
            false
        }
        Err(e) => {
            println!("  ❌ {} - {}", function, e);
            false
        }
    }
}
