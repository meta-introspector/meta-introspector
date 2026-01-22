// Prove coverage of ALL syn types with at least one sample each

#[path = "../../xz_to_syn_mapper.rs"] mod xz_to_syn_mapper;
#[path = "../../rand_shim.rs"] mod rand_shim;

use xz_to_syn_mapper::XzToSynMapper;
use rand_shim::init_rand;
use std::collections::{HashMap, HashSet};

fn main() {
    init_rand();
    
    println!("🔬 PROOF: Complete Syn Type Coverage\n");
    println!("{}", "=".repeat(80));
    
    // All syn::Item types
    let all_syn_types = vec![
        "Const", "Enum", "ExternCrate", "Fn", "ForeignMod", 
        "Impl", "Macro", "Mod", "Static", "Struct", 
        "Trait", "TraitAlias", "Type", "Union", "Use",
    ];
    
    println!("\n📋 All Syn Item Types ({} total):\n", all_syn_types.len());
    for (i, t) in all_syn_types.iter().enumerate() {
        print!("{:<15}", t);
        if (i + 1) % 5 == 0 {
            println!();
        }
    }
    println!("\n");
    
    let rust_src = "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz";
    
    println!("{}", "=".repeat(80));
    println!("\n📦 Scanning Rust stdlib for syn type coverage...\n");
    
    let blocks = XzToSynMapper::scan_xz_blocks(rust_src, 100);
    println!("Loaded {} source blocks\n", blocks.len());
    
    let mut type_coverage: HashMap<String, Vec<String>> = HashMap::new();
    let mut covered_types = HashSet::new();
    
    for (i, block) in blocks.iter().enumerate() {
        let source = String::from_utf8_lossy(&block.data).to_string();
        
        if let Ok(file) = syn::parse_file(&source) {
            for item in &file.items {
                let type_name = match item {
                    syn::Item::Const(_) => "Const",
                    syn::Item::Enum(_) => "Enum",
                    syn::Item::ExternCrate(_) => "ExternCrate",
                    syn::Item::Fn(_) => "Fn",
                    syn::Item::ForeignMod(_) => "ForeignMod",
                    syn::Item::Impl(_) => "Impl",
                    syn::Item::Macro(_) => "Macro",
                    syn::Item::Mod(_) => "Mod",
                    syn::Item::Static(_) => "Static",
                    syn::Item::Struct(_) => "Struct",
                    syn::Item::Trait(_) => "Trait",
                    syn::Item::TraitAlias(_) => "TraitAlias",
                    syn::Item::Type(_) => "Type",
                    syn::Item::Union(_) => "Union",
                    syn::Item::Use(_) => "Use",
                    _ => "Other",
                };
                
                covered_types.insert(type_name.to_string());
                
                type_coverage.entry(type_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("block_{}", i));
            }
        }
        
        // Early exit if we've covered all types
        if covered_types.len() >= all_syn_types.len() {
            println!("✅ All types covered at block {}\n", i);
            break;
        }
    }
    
    println!("{}", "=".repeat(80));
    println!("\n📊 COVERAGE REPORT\n");
    println!("{:<20} {:>10} {:>15}", "Syn Type", "Count", "First Sample");
    println!("{}", "-".repeat(80));
    
    let mut sorted_types: Vec<_> = all_syn_types.iter().collect();
    sorted_types.sort();
    
    let mut covered_count = 0;
    let mut missing_types = Vec::new();
    
    for syn_type in sorted_types {
        if let Some(samples) = type_coverage.get(*syn_type) {
            let first_sample = samples.first().map(|s| s.as_str()).unwrap_or("none");
            println!("{:<20} {:>10} {:>15}", syn_type, samples.len(), first_sample);
            covered_count += 1;
        } else {
            println!("{:<20} {:>10} {:>15}", syn_type, 0, "❌ MISSING");
            missing_types.push(*syn_type);
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("\n✅ PROOF SUMMARY\n");
    println!("Total syn types: {}", all_syn_types.len());
    println!("Types covered: {}", covered_count);
    println!("Coverage: {:.1}%", (covered_count as f64 / all_syn_types.len() as f64) * 100.0);
    
    if missing_types.is_empty() {
        println!("\n🎯 COMPLETE: All syn types have at least one sample!");
    } else {
        println!("\n⚠️  Missing types ({}):", missing_types.len());
        for t in &missing_types {
            println!("  - {}", t);
        }
        println!("\nNote: These types may be rare in stdlib or require more samples");
    }
    
    // Show top 5 most common types
    println!("\n📈 Top 5 Most Common Types:\n");
    let mut sorted_coverage: Vec<_> = type_coverage.iter().collect();
    sorted_coverage.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (i, (type_name, samples)) in sorted_coverage.iter().take(5).enumerate() {
        println!("  {}. {}: {} occurrences", i + 1, type_name, samples.len());
    }
    
    println!("\n{}", "=".repeat(80));
}
