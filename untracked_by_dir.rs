// Generate report of 33,639 untracked Rust files by directory/subproject
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 UNTRACKED RUST FILES BY DIRECTORY/SUBPROJECT");
    println!("===============================================\n");
    
    // Find all untracked .rs files
    let output = std::process::Command::new("find")
        .args(["/mnt/data1", "-name", "*.rs", "-type", "f"])
        .output()?;
    
    let all_files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    println!("Found {} total Rust files", all_files.len());
    
    // Check git status for each
    let mut by_dir: HashMap<String, Vec<String>> = HashMap::new();
    let mut untracked_count = 0;
    
    for file in &all_files {
        let path = Path::new(file);
        
        // Get git status
        if let Some(parent) = path.parent() {
            let git_check = std::process::Command::new("git")
                .args(["-C", parent.to_str().unwrap(), "ls-files", "--error-unmatch", file])
                .output();
            
            if let Ok(result) = git_check {
                if !result.status.success() {
                    // File is untracked
                    untracked_count += 1;
                    
                    // Categorize by top-level directory
                    let parts: Vec<&str> = file.split('/').collect();
                    let category = if parts.len() > 3 {
                        format!("/{}/{}", parts[1], parts[2])
                    } else if parts.len() > 2 {
                        format!("/{}", parts[1])
                    } else {
                        "/root".to_string()
                    };
                    
                    by_dir.entry(category).or_insert_with(Vec::new).push(file.clone());
                }
            }
        }
    }
    
    println!("\n📈 UNTRACKED RUST FILES: {}\n", untracked_count);
    
    // Sort by count
    let mut sorted: Vec<_> = by_dir.iter().collect();
    sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    // Generate report
    let mut report = String::from("# Untracked Rust Files by Directory/Subproject\n\n");
    report.push_str(&format!("**Total untracked**: {} files\n\n", untracked_count));
    report.push_str("## Summary by Directory\n\n");
    report.push_str("| Directory | Count | Percentage |\n");
    report.push_str("|-----------|-------|------------|\n");
    
    for (dir, files) in &sorted {
        let pct = files.len() as f64 / untracked_count as f64 * 100.0;
        report.push_str(&format!("| {} | {} | {:.1}% |\n", dir, files.len(), pct));
        println!("{:50} {:>6} files ({:.1}%)", dir, files.len(), pct);
    }
    
    // Detailed breakdown
    report.push_str("\n## Detailed Breakdown\n\n");
    
    for (dir, files) in &sorted {
        report.push_str(&format!("### {} ({} files)\n\n", dir, files.len()));
        
        // Show first 20 files
        for (i, file) in files.iter().take(20).enumerate() {
            let filename = Path::new(file).file_name().unwrap().to_str().unwrap();
            report.push_str(&format!("{}. `{}`\n", i + 1, filename));
        }
        
        if files.len() > 20 {
            report.push_str(&format!("\n... and {} more files\n", files.len() - 20));
        }
        report.push_str("\n");
    }
    
    // Save report
    fs::write("UNTRACKED_RUST_BY_DIR.md", report)?;
    println!("\n✅ Report saved to: UNTRACKED_RUST_BY_DIR.md");
    
    Ok(())
}
