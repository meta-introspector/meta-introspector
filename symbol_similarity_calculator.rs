use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    file: String,
    cell: usize,
    score: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading symbol scores...");
    
    let data = fs::read_to_string("markov_symbol_scores.json")?;
    println!("   Parsing JSON ({:.1} MB)...", data.len() as f64 / (1024.0 * 1024.0));
    
    // Parse JSON manually (simple approach)
    let mut symbols = Vec::new();
    let mut symbol_index: HashMap<String, Vec<usize>> = HashMap::new();
    
    // Simple JSON parsing for our structure
    for (idx, line) in data.lines().enumerate() {
        if line.contains("\"name\"") {
            let name = extract_field(line, "name");
            let file = extract_field(data.lines().nth(idx + 1).unwrap_or(""), "file");
            let cell = extract_number(data.lines().nth(idx + 2).unwrap_or(""), "cell");
            let score = extract_float(data.lines().nth(idx + 4).unwrap_or(""), "score");
            
            if !name.is_empty() {
                let sym_idx = symbols.len();
                symbols.push(Symbol { name: name.clone(), file, cell, score });
                symbol_index.entry(name).or_default().push(sym_idx);
            }
        }
        
        if (idx + 1) % 100000 == 0 {
            println!("   Parsed {} lines, {} symbols", idx + 1, symbols.len());
        }
    }
    
    println!("✅ Loaded {} symbols", symbols.len());
    println!("   {} unique symbol names", symbol_index.len());
    
    // Find symbols that appear in multiple files
    let mut multi_file_symbols: Vec<(String, usize)> = symbol_index.iter()
        .filter(|(_, indices)| indices.len() > 1)
        .map(|(name, indices)| (name.clone(), indices.len()))
        .collect();
    
    multi_file_symbols.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🔍 Top 20 symbols appearing in most files:");
    for (rank, (name, count)) in multi_file_symbols.iter().take(20).enumerate() {
        println!("   {}. {} ({} files)", rank + 1, name, count);
    }
    
    // Calculate symbol similarity based on resonance score patterns
    println!("\n📈 Computing symbol similarity for top symbols...");
    
    let top_symbols: Vec<String> = multi_file_symbols.iter()
        .take(100)
        .map(|(name, _)| name.clone())
        .collect();
    
    let mut similarity_pairs = Vec::new();
    
    for i in 0..top_symbols.len().min(50) {
        for j in (i+1)..top_symbols.len().min(50) {
            let sym1 = &top_symbols[i];
            let sym2 = &top_symbols[j];
            
            let indices1 = &symbol_index[sym1];
            let indices2 = &symbol_index[sym2];
            
            // Calculate similarity based on score distributions
            let scores1: Vec<f64> = indices1.iter().map(|&idx| symbols[idx].score).collect();
            let scores2: Vec<f64> = indices2.iter().map(|&idx| symbols[idx].score).collect();
            
            let sim = cosine_similarity(&scores1, &scores2);
            
            if sim > 0.5 {
                similarity_pairs.push((sym1.clone(), sym2.clone(), sim));
            }
        }
        
        if (i + 1) % 10 == 0 {
            println!("   Processed {}/50 symbols", i + 1);
        }
    }
    
    similarity_pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    println!("\n🎯 Top 20 most similar symbol pairs:");
    for (rank, (sym1, sym2, sim)) in similarity_pairs.iter().take(20).enumerate() {
        println!("   {}. {} <-> {} (similarity: {:.4})", rank + 1, sym1, sym2, sim);
    }
    
    // Save results
    let mut output = "Symbol Similarity Analysis\n".to_string();
    output.push_str(&format!("Total symbols: {}\n", symbols.len()));
    output.push_str(&format!("Unique names: {}\n\n", symbol_index.len()));
    
    output.push_str("Top 50 symbols by file count:\n");
    for (rank, (name, count)) in multi_file_symbols.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} ({} files)\n", rank + 1, name, count));
    }
    
    output.push_str("\nTop 50 similar symbol pairs:\n");
    for (rank, (sym1, sym2, sim)) in similarity_pairs.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} <-> {} ({:.4})\n", rank + 1, sym1, sym2, sim));
    }
    
    fs::write("symbol_similarity_results.txt", output)?;
    println!("\n💾 Saved results to symbol_similarity_results.txt");
    
    Ok(())
}

fn extract_field(line: &str, field: &str) -> String {
    if let Some(start) = line.find(&format!("\"{}\": \"", field)) {
        let start = start + field.len() + 5;
        if let Some(end) = line[start..].find("\"") {
            return line[start..start + end].to_string();
        }
    }
    String::new()
}

fn extract_number(line: &str, field: &str) -> usize {
    if let Some(start) = line.find(&format!("\"{}\": ", field)) {
        let start = start + field.len() + 4;
        if let Some(end) = line[start..].find(",") {
            return line[start..start + end].trim().parse().unwrap_or(0);
        }
    }
    0
}

fn extract_float(line: &str, field: &str) -> f64 {
    if let Some(start) = line.find(&format!("\"{}\": ", field)) {
        let start = start + field.len() + 4;
        let end_str = &line[start..];
        let end = end_str.find(|c: char| !c.is_numeric() && c != '.' && c != '-').unwrap_or(end_str.len());
        return end_str[..end].trim().parse().unwrap_or(0.0);
    }
    0.0
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() { return 0.0; }
    
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if mag_a == 0.0 || mag_b == 0.0 { return 0.0; }
    dot / (mag_a * mag_b)
}
