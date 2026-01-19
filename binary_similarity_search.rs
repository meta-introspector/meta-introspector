//! Binary Similarity Search - Find similar code in nix store
//! Compare our binaries to all nix store binaries

use std::process::Command;
use std::path::PathBuf;
use arrow::array::{StringArray, Float64Array};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::sync::Arc;

#[derive(Debug)]
struct BinarySimilarity {
    our_binary: String,
    nix_binary: String,
    similarity: f64,
    method: String,
}

fn find_nix_binaries() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let output = Command::new("find")
        .args(&["/nix/store", "-type", "f", "-executable"])
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(PathBuf::from)
        .collect())
}

fn compare_binaries(ours: &str, theirs: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Use objdump to compare
    let our_dump = Command::new("objdump")
        .args(&["-d", ours])
        .output()?;
    
    let their_dump = Command::new("objdump")
        .args(&["-d", theirs])
        .output()?;
    
    // Simple similarity: shared instruction count / total
    let our_insns: Vec<_> = String::from_utf8_lossy(&our_dump.stdout)
        .lines()
        .filter(|l| l.contains(":"))
        .collect();
    
    let their_insns: Vec<_> = String::from_utf8_lossy(&their_dump.stdout)
        .lines()
        .filter(|l| l.contains(":"))
        .collect();
    
    let shared = our_insns.iter()
        .filter(|i| their_insns.contains(i))
        .count();
    
    let total = our_insns.len().max(their_insns.len());
    
    Ok(if total > 0 { shared as f64 / total as f64 } else { 0.0 })
}

fn save_similarities(sims: &[BinarySimilarity], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("our_binary", DataType::Utf8, false),
        Field::new("nix_binary", DataType::Utf8, false),
        Field::new("similarity", DataType::Float64, false),
        Field::new("method", DataType::Utf8, false),
    ]);
    
    let our_bins: Vec<_> = sims.iter().map(|s| s.our_binary.clone()).collect();
    let nix_bins: Vec<_> = sims.iter().map(|s| s.nix_binary.clone()).collect();
    let similarities: Vec<_> = sims.iter().map(|s| s.similarity).collect();
    let methods: Vec<_> = sims.iter().map(|s| s.method.clone()).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(StringArray::from(our_bins)),
            Arc::new(StringArray::from(nix_bins)),
            Arc::new(Float64Array::from(similarities)),
            Arc::new(StringArray::from(methods)),
        ],
    )?;
    
    let file = std::fs::File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Binary Similarity Search");
    
    let our_binaries = vec![
        "./target/release/github_mirror_service",
        "./target/release/p2p_git_mirror",
        "./target/release/git_temporal_morphisms",
    ];
    
    println!("📦 Finding nix store binaries...");
    let nix_binaries = find_nix_binaries()?;
    println!("  Found {} binaries", nix_binaries.len());
    
    let mut similarities = Vec::new();
    
    for our_bin in &our_binaries {
        println!("\n🔬 Comparing {}...", our_bin);
        
        for (i, nix_bin) in nix_binaries.iter().enumerate().take(100) {
            if i % 10 == 0 {
                println!("  Progress: {}/100", i);
            }
            
            if let Ok(sim) = compare_binaries(our_bin, nix_bin.to_str().unwrap()) {
                if sim > 0.1 {
                    similarities.push(BinarySimilarity {
                        our_binary: our_bin.to_string(),
                        nix_binary: nix_bin.to_string_lossy().to_string(),
                        similarity: sim,
                        method: "objdump".to_string(),
                    });
                }
            }
        }
    }
    
    similarities.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
    
    println!("\n📊 Top 10 similar binaries:");
    for sim in similarities.iter().take(10) {
        println!("  {:.2}% - {} ~ {}", 
            sim.similarity * 100.0,
            sim.our_binary,
            sim.nix_binary
        );
    }
    
    std::fs::create_dir_all("data")?;
    save_similarities(&similarities, "data/binary_similarities.parquet")?;
    
    println!("\n✅ Saved to data/binary_similarities.parquet");
    
    Ok(())
}
