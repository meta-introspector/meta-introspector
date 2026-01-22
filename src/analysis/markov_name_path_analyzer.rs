use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading symbols from Parquet...");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut symbols: Vec<(String, String, f64)> = Vec::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file = row.get_string(1)?.to_string();
        let score = row.get_double(4)?;
        symbols.push((name, file, score));
        
        if (symbols.len()).is_multiple_of(200000) {
            println!("   Loaded {} symbols", symbols.len());
        }
    }
    
    println!("✅ Loaded {} symbols\n", symbols.len());
    
    // Build Markov chains for symbol names
    println!("🔤 Building symbol name Markov chains...");
    let mut name_bigrams: HashMap<String, usize> = HashMap::new();
    let mut name_trigrams: HashMap<String, usize> = HashMap::new();
    
    for (name, _, _) in &symbols {
        // Extract tokens from symbol names
        let tokens = tokenize_symbol(name);
        
        for window in tokens.windows(2) {
            let bigram = format!("{}_{}", window[0], window[1]);
            *name_bigrams.entry(bigram).or_insert(0) += 1;
        }
        
        for window in tokens.windows(3) {
            let trigram = format!("{}_{}_{}", window[0], window[1], window[2]);
            *name_trigrams.entry(trigram).or_insert(0) += 1;
        }
    }
    
    println!("   {} unique bigrams, {} trigrams", name_bigrams.len(), name_trigrams.len());
    
    // Build Markov chains for file paths
    println!("📁 Building file path Markov chains...");
    let mut path_bigrams: HashMap<String, usize> = HashMap::new();
    let mut path_components: HashMap<String, usize> = HashMap::new();
    
    for (_, file, _) in &symbols {
        let parts: Vec<&str> = file.split('/').filter(|s| !s.is_empty()).collect();
        
        for part in &parts {
            *path_components.entry(part.to_string()).or_insert(0) += 1;
        }
        
        for window in parts.windows(2) {
            let bigram = format!("{}/{}", window[0], window[1]);
            *path_bigrams.entry(bigram).or_insert(0) += 1;
        }
    }
    
    println!("   {} unique path components, {} path bigrams", path_components.len(), path_bigrams.len());
    
    // Find top patterns
    let mut sorted_name_bigrams: Vec<_> = name_bigrams.iter().collect();
    sorted_name_bigrams.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut sorted_name_trigrams: Vec<_> = name_trigrams.iter().collect();
    sorted_name_trigrams.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut sorted_path_components: Vec<_> = path_components.iter().collect();
    sorted_path_components.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Top 30 symbol name bigrams:");
    for (i, (pattern, count)) in sorted_name_bigrams.iter().take(30).enumerate() {
        println!("   {}. {} ({})", i + 1, pattern, count);
    }
    
    println!("\n🎯 Top 30 symbol name trigrams:");
    for (i, (pattern, count)) in sorted_name_trigrams.iter().take(30).enumerate() {
        println!("   {}. {} ({})", i + 1, pattern, count);
    }
    
    println!("\n🎯 Top 30 path components:");
    for (i, (comp, count)) in sorted_path_components.iter().take(30).enumerate() {
        println!("   {}. {} ({})", i + 1, comp, count);
    }
    
    // Auto-label based on patterns
    println!("\n🏷️  Auto-labeling symbol clusters...");
    let labels = auto_label_symbols(&symbols, &name_bigrams, &name_trigrams, &path_components);
    
    let mut label_counts: HashMap<String, usize> = HashMap::new();
    for label in &labels {
        *label_counts.entry(label.clone()).or_insert(0) += 1;
    }
    
    let mut sorted_labels: Vec<_> = label_counts.iter().collect();
    sorted_labels.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 Symbol cluster labels:");
    for (i, (label, count)) in sorted_labels.iter().take(20).enumerate() {
        println!("   {}. {} ({} symbols)", i + 1, label, count);
    }
    
    // Save results
    let mut output = String::from("Symbol and Path Markov Analysis\n\n");
    
    output.push_str("Top 100 symbol name bigrams:\n");
    for (i, (pattern, count)) in sorted_name_bigrams.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ({})\n", i + 1, pattern, count));
    }
    
    output.push_str("\nTop 100 symbol name trigrams:\n");
    for (i, (pattern, count)) in sorted_name_trigrams.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ({})\n", i + 1, pattern, count));
    }
    
    output.push_str("\nTop 100 path components:\n");
    for (i, (comp, count)) in sorted_path_components.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ({})\n", i + 1, comp, count));
    }
    
    output.push_str("\nAuto-labeled clusters:\n");
    for (i, (label, count)) in sorted_labels.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} ({} symbols)\n", i + 1, label, count));
    }
    
    fs::write("markov_name_path_analysis.txt", output)?;
    println!("\n💾 Saved to markov_name_path_analysis.txt");
    
    Ok(())
}

fn tokenize_symbol(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    
    // Split on common delimiters
    for part in name.split(&['_', '.', ':', '$', '@'][..]) {
        if !part.is_empty() {
            tokens.push(part.to_lowercase());
        }
    }
    
    // Also extract camelCase tokens
    let mut current = String::new();
    for ch in name.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            tokens.push(current.to_lowercase());
            current = String::new();
        }
        current.push(ch);
    }
    if !current.is_empty() {
        tokens.push(current.to_lowercase());
    }
    
    tokens
}

fn auto_label_symbols(
    symbols: &[(String, String, f64)],
    _name_bigrams: &HashMap<String, usize>,
    _name_trigrams: &HashMap<String, usize>,
    _path_components: &HashMap<String, usize>
) -> Vec<String> {
    symbols.iter().map(|(name, file, _)| {
        // Label based on symbol name patterns
        if name.contains("_RNv") || name.contains("_ZN") {
            if name.contains("tracing") {
                "rust_tracing".to_string()
            } else if name.contains("panic") {
                "rust_panic".to_string()
            } else if name.contains("alloc") {
                "rust_alloc".to_string()
            } else if name.contains("std") {
                "rust_std".to_string()
            } else {
                "rust_mangled".to_string()
            }
        } else if name.starts_with("sanei_") {
            "sane_scanner".to_string()
        } else if name.starts_with("ul_") {
            "util_linux".to_string()
        } else if name.contains("usb") {
            "usb_driver".to_string()
        } else if name.contains("xml") {
            "xml_parser".to_string()
        } else if name.contains("md5") || name.contains("sha") || name.contains("crypt") {
            "crypto".to_string()
        } else if name.contains("str") && (name.contains("parse") || name.contains("conv")) {
            "string_util".to_string()
        } else if name.contains("init") || name.contains("fini") {
            "elf_lifecycle".to_string()
        } else if name.contains("stack_chk") {
            "stack_protection".to_string()
        } else if name.contains("plugin") {
            "plugin_system".to_string()
        } else if file.contains("glibc") {
            "glibc".to_string()
        } else if file.contains("lib") && file.contains(".so") {
            "shared_library".to_string()
        } else {
            "other".to_string()
        }
    }).collect()
}
