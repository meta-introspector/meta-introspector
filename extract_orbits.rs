use std::collections::HashMap;
use std::process::Command;

fn main() {
    println!("🌀 Extracting Strongest Orbits from Bootstrap");
    println!("==============================================\n");
    
    // Find first perf data
    let perf_files = find_perf_data();
    if perf_files.is_empty() {
        println!("No perf data found. Run ./bootstrap.sh first.");
        return;
    }
    
    let first = &perf_files[0];
    println!("Analyzing: {}\n", first);
    
    // Extract IP trace
    let ips = extract_ip_trace(first);
    println!("Extracted {} instruction pointers\n", ips.len());
    
    // Find orbits (loops)
    let orbits = find_orbits(&ips);
    println!("Found {} orbits\n", orbits.len());
    
    // Sort by strength (frequency × period)
    let mut scored: Vec<_> = orbits.iter()
        .map(|o| (o, o.frequency * o.period as u64))
        .collect();
    scored.sort_by_key(|x| std::cmp::Reverse(x.1));
    
    println!("🎯 Top 10 Strongest Orbits:");
    println!("===========================\n");
    
    for (i, (orbit, strength)) in scored.iter().take(10).enumerate() {
        println!("{}. IP: 0x{:x}", i+1, orbit.start_ip);
        println!("   Period: {} instructions", orbit.period);
        println!("   Frequency: {} times", orbit.frequency);
        println!("   Strength: {}", strength);
        
        // Check resonance with ZOS primes
        let resonances = check_resonance(orbit.period);
        if !resonances.is_empty() {
            println!("   Resonates: {:?}", resonances);
        }
        println!();
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

fn extract_ip_trace(perf_path: &str) -> Vec<u64> {
    let output = Command::new("perf")
        .args(&["script", "-i", perf_path, "-F", "ip"])
        .output()
        .expect("perf script failed");
    
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("0x") || trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
                u64::from_str_radix(trimmed.trim_start_matches("0x"), 16).ok()
            } else {
                None
            }
        })
        .collect()
}

fn find_orbits(ips: &[u64]) -> Vec<Orbit> {
    let mut orbits = Vec::new();
    let mut last_seen: HashMap<u64, Vec<usize>> = HashMap::new();
    
    for (t, &ip) in ips.iter().enumerate() {
        if let Some(positions) = last_seen.get_mut(&ip) {
            // Found a loop - IP returned to same value
            if let Some(&t0) = positions.last() {
                let period = t - t0;
                if period > 0 && period < 10000 {  // Reasonable orbit size
                    orbits.push(Orbit {
                        start_ip: ip,
                        period,
                        frequency: 1,
                    });
                }
            }
            positions.push(t);
        } else {
            last_seen.insert(ip, vec![t]);
        }
    }
    
    // Merge duplicate orbits and count frequency
    let mut merged: HashMap<(u64, usize), u64> = HashMap::new();
    for orbit in orbits {
        *merged.entry((orbit.start_ip, orbit.period)).or_insert(0) += 1;
    }
    
    merged.into_iter()
        .map(|((ip, period), freq)| Orbit {
            start_ip: ip,
            period,
            frequency: freq,
        })
        .collect()
}

fn check_resonance(period: usize) -> Vec<u64> {
    const ZOS: &[u64] = &[2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71];
    ZOS.iter()
        .filter(|&&p| period % p as usize == 0)
        .copied()
        .collect()
}

#[derive(Debug)]
struct Orbit {
    start_ip: u64,
    period: usize,
    frequency: u64,
}
