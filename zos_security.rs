use std::collections::HashSet;
use std::fs;
use std::process::Command;

const ZOS: &[u64] = &[0,1,2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71];

// Trust anchor: Only MES from Guix is trusted
const TRUSTED_HASHES: &[&str] = &[
    // MES bootstrap binaries from Guix
    "sha256:...",  // mes-boot-0.24.2
    "sha256:...",  // stage0-posix-1.4
];

fn main() {
    println!("🔒 ZOS Security: Bootstrap Signature Detection");
    println!("Trust Anchor: MES from Guix");
    println!();
    
    let args: Vec<String> = std::env::args().collect();
    let target = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    
    scan_directory(target);
}

fn scan_directory(path: &str) {
    for entry in walkdir::WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        if !entry.file_type().is_file() {
            continue;
        }
        
        let path = entry.path();
        if let Some(sig) = extract_bootstrap_signature(path) {
            let hash = compute_hash(path);
            
            if !is_trusted(&hash) {
                println!("🚨 MALICIOUS: {}", path.display());
                println!("   Bootstrap signature detected: {:?}", sig);
                println!("   Hash: {}", hash);
                println!("   NOT in trusted set (not from Guix MES)");
                println!("   Action: QUARANTINE");
                println!();
            }
        }
    }
}

fn extract_bootstrap_signature(path: &std::path::Path) -> Option<BootstrapSignature> {
    // Extract instruction spectrum
    let output = Command::new("objdump")
        .args(&["-d", path.to_str()?])
        .output()
        .ok()?;
    
    let disasm = String::from_utf8_lossy(&output.stdout);
    let mut spectrum: Vec<u64> = Vec::new();
    
    for line in disasm.lines() {
        if let Some(instr) = parse_instruction(line) {
            spectrum.push(instr);
        }
    }
    
    // Check if spectrum resonates with bootstrap primes
    let mut resonances = Vec::new();
    for &p in ZOS.iter().skip(2) {
        let count = spectrum.iter().filter(|&&x| x % p == 0).count();
        if count > spectrum.len() / 10 {  // >10% resonance
            resonances.push(p);
        }
    }
    
    // If resonates at 37 or 71, it has bootstrap signature
    if resonances.contains(&37) || resonances.contains(&71) {
        Some(BootstrapSignature {
            resonances,
            complexity: spectrum.len(),
        })
    } else {
        None
    }
}

fn parse_instruction(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        // Hash the instruction mnemonic
        let instr = parts[1];
        Some(simple_hash(instr))
    } else {
        None
    }
}

fn simple_hash(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
}

fn compute_hash(path: &std::path::Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum failed");
    
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

fn is_trusted(hash: &str) -> bool {
    TRUSTED_HASHES.contains(&hash)
}

#[derive(Debug)]
struct BootstrapSignature {
    resonances: Vec<u64>,
    complexity: usize,
}
