use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SCANNING EXISTING REPOSITORIES FOR COMPRESSION");
    
    let repos_dir = "/mnt/data1/meta-introspector/data/repos";
    
    // Read existing repos
    let entries = fs::read_dir(repos_dir)?;
    let mut rust_repos = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() || path.is_symlink() {
            // Check if it has Rust files
            let repo_name = path.file_name().unwrap().to_string_lossy();
            
            // Skip domain directories and focus on actual repos
            if !["com", "org", "io", "dev", "net", "edu", "us", "fr", "de", "cz", "me", "ht", "co"].contains(&repo_name.as_ref()) {
                rust_repos.push((repo_name.to_string(), path));
            }
        }
    }
    
    println!("📊 Found {} existing repositories:", rust_repos.len());
    for (name, path) in &rust_repos {
        println!("  {} -> {}", name, path.display());
    }
    
    // Create batch config for existing repos
    let mut jobs = Vec::new();
    
    for (i, (name, path)) in rust_repos.iter().enumerate().take(10) {
        jobs.push(serde_json::json!({
            "name": format!("Compress {}", name),
            "binary": "syn_compressor",
            "args": [path.to_string_lossy()],
            "timeout_seconds": 300,
            "output_file": format!("compression_{}.log", name),
            "depends_on": []
        }));
    }
    
    let batch_config = serde_json::json!({
        "jobs": jobs,
        "max_parallel": 5,
        "global_timeout_minutes": 30
    });
    
    fs::write("existing_repos_compression_batch.json", serde_json::to_string_pretty(&batch_config)?)?;
    
    println!("\n📝 Created batch config: existing_repos_compression_batch.json");
    println!("🚀 Ready to compress {} existing repositories", jobs.len());
    
    Ok(())
}
