// Extract top instruction pointers and calculate their Galois numbers
use std::collections::HashMap;
use std::process::Command;
use std::path::Path;

fn find_galois_break(samples: &[usize]) -> u32 {
    use std::collections::HashSet;
    let start_bits = if samples.len() < 100 { 4 }
        else if samples.len() < 1000 { 8 }
        else { 12 };
    
    for bits in start_bits..=20 {
        let size = 1usize << bits;
        let mut seen = HashSet::new();
        for &s in samples {
            seen.insert(s % size);
        }
        let coverage = seen.len() as f64 / size as f64 * 100.0;
        if coverage < 99.0 {
            return bits;
        }
    }
    20
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <perf.data>", args[0]);
        std::process::exit(1);
    }
    
    let perf_file = &args[1];
    let lang = Path::new(perf_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    println!("🔍 Analyzing top IPs for: {}\n", lang);
    
    // Run perf script to get IP addresses
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file, "-F", "ip"])
        .output()
        .expect("Failed to run perf script");
    
    let script_output = String::from_utf8_lossy(&output.stdout);
    
    // Parse IPs and track sample indices
    let mut ip_samples: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (idx, line) in script_output.lines().enumerate() {
        let ip = line.trim();
        if !ip.is_empty() && ip != "0" {
            ip_samples.entry(ip.to_string())
                .or_insert_with(Vec::new)
                .push(idx);
        }
    }
    
    // Sort by sample count
    let mut ip_counts: Vec<_> = ip_samples.iter()
        .map(|(ip, samples)| (ip.clone(), samples.len(), samples.clone()))
        .collect();
    ip_counts.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("| IP Address       | Samples | Galois | Coverage |");
    println!("|------------------|---------|--------|----------|");
    
    for (ip, count, samples) in ip_counts.iter().take(10) {
        let galois_bits = find_galois_break(samples);
        let size = 1usize << galois_bits;
        let coverage = (samples.len() as f64 / size as f64 * 100.0).min(100.0);
        
        // Classify IP
        let ip_type = if ip.starts_with("ffff") {
            "🔴 kernel"
        } else if ip.starts_with("7fff") {
            "🟢 user"
        } else {
            "⚪ other"
        };
        
        println!("| {:16} | {:7} | GF(2^{:2}) | {:5.1}% {} |", 
            &ip[..16.min(ip.len())], count, galois_bits, coverage, ip_type);
    }
    
    println!("\n📊 Summary:");
    let total_samples: usize = ip_counts.iter().map(|(_, c, _)| c).sum();
    let top10_samples: usize = ip_counts.iter().take(10).map(|(_, c, _)| c).sum();
    println!("  Top 10 IPs cover {}/{} samples ({:.1}%)", 
        top10_samples, total_samples, 
        top10_samples as f64 / total_samples as f64 * 100.0);
}
