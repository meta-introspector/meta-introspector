use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone)]
struct CargoDeps {
    repo_path: String,
    deps_hash: String,
    dependencies: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Analyzing Cargo.toml files...");
    
    let mut deps_by_hash: HashMap<String, Vec<String>> = HashMap::new();
    let gitdirs = fs::read_to_string("gitdirs.txt")?;
    
    for gitdir in gitdirs.lines() {
        let repo_dir = Path::new(gitdir).parent().unwrap();
        let cargo_toml = repo_dir.join("Cargo.toml");
        
        if !cargo_toml.exists() { continue; }
        
        let content = fs::read_to_string(&cargo_toml)?;
        
        // Hash dependencies section only
        let deps_section = content.lines()
            .skip_while(|l| !l.starts_with("[dependencies]"))
            .take_while(|l| !l.starts_with("[") || l.starts_with("[dependencies"))
            .collect::<Vec<_>>()
            .join("\n");
        
        let mut hasher = Sha256::new();
        hasher.update(deps_section.as_bytes());
        let hash = format!("{:x}", hasher.finalize())[..16].to_string();
        
        deps_by_hash.entry(hash).or_insert_with(Vec::new).push(
            repo_dir.to_string_lossy().to_string()
        );
    }
    
    println!("✅ Found {} unique dependency sets", deps_by_hash.len());
    println!("📊 Repos per dep set:");
    
    let mut counts: Vec<_> = deps_by_hash.iter()
        .map(|(h, repos)| (repos.len(), h))
        .collect();
    counts.sort_by(|a, b| b.0.cmp(&a.0));
    
    for (count, hash) in counts.iter().take(10) {
        println!("  {} repos share deps hash {}", count, hash);
    }
    
    fs::write("cargo_deps_groups.json", serde_json::to_string_pretty(&deps_by_hash)?)?;
    println!("💾 Saved to cargo_deps_groups.json");
    
    Ok(())
}
