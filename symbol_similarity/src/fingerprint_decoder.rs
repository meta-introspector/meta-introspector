use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting binary fingerprints from mangled names...\n");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    // Collect mangled symbols with their resonance patterns
    let mut rust_mangled: HashMap<String, Vec<(u64, f64)>> = HashMap::new();
    let mut cpp_mangled: HashMap<String, Vec<(u64, f64)>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let cell = row.get_ulong(2)?;
        let score = row.get_double(4)?;
        
        if name.contains("_RNv") || name.starts_with("_ZN") && name.contains("rust") {
            // Rust mangled - extract pattern
            let pattern = extract_rust_pattern(&name);
            rust_mangled.entry(pattern).or_insert_with(Vec::new).push((cell, score));
        } else if name.starts_with("_Z") {
            // C++ mangled - extract pattern
            let pattern = extract_cpp_pattern(&name);
            cpp_mangled.entry(pattern).or_insert_with(Vec::new).push((cell, score));
        }
    }
    
    println!("✅ Found {} unique Rust patterns", rust_mangled.len());
    println!("✅ Found {} unique C++ patterns\n", cpp_mangled.len());
    
    // Compute binary fingerprints for each pattern
    println!("🔬 Computing binary fingerprints...");
    
    let mut rust_fingerprints: Vec<(String, Fingerprint)> = rust_mangled.iter()
        .filter(|(_, v)| v.len() >= 5)
        .map(|(pattern, cells)| {
            let fp = compute_fingerprint(cells);
            (pattern.clone(), fp)
        })
        .collect();
    
    rust_fingerprints.sort_by(|a, b| b.1.uniqueness.partial_cmp(&a.1.uniqueness).unwrap());
    
    let mut cpp_fingerprints: Vec<(String, Fingerprint)> = cpp_mangled.iter()
        .filter(|(_, v)| v.len() >= 5)
        .map(|(pattern, cells)| {
            let fp = compute_fingerprint(cells);
            (pattern.clone(), fp)
        })
        .collect();
    
    cpp_fingerprints.sort_by(|a, b| b.1.uniqueness.partial_cmp(&a.1.uniqueness).unwrap());
    
    println!("\n🎯 Top 30 Rust mangling patterns by fingerprint uniqueness:");
    for (i, (pattern, fp)) in rust_fingerprints.iter().take(30).enumerate() {
        println!("   {}. {} → fingerprint: {}", i + 1, pattern, fp);
    }
    
    println!("\n🎯 Top 30 C++ mangling patterns by fingerprint uniqueness:");
    for (i, (pattern, fp)) in cpp_fingerprints.iter().take(30).enumerate() {
        println!("   {}. {} → fingerprint: {}", i + 1, pattern, fp);
    }
    
    // Build decoder mapping
    println!("\n🔓 Building mangling decoder from binary fingerprints...");
    
    let mut decoder = HashMap::new();
    for (pattern, fp) in &rust_fingerprints {
        if fp.uniqueness > 0.8 {
            decoder.insert(fp.signature.clone(), pattern.clone());
        }
    }
    
    println!("   {} unique Rust patterns with >0.8 uniqueness", decoder.len());
    
    // Save results
    let mut output = String::from("Binary Fingerprint Decoder\n\n");
    
    output.push_str("Rust Mangling Patterns:\n");
    for (i, (pattern, fp)) in rust_fingerprints.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} → {}\n", i + 1, pattern, fp));
    }
    
    output.push_str("\nC++ Mangling Patterns:\n");
    for (i, (pattern, fp)) in cpp_fingerprints.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} → {}\n", i + 1, pattern, fp));
    }
    
    output.push_str("\nDecoder Mapping (uniqueness > 0.8):\n");
    for (sig, pattern) in &decoder {
        output.push_str(&format!("{} → {}\n", sig, pattern));
    }
    
    fs::write("binary_fingerprint_decoder.txt", output)?;
    println!("\n💾 Saved to binary_fingerprint_decoder.txt");
    
    Ok(())
}

#[derive(Debug, Clone)]
struct Fingerprint {
    signature: String,
    mean_score: f64,
    std_dev: f64,
    cell_distribution: Vec<u64>,
    uniqueness: f64,
}

impl std::fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sig={}, μ={:.3}, σ={:.3}, uniq={:.3}", 
               self.signature, self.mean_score, self.std_dev, self.uniqueness)
    }
}

fn compute_fingerprint(cells: &[(u64, f64)]) -> Fingerprint {
    let scores: Vec<f64> = cells.iter().map(|(_, s)| *s).collect();
    let cell_ids: Vec<u64> = cells.iter().map(|(c, _)| *c).collect();
    
    let mean = scores.iter().sum::<f64>() / scores.len() as f64;
    let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
    let std_dev = variance.sqrt();
    
    // Compute cell distribution histogram (10 bins)
    let max_cell = *cell_ids.iter().max().unwrap_or(&0);
    let bin_size = (max_cell / 10).max(1);
    let mut histogram = vec![0u64; 10];
    
    for &cell in &cell_ids {
        let bin = ((cell / bin_size) as usize).min(9);
        histogram[bin] += 1;
    }
    
    // Uniqueness score based on std_dev and distribution
    let distribution_entropy = histogram.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / cell_ids.len() as f64;
            -p * p.log2()
        })
        .sum::<f64>();
    
    let uniqueness = (std_dev / (mean + 1.0)) * (distribution_entropy / 3.32); // normalize by max entropy
    
    // Create signature from histogram
    let signature = histogram.iter()
        .map(|c| format!("{:x}", c))
        .collect::<Vec<_>>()
        .join("");
    
    Fingerprint {
        signature,
        mean_score: mean,
        std_dev,
        cell_distribution: histogram,
        uniqueness,
    }
}

fn extract_rust_pattern(name: &str) -> String {
    // Extract Rust mangling components
    let mut pattern = String::new();
    
    if name.contains("_RNv") {
        pattern.push_str("RNv:");
        // Extract crate hash
        if let Some(start) = name.find("Cs") {
            if let Some(end) = name[start..].find("_") {
                pattern.push_str(&name[start..start+end]);
            }
        }
        // Extract module depth
        let module_count = name.matches("Nt").count();
        pattern.push_str(&format!(":M{}", module_count));
    } else if name.contains("_ZN") {
        pattern.push_str("ZN:");
        // Extract namespace depth
        let ns_count = name.matches("N").count();
        pattern.push_str(&format!(":N{}", ns_count));
    }
    
    // Extract type indicators
    if name.contains("$LT$") { pattern.push_str(":generic"); }
    if name.contains("$GT$") { pattern.push_str(":close"); }
    if name.contains("$u20$") { pattern.push_str(":space"); }
    if name.contains("17h") { pattern.push_str(":hash"); }
    
    pattern
}

fn extract_cpp_pattern(name: &str) -> String {
    let mut pattern = String::new();
    
    // Extract C++ mangling structure
    if name.starts_with("_ZN") {
        pattern.push_str("ZN:");
        
        // Count nested namespaces
        let digits: String = name.chars().skip(3).take_while(|c| c.is_numeric()).collect();
        if !digits.is_empty() {
            pattern.push_str(&format!("L{}", digits));
        }
    } else if name.starts_with("_Z") {
        pattern.push_str("Z:");
    }
    
    // Type indicators
    if name.contains("St") { pattern.push_str(":std"); }
    if name.contains("basic_string") { pattern.push_str(":string"); }
    if name.contains("vector") { pattern.push_str(":vector"); }
    if name.contains("allocator") { pattern.push_str(":alloc"); }
    
    pattern
}
