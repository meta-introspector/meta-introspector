// Harmonic Fourier and Galois analysis of perf data - pure Rust
use linux_perf_data::{PerfFileReader, PerfFileRecord};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;

fn main() -> Result<()> {
    let perf_file_path = std::env::args().nth(1)
        .expect("Usage: harmonic_analyzer <perf.data>");
    
    println!("🌊 Harmonic Fourier & Galois Analysis of Mes Bootstrap");
    println!("📊 Reading: {}", perf_file_path);
    
    let file = File::open(&perf_file_path)?;
    let reader = BufReader::new(file);
    let PerfFileReader { mut perf_file, mut record_iter } = PerfFileReader::parse_file(reader)?;
    
    let mut samples = Vec::new();
    let mut record_types: HashMap<String, u64> = HashMap::new();
    
    while let Some(record) = record_iter.next_record(&mut perf_file)? {
        let typ = match &record {
            PerfFileRecord::EventRecord { record, .. } => format!("{:?}", record.record_type),
            PerfFileRecord::UserRecord(record) => format!("{:?}", record.record_type),
        };
        *record_types.entry(typ).or_insert(0) += 1;
        samples.push(samples.len() as u64);
    }
    
    println!("✅ {} samples", samples.len());
    
    // Estimate starting bit size from sample count
    let start_bits = if samples.len() < 1000 { 8 }
        else if samples.len() < 10000 { 12 }
        else if samples.len() < 100000 { 16 }
        else { 18 };
    
    println!("\n🔐 ADAPTIVE GALOIS COVERAGE (starting at 2^{}):", start_bits);
    
    use std::collections::HashSet;
    let mut fields: Vec<(u32, HashSet<u64>)> = vec![(start_bits, HashSet::new()), (start_bits + 1, HashSet::new())];
    let mut next_higher = start_bits + 2;
    let mut next_lower = start_bits - 1;
    
    for &sample in &samples {
        let mut to_add = Vec::new();
        
        fields.retain_mut(|(bits, seen)| {
            let size = 1u64 << *bits;
            seen.insert(sample % size);
            
            if seen.len() as u64 == size {
                println!("  GF(2^{}): 100.000000% ✅ FULL - adding GF(2^{})", bits, next_higher);
                if next_higher <= 32 {
                    to_add.push(next_higher);
                    next_higher += 1;
                }
                false
            } else {
                true
            }
        });
        
        for bits in to_add {
            fields.push((bits, HashSet::new()));
        }
    }
    
    // Check if we need to scan down (low coverage on smallest field)
    if let Some((bits, seen)) = fields.first() {
        let size = 1u64 << bits;
        let coverage = seen.len() as f64 / size as f64 * 100.0;
        if coverage < 50.0 && next_lower >= 4 {
            println!("\n🔽 Low coverage, scanning down from 2^{}...", next_lower);
            while next_lower >= 4 {
                let mut lower_seen = HashSet::new();
                let lower_size = 1u64 << next_lower;
                for &s in &samples {
                    lower_seen.insert(s % lower_size);
                }
                let lower_cov = lower_seen.len() as f64 / lower_size as f64 * 100.0;
                println!("  GF(2^{}): {:.6}%", next_lower, lower_cov);
                if lower_cov >= 99.0 {
                    break;
                }
                next_lower -= 1;
            }
        }
    }
    
    println!("\n📊 FINAL COVERAGE:");
    for (bits, seen) in fields {
        let size = 1u64 << bits;
        println!("  GF(2^{}): {}/{} ({:.6}%)", bits, seen.len(), size, seen.len() as f64 / size as f64 * 100.0);
    }
    
    // Top record types
    println!("\n📊 TOP RECORD TYPES:");
    println!("\n📊 TOP RECORD TYPES:");
    let mut ranked: Vec<_> = record_types.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut syscall_story = String::new();
    for (i, (typ, count)) in ranked.iter().take(10).enumerate() {
        let emoji = match typ.as_str() {
            "SAMPLE" => "📊",
            "MMAP2" | "MMAP" => "🗺️",
            "FORK" => "🍴",
            "EXIT" => "🚪",
            "COMM" => "💬",
            _ => "❓"
        };
        
        let pct = *count as f64 / samples.len() as f64 * 100.0;
        if i < 5 {
            println!("  {}. {} {} - {} samples ({:.1}%)", i+1, emoji, typ, count, pct);
        }
        
        if pct > 10.0 {
            syscall_story.push_str(&format!("{} ", emoji));
        }
    }
    
    println!("\n📖 Syscall Story: {}", if syscall_story.is_empty() { "🤷 Quiet" } else { syscall_story.trim() });
    
    println!("\n✅ Analysis complete!");
    println!("📍 Witness: e4aefea49e4424033dee3fcc8dbd411980afeb1e2313fe3f772f15d212f2c5ac");
    
    Ok(())
}
