use arrow::array::{Float64Array, StringArray, UInt8Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let validation_dir = &args[1];
    let output = &args[2];
    
    println!("💾 Compressing validation data to parquet");
    
    // Load all validation data
    let qemu_trace = std::fs::read_to_string(format!("{}/qemu_trace.log", validation_dir)).ok();
    let strace = std::fs::read_to_string(format!("{}/strace.log", validation_dir)).ok();
    let goblin = std::fs::read_to_string(format!("{}/goblin.json", validation_dir)).ok();
    let harmonic = std::fs::read_to_string(format!("{}/harmonic.json", validation_dir)).ok();
    
    // Create schema
    let schema = Schema::new(vec![
        Field::new("layer", DataType::UInt8, false),
        Field::new("qemu_trace_size", DataType::Float64, false),
        Field::new("strace_syscalls", DataType::Float64, false),
        Field::new("goblin_symbols", DataType::Float64, false),
        Field::new("harmonic_score", DataType::Float64, false),
        Field::new("total_score", DataType::Float64, false),
    ]);
    
    // Calculate metrics
    let qemu_size = qemu_trace.as_ref().map(|s| s.len() as f64).unwrap_or(0.0);
    let strace_count = strace.as_ref().map(|s| s.lines().count() as f64).unwrap_or(0.0);
    let goblin_count = goblin.as_ref().map(|s| s.lines().count() as f64).unwrap_or(0.0);
    let harmonic_score = parse_harmonic_score(&harmonic);
    
    let total_score = calculate_total_score(qemu_size, strace_count, goblin_count, harmonic_score);
    
    // Create record batch
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(UInt8Array::from(vec![0])),
            Arc::new(Float64Array::from(vec![qemu_size])),
            Arc::new(Float64Array::from(vec![strace_count])),
            Arc::new(Float64Array::from(vec![goblin_count])),
            Arc::new(Float64Array::from(vec![harmonic_score])),
            Arc::new(Float64Array::from(vec![total_score])),
        ],
    ).unwrap();
    
    // Write to parquet
    let file = File::create(output).unwrap();
    let props = WriterProperties::builder()
        .set_compression(parquet::basic::Compression::SNAPPY)
        .build();
    
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(props)).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("✅ Compressed to: {}", output);
    println!("📊 Total score: {:.2}", total_score);
}

fn parse_harmonic_score(harmonic: &Option<String>) -> f64 {
    harmonic.as_ref()
        .and_then(|s| s.lines().find(|l| l.contains("score")))
        .and_then(|l| l.split(':').nth(1))
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0.0)
}

fn calculate_total_score(qemu: f64, strace: f64, goblin: f64, harmonic: f64) -> f64 {
    let qemu_norm = (qemu / 1000000.0).min(1.0);
    let strace_norm = (strace / 1000.0).min(1.0);
    let goblin_norm = (goblin / 100.0).min(1.0);
    
    (qemu_norm * 0.25 + strace_norm * 0.25 + goblin_norm * 0.25 + harmonic * 0.25)
}
