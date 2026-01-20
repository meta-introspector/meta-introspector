use std::collections::HashMap;
use std::process::Command;

fn main() {
    println!("🔬 Computing bootstrap eigenvector");
    
    // Parse perf data
    let output = Command::new("perf")
        .args(&["report", "-i", "bootstrap.perf.data", "--stdio", "--no-children", "--sort", "symbol"])
        .output()
        .expect("perf report failed");
    
    let report = String::from_utf8_lossy(&output.stdout);
    
    // Extract symbol frequencies
    let mut symbols: HashMap<String, f64> = HashMap::new();
    for line in report.lines() {
        if let Some(parts) = parse_perf_line(line) {
            symbols.insert(parts.0, parts.1);
        }
    }
    
    // Find dominant eigenvector (top patterns)
    let mut sorted: Vec<_> = symbols.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    println!("\n📊 Bootstrap Eigenvector (top 20):");
    for (i, (sym, weight)) in sorted.iter().take(20).enumerate() {
        println!("{:2}. {:6.2}% {}", i+1, weight, sym);
    }
    
    // Compute resonance with ZOS primes
    const ZOS: &[u64] = &[0,1,2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71];
    println!("\n🎯 ZOS Resonances:");
    for p in ZOS.iter().skip(2) {
        let resonance = symbols.values()
            .filter(|&&w| (w * 100.0) as u64 % p == 0)
            .count();
        if resonance > 0 {
            println!("  Prime {}: {} resonant symbols", p, resonance);
        }
    }
}

fn parse_perf_line(line: &str) -> Option<(String, f64)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 && parts[0].ends_with('%') {
        let weight = parts[0].trim_end_matches('%').parse().ok()?;
        let symbol = parts[1..].join(" ");
        Some((symbol, weight))
    } else {
        None
    }
}
