// Rust-based perf data decoder using linux-perf-data + goblin
// Reads perf.data and ranks symbols by actual runtime usage

use linux_perf_data::{AttributeDescription, PerfFileReader, PerfFileRecord};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;

fn main() -> Result<()> {
    let perf_file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/perf_rankings/nix_rust_beta_1768351567.perf.data".to_string());
    
    println!("🔬 Rust-based perf.data decoder");
    println!("📊 Reading: {}", perf_file_path);
    
    let file = File::open(&perf_file_path)?;
    let reader = BufReader::new(file);
    
    let PerfFileReader { mut perf_file, mut record_iter } =
        PerfFileReader::parse_file(reader)?;
    
    // List event names
    let event_names: Vec<_> = perf_file
        .event_attributes()
        .iter()
        .filter_map(AttributeDescription::name)
        .map(|s| s.to_string())
        .collect();
    println!("✅ Events: {}", event_names.join(", "));
    
    // Count symbols
    let mut symbol_counts: HashMap<String, u64> = HashMap::new();
    let mut total_samples = 0u64;
    
    println!("\n📈 Processing records...");
    
    while let Some(record) = record_iter.next_record(&mut perf_file)? {
        total_samples += 1;
        
        // Extract symbol from record type
        let symbol = match &record {
            PerfFileRecord::EventRecord { record, .. } => {
                format!("{:?}", record.record_type)
            }
            PerfFileRecord::UserRecord(record) => {
                format!("{:?}", record.record_type)
            }
        };
        
        *symbol_counts.entry(symbol).or_insert(0) += 1;
    }
    
    println!("✅ Processed {} samples", total_samples);
    println!("✅ Found {} unique symbols", symbol_counts.len());
    
    // Rank by count
    let mut ranked: Vec<_> = symbol_counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🔥 Top 50 symbols by sample count:\n");
    for (i, (symbol, count)) in ranked.iter().take(50).enumerate() {
        let priority = if *count > 100 { "HIGH" } else if *count > 10 { "MED" } else { "LOW" };
        println!("{:3}. {:60} {:5} [{}]", i+1, symbol, count, priority);
    }
    
    // Save as JSON
    let output = serde_json::json!({
        "source": "linux_perf_data",
        "perf_file": perf_file_path,
        "total_samples": total_samples,
        "unique_symbols": ranked.len(),
        "events": event_names,
        "ranked_symbols": ranked.iter().take(200).map(|(sym, count)| {
            serde_json::json!({
                "symbol": sym,
                "count": count,
                "priority": if *count > 100 { "high" } else if *count > 10 { "medium" } else { "low" }
            })
        }).collect::<Vec<_>>()
    });
    
    let output_path = "data/perf_rankings/rust_perf_ranking.json";
    std::fs::write(output_path, serde_json::to_string_pretty(&output)?)?;
    
    println!("\n💾 Saved ranking to: {}", output_path);
    
    Ok(())
}

// fn extract_symbol_from_record(parsed: &linux_perf_data::PerfFileRecord) -> Option<String> {
//     use linux_perf_data::PerfFileRecord;
//     
//     match parsed {
//         PerfFileRecord::EventRecord { record, .. } => {
//             // Get raw record type for now
//             Some(format!("{:?}", record.record_type))
//         }
//         PerfFileRecord::UserRecord(record) => {
//             Some(format!("{:?}", record.record_type))
//         }
//     }
// }

fn extract_symbol_from_record(parsed: &linux_perf_data::PerfFileRecord) -> Option<String> {
    use linux_perf_data::PerfFileRecord;
    
    match parsed {
        PerfFileRecord::EventRecord { record, .. } => {
            // Get raw record type for now
            Some(format!("{:?}", record.record_type))
        }
        PerfFileRecord::UserRecord(record) => {
            Some(format!("{:?}", record.record_type))
        }
    }
}
