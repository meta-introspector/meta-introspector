//! Compute LMFDB orbit from perf trace
//! Maps execution trace to elliptic curve orbit

use std::env;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LMFDBOrbit {
    /// LMFDB orbit label (e.g., "11.a1")
    orbit: String,
    
    /// Conductor
    conductor: u64,
    
    /// Rank
    rank: u64,
    
    /// Torsion structure
    torsion: Vec<u64>,
    
    /// Hash of perf trace
    trace_hash: String,
    
    /// Galois field
    galois_field: String,
    
    /// Coverage
    coverage: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: compute-orbit <perf.data> [perf.data...]");
        std::process::exit(1);
    }
    
    // Parse all perf traces
    let mut trace_data = Vec::new();
    for perf_file in &args[1..] {
        let data = parse_perf_trace(perf_file);
        trace_data.extend(data);
    }
    
    // Compute hash of trace
    let trace_hash = compute_trace_hash(&trace_data);
    
    // Map to LMFDB orbit via arithmetization
    let orbit = arithmetize_to_orbit(&trace_data, &trace_hash);
    
    // Output JSON
    let json = serde_json::to_string_pretty(&orbit).unwrap();
    println!("{}", json);
}

fn parse_perf_trace(path: &str) -> Vec<u64> {
    use std::process::Command;
    
    let output = Command::new("perf")
        .args(&["script", "-i", path])
        .output()
        .expect("Failed to run perf script");
    
    let content = String::from_utf8_lossy(&output.stdout);
    
    let mut data = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        // Extract instruction pointer
        let parts: Vec<&str> = line.split_whitespace().collect();
        for part in parts {
            if let Ok(ip) = u64::from_str_radix(part.trim_start_matches("0x"), 16) {
                data.push(ip);
                break;
            }
        }
    }
    
    data
}

fn compute_trace_hash(data: &[u64]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn arithmetize_to_orbit(data: &[u64], trace_hash: &str) -> LMFDBOrbit {
    // Compute conductor from trace size
    let conductor = next_prime(data.len() as u64);
    
    // Compute rank from unique values
    let unique_count = data.iter().collect::<std::collections::HashSet<_>>().len();
    let rank = (unique_count as f64).log2().floor() as u64;
    
    // Compute torsion from hash
    let hash_val = u64::from_str_radix(&trace_hash[..8], 16).unwrap_or(0);
    let torsion = vec![hash_val % 12 + 1]; // Torsion order 1-12
    
    // Determine Galois field from data size
    let bits = (data.len() as f64).log2().ceil() as u32;
    let galois_field = format!("GF(2^{})", bits);
    
    // Compute coverage
    let coverage = unique_count as f64 / data.len() as f64;
    
    // Map to LMFDB orbit label
    let orbit = format!("{}.a{}", conductor, rank + 1);
    
    LMFDBOrbit {
        orbit,
        conductor,
        rank,
        torsion,
        trace_hash: trace_hash.to_string(),
        galois_field,
        coverage,
    }
}

fn next_prime(n: u64) -> u64 {
    let mut candidate = n;
    loop {
        if is_prime(candidate) {
            return candidate;
        }
        candidate += 1;
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}
