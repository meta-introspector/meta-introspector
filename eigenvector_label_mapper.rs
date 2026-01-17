use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading symbols and computing eigenvector-label mapping...\n");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut symbols: Vec<(String, String, f64)> = Vec::new();
    let mut symbol_index: HashMap<String, Vec<usize>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file = row.get_string(1)?.to_string();
        let score = row.get_double(4)?;
        
        let idx = symbols.len();
        symbols.push((name.clone(), file, score));
        symbol_index.entry(name).or_default().push(idx);
    }
    
    println!("✅ Loaded {} symbols\n", symbols.len());
    
    // Get top symbols by frequency
    let mut multi_file: Vec<(String, usize)> = symbol_index.iter()
        .filter(|(_, v)| v.len() > 10)
        .map(|(k, v)| (k.clone(), v.len()))
        .collect();
    multi_file.sort_by(|a, b| b.1.cmp(&a.1));
    
    let top_symbols: Vec<String> = multi_file.iter().take(500).map(|(s, _)| s.clone()).collect();
    
    // Auto-label each top symbol
    println!("🏷️  Labeling top 500 symbols...");
    let mut label_to_symbols: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    
    for sym in &top_symbols {
        let indices = &symbol_index[sym];
        let (name, file, _) = &symbols[indices[0]];
        let label = auto_label(name, file);
        label_to_symbols.entry(label).or_default().push((sym.clone(), indices.len()));
    }
    
    // Load eigenvector results
    let eigenvector_text = fs::read_to_string("symbol_eigenvector_results.txt")?;
    let mut eigenvector_symbols = Vec::new();
    
    for line in eigenvector_text.lines() {
        if line.starts_with(char::is_numeric) {
            if let Some(sym_start) = line.find(". ") {
                if let Some(sym_end) = line[sym_start+2..].find(" (") {
                    let sym = line[sym_start+2..sym_start+2+sym_end].to_string();
                    eigenvector_symbols.push(sym);
                }
            }
        }
    }
    
    println!("✅ Loaded {} eigenvector symbols\n", eigenvector_symbols.len());
    
    // Map eigenvector to labels
    println!("🎯 Eigenvector symbol labels (top 50):");
    for (i, sym) in eigenvector_symbols.iter().take(50).enumerate() {
        if let Some(indices) = symbol_index.get(sym) {
            let (name, file, _) = &symbols[indices[0]];
            let label = auto_label(name, file);
            println!("   {}. {} → {}", i + 1, sym, label);
        }
    }
    
    // Cluster analysis
    println!("\n📊 Label distribution in top 50 eigenvector symbols:");
    let mut label_counts: HashMap<String, usize> = HashMap::new();
    for sym in eigenvector_symbols.iter().take(50) {
        if let Some(indices) = symbol_index.get(sym) {
            let (name, file, _) = &symbols[indices[0]];
            let label = auto_label(name, file);
            *label_counts.entry(label).or_insert(0) += 1;
        }
    }
    
    let mut sorted_labels: Vec<_> = label_counts.iter().collect();
    sorted_labels.sort_by(|a, b| b.1.cmp(a.1));
    
    for (label, count) in &sorted_labels {
        println!("   {}: {} symbols ({:.1}%)", label, count, (**count as f64 / 50.0) * 100.0);
    }
    
    // Save subgraph mapping
    let mut output = String::from("Eigenvector to Label Mapping\n\n");
    output.push_str("Top 100 eigenvector symbols with labels:\n");
    
    for (i, sym) in eigenvector_symbols.iter().take(100).enumerate() {
        if let Some(indices) = symbol_index.get(sym) {
            let (name, file, _) = &symbols[indices[0]];
            let label = auto_label(name, file);
            output.push_str(&format!("{}. {} → {} ({} files)\n", i + 1, sym, label, indices.len()));
        }
    }
    
    output.push_str("\nLabel clusters in eigenvector:\n");
    for (label, count) in &sorted_labels {
        output.push_str(&format!("{}: {} symbols ({:.1}%)\n", label, count, (**count as f64 / eigenvector_symbols.len() as f64) * 100.0));
    }
    
    output.push_str("\nSymbols by label:\n");
    for (label, syms) in &label_to_symbols {
        output.push_str(&format!("\n{}:\n", label));
        for (sym, count) in syms.iter().take(20) {
            output.push_str(&format!("  - {} ({} files)\n", sym, count));
        }
    }
    
    fs::write("eigenvector_label_mapping.txt", output)?;
    println!("\n💾 Saved to eigenvector_label_mapping.txt");
    
    Ok(())
}

fn auto_label(name: &str, file: &str) -> String {
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
}
