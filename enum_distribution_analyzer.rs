use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Item, ItemEnum, Variant, Fields};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnumDistribution {
    variant_count: usize,
    enum_count: u32,
    enum_names: Vec<String>,
    variant_names: Vec<String>,
}

struct EnumAnalyzer {
    distributions: HashMap<usize, EnumDistribution>,
    total_enums: u32,
}

impl EnumAnalyzer {
    fn new() -> Self {
        Self {
            distributions: HashMap::new(),
            total_enums: 0,
        }
    }

    fn add_enum(&mut self, enum_item: &ItemEnum) {
        // Only count simple enums (no data in variants)
        let simple_variants: Vec<_> = enum_item.variants.iter()
            .filter(|variant| matches!(variant.fields, Fields::Unit))
            .collect();

        // Only process if ALL variants are simple (no data)
        if simple_variants.len() == enum_item.variants.len() {
            let variant_count = simple_variants.len();
            let enum_name = enum_item.ident.to_string();
            
            let variant_names: Vec<String> = simple_variants.iter()
                .map(|v| v.ident.to_string())
                .collect();

            let distribution = self.distributions.entry(variant_count).or_insert_with(|| {
                EnumDistribution {
                    variant_count,
                    enum_count: 0,
                    enum_names: Vec::new(),
                    variant_names: Vec::new(),
                }
            });

            distribution.enum_count += 1;
            distribution.enum_names.push(enum_name);
            distribution.variant_names.extend(variant_names);
            
            self.total_enums += 1;
        }
    }
}

impl<'ast> Visit<'ast> for EnumAnalyzer {
    fn visit_item(&mut self, item: &'ast Item) {
        if let Item::Enum(enum_item) = item {
            self.add_enum(enum_item);
        }
        
        syn::visit::visit_item(self, item);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 SIMPLE ENUM DISTRIBUTION ANALYSIS");
    println!("===================================");

    let mut analyzer = EnumAnalyzer::new();
    let mut processed_files = 0;

    // Process zombie_driver2 files
    let base_dir = "/home/mdupont/zombie_driver2";
    
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rs") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(file) = parse_file(&content) {
                            analyzer.visit_file(&file);
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 SIMPLE ENUM DISTRIBUTION:");
    println!("Variants | Count | Percentage | Examples");
    println!("---------|-------|------------|----------");

    let mut sorted_distributions: Vec<_> = analyzer.distributions.iter().collect();
    sorted_distributions.sort_by_key(|(variant_count, _)| *variant_count);

    for (variant_count, distribution) in &sorted_distributions {
        let percentage = if analyzer.total_enums > 0 {
            distribution.enum_count as f64 / analyzer.total_enums as f64 * 100.0
        } else {
            0.0
        };

        let examples: Vec<String> = distribution.enum_names.iter()
            .take(3)
            .cloned()
            .collect();
        let examples_str = examples.join(", ");

        println!("{:8} | {:5} | {:9.1}% | {}", 
                 variant_count, 
                 distribution.enum_count, 
                 percentage,
                 examples_str);
    }

    // Analyze variant name patterns
    println!("\n🏷️  VARIANT NAME ANALYSIS:");
    let mut all_variant_names = Vec::new();
    for distribution in analyzer.distributions.values() {
        all_variant_names.extend(distribution.variant_names.iter().cloned());
    }

    let mut variant_frequency: HashMap<String, u32> = HashMap::new();
    for name in &all_variant_names {
        *variant_frequency.entry(name.clone()).or_default() += 1;
    }

    let mut sorted_variants: Vec<_> = variant_frequency.iter().collect();
    sorted_variants.sort_by(|a, b| b.1.cmp(a.1));

    println!("Most common variant names:");
    for (name, count) in sorted_variants.iter().take(10) {
        println!("  {}: {} times", name, count);
    }

    // Binary enum analysis (2 variants)
    if let Some(binary_enums) = analyzer.distributions.get(&2) {
        println!("\n🔀 BINARY ENUM ANALYSIS:");
        println!("Total binary enums: {}", binary_enums.enum_count);
        
        // Look for common binary patterns
        let mut binary_patterns: HashMap<String, u32> = HashMap::new();
        for i in (0..binary_enums.variant_names.len()).step_by(2) {
            if i + 1 < binary_enums.variant_names.len() {
                let mut pair = vec![
                    binary_enums.variant_names[i].clone(),
                    binary_enums.variant_names[i + 1].clone()
                ];
                pair.sort();
                let pattern = pair.join(" | ");
                *binary_patterns.entry(pattern).or_default() += 1;
            }
        }

        let mut sorted_patterns: Vec<_> = binary_patterns.iter().collect();
        sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));

        println!("Common binary patterns:");
        for (pattern, count) in sorted_patterns.iter().take(5) {
            println!("  {}: {} enums", pattern, count);
        }
    }

    // Save detailed analysis
    let analysis_result = serde_json::to_string_pretty(&analyzer.distributions)?;
    fs::write("enum_distribution_analysis.json", analysis_result)?;

    println!("\n🎯 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Simple enums found: {}", analyzer.total_enums);
    println!("Variant count range: {} to {}", 
             sorted_distributions.first().map(|(k, _)| **k).unwrap_or(0),
             sorted_distributions.last().map(|(k, _)| **k).unwrap_or(0));
    println!("Analysis saved to enum_distribution_analysis.json");

    Ok(())
}
