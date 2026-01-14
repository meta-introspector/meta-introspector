// 🔥 MONOLITHIC TELEMETRY RUNNER: See all gcc/rustc calls in our stats
use std::collections::HashMap;
use std::time::Instant;

// Include all our wrapped calls
include!("latest_dev.rs");

// Global telemetry stats
static mut CALL_STATS: Option<HashMap<String, CallStat>> = None;

#[derive(Debug, Clone, serde::Serialize)]
struct CallStat {
    count: u64,
    total_duration_us: u64,
    avg_duration_us: u64,
    binary: String,
}

// Override call_telemetry to collect local stats
macro_rules! call_telemetry {
    ($name:literal, $call:expr) => {{
        let start = Instant::now();
        let result = $call;
        let duration = start.elapsed().as_micros() as u64;
        
        // Record in local stats
        record_call_stat($name, duration);
        
        result
    }};
}

fn record_call_stat(symbol: &str, duration_us: u64) {
    unsafe {
        if CALL_STATS.is_none() {
            CALL_STATS = Some(HashMap::new());
        }
        
        if let Some(ref mut stats) = CALL_STATS {
            let stat = stats.entry(symbol.to_string()).or_insert(CallStat {
                count: 0,
                total_duration_us: 0,
                avg_duration_us: 0,
                binary: "unknown".to_string(),
            });
            
            stat.count += 1;
            stat.total_duration_us += duration_us;
            stat.avg_duration_us = stat.total_duration_us / stat.count;
        }
    }
}

fn main() {
    println!("🔥 MONOLITHIC TELEMETRY RUNNER");
    println!("===============================");
    
    // Initialize all wrappers
    init_all_call_wrappers!();
    
    panic!("FIXME NOW");
    run_nix_rebuild_of_rust();
    
    // Show telemetry stats
    show_call_statistics();
}


fn show_call_statistics() {
    println!("\n📊 CALL TELEMETRY STATISTICS");
    println!("============================");
    
    unsafe {
        if let Some(ref stats) = CALL_STATS {
            let mut sorted_stats: Vec<_> = stats.iter().collect();
            sorted_stats.sort_by(|a, b| b.1.count.cmp(&a.1.count));
            
            println!("🔥 TOP FUNCTION CALLS:");
            println!("Symbol                 | Count | Total μs | Avg μs");
            println!("----------------------|-------|----------|--------");
            
            for (symbol, stat) in sorted_stats.iter() {
                println!("{:20} | {:5} | {:8} | {:6}", 
                    symbol, stat.count, stat.total_duration_us, stat.avg_duration_us);
            }
            
            // Summary stats
            let total_calls: u64 = stats.values().map(|s| s.count).sum();
            let total_duration: u64 = stats.values().map(|s| s.total_duration_us).sum();
            let unique_symbols = stats.len();
            
            println!("\n📈 SUMMARY:");
            println!("  Total calls: {}", total_calls);
            println!("  Unique symbols: {}", unique_symbols);
            println!("  Total duration: {}μs", total_duration);
            println!("  Average per call: {}μs", if total_calls > 0 { total_duration / total_calls } else { 0 });
            
            // Show gcc vs rustc breakdown
            show_compiler_breakdown(stats);
            
            // Save stats to file
            save_telemetry_stats(stats);
        }
    }
}

fn show_compiler_breakdown(stats: &HashMap<String, CallStat>) {
    println!("\n🔧 COMPILER BREAKDOWN:");
    println!("=====================");
    
    let mut gcc_calls = 0;
    let mut rustc_calls = 0;
    let mut other_calls = 0;
    
    for (symbol, stat) in stats {
        if symbol.contains("gcc") || symbol == "malloc" || symbol == "fopen" || symbol == "execve" {
            gcc_calls += stat.count;
        } else if symbol.contains("rustc") || symbol.contains("llvm") || symbol.contains("codegen") {
            rustc_calls += stat.count;
        } else {
            other_calls += stat.count;
        }
    }
    
    println!("  🔧 GCC-related calls: {}", gcc_calls);
    println!("  🦀 Rustc-related calls: {}", rustc_calls);
    println!("  📦 Other calls: {}", other_calls);
    
    let total = gcc_calls + rustc_calls + other_calls;
    if total > 0 {
        println!("  📊 GCC: {:.1}%, Rustc: {:.1}%, Other: {:.1}%", 
            (gcc_calls as f64 / total as f64) * 100.0,
            (rustc_calls as f64 / total as f64) * 100.0,
            (other_calls as f64 / total as f64) * 100.0);
    }
}

fn save_telemetry_stats(stats: &HashMap<String, CallStat>) {
    let stats_json = serde_json::to_string_pretty(stats).unwrap_or_default();
    let filename = format!("monolithic_telemetry_stats_{}.json", 
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    
    if std::fs::write(&filename, stats_json).is_ok() {
        println!("\n✅ Saved telemetry stats: {}", filename);
    }
    
    println!("\n🎯 MONOLITHIC TELEMETRY COMPLETE!");
    println!("All gcc and rustc calls captured in local stats!");
}
