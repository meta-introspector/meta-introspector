// Find where enum and struct compilation diverge in rustc

use std::collections::{HashMap, HashSet};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Finding Enum vs Struct Divergence Points\n");
    
    // Get function samples from both profiles
    let enum_funcs = get_perf_functions("/tmp/enum_perf.data")?;
    let struct_funcs = get_perf_functions("/tmp/struct_perf.data")?;
    
    // Find common functions with different sample counts
    let mut divergences = Vec::new();
    
    for (func, enum_count) in &enum_funcs {
        if let Some(&struct_count) = struct_funcs.get(func) {
            let diff = (struct_count as i64 - *enum_count as i64).abs();
            if diff > 0 {
                divergences.push((func.clone(), *enum_count, struct_count, diff));
            }
        }
    }
    
    divergences.sort_by_key(|x| std::cmp::Reverse(x.3));
    
    println!("📊 Top 20 Divergence Points (common functions with different counts):\n");
    for (i, (func, enum_c, struct_c, diff)) in divergences.iter().take(20).enumerate() {
        let direction = if struct_c > enum_c { "STRUCT" } else { "ENUM" };
        println!("{:2}. {} enum:{} struct:{} diff:{} [{}]", 
            i+1, 
            func.split('/').last().unwrap_or(func),
            enum_c, 
            struct_c, 
            diff,
            direction
        );
    }
    
    // Find rustc_driver specific divergences
    println!("\n🎯 Rustc Driver Divergences:\n");
    let rustc_divs: Vec<_> = divergences.iter()
        .filter(|(f, _, _, _)| f.contains("rustc_driver") || f.contains("rustc_"))
        .take(10)
        .collect();
    
    for (i, (func, enum_c, struct_c, diff)) in rustc_divs.iter().enumerate() {
        println!("{:2}. enum:{} struct:{} diff:{}", i+1, enum_c, struct_c, diff);
        println!("    {}\n", func);
    }
    
    Ok(())
}

fn get_perf_functions(perf_file: &str) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file])
        .output()?;
    
    let mut funcs = HashMap::new();
    let script = String::from_utf8_lossy(&output.stdout);
    
    for line in script.lines() {
        // Extract function name from perf script output
        if let Some(func) = line.split_whitespace().last() {
            if func.starts_with("(") || func.starts_with("_R") || func.contains(".so") {
                *funcs.entry(func.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    Ok(funcs)
}
