// perf-runtime/src/telemetry.rs
use crate::data::PerfData;
use std::fs::{self, OpenOptions};
use std::io::Write;

const TELEMETRY_DIR: &str = "data/telemetry";

pub fn telemetry_send(perf_data: &PerfData) {
    // Create telemetry directory
    fs::create_dir_all(TELEMETRY_DIR).ok();
    
    // Write to JSONL file
    let log_file = format!("{}/perf_auto_{}.jsonl", TELEMETRY_DIR, perf_data.timestamp);
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        if let Ok(json) = serde_json::to_string(perf_data) {
            writeln!(file, "{}", json).ok();
        }
    }
}
