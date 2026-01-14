// 🔥 UNIFIED NIX BUILD + SO WRAPPING: Migrated to canonical builder
use std::env;
mod nix_canonical_builder;
use nix_canonical_builder::nix_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 UNIFIED NIX BUILD + SO WRAPPING");
    println!("==================================");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <nix-build-args...>", args[0]);
        println!("Example: {} build .#hello", args[0]);
        return Ok(());
    }
    
    let nix_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    
    // Use canonical builder - automatic perf + telemetry + parquet
    match nix_build(&nix_args) {
        Ok(result) => {
            println!("✅ Build successful in {:.2}s", result.duration_secs);
            
            if !result.store_paths.is_empty() {
                println!("📦 Analyzing store paths:");
                for path in &result.store_paths {
                    println!("  {}", path);
                    // TODO: Add SO analysis here if needed
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Build failed: {}", e);
        }
    }
    
    Ok(())
}
