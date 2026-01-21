// Analyze instruction pointer bit patterns and spectrum

use std::collections::HashMap;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <perf.data>", args[0]);
        std::process::exit(1);
    }
    
    let perf_file = &args[1];
    
    println!("🔬 Instruction Pointer Spectrum Analysis\n");
    
    // Get all IPs
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file, "-F", "ip"])
        .output()
        .expect("Failed to run perf script");
    
    let script_output = String::from_utf8_lossy(&output.stdout);
    
    let mut ips: Vec<u64> = Vec::new();
    for line in script_output.lines() {
        if let Ok(ip) = u64::from_str_radix(line.trim().trim_start_matches("0x"), 16) {
            ips.push(ip);
        }
    }
    
    println!("📊 Total IPs: {}", ips.len());
    println!("📊 Unique IPs: {}\n", ips.iter().collect::<std::collections::HashSet<_>>().len());
    
    // Analyze bit patterns
    println!("🔢 Bit Pattern Analysis:");
    
    // Count set bits per IP
    let mut bit_counts: HashMap<u32, usize> = HashMap::new();
    for ip in &ips {
        let bits = ip.count_ones();
        *bit_counts.entry(bits).or_insert(0) += 1;
    }
    
    println!("\n  Hamming Weight Distribution:");
    let mut sorted: Vec<_> = bit_counts.iter().collect();
    sorted.sort_by_key(|&(k, _)| k);
    for (bits, count) in sorted {
        let pct = *count as f64 / ips.len() as f64 * 100.0;
        println!("    {} bits set: {:6} samples ({:.1}%)", bits, count, pct);
    }
    
    // Analyze address space usage
    println!("\n📍 Address Space Usage:");
    
    let user_space = ips.iter().filter(|&&ip| ip < 0x8000_0000_0000_0000).count();
    let kernel_space = ips.iter().filter(|&&ip| ip >= 0x8000_0000_0000_0000).count();
    
    println!("  User space:   {:6} ({:.1}%)", user_space, user_space as f64 / ips.len() as f64 * 100.0);
    println!("  Kernel space: {:6} ({:.1}%)", kernel_space, kernel_space as f64 / ips.len() as f64 * 100.0);
    
    // Analyze high bits
    println!("\n🎯 High Bits Analysis (bits 48-63):");
    let mut high_bits: HashMap<u16, usize> = HashMap::new();
    for ip in &ips {
        let high = (ip >> 48) as u16;
        *high_bits.entry(high).or_insert(0) += 1;
    }
    
    let mut sorted: Vec<_> = high_bits.iter().collect();
    sorted.sort_by_key(|&(_, v)| std::cmp::Reverse(*v));
    for (bits, count) in sorted.iter().take(10) {
        println!("  0x{:04x}...: {:6} samples", bits, count);
    }
    
    // Analyze instruction alignment
    println!("\n⚙️  Instruction Alignment:");
    let aligned_1 = ips.iter().filter(|&&ip| ip % 1 == 0).count();
    let aligned_2 = ips.iter().filter(|&&ip| ip % 2 == 0).count();
    let aligned_4 = ips.iter().filter(|&&ip| ip % 4 == 0).count();
    let aligned_8 = ips.iter().filter(|&&ip| ip % 8 == 0).count();
    let aligned_16 = ips.iter().filter(|&&ip| ip % 16 == 0).count();
    
    println!("  1-byte:  100.0%");
    println!("  2-byte:  {:.1}%", aligned_2 as f64 / ips.len() as f64 * 100.0);
    println!("  4-byte:  {:.1}%", aligned_4 as f64 / ips.len() as f64 * 100.0);
    println!("  8-byte:  {:.1}%", aligned_8 as f64 / ips.len() as f64 * 100.0);
    println!("  16-byte: {:.1}%", aligned_16 as f64 / ips.len() as f64 * 100.0);
}
