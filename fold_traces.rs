use arrow::array::{Float64Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::{ArrowWriter, ParquetRecordBatchReaderBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

#[derive(Debug, Default)]
struct FoldedAnalysis {
    file_path: String,
    
    // Markov analysis
    markov_score: f64,
    markov_neighbors: Vec<String>,
    
    // Context
    context_usage: Vec<String>,
    context_frequency: u64,
    
    // Compile traces
    compile_time_ms: u64,
    compile_dependencies: Vec<String>,
    
    // Perf traces
    perf_cycles: u64,
    perf_instructions: u64,
    perf_cache_misses: u64,
    
    // Strace
    syscalls: Vec<String>,
    syscall_count: u64,
    
    // Network access
    network_hosts: Vec<String>,
    network_bytes: u64,
    
    // Build logs
    build_success: bool,
    build_warnings: u64,
}

fn main() {
    println!("🔀 Folding validation traces");
    
    let validation_dir = std::env::args().nth(1).expect("Usage: fold_traces <dir>");
    
    // Load all trace files
    let markov = load_markov(&format!("{}/markov_symbol_scores.parquet", validation_dir));
    let compile = load_compile_traces(&format!("{}/nix_build_logs.parquet", validation_dir));
    let perf = load_perf_traces(&format!("{}/perf.data", validation_dir));
    let strace = load_strace(&format!("{}/strace.log", validation_dir));
    let network = load_network(&format!("{}/network.log", validation_dir));
    
    // Fold by file path
    let folded = fold_traces(markov, compile, perf, strace, network);
    
    // Save folded analysis
    save_folded(&folded, &format!("{}/folded_analysis.parquet", validation_dir));
    
    println!("✅ Folded {} files", folded.len());
}

fn load_markov(path: &str) -> HashMap<String, (f64, Vec<String>)> {
    let mut map = HashMap::new();
    
    if let Ok(file) = File::open(path) {
        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
        let reader = builder.build().unwrap();
        
        for batch in reader.flatten() {
            // Extract symbol, score, neighbors
            // map.insert(file_path, (score, neighbors));
        }
    }
    
    map
}

fn load_compile_traces(path: &str) -> HashMap<String, (u64, Vec<String>)> {
    let mut map = HashMap::new();
    
    if let Ok(file) = File::open(path) {
        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
        let reader = builder.build().unwrap();
        
        for batch in reader.flatten() {
            // Extract compile time, dependencies
        }
    }
    
    map
}

fn load_perf_traces(path: &str) -> HashMap<String, (u64, u64, u64)> {
    let mut map = HashMap::new();
    
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            if line.contains("cycles") {
                // Parse perf output
            }
        }
    }
    
    map
}

fn load_strace(path: &str) -> HashMap<String, (Vec<String>, u64)> {
    let mut map = HashMap::new();
    
    if let Ok(content) = std::fs::read_to_string(path) {
        let mut syscalls = Vec::new();
        for line in content.lines() {
            if let Some(syscall) = line.split('(').next() {
                syscalls.push(syscall.trim().to_string());
            }
        }
        map.insert("all".to_string(), (syscalls.clone(), syscalls.len() as u64));
    }
    
    map
}

fn load_network(path: &str) -> HashMap<String, (Vec<String>, u64)> {
    let mut map = HashMap::new();
    
    if let Ok(content) = std::fs::read_to_string(path) {
        let mut hosts = Vec::new();
        let mut bytes = 0u64;
        
        for line in content.lines() {
            if line.contains("connect") {
                // Extract host
            }
            if line.contains("bytes") {
                // Extract byte count
            }
        }
        
        map.insert("all".to_string(), (hosts, bytes));
    }
    
    map
}

fn fold_traces(
    markov: HashMap<String, (f64, Vec<String>)>,
    compile: HashMap<String, (u64, Vec<String>)>,
    perf: HashMap<String, (u64, u64, u64)>,
    strace: HashMap<String, (Vec<String>, u64)>,
    network: HashMap<String, (Vec<String>, u64)>,
) -> Vec<FoldedAnalysis> {
    let mut folded = Vec::new();
    
    // Collect all file paths
    let mut all_files: Vec<String> = markov.keys().cloned().collect();
    all_files.extend(compile.keys().cloned());
    all_files.sort();
    all_files.dedup();
    
    for file_path in all_files {
        let mut analysis = FoldedAnalysis {
            file_path: file_path.clone(),
            ..Default::default()
        };
        
        // Fold markov
        if let Some((score, neighbors)) = markov.get(&file_path) {
            analysis.markov_score = *score;
            analysis.markov_neighbors = neighbors.clone();
        }
        
        // Fold compile
        if let Some((time, deps)) = compile.get(&file_path) {
            analysis.compile_time_ms = *time;
            analysis.compile_dependencies = deps.clone();
        }
        
        // Fold perf
        if let Some((cycles, instructions, misses)) = perf.get(&file_path) {
            analysis.perf_cycles = *cycles;
            analysis.perf_instructions = *instructions;
            analysis.perf_cache_misses = *misses;
        }
        
        // Fold strace
        if let Some((syscalls, count)) = strace.get("all") {
            analysis.syscalls = syscalls.clone();
            analysis.syscall_count = *count;
        }
        
        // Fold network
        if let Some((hosts, bytes)) = network.get("all") {
            analysis.network_hosts = hosts.clone();
            analysis.network_bytes = *bytes;
        }
        
        folded.push(analysis);
    }
    
    folded
}

fn save_folded(folded: &[FoldedAnalysis], output: &str) {
    let schema = Schema::new(vec![
        Field::new("file_path", DataType::Utf8, false),
        Field::new("markov_score", DataType::Float64, false),
        Field::new("compile_time_ms", DataType::UInt64, false),
        Field::new("perf_cycles", DataType::UInt64, false),
        Field::new("perf_instructions", DataType::UInt64, false),
        Field::new("syscall_count", DataType::UInt64, false),
        Field::new("network_bytes", DataType::UInt64, false),
    ]);
    
    let file_paths: Vec<String> = folded.iter().map(|f| f.file_path.clone()).collect();
    let markov_scores: Vec<f64> = folded.iter().map(|f| f.markov_score).collect();
    let compile_times: Vec<u64> = folded.iter().map(|f| f.compile_time_ms).collect();
    let perf_cycles: Vec<u64> = folded.iter().map(|f| f.perf_cycles).collect();
    let perf_instructions: Vec<u64> = folded.iter().map(|f| f.perf_instructions).collect();
    let syscall_counts: Vec<u64> = folded.iter().map(|f| f.syscall_count).collect();
    let network_bytes: Vec<u64> = folded.iter().map(|f| f.network_bytes).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(file_paths)),
            Arc::new(Float64Array::from(markov_scores)),
            Arc::new(UInt64Array::from(compile_times)),
            Arc::new(UInt64Array::from(perf_cycles)),
            Arc::new(UInt64Array::from(perf_instructions)),
            Arc::new(UInt64Array::from(syscall_counts)),
            Arc::new(UInt64Array::from(network_bytes)),
        ],
    ).unwrap();
    
    let file = File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("💾 Saved folded analysis: {}", output);
}
