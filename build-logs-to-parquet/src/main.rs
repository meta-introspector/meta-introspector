// build_logs_to_parquet.rs - Convert Nix build logs to Parquet

use anyhow::Result;
use arrow::array::{StringArray, UInt64Array, Int32Array, ArrayRef};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Utc};
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct BuildSummary {
    project: String,
    git_commit: String,
    build_status: String,
    log_derivation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BuildState {
    status: String,
    exit_code: i32,
    system: String,
    nix_version: String,
    build_time: u64,
}

fn main() -> Result<()> {
    println!("🔬 build_logs_to_parquet - Convert Nix build logs");
    
    let store_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/nix/store".to_string());
    
    let output_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "nix_build_logs.parquet".to_string());
    
    println!("📂 Scanning: {}", store_path);
    println!("📊 Output: {}", output_path);
    
    // Find all build log derivations
    let log_dirs = find_build_logs(&store_path)?;
    println!("✅ Found {} build logs", log_dirs.len());
    
    // Define schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("project", DataType::Utf8, false),
        Field::new("git_commit", DataType::Utf8, false),
        Field::new("build_status", DataType::Utf8, false),
        Field::new("exit_code", DataType::Int32, false),
        Field::new("build_time", DataType::UInt64, false),
        Field::new("system", DataType::Utf8, false),
        Field::new("nix_version", DataType::Utf8, false),
        Field::new("build_log", DataType::Utf8, false),
        Field::new("log_derivation", DataType::Utf8, false),
    ]));
    
    // Create Parquet writer
    let file = File::create(&output_path)?;
    let props = WriterProperties::builder()
        .set_compression(parquet::basic::Compression::SNAPPY)
        .build();
    
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))?;
    
    // Process logs in batches
    let batch_size = 1000;
    let mut records = Vec::new();
    
    for (i, log_dir) in log_dirs.iter().enumerate() {
        if let Ok(record) = extract_log_data(log_dir) {
            records.push(record);
            
            if records.len() >= batch_size {
                write_batch(&mut writer, &schema, &records)?;
                println!("📊 Wrote batch {} - {} records", i / batch_size + 1, records.len());
                records.clear();
            }
        }
    }
    
    // Write remaining
    if !records.is_empty() {
        write_batch(&mut writer, &schema, &records)?;
        println!("📊 Wrote final batch - {} records", records.len());
    }
    
    writer.close()?;
    
    println!("\n✅ Wrote {} build logs to Parquet", log_dirs.len());
    println!("💾 Saved to: {}", output_path);
    
    // Show file size
    let metadata = fs::metadata(&output_path)?;
    println!("📊 File size: {:.2} MB", metadata.len() as f64 / 1024.0 / 1024.0);
    
    Ok(())
}

fn find_build_logs(store_path: &str) -> Result<Vec<String>> {
    let mut logs = Vec::new();
    
    for entry in fs::read_dir(store_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            
            // Look for -build-log or -with-logs suffix
            if name.contains("-build-log") || name.contains("-with-logs") {
                // Check if it has summary.json
                if path.join("summary.json").exists() {
                    logs.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }
    
    Ok(logs)
}

#[derive(Debug)]
struct LogRecord {
    project: String,
    git_commit: String,
    build_status: String,
    exit_code: i32,
    build_time: u64,
    system: String,
    nix_version: String,
    build_log: String,
    log_derivation: String,
}

fn extract_log_data(log_dir: &str) -> Result<LogRecord> {
    let path = Path::new(log_dir);
    
    // Read summary.json
    let summary_path = path.join("summary.json");
    let summary: BuildSummary = serde_json::from_str(&fs::read_to_string(summary_path)?)?;
    
    // Read build state
    let state_path = path.join("3-build-state.json");
    let state: BuildState = serde_json::from_str(&fs::read_to_string(state_path)?)?;
    
    // Read build log
    let log_path = path.join("build.log");
    let build_log = fs::read_to_string(log_path).unwrap_or_else(|_| "".to_string());
    
    Ok(LogRecord {
        project: summary.project,
        git_commit: summary.git_commit,
        build_status: summary.build_status,
        exit_code: state.exit_code,
        build_time: state.build_time,
        system: state.system,
        nix_version: state.nix_version,
        build_log,
        log_derivation: summary.log_derivation,
    })
}

fn write_batch(
    writer: &mut ArrowWriter<File>,
    schema: &Arc<Schema>,
    records: &[LogRecord],
) -> Result<()> {
    let project: Vec<String> = records.iter().map(|r| r.project.clone()).collect();
    let git_commit: Vec<String> = records.iter().map(|r| r.git_commit.clone()).collect();
    let build_status: Vec<String> = records.iter().map(|r| r.build_status.clone()).collect();
    let exit_code: Vec<i32> = records.iter().map(|r| r.exit_code).collect();
    let build_time: Vec<u64> = records.iter().map(|r| r.build_time).collect();
    let system: Vec<String> = records.iter().map(|r| r.system.clone()).collect();
    let nix_version: Vec<String> = records.iter().map(|r| r.nix_version.clone()).collect();
    let build_log: Vec<String> = records.iter().map(|r| r.build_log.clone()).collect();
    let log_derivation: Vec<String> = records.iter().map(|r| r.log_derivation.clone()).collect();
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(project)) as ArrayRef,
            Arc::new(StringArray::from(git_commit)) as ArrayRef,
            Arc::new(StringArray::from(build_status)) as ArrayRef,
            Arc::new(Int32Array::from(exit_code)) as ArrayRef,
            Arc::new(UInt64Array::from(build_time)) as ArrayRef,
            Arc::new(StringArray::from(system)) as ArrayRef,
            Arc::new(StringArray::from(nix_version)) as ArrayRef,
            Arc::new(StringArray::from(build_log)) as ArrayRef,
            Arc::new(StringArray::from(log_derivation)) as ArrayRef,
        ],
    )?;
    
    writer.write(&batch)?;
    Ok(())
}
