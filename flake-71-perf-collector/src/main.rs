use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

mod perf_reader {
    use linux_perf_data::{PerfFileReader, PerfFileRecord};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use anyhow::Result;
    
    pub fn analyze_syscalls(perf_path: &Path) -> Result<(u64, Vec<String>)> {
        let file = File::open(perf_path)?;
        let reader = BufReader::new(file);
        let PerfFileReader { mut perf_file, mut record_iter } =
            PerfFileReader::parse_file(reader)?;
        
        let mut record_types: HashMap<String, u64> = HashMap::new();
        let mut total_records = 0u64;
        
        while let Some(record) = record_iter.next_record(&mut perf_file)? {
            total_records += 1;
            
            let record_type = match &record {
                PerfFileRecord::EventRecord { record, .. } => {
                    format!("{:?}", record.record_type)
                }
                PerfFileRecord::UserRecord(record) => {
                    format!("{:?}", record.record_type)
                }
            };
            
            *record_types.entry(record_type).or_insert(0) += 1;
        }
        
        // Rank record types
        let mut ranked: Vec<(String, u64)> = record_types.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Format top types
        let top_types: Vec<String> = ranked.into_iter()
            .take(5)
            .map(|(typ, count)| format!("{}:{}", typ, count))
            .collect();
        
        Ok((total_records, top_types))
    }
}

#[derive(Serialize, Deserialize)]
struct FlakePerfResult {
    language: String,
    flake_path: PathBuf,
    timestamp: u64,
    build_success: bool,
    build_perf_data: Option<PathBuf>,
    build_samples: Option<u64>,
    build_syscalls: Option<Vec<String>>,
    run_success: bool,
    run_perf_data: Option<PathBuf>,
    run_samples: Option<u64>,
    run_syscalls: Option<Vec<String>>,
    run_output: Option<String>,
    derivations_built: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = PathBuf::from("/mnt/data1/meta-introspector/const_71_test");
    let output_dir = PathBuf::from("/mnt/data1/meta-introspector/data/71_flakes_perf");
    std::fs::create_dir_all(&output_dir)?;
    
    // Get all language directories
    let mut languages: Vec<String> = std::fs::read_dir(&base_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    languages.sort();
    
    println!("🎯 Collecting build + run perf data for {} flakes\n", languages.len());
    
    let mut results = Vec::new();
    
    for (idx, lang) in languages.iter().enumerate() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        println!("[{}/{}] 📊 Processing: {}", idx + 1, languages.len(), lang);
        
        let flake_path = base_dir.join(lang);
        let build_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "build"));
        
        // Build with perf
        let build_status = Command::new("perf")
            .args(&["record", "-o", build_perf.to_str().unwrap(), "--call-graph", "dwarf",
                    "nix", "build", "--print-build-logs"])
            .current_dir(&flake_path)
            .status()?;
        
        let build_success = build_status.success();
        
        // Analyze build perf immediately
        let (build_samples, build_syscalls) = if build_success {
            match perf_reader::analyze_syscalls(&build_perf) {
                Ok((samples, types)) => (Some(samples), Some(types)),
                Err(e) => {
                    eprintln!("  ⚠️  Perf analysis failed: {}", e);
                    (None, None)
                }
            }
        } else {
            (None, None)
        };
        
        println!("  Build: {} | Records: {:?} | Types: {:?}", 
                 if build_success { "✅" } else { "❌" },
                 build_samples,
                 build_syscalls);
        
        // Run with perf (if build succeeded)
        let (run_success, run_perf_path, run_samples, run_syscalls, run_output) = if build_success {
            let run_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "run"));
            
            let run_result = Command::new("perf")
                .args(&["record", "-o", run_perf.to_str().unwrap(), 
                        "--call-graph", "dwarf",
                        "-a",  // Record all CPUs (captures child processes)
                        "nix", "run"])
                .current_dir(&flake_path)
                .output()?;
            
            let success = run_result.status.success();
            let output = String::from_utf8_lossy(&run_result.stdout).to_string();
            
            // Analyze run perf immediately
            let (samples, syscalls) = if success {
                match perf_reader::analyze_syscalls(&run_perf) {
                    Ok((s, t)) => (Some(s), Some(t)),
                    Err(_) => (None, None)
                }
            } else {
                (None, None)
            };
            
            println!("  Run: {} | Output: {} | Records: {:?} | Types: {:?}", 
                     if success { "✅" } else { "❌" }, 
                     output.trim(),
                     samples,
                     syscalls);
            
            (success, Some(run_perf), samples, syscalls, Some(output))
        } else {
            (false, None, None, None, None)
        };
        
        let result = FlakePerfResult {
            language: lang.clone(),
            flake_path: flake_path.clone(),
            timestamp,
            build_success,
            build_perf_data: if build_success { Some(build_perf) } else { None },
            build_samples,
            build_syscalls,
            run_success,
            run_perf_data: run_perf_path,
            run_samples,
            run_syscalls,
            run_output,
            derivations_built: 0,
        };
        
        let json_output = output_dir.join(format!("{}_{}.json", lang, timestamp));
        let json = serde_json::to_string_pretty(&result)?;
        std::fs::write(&json_output, json)?;
        
        results.push(result);
        println!();
    }
    
    // Summary
    let successful = results.iter().filter(|r| r.build_success && r.run_success).count();
    println!("🎉 Complete: {}/{} flakes successful", successful, results.len());
    
    Ok(())
}
