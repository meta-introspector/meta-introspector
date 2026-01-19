// Find duplicate code snippets across the codebase
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use rayon::prelude::*;
use sha2::{Sha256, Digest};

#[derive(Clone)]
struct CodeSnippet {
    file: String,
    line_start: usize,
    line_end: usize,
    hash: String,
    content: String,
}

fn hash_snippet(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn extract_snippets(file_path: &str, min_lines: usize) -> Vec<CodeSnippet> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    
    let lines: Vec<&str> = content.lines().collect();
    let mut snippets = Vec::new();
    
    // Only extract larger snippets to reduce memory
    for size in [10, 20] {
        if size < min_lines {
            continue;
        }
        
        // Sample every 5 lines to reduce memory
        for i in (0..lines.len().saturating_sub(size)).step_by(5) {
            let snippet_lines = &lines[i..i+size];
            let snippet_text = snippet_lines.join("\n");
            
            // Skip if mostly whitespace or comments
            let code_lines: Vec<_> = snippet_lines.iter()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
                .collect();
            
            if code_lines.len() < size / 2 {
                continue;
            }
            
            snippets.push(CodeSnippet {
                file: file_path.to_string(),
                line_start: i + 1,
                line_end: i + size,
                hash: hash_snippet(&snippet_text),
                content: snippet_text,
            });
        }
    }
    
    snippets
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 DUPLICATE CODE SNIPPET DETECTOR\n");
    
    // Find all Rust files
    let rust_files: Vec<String> = glob::glob("**/*.rs")?
        .filter_map(|e| e.ok())
        .filter(|p| !p.to_str().unwrap().contains("/target/"))
        .filter(|p| !p.to_str().unwrap().contains("/demos/archived/"))
        .map(|p| p.to_str().unwrap().to_string())
        .collect();
    
    println!("📊 Scanning {} Rust files...\n", rust_files.len());
    
    // Extract snippets in parallel with progress
    use std::sync::atomic::{AtomicUsize, Ordering};
    let processed = AtomicUsize::new(0);
    
    let all_snippets: Vec<CodeSnippet> = rust_files.par_iter()
        .flat_map(|f| {
            let snippets = extract_snippets(f, 5);
            let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if count % 100 == 0 {
                println!("   Processed {}/{} files ({:.1}%)", 
                    count, rust_files.len(), 
                    count as f64 / rust_files.len() as f64 * 100.0);
            }
            snippets
        })
        .collect();
    
    println!("   Extracted {} code snippets\n", all_snippets.len());
    
    // Group by hash to find duplicates
    let mut by_hash: HashMap<String, Vec<CodeSnippet>> = HashMap::new();
    for snippet in all_snippets {
        by_hash.entry(snippet.hash.clone()).or_insert_with(Vec::new).push(snippet);
    }
    
    // Find duplicates (hash appears in multiple files)
    let mut duplicates: Vec<_> = by_hash.iter()
        .filter(|(_, snippets)| {
            let files: std::collections::HashSet<_> = snippets.iter().map(|s| &s.file).collect();
            files.len() > 1
        })
        .collect();
    
    duplicates.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("🔍 Found {} duplicate code patterns\n", duplicates.len());
    
    // Generate report
    let mut report = String::from("# Duplicate Code Snippets Report\n\n");
    report.push_str(&format!("**Files scanned**: {}\n", rust_files.len()));
    report.push_str(&format!("**Duplicate patterns found**: {}\n\n", duplicates.len()));
    
    for (i, (hash, snippets)) in duplicates.iter().take(100).enumerate() {
        let files: std::collections::HashSet<_> = snippets.iter().map(|s| &s.file).collect();
        let snippet_size = snippets[0].line_end - snippets[0].line_start;
        
        report.push_str(&format!("## Duplicate #{} ({} lines, {} occurrences in {} files)\n\n", 
            i + 1, snippet_size, snippets.len(), files.len()));
        
        report.push_str("**Hash**: `");
        report.push_str(&hash[..16]);
        report.push_str("...`\n\n");
        
        report.push_str("**Locations**:\n");
        for snippet in snippets.iter().take(10) {
            report.push_str(&format!("- `{}` lines {}-{}\n", 
                snippet.file, snippet.line_start, snippet.line_end));
        }
        
        if snippets.len() > 10 {
            report.push_str(&format!("\n... and {} more occurrences\n", snippets.len() - 10));
        }
        
        report.push_str("\n**Code**:\n```rust\n");
        report.push_str(&snippets[0].content.lines().take(20).collect::<Vec<_>>().join("\n"));
        if snippets[0].content.lines().count() > 20 {
            report.push_str("\n... (truncated)");
        }
        report.push_str("\n```\n\n");
        report.push_str("---\n\n");
    }
    
    fs::write("DUPLICATE_CODE_REPORT.md", report)?;
    println!("✅ Report saved to DUPLICATE_CODE_REPORT.md");
    
    Ok(())
}
