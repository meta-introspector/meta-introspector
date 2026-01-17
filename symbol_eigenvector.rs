use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading symbols from Parquet...");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut symbols: Vec<(String, f64)> = Vec::new();
    let mut symbol_index: HashMap<String, Vec<usize>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let score = row.get_double(4)?;
        
        let idx = symbols.len();
        symbols.push((name.clone(), score));
        symbol_index.entry(name).or_default().push(idx);
        
        if (idx + 1).is_multiple_of(200000) {
            println!("   Loaded {} symbols", idx + 1);
        }
    }
    
    println!("✅ Loaded {} symbols, {} unique names", symbols.len(), symbol_index.len());
    
    // Select top N symbols by frequency for matrix
    let mut multi_file: Vec<(String, usize)> = symbol_index.iter()
        .filter(|(_, v)| v.len() > 10)
        .map(|(k, v)| (k.clone(), v.len()))
        .collect();
    multi_file.sort_by(|a, b| b.1.cmp(&a.1));
    
    let n = 500; // Top 500 symbols
    let top_symbols: Vec<String> = multi_file.iter().take(n).map(|(s, _)| s.clone()).collect();
    
    println!("\n📈 Building {}x{} similarity matrix for top symbols...", n, n);
    let mut matrix = vec![vec![0.0f64; n]; n];
    
    for i in 0..n {
        let scores_i: Vec<f64> = symbol_index[&top_symbols[i]].iter()
            .map(|&idx| symbols[idx].1).collect();
        
        for j in i..n {
            let scores_j: Vec<f64> = symbol_index[&top_symbols[j]].iter()
                .map(|&idx| symbols[idx].1).collect();
            
            let sim = cosine_similarity(&scores_i, &scores_j);
            matrix[i][j] = sim;
            matrix[j][i] = sim;
        }
        
        if (i + 1) % 50 == 0 {
            println!("   Computed {}/{} rows", i + 1, n);
        }
    }
    
    println!("\n🔢 Computing dominant eigenvector (power iteration)...");
    let mut v: Vec<f64> = (0..n).map(|i| (i as f64).sin()).collect();
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    v.iter_mut().for_each(|x| *x /= norm);
    
    let mut eigenvalue = 0.0;
    for iter in 0..100 {
        let mut v_new = vec![0.0; n];
        for i in 0..n {
            v_new[i] = matrix[i].iter().zip(&v).map(|(a, b)| a * b).sum();
        }
        
        let new_eigenvalue: f64 = v_new.iter().zip(&v).map(|(a, b)| a * b).sum();
        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        v_new.iter_mut().for_each(|x| *x /= norm);
        
        let diff = (new_eigenvalue - eigenvalue).abs();
        if diff < 1e-6 {
            println!("   ✓ Converged after {} iterations", iter + 1);
            eigenvalue = new_eigenvalue;
            v = v_new;
            break;
        }
        
        eigenvalue = new_eigenvalue;
        v = v_new;
        
        if (iter + 1) % 10 == 0 {
            println!("   Iteration {}: eigenvalue = {:.6}", iter + 1, eigenvalue);
        }
    }
    
    println!("\n✅ Dominant eigenvalue: {:.6}", eigenvalue);
    
    let mut components: Vec<(usize, f64)> = v.iter().enumerate()
        .map(|(i, &val)| (i, val.abs())).collect();
    components.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🎯 Top 30 symbols by eigenvector centrality:");
    for (rank, (idx, val)) in components.iter().take(30).enumerate() {
        let sym = &top_symbols[*idx];
        let count = symbol_index[sym].len();
        println!("   {}. {} ({} files, centrality: {:.6})", rank + 1, sym, count, val);
    }
    
    // Save results
    let mut output = "Symbol Eigenvector Analysis\n\n".to_string();
    output.push_str(&format!("Matrix size: {}x{}\n", n, n));
    output.push_str(&format!("Dominant eigenvalue: {:.6}\n\n", eigenvalue));
    
    output.push_str("Top 100 symbols by eigenvector centrality:\n");
    for (rank, (idx, val)) in components.iter().take(100).enumerate() {
        let sym = &top_symbols[*idx];
        let count = symbol_index[sym].len();
        output.push_str(&format!("{}. {} ({} files, {:.6})\n", rank + 1, sym, count, val));
    }
    
    fs::write("symbol_eigenvector_results.txt", output)?;
    println!("\n💾 Saved to symbol_eigenvector_results.txt");
    
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
