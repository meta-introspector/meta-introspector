// 🔥 DUPLICATE CODE DETECTOR
// Official workflow using structured reporting and telemetry_lib

use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::telemetry_lib::telemetry_lib::*;
use telemetry_macros::{report_start, report_section, report_count, report_item, report_summary, report_end};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT_NAME").unwrap_or_else(|_| "duplicate_detector".to_string());
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    
    // Start structured reporting
    report_start!("Duplicate Code Detector", &project);
    
    // Initialize telemetry
    let entry = TelemetryEntry {
        r#type: "duplicate_scan_start".to_string(),
        message: "Starting duplicate code detection".to_string(),
        timestamp: start_time,
        project: project.clone(),
        binaries: 0,
        libraries: 0,
        symbols: 0,
    };
    let log_file = get_log_file(&project, start_time);
    write_telemetry_entry(&entry, &log_file)?;
    
    // Step 1: Scan all Rust files
    report_section!("Scanning Rust files");
    let rust_files = find_rust_files(".")?;
    report_count!("Files found", rust_files.len());
    
    // Step 2: Extract and count lines
    report_section!("Analyzing code lines");
    let mut line_counts: HashMap<String, u32> = HashMap::new();
    let mut total_lines = 0;
    
    for file_path in &rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            for line in content.lines() {
                let normalized = normalize_line(line);
                if !normalized.is_empty() {
                    *line_counts.entry(normalized).or_insert(0) += 1;
                    total_lines += 1;
                }
            }
        }
    }
    
    report_count!("Total lines analyzed", total_lines);
    report_count!("Unique patterns", line_counts.len());
    
    // Step 3: Find top duplicates
    report_section!("Finding top duplicates");
    let mut duplicates: Vec<(String, u32)> = line_counts.into_iter().collect();
    duplicates.sort_by(|a, b| b.1.cmp(&a.1));
    
    let top_duplicates = &duplicates[..5.min(duplicates.len())];
    
    for (i, (line, count)) in top_duplicates.iter().enumerate() {
        report_item!(&format!("#{}: {} occurrences - {}", i+1, count, 
                             if line.len() > 60 { &line[..60] } else { line }));
    }
    
    // Step 4: Log telemetry results
    let final_entry = TelemetryEntry {
        r#type: "duplicate_scan_complete".to_string(),
        message: format!("Found {} duplicates in {} files", duplicates.len(), rust_files.len()),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        project: project.clone(),
        binaries: rust_files.len() as u32,
        libraries: duplicates.len() as u32,
        symbols: total_lines as u32,
    };
    write_telemetry_entry(&final_entry, &log_file)?;
    
    // Summary and end
    report_summary!(
        "Files scanned" => rust_files.len(),
        "Total lines" => total_lines,
        "Duplicate patterns" => duplicates.len(),
        "Top duplicate count" => top_duplicates.first().map(|(_, c)| *c as usize).unwrap_or(0)
    );
    
    report_end!("Duplicate Code Detector", &project, start_time);
    
    Ok(())
}

fn find_rust_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if !dir_name.starts_with('.') && dir_name != "target" {
                files.extend(find_rust_files(path.to_str().unwrap())?);
            }
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path.to_string_lossy().to_string());
        }
    }
    
    Ok(files)
}

fn normalize_line(line: &str) -> String {
    let trimmed = line.trim();
    
    // Skip empty lines, braces, and common patterns
    if trimmed.is_empty() || 
       trimmed == "}" || 
       trimmed == "{" ||
       trimmed.starts_with("//") ||
       trimmed.starts_with("#[derive") ||
       trimmed.starts_with("use std::") ||
       trimmed.starts_with("use serde::") ||
       trimmed == "fn main() {" {
        return String::new();
    }
    
    trimmed.to_string()
}

// Import telemetry_lib module
#[path = "telemetry_lib.rs"]
mod telemetry_lib;
