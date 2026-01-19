//! Simple URL Scanner - Extract git URLs from repos
//! Uses git-sources registry (no find needed)

use std::collections::HashSet;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Simple URL Scanner");
    println!("Uses git-sources registry (no find needed)");
    
    let url_pattern = Regex::new(
        r#"(?:https?://|git@)(?:github\.com|gitlab\.com|bitbucket\.org)[:/][^\s"'<>]+"#
    )?;
    
    let mut urls = HashSet::new();
    
    // Get repos from git-sources registry
    let output = std::process::Command::new("./target/release/git-sources")
        .arg("list")
        .output()?;
    
    let registry = String::from_utf8_lossy(&output.stdout);
    
    println!("📋 Scanning repos from git-sources registry...");
    
    for line in registry.lines() {
        if line.contains("Path:") {
            let repo_path = line.split("Path:").nth(1).unwrap().trim();
            println!("  Scanning: {}", repo_path);
            
            // Scan common files in this repo
            for file in &["flake.nix", "default.nix", "Cargo.toml", "README.md", ".gitmodules"] {
                let path = std::path::Path::new(repo_path).join(file);
                if let Ok(content) = std::fs::read_to_string(&path) {
                    for cap in url_pattern.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            urls.insert(url.as_str().to_string());
                        }
                    }
                }
            }
        }
    }
    
    println!("✅ Found {} unique URLs", urls.len());
    
    // Save as text file
    std::fs::create_dir_all("data")?;
    let output = urls.iter().cloned().collect::<Vec<_>>().join("\n");
    std::fs::write("data/scanned_urls.txt", output)?;
    
    println!("💾 Saved to data/scanned_urls.txt");
    
    Ok(())
}
