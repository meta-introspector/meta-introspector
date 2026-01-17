// 🔧 NIX BUILD TELEMETRY: Migrated to use canonical builder
use std::env;
mod nix_canonical_builder;
use nix_canonical_builder::nix_build;

fn main() {
    println!("🔧 NIX BUILD TELEMETRY WRAPPER");
    println!("===============================");
    
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
            println!("📤 Exit code: {}", result.exit_code);
            
            if !result.stdout.is_empty() {
                println!("📋 Build output:");
                for line in result.stdout.lines() {
                    println!("  {}", line);
                }
            }
            
            if !result.store_paths.is_empty() {
                println!("📦 Store paths:");
                for path in &result.store_paths {
                    println!("  {}", path);
                }
            }
        }
        Err(e) => {
            println!("❌ Build failed: {}", e);
        }
    }
}
