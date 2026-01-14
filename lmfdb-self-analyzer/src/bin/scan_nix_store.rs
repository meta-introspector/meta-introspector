// Scan ALL .so files in /nix/store and extract grammars

use std::path::{Path, PathBuf};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Grammar Extraction from ALL /nix/store .so files\n");
    
    // Find all .so files
    let so_files = find_all_so_files()?;
    println!("✅ Found {} .so files\n", so_files.len());
    
    // Sample first 100 for now
    let sample: Vec<_> = so_files.into_iter().take(100).collect();
    
    println!("📊 Analyzing {} .so files...\n", sample.len());
    
    for (i, so_path) in sample.iter().enumerate() {
        println!("{}. {}", i+1, so_path.display());
    }
    
    println!("\n💾 Found {} .so files ready for grammar extraction", sample.len());
    
    Ok(())
}

fn find_all_so_files() -> Result<Vec<PathBuf>, std::io::Error> {
    println!("🔍 Scanning /nix/store for .so files...");
    
    let mut results = Vec::new();
    walk_dir(Path::new("/nix/store"), 0, 3, &mut results)?;
    Ok(results)
}

fn walk_dir(dir: &Path, depth: usize, max_depth: usize, results: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    if depth > max_depth {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|e| e.to_str()).map(|e| e == "so").unwrap_or(false) {
            results.push(path.clone());
        }
        
        if path.is_dir() && !path.is_symlink() {
            walk_dir(&path, depth + 1, max_depth, results)?;
        }
    }
    
    Ok(())
}
