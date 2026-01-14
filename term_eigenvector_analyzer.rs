use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔑 Computing eigenvectors for fundamental programming terms\n");
    
    // Define fundamental terms from all languages
    let terms = vec![
        // Type/Structure keywords
        "enum", "struct", "type", "class", "interface", "trait",
        // Function keywords
        "fn", "function", "def", "lambda", "proc", "method",
        // Control flow
        "if", "then", "else", "elif", "match", "case", "switch",
        "for", "while", "loop", "until", "foreach", "each",
        // Operations
        "return", "yield", "break", "continue",
        "call", "apply", "invoke",
        // Binding
        "let", "var", "const", "bind", "set",
        "arg", "param", "argument",
        // Memory/Ownership (Rust-specific but important)
        "alloc", "free", "drop", "move", "copy", "clone",
        "ref", "mut", "borrow",
        // LLVM IR terms
        "load", "store", "getelementptr", "phi", "br",
        "add", "sub", "mul", "div",
    ];
    
    println!("Analyzing {} terms across all languages\n", terms.len());
    println!("Terms: {:?}\n", terms);
    
    // Load symbols
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    // Build term → symbols mapping
    let mut term_symbols: HashMap<String, Vec<SymbolData>> = HashMap::new();
    
    for term in &terms {
        term_symbols.insert(term.to_string(), Vec::new());
    }
    
    println!("📊 Scanning {} symbols...", reader.metadata().file_metadata().num_rows());
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file = row.get_string(1)?.to_string();
        let cell = row.get_ulong(2)?;
        let score = row.get_double(4)?;
        
        // Check which terms appear in symbol name
        for term in &terms {
            if name.to_lowercase().contains(term) {
                term_symbols.get_mut(*term).unwrap().push(SymbolData {
                    name: name.clone(),
                    file: file.clone(),
                    cell,
                    score,
                });
            }
        }
    }
    
    println!("\n🎯 Term Frequencies:");
    for term in &terms {
        let count = term_symbols[*term].len();
        println!("  {}: {} symbols", term, count);
    }
    
    // Compute eigenvector for each term
    println!("\n🌟 Computing eigenvectors (resonance patterns)...\n");
    
    let mut term_eigenvectors = Vec::new();
    
    for term in &terms {
        let symbols = &term_symbols[*term];
        
        if symbols.is_empty() {
            println!("⚠️  {}: No symbols found", term);
            continue;
        }
        
        let eigenvector = compute_term_eigenvector(symbols);
        
        println!("📈 {}:", term);
        println!("   Symbols: {}", symbols.len());
        println!("   Mean score: {:.3}", eigenvector.mean_score);
        println!("   Std dev: {:.3}", eigenvector.std_dev);
        println!("   Dominant cell: {}", eigenvector.dominant_cell);
        println!("   Energy: {:.3}", eigenvector.energy);
        println!("   Top file: {}", eigenvector.top_file.split('/').last().unwrap_or("?"));
        println!();
        
        term_eigenvectors.push((term.to_string(), eigenvector));
    }
    
    // Find most resonant code for each term
    println!("🔬 Most Resonant Code Patterns:\n");
    
    for (term, eigenvec) in &term_eigenvectors {
        let symbols = &term_symbols[term];
        
        // Find top 5 symbols by score
        let mut sorted: Vec<_> = symbols.iter().collect();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        println!("🎯 '{}' resonates most with:", term);
        for (i, sym) in sorted.iter().take(5).enumerate() {
            let file_short = sym.file.split('/').rev().take(3).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("/");
            println!("   {}. Symbol: {}", i + 1, sym.name.chars().take(80).collect::<String>());
            println!("      File: {}", file_short);
            println!("      Cell: {}, Score: {:.2}", sym.cell, sym.score);
            
            // Try to extract embedded strings from symbol name
            if let Some(strings) = extract_embedded_strings(&sym.name) {
                println!("      Strings: {}", strings);
            }
        }
        println!();
    }
    
    // Compute term-to-term similarity
    println!("🔗 Term Similarity Matrix:\n");
    
    for i in 0..term_eigenvectors.len() {
        for j in i+1..term_eigenvectors.len() {
            let (term1, ev1) = &term_eigenvectors[i];
            let (term2, ev2) = &term_eigenvectors[j];
            
            let similarity = compute_eigenvector_similarity(ev1, ev2);
            
            if similarity > 0.3 {
                println!("  {} ↔ {}: {:.3}", term1, term2, similarity);
            }
        }
    }
    
    // Save results
    let mut output = String::from("Programming Term Eigenvectors\n\n");
    
    for (term, eigenvec) in &term_eigenvectors {
        output.push_str(&format!("{}:\n", term));
        output.push_str(&format!("  Symbols: {}\n", term_symbols[term].len()));
        output.push_str(&format!("  Mean score: {:.3}\n", eigenvec.mean_score));
        output.push_str(&format!("  Energy: {:.3}\n", eigenvec.energy));
        output.push_str(&format!("  Dominant cell: {}\n", eigenvec.dominant_cell));
        output.push_str(&format!("  Top file: {}\n\n", eigenvec.top_file));
        
        // Top 20 resonant symbols
        let symbols = &term_symbols[term];
        let mut sorted: Vec<_> = symbols.iter().collect();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        output.push_str("  Top resonant symbols:\n");
        for (i, sym) in sorted.iter().take(20).enumerate() {
            let file_short = sym.file.split('/').rev().take(2).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("/");
            output.push_str(&format!("    {}. {}\n", i + 1, sym.name.chars().take(100).collect::<String>()));
            output.push_str(&format!("       File: {}\n", file_short));
            output.push_str(&format!("       Cell: {}, Score: {:.2}\n", sym.cell, sym.score));
            if let Some(strings) = extract_embedded_strings(&sym.name) {
                output.push_str(&format!("       Keywords: {}\n", strings));
            }
        }
        output.push_str("\n");
    }
    
    fs::write("term_eigenvectors.txt", output)?;
    println!("\n💾 Saved to term_eigenvectors.txt");
    
    Ok(())
}

#[derive(Debug, Clone)]
struct SymbolData {
    name: String,
    file: String,
    cell: u64,
    score: f64,
}

#[derive(Debug, Clone)]
struct TermEigenvector {
    mean_score: f64,
    std_dev: f64,
    dominant_cell: u64,
    energy: f64,
    top_file: String,
}

fn compute_term_eigenvector(symbols: &[SymbolData]) -> TermEigenvector {
    let n = symbols.len() as f64;
    
    let mean_score = symbols.iter().map(|s| s.score).sum::<f64>() / n;
    
    let variance = symbols.iter()
        .map(|s| (s.score - mean_score).powi(2))
        .sum::<f64>() / n;
    let std_dev = variance.sqrt();
    
    // Find dominant cell (most common)
    let mut cell_counts: HashMap<u64, usize> = HashMap::new();
    for sym in symbols {
        *cell_counts.entry(sym.cell).or_insert(0) += 1;
    }
    let dominant_cell = cell_counts.iter()
        .max_by_key(|(_, &count)| count)
        .map(|(&cell, _)| cell)
        .unwrap_or(0);
    
    // Energy = sum of scores
    let energy = symbols.iter().map(|s| s.score).sum::<f64>();
    
    // Top file = file with highest total score
    let mut file_scores: HashMap<String, f64> = HashMap::new();
    for sym in symbols {
        *file_scores.entry(sym.file.clone()).or_insert(0.0) += sym.score;
    }
    let top_file = file_scores.iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(file, _)| file.clone())
        .unwrap_or_default();
    
    TermEigenvector {
        mean_score,
        std_dev,
        dominant_cell,
        energy,
        top_file,
    }
}

fn compute_eigenvector_similarity(ev1: &TermEigenvector, ev2: &TermEigenvector) -> f64 {
    // Similarity based on score distribution and energy
    let score_sim = 1.0 / (1.0 + (ev1.mean_score - ev2.mean_score).abs());
    let energy_ratio = ev1.energy.min(ev2.energy) / ev1.energy.max(ev2.energy);
    
    (score_sim + energy_ratio) / 2.0
}

fn extract_embedded_strings(symbol: &str) -> Option<String> {
    // Extract readable strings from mangled names
    let mut strings = Vec::new();
    
    // Look for common patterns
    let patterns = [
        "error", "panic", "assert", "format", "fmt",
        "alloc", "drop", "clone", "copy", "move",
        "print", "write", "read", "open", "close",
        "new", "init", "create", "destroy",
        "get", "set", "put", "take",
    ];
    
    for pattern in &patterns {
        if symbol.to_lowercase().contains(pattern) {
            strings.push(*pattern);
        }
    }
    
    // Extract words between underscores (limit to readable ones)
    for part in symbol.split('_') {
        if part.len() >= 3 && part.len() <= 15 
           && part.chars().all(|c| c.is_ascii_alphabetic()) 
           && !strings.contains(&part) {
            strings.push(part);
        }
    }
    
    if strings.is_empty() {
        None
    } else {
        Some(strings.into_iter().take(5).collect::<Vec<_>>().join(", "))
    }
}
