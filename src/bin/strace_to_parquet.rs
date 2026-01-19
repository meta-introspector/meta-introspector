use anyhow::Result;
use arrow::array::*;
use arrow::datatypes::*;
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

#[derive(Debug)]
struct StraceEntry {
    timestamp: f64,
    pid: u32,
    syscall: String,
    args: String,
    result: String,
    duration: f64,
}

fn parse_strace_line(line: &str) -> Option<StraceEntry> {
    // Parse: 12:34:56.789012 [pid 1234] syscall(args) = result <0.000123>
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 { return None; }
    
    let timestamp = parts[0].parse().ok()?;
    let pid = parts[1].trim_matches(|c| c == '[' || c == ']').parse().ok()?;
    
    let syscall_start = line.find(']')? + 2;
    let syscall_end = line[syscall_start..].find('(')?;
    let syscall = line[syscall_start..syscall_start + syscall_end].to_string();
    
    let args_start = syscall_start + syscall_end + 1;
    let args_end = line[args_start..].find(')')?;
    let args = line[args_start..args_start + args_end].to_string();
    
    let result_start = line.find(" = ")? + 3;
    let result_end = line[result_start..].find(" <").unwrap_or(line.len() - result_start);
    let result = line[result_start..result_start + result_end].to_string();
    
    let duration = if let Some(dur_start) = line.find(" <") {
        line[dur_start + 2..].trim_end_matches('>').parse().unwrap_or(0.0)
    } else {
        0.0
    };
    
    Some(StraceEntry { timestamp, pid, syscall, args, result, duration })
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <strace_file> <output.parquet>", args[0]);
        std::process::exit(1);
    }
    
    let input = &args[1];
    let output = &args[2];
    
    println!("📖 Reading strace: {}", input);
    
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    
    let mut entries = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(entry) = parse_strace_line(&line) {
                entries.push(entry);
            }
        }
    }
    
    println!("✅ Parsed {} syscalls", entries.len());
    
    // Build Arrow schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("timestamp", DataType::Float64, false),
        Field::new("pid", DataType::UInt32, false),
        Field::new("syscall", DataType::Utf8, false),
        Field::new("args", DataType::Utf8, false),
        Field::new("result", DataType::Utf8, false),
        Field::new("duration", DataType::Float64, false),
    ]));
    
    // Build arrays
    let timestamps: Float64Array = entries.iter().map(|e| e.timestamp).collect();
    let pids: UInt32Array = entries.iter().map(|e| e.pid).collect();
    let syscalls: Vec<_> = entries.iter().map(|e| e.syscall.as_str()).collect();
    let args: Vec<_> = entries.iter().map(|e| e.args.as_str()).collect();
    let results: Vec<_> = entries.iter().map(|e| e.result.as_str()).collect();
    let durations: Float64Array = entries.iter().map(|e| e.duration).collect();
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(timestamps),
            Arc::new(pids),
            Arc::new(syscalls),
            Arc::new(args),
            Arc::new(results),
            Arc::new(durations),
        ],
    )?;
    
    println!("💾 Writing parquet: {}", output);
    
    let file = File::create(output)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    
    writer.write(&batch)?;
    writer.close()?;
    
    println!("✅ Witness v1 saved: {} syscalls", entries.len());
    
    Ok(())
}
