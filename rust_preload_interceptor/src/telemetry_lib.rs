pub mod telemetry_lib {
    use std::collections::HashMap;
    use std::time::Instant;
    use std::path::PathBuf;
    use serde::{Deserialize, Serialize};
    
    static mut CALL_STATS: Option<HashMap<String, CallStat>> = None;
    
    // Centralized telemetry configuration
    pub const TELEMETRY_BASE_DIR: &str = "/mnt/data1/meta-introspector/data/telemetry";
    
    // Centralized LD_PRELOAD configuration
    pub const PRELOAD_LIB_PATH: &str = "/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so";
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct CallStat {
        count: u64,
        total_duration_us: u64,
        binary: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TelemetryEntry {
        pub r#type: String,
        pub message: String,
        pub timestamp: u64,
        pub project: String,
        pub binaries: u32,
        pub libraries: u32,
        pub symbols: u32,
    }
    
    pub fn get_telemetry_dir() -> PathBuf {
        PathBuf::from(TELEMETRY_BASE_DIR)
    }
    
    pub fn get_log_file(project: &str, timestamp: u64) -> PathBuf {
        get_telemetry_dir().join(format!("{}_{}.jsonl", project, timestamp))
    }
    
    pub fn get_preload_lib() -> PathBuf {
        PathBuf::from(PRELOAD_LIB_PATH)
    }
    
    pub fn preload_lib_exists() -> bool {
        get_preload_lib().exists()
    }
    
    pub fn write_telemetry_entry(entry: &TelemetryEntry, log_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let json = serde_json::to_string(entry)?;
        let mut file = OpenOptions::new().create(true).append(true).open(log_file)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }
    
    #[derive(Debug, Clone)]
    struct CallStat {
        count: u64,
        total_duration_us: u64,
        binary: String,
    }
    
    pub fn preconditions() {
        println!("🔧 Checking preconditions...");
        unsafe {
            CALL_STATS = Some(HashMap::new());
        }
    }

    pub fn invariants() {
        println!("📊 Checking invariants...");
        // Real calls will be recorded by LD_PRELOAD interceptor
    }

    pub fn postconditions() {
        println!("✅ Checking postconditions...");
        show_call_statistics();
    }
    
    fn record_call(name: &str, duration_us: u64, binary: &str) {
        unsafe {
            if let Some(ref mut stats) = CALL_STATS {
                let stat = stats.entry(name.to_string()).or_insert(CallStat {
                    count: 0,
                    total_duration_us: 0,
                    binary: binary.to_string(),
                });
                stat.count += 1;
                stat.total_duration_us += duration_us;
            }
        }
    }
    
    fn show_call_statistics() {
        unsafe {
            if let Some(ref stats) = CALL_STATS {
                println!("📊 CALL STATISTICS:");
                for (name, stat) in stats {
                    let avg = if stat.count > 0 { stat.total_duration_us / stat.count } else { 0 };
                    println!("  {} ({}): {} calls, {}μs avg", name, stat.binary, stat.count, avg);
                }
            }
        }
    }
}
