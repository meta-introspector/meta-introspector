// Demo: Code Duplication Scanner

mod code_duplication_scanner;
use code_duplication_scanner::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 CODE DUPLICATION SCANNER");
    println!("===========================\n");
    println!("Mathematical fingerprints for Rust code analysis\n");
    
    // Repos to scan
    let repos = vec![
        "/mnt/data1/meta-introspector",
        "/home/mdupont/zos-server",
        "/home/mdupont/librustc",
    ];
    
    println!("📋 Scanning repositories:");
    for repo in &repos {
        println!("  • {}", repo);
    }
    println!();
    
    // Scan all repos
    let scanner = scan_repos(&repos)?;
    
    println!("\n📊 Scan Results:");
    println!("  Total fingerprints: {}", scanner.fingerprints.len());
    println!("  Total duplicates: {}", scanner.duplicates.len());
    
    // Show exact duplicates
    println!("\n🎯 Exact Duplicates (100% match):");
    let exact: Vec<_> = scanner.duplicates.iter()
        .filter(|d| d.similarity == 1.0)
        .collect();
    
    for (i, dup) in exact.iter().take(10).enumerate() {
        println!("\n  {}. Duplicate found in {} locations:", i + 1, dup.locations.len());
        for loc in &dup.locations {
            println!("     - {}:{}", loc.file, loc.function.as_ref().unwrap_or(&"<unknown>".to_string()));
        }
        println!("     Fingerprint: {}", &dup.fingerprint.ast_hash[..16]);
    }
    
    // Show near-duplicates
    println!("\n🔎 Near-Duplicates (80% structural similarity):");
    let near: Vec<_> = scanner.duplicates.iter()
        .filter(|d| d.similarity < 1.0)
        .collect();
    
    for (i, dup) in near.iter().take(10).enumerate() {
        println!("\n  {}. Similar code in {} locations:", i + 1, dup.locations.len());
        for loc in &dup.locations {
            println!("     - {}:{}", loc.file, loc.function.as_ref().unwrap_or(&"<unknown>".to_string()));
        }
        println!("     Similarity: {:.0}%", dup.similarity * 100.0);
        println!("     Structure hash: {}", &dup.fingerprint.structure_hash[..16]);
    }
    
    // Export report
    println!("\n💾 Exporting report...");
    scanner.export_report("/tmp/duplication-report.json")?;
    
    // Export to nix store
    let nix_path = scanner.export_to_nix_store()?;
    println!("📦 Exported to nix store: {}", nix_path);
    
    println!("\n🔬 Fingerprint Types:");
    println!("  • AST Hash: Exact syntax tree match");
    println!("  • Token Hash: Token sequence match");
    println!("  • Structure Hash: Control flow match (ignores names)");
    println!("  • Semantic Hash: Type and semantic match");
    
    println!("\n💡 Use Cases:");
    println!("  • Find copy-pasted code");
    println!("  • Identify refactoring opportunities");
    println!("  • Detect code clones across repos");
    println!("  • Measure code reuse");
    println!("  • Find similar implementations");
    
    println!("\n📈 Statistics:");
    let total_locs: usize = scanner.duplicates.iter()
        .map(|d| d.locations.len())
        .sum();
    println!("  Total duplicate locations: {}", total_locs);
    
    let avg_dups = if !scanner.duplicates.is_empty() {
        total_locs as f64 / scanner.duplicates.len() as f64
    } else {
        0.0
    };
    println!("  Average duplicates per pattern: {:.1}", avg_dups);
    
    println!("\n✅ Scan complete!");
    
    Ok(())
}
