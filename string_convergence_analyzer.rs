use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📏 STRING LENGTH CONVERGENCE ANALYSIS");
    println!("====================================");
    
    let base_dir = "/mnt/data1/meta-introspector/analysis/value-lattice";
    
    let mut length_stats = Vec::new();
    
    // Analyze lengths 1-200 to find convergence point
    for length in 1..=200 {
        let length_dir = format!("{}/length-{}", base_dir, length);
        
        if let Ok(entries) = fs::read_dir(&length_dir) {
            let mut unique_values = 0;
            let mut total_usages = 0;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().extension().is_some_and(|ext| ext == "json") {
                        unique_values += 1;
                        
                        // Count usages
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                if let Some(usages) = json.get("total_usages").and_then(|v| v.as_u64()) {
                                    total_usages += usages;
                                }
                            }
                        }
                    }
                }
            }
            
            if unique_values > 0 {
                let reuse_ratio = total_usages as f64 / unique_values as f64;
                length_stats.push((length, unique_values, total_usages, reuse_ratio));
            }
        }
    }
    
    println!("\n📊 LENGTH CONVERGENCE ANALYSIS:");
    println!("Length | Unique | Usages | Reuse | Convergence");
    println!("-------|--------|--------|-------|------------");
    
    let mut cutoff_length: usize = 0;
    
    for (length, unique, usages, reuse) in &length_stats {
        let convergence = if *reuse <= 1.1 { "CONVERGED" } else { "" };
        
        println!("{:6} | {:6} | {:6} | {:5.2} | {}", 
                 length, unique, usages, reuse, convergence);
        
        // Find cutoff: last length with reuse > 1.0 (multiple instances of same value)
        if *reuse > 1.0 && cutoff_length == 0 {
            // Keep looking for the last one above 1.0
        } else if *reuse <= 1.0 && cutoff_length == 0 {
            cutoff_length = *length - 1; // Previous length was the cutoff
        }
    }
    
    // Find actual cutoff by looking for last length with reuse > 1.0
    for (length, _, _, reuse) in length_stats.iter().rev() {
        if *reuse > 1.0 {
            cutoff_length = *length;
            break;
        }
    }
    
    println!("\n🎯 CONVERGENCE POINT FOUND:");
    println!("Optimal cutoff length: {}", cutoff_length);
    println!("Strings longer than {} chars are mostly unique", cutoff_length);
    
    // Show statistics around cutoff
    println!("\n🔍 CUTOFF REGION ANALYSIS:");
    for (length, unique, usages, reuse) in &length_stats {
        if *length >= cutoff_length.saturating_sub(5) && *length <= cutoff_length + 5 {
            let status = if *length == cutoff_length { " <- CUTOFF" } else { "" };
            println!("Length {:3}: {:3} unique, {:4} usages, {:.2} reuse{}", 
                     length, unique, usages, reuse, status);
        }
    }
    
    // Calculate total savings with cutoff
    let (before_cutoff, after_cutoff): (Vec<_>, Vec<_>) = length_stats.iter()
        .partition(|(length, _, _, _)| *length <= cutoff_length);
    
    let reusable_values: u64 = before_cutoff.iter().map(|(_, _, usages, _)| usages).sum();
    let unique_values: u64 = after_cutoff.iter().map(|(_, _, usages, _)| usages).sum();
    let total_values = reusable_values + unique_values;
    
    println!("\n📈 EFFICIENCY ANALYSIS:");
    println!("Reusable values (≤{}): {} ({:.1}%)", 
             cutoff_length, reusable_values, 
             reusable_values as f64 / total_values as f64 * 100.0);
    println!("Unique values (>{}): {} ({:.1}%)", 
             cutoff_length, unique_values,
             unique_values as f64 / total_values as f64 * 100.0);
    
    Ok(())
}
