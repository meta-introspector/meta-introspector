// perf_canonical_reader.rs
// THE ONLY PLACE TO READ PERF.DATA FILES
// Centralizes all perf data parsing and analysis

use linux_perf_data::{PerfFileReader, PerfFileRecord};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfAnalysis {
    pub source_file: String,
    pub total_samples: u64,
    pub total_events: u64,
    pub unique_symbols: usize,
    pub events: Vec<String>,
    pub ranked_symbols: Vec<SymbolRank>,
    pub commands: Vec<CommandStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolRank {
    pub symbol: String,
    pub samples: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandStats {
    pub command: String,
    pub samples: u64,
    pub percentage: f64,
}

pub struct PerfCanonicalReader {
    perf_file_path: PathBuf,
}

impl PerfCanonicalReader {
    pub fn new(perf_file_path: PathBuf) -> Self {
        Self { perf_file_path }
    }
    
    pub fn analyze(&self) -> Result<PerfAnalysis> {
        println!("📊 Reading: {}", self.perf_file_path.display());
        
        let file = File::open(&self.perf_file_path)?;
        let reader = BufReader::new(file);
        
        let PerfFileReader { mut perf_file, mut record_iter } =
            PerfFileReader::parse_file(reader)?;
        
        // Extract event names
        let events: Vec<String> = perf_file
            .event_attributes()
            .iter()
            .filter_map(|attr| attr.name())
            .map(|s| s.to_string())
            .collect();
        
        println!("✅ Events: {}", events.join(", "));
        
        // Count symbols and commands
        let mut symbol_counts: HashMap<String, u64> = HashMap::new();
        let mut command_counts: HashMap<String, u64> = HashMap::new();
        let mut total_samples = 0u64;
        let mut total_events = 0u64;
        
        println!("📈 Processing records...");
        
        while let Some(record) = record_iter.next_record(&mut perf_file)? {
            total_events += 1;
            
            match &record {
                PerfFileRecord::EventRecord { record, .. } => {
                    total_samples += 1;
                    
                    // Extract symbol
                    let symbol = format!("{:?}", record.record_type);
                    *symbol_counts.entry(symbol).or_insert(0) += 1;
                    
                    // Extract command if available
                    // if let Some(parsed) = &record.parsed_data {
                    //     if let Some(comm) = parsed.common_data().comm {
                    //         *command_counts.entry(comm.to_string()).or_insert(0) += 1;
                    //     }
                    // }
                }
                PerfFileRecord::UserRecord(record) => {
                    let symbol = format!("{:?}", record.record_type);
                    *symbol_counts.entry(symbol).or_insert(0) += 1;
                }
            }
        }
        
        println!("✅ Processed {} events, {} samples", total_events, total_samples);
        
        // Rank symbols
        let mut ranked_symbols: Vec<SymbolRank> = symbol_counts
            .into_iter()
            .map(|(symbol, samples)| SymbolRank {
                symbol,
                samples,
                percentage: (samples as f64 / total_samples as f64) * 100.0,
            })
            .collect();
        ranked_symbols.sort_by(|a, b| b.samples.cmp(&a.samples));
        
        // Rank commands
        let mut commands: Vec<CommandStats> = command_counts
            .into_iter()
            .map(|(command, samples)| CommandStats {
                command,
                samples,
                percentage: (samples as f64 / total_samples as f64) * 100.0,
            })
            .collect();
        commands.sort_by(|a, b| b.samples.cmp(&a.samples));
        
        Ok(PerfAnalysis {
            source_file: self.perf_file_path.display().to_string(),
            total_samples,
            total_events,
            unique_symbols: ranked_symbols.len(),
            events,
            ranked_symbols,
            commands,
        })
    }
}

fn main() -> Result<()> {
    let perf_file_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("perf.data"));
    
    let reader = PerfCanonicalReader::new(perf_file_path);
    let analysis = reader.analyze()?;
    
    println!("\n📊 Analysis Summary:");
    println!("  Total samples: {}", analysis.total_samples);
    println!("  Total events: {}", analysis.total_events);
    println!("  Unique symbols: {}", analysis.unique_symbols);
    println!("  Commands: {}", analysis.commands.len());
    
    println!("\n🔝 Top 10 Commands:");
    for (i, cmd) in analysis.commands.iter().take(10).enumerate() {
        println!("  {}. {} - {} samples ({:.2}%)", 
                 i + 1, cmd.command, cmd.samples, cmd.percentage);
    }
    
    println!("\n🔝 Top 10 Symbols:");
    for (i, sym) in analysis.ranked_symbols.iter().take(10).enumerate() {
        println!("  {}. {} - {} samples ({:.2}%)", 
                 i + 1, sym.symbol, sym.samples, sym.percentage);
    }
    
    // Output JSON
    let json_path = format!("{}.analysis.json", analysis.source_file);
    let json = serde_json::to_string_pretty(&analysis)?;
    std::fs::write(&json_path, json)?;
    println!("\n💾 Saved: {}", json_path);
    
    Ok(())
}
