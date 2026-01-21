// Extract function names from top IPs using goblin ELF reader
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <perf.data>", args[0]);
        std::process::exit(1);
    }
    
    let perf_file = &args[1];
    
    println!("🔍 Extracting function names from top IPs\n");
    
    // Get top IPs
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file, "-F", "ip,dso"])
        .output()
        .expect("Failed to run perf script");
    
    let script_output = String::from_utf8_lossy(&output.stdout);
    
    // Count IPs and track DSO
    let mut ip_info: HashMap<String, (usize, String)> = HashMap::new();
    
    for line in script_output.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let ip = parts[0];
            let dso = parts[1];
            let entry = ip_info.entry(ip.to_string()).or_insert((0, dso.to_string()));
            entry.0 += 1;
        }
    }
    
    // Sort by count
    let mut sorted: Vec<_> = ip_info.iter().collect();
    sorted.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    
    println!("| IP Address       | Count   | Binary/DSO | Symbol |");
    println!("|------------------|---------|------------|--------|");
    
    for (ip, (count, dso)) in sorted.iter().take(20) {
        // Try to resolve symbol
        let symbol = if dso.contains(".so") || dso.contains("bin/") {
            resolve_symbol(ip, dso)
        } else {
            "?".to_string()
        };
        
        let short_dso = dso.split('/').last().unwrap_or(dso);
        println!("| {:16} | {:7} | {:10} | {} |", 
            &ip[..16.min(ip.len())], count, 
            &short_dso[..10.min(short_dso.len())], symbol);
    }
}

fn resolve_symbol(ip_str: &str, dso: &str) -> String {
    // Parse IP
    let ip = u64::from_str_radix(ip_str.trim_start_matches("0x"), 16).unwrap_or(0);
    
    // Try to read ELF
    if let Ok(buffer) = fs::read(dso) {
        if let Ok(elf) = Elf::parse(&buffer) {
            // Find closest symbol
            for sym in elf.syms.iter() {
                if sym.st_value <= ip && ip < sym.st_value + sym.st_size {
                    if let Some(name) = elf.strtab.get_at(sym.st_name) {
                        return name.to_string();
                    }
                }
            }
        }
    }
    
    "?".to_string()
}
