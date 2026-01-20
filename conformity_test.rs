use std::collections::HashMap;
use std::fs;
use std::process::Command;

fn main() {
    println!("🔬 ZOS Conformity Test - Comparing bootstrap runs");
    println!("==================================================\n");
    
    // Find all perf data in Nix store
    let perf_files = find_perf_data();
    
    if perf_files.len() < 2 {
        println!("Need at least 2 runs to compare. Found: {}", perf_files.len());
        println!("Run ./bootstrap.sh multiple times first.");
        return;
    }
    
    println!("Found {} bootstrap runs\n", perf_files.len());
    
    // Extract spectra from each run
    let mut spectra = Vec::new();
    for (i, path) in perf_files.iter().enumerate() {
        println!("Run {}: {}", i+1, path);
        if let Some(spectrum) = extract_spectrum(path) {
            spectra.push((i+1, spectrum));
        }
    }
    
    println!("\n📊 Conformity Analysis");
    println!("======================\n");
    
    // Compare each pair
    for i in 0..spectra.len() {
        for j in i+1..spectra.len() {
            let similarity = compute_similarity(&spectra[i].1, &spectra[j].1);
            println!("Run {} ↔ Run {}: {:.2}% similar", 
                spectra[i].0, spectra[j].0, similarity * 100.0);
        }
    }
    
    // Compute average similarity
    let mut total_sim = 0.0;
    let mut count = 0;
    for i in 0..spectra.len() {
        for j in i+1..spectra.len() {
            total_sim += compute_similarity(&spectra[i].1, &spectra[j].1);
            count += 1;
        }
    }
    
    let avg_similarity = if count > 0 { total_sim / count as f64 } else { 0.0 };
    
    println!("\n🎯 Conformity Score: {:.2}%", avg_similarity * 100.0);
    
    if avg_similarity > 0.95 {
        println!("✅ CONFORMS - Same modular form across runs");
    } else if avg_similarity > 0.80 {
        println!("⚠️  MOSTLY CONFORMS - Minor variations");
    } else {
        println!("❌ DOES NOT CONFORM - Significant differences");
    }
    
    println!("\n🌀 Modular Form Invariance");
    println!("==========================\n");
    
    // Check if all runs have same dominant symbols
    let dominant = extract_dominant(&spectra[0].1, 10);
    println!("Top 10 symbols from Run 1:");
    for (sym, weight) in &dominant {
        println!("  {:6.2}% {}", weight, sym);
    }
    
    println!("\nChecking invariance across runs...");
    for (run, spectrum) in &spectra[1..] {
        let mut matches = 0;
        for (sym, _) in &dominant {
            if spectrum.contains_key(sym) {
                matches += 1;
            }
        }
        println!("  Run {}: {}/10 dominant symbols match", run, matches);
    }
}

fn find_perf_data() -> Vec<String> {
    let output = Command::new("find")
        .args(&["/nix/store", "-name", "build.perf.data", "-type", "f"])
        .output()
        .expect("find failed");
    
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn extract_spectrum(perf_path: &str) -> Option<HashMap<String, f64>> {
    let output = Command::new("perf")
        .args(&["report", "-i", perf_path, "--stdio", "--no-children"])
        .output()
        .ok()?;
    
    let report = String::from_utf8_lossy(&output.stdout);
    let mut spectrum = HashMap::new();
    
    for line in report.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].ends_with('%') {
            if let Ok(weight) = parts[0].trim_end_matches('%').parse::<f64>() {
                let symbol = parts[1..].join(" ");
                spectrum.insert(symbol, weight);
            }
        }
    }
    
    Some(spectrum)
}

fn compute_similarity(s1: &HashMap<String, f64>, s2: &HashMap<String, f64>) -> f64 {
    let mut common = 0.0;
    let mut total = 0.0;
    
    for (sym, w1) in s1 {
        total += w1;
        if let Some(w2) = s2.get(sym) {
            common += w1.min(*w2);
        }
    }
    
    if total > 0.0 { common / total } else { 0.0 }
}

fn extract_dominant(spectrum: &HashMap<String, f64>, n: usize) -> Vec<(String, f64)> {
    let mut sorted: Vec<_> = spectrum.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    sorted.into_iter().take(n).collect()
}
