use std::collections::HashMap;
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 OPTIMIZED VALUE LATTICE USAGE COUNTER");
    println!("=======================================");
    
    let base_dir = "/mnt/data1/meta-introspector/analysis/value-lattice";
    
    let mut length_stats = HashMap::new();
    let mut total_files = 0;
    let mut total_usages = 0;
    
    // Process only meaningful length categories (1-100, skip empty ones)
    for length in 1..=100 {
        let length_dir = format!("{}/length-{}", base_dir, length);
        
        if let Ok(entries) = fs::read_dir(&length_dir) {
            let mut category_files = 0;
            let mut category_usages = 0;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().extension().map_or(false, |ext| ext == "json") {
                        category_files += 1;
                        
                        // Read usage count from JSON
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                if let Some(usages) = json.get("total_usages").and_then(|v| v.as_u64()) {
                                    category_usages += usages;
                                }
                            }
                        }
                    }
                }
            }
            
            if category_files > 0 {
                length_stats.insert(length, (category_files, category_usages));
                total_files += category_files;
                total_usages += category_usages;
            }
        }
    }
    
    // Count truncated long categories (100+)
    let mut long_files = 0;
    let mut long_usages = 0;
    
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            
            if let Some(length_str) = dir_name.strip_prefix("length-") {
                if let Ok(length) = length_str.parse::<usize>() {
                    if length > 100 {
                        if let Ok(json_entries) = fs::read_dir(entry.path()) {
                            for json_entry in json_entries {
                                if let Ok(json_entry) = json_entry {
                                    if json_entry.path().extension().map_or(false, |ext| ext == "json") {
                                        long_files += 1;
                                        
                                        if let Ok(content) = fs::read_to_string(json_entry.path()) {
                                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                                if let Some(usages) = json.get("total_usages").and_then(|v| v.as_u64()) {
                                                    long_usages += usages;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n📊 USAGE STATISTICS BY LENGTH:");
    println!("Length | Files | Usages | Usage/File");
    println!("-------|-------|--------|------------");
    
    for length in 1..=100 {
        if let Some((files, usages)) = length_stats.get(&length) {
            let ratio = if *files > 0 { *usages as f64 / *files as f64 } else { 0.0 };
            if *usages > 0 || *files > 50 { // Show categories with usage or many files
                println!("{:6} | {:5} | {:6} | {:10.2}", length, files, usages, ratio);
            }
        }
    }
    
    if long_files > 0 {
        let long_ratio = long_usages as f64 / long_files as f64;
        println!(" 100+ | {:5} | {:6} | {:10.2}", long_files, long_usages, long_ratio);
    }
    
    println!("\n🎯 SUMMARY:");
    println!("Files (1-100): {}", total_files);
    println!("Files (100+): {}", long_files);
    println!("Total files: {}", total_files + long_files);
    println!("Usages (1-100): {}", total_usages);
    println!("Usages (100+): {}", long_usages);
    println!("Total usages: {}", total_usages + long_usages);
    
    // Find most used categories
    let mut usage_vec: Vec<_> = length_stats.iter().collect();
    usage_vec.sort_by(|a, b| b.1.1.cmp(&a.1.1));
    
    println!("\n🔥 TOP USAGE CATEGORIES:");
    for (length, (files, usages)) in usage_vec.iter().take(10) {
        if *usages > 0 {
            println!("Length {:2}: {:6} usages across {:3} files", length, usages, files);
        }
    }
    
    Ok(())
}
