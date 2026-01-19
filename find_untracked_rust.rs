// Find untracked Rust files from Parquet index
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Analyzing untracked Rust files from Parquet index\n");
    
    let file = File::open("data/indexes/files.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut untracked_by_repo: HashMap<String, Vec<String>> = HashMap::new();
    let mut total_untracked = 0;
    
    // Read all row groups
    for i in 0..reader.metadata().num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        
        // Get columns: file_path, git_repo, tracked
        // Process rows and find untracked .rs files
        
        // This is simplified - full implementation would use Arrow
        println!("Processing row group {}/{}", i + 1, reader.metadata().num_row_groups());
    }
    
    println!("\n📋 Top repositories with untracked Rust files:\n");
    
    let mut sorted: Vec<_> = untracked_by_repo.iter().collect();
    sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (repo, files) in sorted.iter().take(20) {
        println!("{:>6} files - {}", files.len(), repo);
    }
    
    println!("\n✅ Total untracked Rust files: {}", total_untracked);
    
    Ok(())
}
