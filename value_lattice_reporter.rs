use std::collections::HashMap;
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 VALUE LATTICE DISTRIBUTION REPORT");
    println!("===================================");
    
    let base_dir = "/mnt/data1/meta-introspector/analysis/value-lattice";
    
    let mut length_counts = HashMap::new();
    let mut total_files = 0;
    let mut total_usages = 0;
    let mut value_samples = Vec::new();
    
    // Read all directories (length-X)
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            
            if let Some(length_str) = dir_name.strip_prefix("length-") {
                if let Ok(length) = length_str.parse::<usize>() {
                    let mut dir_files = 0;
                    let mut dir_usages = 0;
                    
                    // Count JSON files in this length directory
                    let length_dir = entry.path();
                    if let Ok(json_entries) = fs::read_dir(&length_dir) {
                        for json_entry in json_entries {
                            if let Ok(json_entry) = json_entry {
                                if json_entry.path().extension().is_some_and(|ext| ext == "json") {
                                    dir_files += 1;
                                    total_files += 1;
                                    
                                    // Sample first few values for display
                                    if value_samples.len() < 10 {
                                        if let Ok(content) = fs::read_to_string(json_entry.path()) {
                                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                                if let Some(value) = json.get("value").and_then(|v| v.as_str()) {
                                                    if let Some(usages) = json.get("total_usages").and_then(|v| v.as_u64()) {
                                                        dir_usages += usages as usize;
                                                        value_samples.push((length, value.to_string(), usages));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    length_counts.insert(length, (dir_files, dir_usages));
                    total_usages += dir_usages;
                }
            }
        }
    }
    
    println!("\n📈 DISTRIBUTION BY LENGTH:");
    println!("Length | Files | Usages | Avg/File");
    println!("-------|-------|--------|----------");
    
    let mut sorted_lengths: Vec<_> = length_counts.keys().collect();
    sorted_lengths.sort();
    
    for &length in &sorted_lengths {
        if let Some((files, usages)) = length_counts.get(length) {
            let avg = if *files > 0 { *usages as f64 / *files as f64 } else { 0.0 };
            println!("{:6} | {:5} | {:6} | {:8.1}", length, files, usages, avg);
        }
    }
    
    println!("\n📊 SUMMARY:");
    println!("Total unique values: {}", total_files);
    println!("Total usage instances: {}", total_usages);
    println!("Length categories: {}", length_counts.len());
    println!("Average usages per value: {:.2}", total_usages as f64 / total_files as f64);
    
    println!("\n🔍 VALUE SAMPLES:");
    for (length, value, usages) in value_samples {
        let display_value = if value.len() > 50 {
            format!("{}...", &value[..47])
        } else {
            value
        };
        println!("L{:2}: {} (used {} times)", length, display_value, usages);
    }
    
    Ok(())
}
