// Convert perf data to parquet for SQL queries

use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::parser::parse_message_type;
use parquet::record::RecordWriter;
use std::fs::File;
use std::process::Command;
use std::sync::Arc;

#[derive(Debug)]
struct PerfSample {
    language: String,
    ip: String,
    dso: String,
    symbol: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <language> <perf.data>", args[0]);
        std::process::exit(1);
    }
    
    let language = &args[1];
    let perf_file = &args[2];
    let output_file = format!("{}_perf.parquet", language);
    
    println!("🔄 Converting {} perf data to parquet...", language);
    
    // Run perf script
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file, "-F", "ip,dso,sym"])
        .output()
        .expect("Failed to run perf script");
    
    let script_output = String::from_utf8_lossy(&output.stdout);
    
    // Parse samples
    let mut samples = Vec::new();
    for line in script_output.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let ip = parts[0].to_string();
            let dso = parts.get(1).unwrap_or(&"unknown").to_string();
            let symbol = parts.get(2..).map(|s| s.join(" ")).unwrap_or_else(|| "?".to_string());
            
            samples.push(PerfSample {
                language: language.to_string(),
                ip,
                dso,
                symbol,
            });
        }
    }
    
    println!("📊 Parsed {} samples", samples.len());
    
    // Create parquet schema
    let schema = "
        message schema {
            REQUIRED BYTE_ARRAY language (UTF8);
            REQUIRED BYTE_ARRAY ip (UTF8);
            REQUIRED BYTE_ARRAY dso (UTF8);
            REQUIRED BYTE_ARRAY symbol (UTF8);
        }
    ";
    
    let schema = Arc::new(parse_message_type(schema).unwrap());
    let props = Arc::new(WriterProperties::builder().build());
    let file = File::create(&output_file).unwrap();
    
    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    
    // Write samples
    let mut row_group = writer.next_row_group().unwrap();
    
    // Write language column
    if let Some(mut col_writer) = row_group.next_column().unwrap() {
        col_writer
            .typed::<parquet::data_type::ByteArrayType>()
            .write_batch(
                &samples.iter().map(|s| s.language.as_bytes().into()).collect::<Vec<_>>(),
                None,
                None,
            )
            .unwrap();
        col_writer.close().unwrap();
    }
    
    // Write ip column
    if let Some(mut col_writer) = row_group.next_column().unwrap() {
        col_writer
            .typed::<parquet::data_type::ByteArrayType>()
            .write_batch(
                &samples.iter().map(|s| s.ip.as_bytes().into()).collect::<Vec<_>>(),
                None,
                None,
            )
            .unwrap();
        col_writer.close().unwrap();
    }
    
    // Write dso column
    if let Some(mut col_writer) = row_group.next_column().unwrap() {
        col_writer
            .typed::<parquet::data_type::ByteArrayType>()
            .write_batch(
                &samples.iter().map(|s| s.dso.as_bytes().into()).collect::<Vec<_>>(),
                None,
                None,
            )
            .unwrap();
        col_writer.close().unwrap();
    }
    
    // Write symbol column
    if let Some(mut col_writer) = row_group.next_column().unwrap() {
        col_writer
            .typed::<parquet::data_type::ByteArrayType>()
            .write_batch(
                &samples.iter().map(|s| s.symbol.as_bytes().into()).collect::<Vec<_>>(),
                None,
                None,
            )
            .unwrap();
        col_writer.close().unwrap();
    }
    
    row_group.close().unwrap();
    writer.close().unwrap();
    
    println!("✅ Wrote {} to {}", samples.len(), output_file);
}
