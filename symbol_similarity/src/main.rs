use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading symbols from Parquet...");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut symbols: Vec<(String, String, u64, f64)> = Vec::new();
    let mut symbol_index: HashMap<String, Vec<usize>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file = row.get_string(1)?.to_string();
        let cell = row.get_ulong(2)?;
        let score = row.get_double(4)?;
        
        let idx = symbols.len();
        symbols.push((name.clone(), file, cell, score));
        symbol_index.entry(name).or_insert_with(Vec::new).push(idx);
        
        if (idx + 1) % 100000 == 0 {
            println!("   Loaded {} symbols", idx + 1);
        }
    }
    
    println!("✅ Loaded {} symbols", symbols.len());
    println!("   {} unique symbol names\n", symbol_index.len());
    
    // Find symbols appearing in multiple files
    let mut multi_file: Vec<(String, usize)> = symbol_index.iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(k, v)| (k.clone(), v.len()))
        .collect();
    multi_file.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("🔍 Top 30 symbols by file count:");
    for (i, (name, count)) in multi_file.iter().take(30).enumerate() {
        println!("   {}. {} ({} files)", i + 1, name, count);
    }
    
    // Calculate similarity for top symbols
    println!("\n📈 Computing symbol similarity...");
    let top_50: Vec<String> = multi_file.iter().take(50).map(|(n, _)| n.clone()).collect();
    
    let mut similarities = Vec::new();
    for i in 0..top_50.len() {
        for j in (i+1)..top_50.len() {
            let scores1: Vec<f64> = symbol_index[&top_50[i]].iter()
                .map(|&idx| symbols[idx].3).collect();
            let scores2: Vec<f64> = symbol_index[&top_50[j]].iter()
                .map(|&idx| symbols[idx].3).collect();
            
            let sim = cosine_similarity(&scores1, &scores2);
            if sim > 0.3 {
                similarities.push((top_50[i].clone(), top_50[j].clone(), sim));
            }
        }
        if (i + 1) % 10 == 0 {
            println!("   Processed {}/{} symbols", i + 1, top_50.len());
        }
    }
    
    similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    println!("\n🎯 Top 30 similar symbol pairs:");
    for (i, (s1, s2, sim)) in similarities.iter().take(30).enumerate() {
        println!("   {}. {} <-> {} ({:.4})", i + 1, s1, s2, sim);
    }
    
    // Save results
    let mut output = format!("Symbol Similarity Analysis (from Parquet)\n\n");
    output.push_str(&format!("Total symbols: {}\n", symbols.len()));
    output.push_str(&format!("Unique names: {}\n\n", symbol_index.len()));
    
    output.push_str("Top 100 symbols by file count:\n");
    for (i, (name, count)) in multi_file.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ({} files)\n", i + 1, name, count));
    }
    
    output.push_str("\nTop 100 similar symbol pairs:\n");
    for (i, (s1, s2, sim)) in similarities.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} <-> {} ({:.4})\n", i + 1, s1, s2, sim));
    }
    
    fs::write("symbol_similarity_results.txt", output)?;
    println!("\n💾 Saved to symbol_similarity_results.txt");
    
    Ok(())
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() { return 0.0; }
    let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 { return 0.0; }
    dot / (mag_a * mag_b)
}
