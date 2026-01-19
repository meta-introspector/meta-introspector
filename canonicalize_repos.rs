//! Canonical Git Structure Reorganizer
//! Deduplicates and organizes repos into canonical structure

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CanonicalRepo {
    canonical_path: PathBuf,
    aliases: Vec<String>,
    remotes: Vec<String>,
    object_count: u64,
}

struct RepoCanonicalizer {
    base: PathBuf,
    repos: HashMap<String, CanonicalRepo>,
}

impl RepoCanonicalizer {
    fn new(base: PathBuf) -> Self {
        Self {
            base,
            repos: HashMap::new(),
        }
    }

    fn canonicalize(&mut self, repo_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Get all remotes
        let remotes = self.get_remotes(repo_path)?;
        
        // Compute canonical key (normalized first remote)
        let canonical_key = self.normalize_url(&remotes[0]);
        
        // Check if we already have this repo
        if let Some(canonical) = self.repos.get(&canonical_key) {
            println!("♻️  Duplicate: {} -> {}", repo_path.display(), canonical.canonical_path.display());
            return Ok(canonical.canonical_path.clone());
        }

        // Create canonical path: /mnt/data1/canonical-git/{host}/{org}/{repo}
        let canonical_path = self.canonical_path(&canonical_key);
        
        // Move or link to canonical location
        if !canonical_path.exists() {
            std::fs::create_dir_all(canonical_path.parent().unwrap())?;
            
            // Move objects to canonical location
            Command::new("git")
                .args(&["clone", "--mirror", repo_path.to_str().unwrap(), canonical_path.to_str().unwrap()])
                .status()?;
        }

        // Record it
        let object_count = self.count_objects(&canonical_path)?;
        self.repos.insert(canonical_key.clone(), CanonicalRepo {
            canonical_path: canonical_path.clone(),
            aliases: vec![repo_path.to_string_lossy().to_string()],
            remotes,
            object_count,
        });

        Ok(canonical_path)
    }

    fn normalize_url(&self, url: &str) -> String {
        url.replace("https://", "")
           .replace("http://", "")
           .replace("git@", "")
           .replace(":", "/")
           .replace(".git", "")
    }

    fn canonical_path(&self, key: &str) -> PathBuf {
        // github.com/user/repo -> /mnt/data1/canonical-git/github.com/user/repo
        self.base.join(key)
    }

    fn get_remotes(&self, repo: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo.to_str().unwrap(), "remote", "-v"])
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| l.contains("(fetch)"))
            .map(|l| l.split_whitespace().nth(1).unwrap_or("").to_string())
            .collect())
    }

    fn count_objects(&self, repo: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo.to_str().unwrap(), "count-objects", "-v"])
            .output()?;
        
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.starts_with("count:") {
                return Ok(line.split_whitespace().nth(1).unwrap_or("0").parse()?);
            }
        }
        Ok(0)
    }

    fn save_index(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.repos)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from("/mnt/data1/canonical-git");
    let mut canonicalizer = RepoCanonicalizer::new(base);

    // Read all repos from populate script
    let repos = std::fs::read_to_string(
        std::env::var("HOME")? + "/nix/index/all_repo_paths.txt"
    )?;

    println!("🔄 Canonicalizing {} repos...", repos.lines().count());

    for repo_path in repos.lines() {
        if repo_path.trim().is_empty() { continue; }
        let path = PathBuf::from(repo_path);
        if !path.exists() { continue; }

        match canonicalizer.canonicalize(&path) {
            Ok(canonical) => println!("✓ {} -> {}", repo_path, canonical.display()),
            Err(e) => eprintln!("✗ {}: {}", repo_path, e),
        }
    }

    // Save canonical index
    canonicalizer.save_index(&PathBuf::from("canonical_git_index.json"))?;
    
    println!("\n📊 Canonical structure created");
    println!("  Unique repos: {}", canonicalizer.repos.len());
    
    Ok(())
}
