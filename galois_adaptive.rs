// Adaptive Galois scanner - start at break point, expand as needed
use linux_perf_data::{PerfFileReader, PerfFileRecord};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;

fn main() -> Result<()> {
    let perf_file = std::env::args().nth(1).expect("Usage: galois_adaptive <perf.data>");
    
    println!("🎯 Adaptive Galois Scanner (start at break point)");
    
    let file = File::open(&perf_file)?;
    let reader = BufReader::new(file);
    let PerfFileReader { mut perf_file, mut record_iter } = PerfFileReader::parse_file(reader)?;
    
    let mut fields: Vec<(u32, HashSet<u64>)> = vec![(18, HashSet::new()), (19, HashSet::new())];
    let mut sample_count = 0u64;
    let mut next_lower = 17;
    let mut next_higher = 20;
    
    while let Some(_) = record_iter.next_record(&mut perf_file)? {
        sample_count += 1;
        
        fields.retain_mut(|(bits, seen)| {
            let size = 1u64 << bits;
            seen.insert(sample_count % size);
            
            if seen.len() as u64 == size {
                println!("✅ GF(2^{}): 100% - adding GF(2^{})", bits, next_higher);
                fields.push((next_higher, HashSet::new()));
                next_higher += 1;
                false
            } else {
                true
            }
        });
        
        if fields.is_empty() { break; }
    }
    
    println!("\n📊 Results ({} samples):", sample_count);
    for (bits, seen) in fields {
        let size = 1u64 << bits;
        println!("  GF(2^{}): {:.6}%", bits, seen.len() as f64 / size as f64 * 100.0);
    }
    
    Ok(())
}
