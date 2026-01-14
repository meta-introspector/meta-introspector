// 🔧 NIX BUILD + TRANSPARENT TELEMETRY: Migrated to canonical builder
use std::env;
mod nix_canonical_builder;
use nix_canonical_builder::nix_build;

fn main() {
    println!("🔧 NIX BUILD + TRANSPARENT TELEMETRY");
    println!("====================================");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <nix-build-args...>", args[0]);
        println!("Example: {} build .#hello", args[0]);
        return;
    }
    
    let nix_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    
    // Use canonical builder - automatic perf + telemetry + parquet
    match nix_build(&nix_args) {
        Ok(result) => {
            println!("✅ Build successful in {:.2}s", result.duration_secs);
            
            for path in &result.store_paths {
                println!("📦 {}", path);
            }
        }
        Err(e) => {
            eprintln!("❌ Build failed: {}", e);
        }
    }
}
