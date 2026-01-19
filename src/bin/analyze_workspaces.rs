use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use glob::glob;

#[derive(Serialize, Deserialize)]
struct WorkspaceInfo {
    root: String,
    members: Vec<String>,
    member_deps: HashMap<String, String>, // member -> deps_hash
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Analyzing workspaces and members...");
    
    let mut workspaces = Vec::new();
    let mut deps_by_hash: HashMap<String, Vec<String>> = HashMap::new();
    let gitdirs = fs::read_to_string("gitdirs.txt")?;
    
    for gitdir in gitdirs.lines() {
        let repo_dir = Path::new(gitdir).parent().unwrap();
        let cargo_toml = repo_dir.join("Cargo.toml");
        
        if !cargo_toml.exists() { continue; }
        
        let content = fs::read_to_string(&cargo_toml)?;
        
        // Check if workspace
        if content.contains("[workspace]") {
            let mut workspace = WorkspaceInfo {
                root: repo_dir.to_string_lossy().to_string(),
                members: Vec::new(),
                member_deps: HashMap::new(),
            };
            
            // Parse workspace members
            for line in content.lines() {
                if line.contains("members = [") {
                    // Extract member patterns
                    let members_line = content.lines()
                        .skip_while(|l| !l.contains("members = ["))
                        .take_while(|l| !l.contains("]"))
                        .collect::<Vec<_>>()
                        .join("");
                    
                    // Parse member globs
                    for member in members_line.split('"').filter(|s| s.contains('/') || s.contains('*')) {
                        let pattern = repo_dir.join(member).join("Cargo.toml");
                        
                        for entry in glob(pattern.to_str().unwrap()).ok().into_iter().flatten() {
                            if let Ok(member_toml) = entry {
                                let member_dir = member_toml.parent().unwrap();
                                workspace.members.push(member_dir.to_string_lossy().to_string());
                                
                                // Hash member deps
                                if let Ok(member_content) = fs::read_to_string(&member_toml) {
                                    let hash = hash_deps(&member_content);
                                    workspace.member_deps.insert(
                                        member_dir.to_string_lossy().to_string(),
                                        hash.clone()
                                    );
                                    deps_by_hash.entry(hash).or_insert_with(Vec::new).push(
                                        member_dir.to_string_lossy().to_string()
                                    );
                                }
                            }
                        }
                    }
                    break;
                }
            }
            
            workspaces.push(workspace);
        } else {
            // Regular crate
            let hash = hash_deps(&content);
            deps_by_hash.entry(hash).or_insert_with(Vec::new).push(
                repo_dir.to_string_lossy().to_string()
            );
        }
    }
    
    println!("✅ Found {} workspaces", workspaces.len());
    println!("✅ Found {} unique dependency sets", deps_by_hash.len());
    
    fs::write("workspaces.json", serde_json::to_string_pretty(&workspaces)?)?;
    fs::write("cargo_deps_groups.json", serde_json::to_string_pretty(&deps_by_hash)?)?;
    
    println!("💾 Saved to workspaces.json and cargo_deps_groups.json");
    
    Ok(())
}

fn hash_deps(content: &str) -> String {
    let deps_section = content.lines()
        .skip_while(|l| !l.starts_with("[dependencies]"))
        .take_while(|l| !l.starts_with("[") || l.starts_with("[dependencies"))
        .collect::<Vec<_>>()
        .join("\n");
    
    let mut hasher = Sha256::new();
    hasher.update(deps_section.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}
