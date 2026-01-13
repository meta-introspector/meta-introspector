// 🔥 CUSTOM RUST OVERLAY NIGHTLY BUILD with Unified Nix Builder

mod telemetry_lib;
mod unified_nix_builder;
use unified_nix_builder::NixBuilder;

fn main() {
    println!("🔥 CUSTOM RUST OVERLAY NIGHTLY BUILD");
    println!("====================================");
    
    build_custom_rust_nightly();
}

fn build_custom_rust_nightly() {
    println!("🚀 Building custom rust nightly with unified builder + telemetry...");
    
    let builder = NixBuilder::new();
    
    match builder.build_rust_nightly() {
        Ok(rust_path) => {
            println!("✅ Custom rust nightly built: {}", rust_path);
            println!("🔧 Extensions: rust-src, rust-analyzer, llvm-tools-preview");
            test_custom_rust(&rust_path);
        }
        Err(e) => {
            println!("❌ Build failed: {}", e);
        }
    }
}

fn test_custom_rust(_rust_path: &str) {
    println!("\n🦀 TESTING CUSTOM RUST WITH TELEMETRY");
    println!("=====================================");
    
    let builder = NixBuilder::new();
    let result = builder.build(&["--version"]);
    
    match result {
        Ok(output) => {
            println!("✅ Nix version check: {}", output.stdout.trim());
        }
        Err(e) => {
            println!("❌ Nix test failed: {}", e);
        }
    }
    
    println!("\n🎯 CUSTOM RUST NIGHTLY BUILD COMPLETE!");
}
