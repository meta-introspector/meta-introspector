use lmfdb_rust_mapping::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = LMFDBMapper::new();
    
    println!("🔬 Analyzing LMFDB library on itself\n");
    
    let binary = "target/debug/deps/libserde_derive-0a121a9dfc6e5f96.so";
    println!("📊 Analyzing: {}", binary);
    
    match mapper.analyze_binary(binary) {
        Ok(analysis) => {
            println!("\n✅ Analysis Complete:");
            println!("  Total symbols: {}", analysis.total_symbols);
            println!("  Conductor: {}", analysis.conductor);
            println!("\n📈 Orbit Distribution:");
            for (orbit, count) in &analysis.orbit_distribution {
                println!("  {:?}: {} symbols", orbit, count);
            }
            
            println!("\n🔝 Top 10 Symbols:");
            for (i, mapping) in analysis.symbol_mappings.iter().take(10).enumerate() {
                println!("  {}. {} → {} (orbit: {:?})", 
                    i+1, 
                    mapping.symbol_name.chars().take(50).collect::<String>(),
                    mapping.lmfdb_label.to_string(),
                    mapping.orbit_level
                );
            }
            
            // Save to JSON
            let json = serde_json::to_string_pretty(&analysis)?;
            std::fs::write("lmfdb_self_analysis.json", json)?;
            println!("\n💾 Saved to: lmfdb_self_analysis.json");
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    
    Ok(())
}
