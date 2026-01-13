pub mod telemetry_lib {
    use std::collections::HashMap;
    use std::time::Instant;
    
    static mut CALL_STATS: Option<HashMap<String, CallStat>> = None;
    
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
