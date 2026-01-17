// 🔥 SIMPLE EXISTING CODE COLLECTOR
// Find and document all LMFDB/meme/godel files

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 EXISTING CODE DOCUMENTATION COLLECTOR");
    println!("========================================");
    
    let patterns = ["lmfdb", "meme", "godel", "solfunmeme", "golem", "muse"];
    let mut total_files = 0;
    
    for pattern in &patterns {
        println!("\n🔍 Collecting {} files...", pattern);
        
        let files = locate_pattern(pattern)?;
        println!("📊 Found {} files", files.len());
        total_files += files.len();
        
        // Show top 5 examples
        for (i, file) in files.iter().take(5).enumerate() {
            let filename = file.split('/').next_back().unwrap_or(file);
            println!("  {}. {}", i+1, filename);
        }
        
        if files.len() > 5 {
            println!("  ... and {} more", files.len() - 5);
        }
    }
    
    println!("\n✅ COLLECTION COMPLETE");
    println!("📊 Total files found: {}", total_files);
    println!("📁 Patterns searched: {}", patterns.len());
    
    Ok(())
}

fn locate_pattern(pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    // Use locate to find files
    let output = Command::new("locate")
        .arg("-i")  // Case insensitive
        .arg(pattern)
        .output()?;
        
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            let line = line.trim();
            if !line.is_empty() && is_relevant_file(line, pattern) {
                files.push(line.to_string());
            }
        }
    }
    
    // Also search for Rust files containing the pattern
    let pattern_query = format!("{}*.rs", pattern);
    let output = Command::new("locate")
        .arg("-i")
        .arg(&pattern_query)
        .output()?;
        
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            let line = line.trim();
            if line.ends_with(".rs") && is_in_relevant_directory(line) {
                files.push(line.to_string());
            }
        }
    }
    
    // Deduplicate and sort
    files.sort();
    files.dedup();
    
    Ok(files)
}

fn is_relevant_file(path: &str, pattern: &str) -> bool {
    // Filter criteria
    (path.contains("meta-introspector") || 
     path.contains("zos") ||
     path.contains("solfunmeme") ||
     (path.ends_with(".rs") && path.contains(pattern))) &&
    !path.contains("/.git/") &&
    !path.contains("/target/") &&
    !path.contains("/.cache/")
}

fn is_in_relevant_directory(path: &str) -> bool {
    path.contains("meta-introspector") ||
    path.contains("zos-qa") ||
    path.contains("zos-server") ||
    path.contains("solfunmeme") ||
    path.contains("/nix/time/")
}
