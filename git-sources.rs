// Centralized Git Repository Management System
// Manages canonical checkouts with symlinks (no git submodules)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSource {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub checkout_path: PathBuf,
    pub canonical_link: PathBuf,
    pub last_commit: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRegistry {
    pub sources: HashMap<String, GitSource>,
    pub base_checkout_dir: PathBuf,
    pub canonical_links_dir: PathBuf,
}

impl GitRegistry {
    pub fn new(base_dir: &Path, links_dir: &Path) -> Self {
        Self {
            sources: HashMap::new(),
            base_checkout_dir: base_dir.to_path_buf(),
            canonical_links_dir: links_dir.to_path_buf(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn register_existing(&mut self, name: &str, existing_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let canonical_name = Self::canonicalize_name(name);
        let canonical_link = self.canonical_links_dir.join(&canonical_name);

        let url = Self::get_remote_url(existing_path)?;
        let branch = Self::get_current_branch(existing_path)?;
        let last_commit = Self::get_last_commit(existing_path)?;

        let source = GitSource {
            name: name.to_string(),
            url,
            branch,
            checkout_path: existing_path.to_path_buf(),
            canonical_link: canonical_link.clone(),
            last_commit: Some(last_commit),
            last_updated: Some(chrono::Utc::now().to_rfc3339()),
        };

        if canonical_link.exists() {
            fs::remove_file(&canonical_link)?;
        }
        std::os::unix::fs::symlink(existing_path, &canonical_link)?;

        self.sources.insert(canonical_name, source);
        Ok(())
    }

    pub fn scan_directory(&mut self, scan_dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        for entry in fs::read_dir(scan_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.join(".git").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(_) = self.register_existing(name, &path) {
                        count += 1;
                        println!("Registered: {}", name);
                    }
                }
            }
        }
        Ok(count)
    }

    pub fn ingest_list(&mut self, list_file: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(list_file)?;
        let mut count = 0;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let path = PathBuf::from(line);
            if !path.exists() {
                eprintln!("Path not found: {}", line);
                continue;
            }

            if path.is_dir() {
                if path.join(".git").exists() {
                    // Single git repo
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        match self.register_existing(name, &path) {
                            Ok(_) => {
                                count += 1;
                                println!("Registered: {}", name);
                            }
                            Err(e) => eprintln!("Failed to register {}: {}", name, e),
                        }
                    }
                } else {
                    // Directory containing repos - scan it
                    match self.scan_directory(&path) {
                        Ok(n) => count += n,
                        Err(e) => eprintln!("Failed to scan {}: {}", line, e),
                    }
                }
            }
        }
        
        Ok(count)
    }

    fn canonicalize_name(name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }

    fn get_remote_url(repo_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo_path.to_str().unwrap(), "remote", "get-url", "origin"])
            .output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn get_current_branch(repo_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo_path.to_str().unwrap(), "branch", "--show-current"])
            .output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn get_last_commit(repo_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo_path.to_str().unwrap(), "rev-parse", "HEAD"])
            .output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn list(&self) {
        println!("Git Sources Registry ({} repos):", self.sources.len());
        println!();
        for (canonical_name, source) in &self.sources {
            println!("  {} -> {}", canonical_name, source.name);
            println!("    URL: {}", source.url);
            println!("    Branch: {}", source.branch);
            println!("    Path: {}", source.checkout_path.display());
            println!("    Link: {}", source.canonical_link.display());
            if let Some(commit) = &source.last_commit {
                println!("    Commit: {}", &commit[..8]);
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let registry_file = PathBuf::from("data/git-sources-registry.json");
    let base_dir = PathBuf::from("/mnt/data1/nix/time/2025/08/07");
    let links_dir = PathBuf::from("data/git-sources");

    fs::create_dir_all(&links_dir)?;
    fs::create_dir_all(registry_file.parent().unwrap())?;

    let mut registry = if registry_file.exists() {
        GitRegistry::load(&registry_file)?
    } else {
        GitRegistry::new(&base_dir, &links_dir)
    };

    match args.get(1).map(|s| s.as_str()) {
        Some("ingest") => {
            let list_file = PathBuf::from(args.get(2).unwrap_or(&"list.txt".to_string()));
            let count = registry.ingest_list(&list_file)?;
            println!("Ingested {} repositories from {}", count, list_file.display());
            registry.save(&registry_file)?;
        }
        Some("scan") => {
            let scan_dir = args.get(2).map(PathBuf::from).unwrap_or(base_dir);
            let count = registry.scan_directory(&scan_dir)?;
            println!("Scanned and registered {} repositories", count);
            registry.save(&registry_file)?;
        }
        Some("register") => {
            let name = args.get(2).ok_or("Missing name")?;
            let path = PathBuf::from(args.get(3).ok_or("Missing path")?);
            registry.register_existing(name, &path)?;
            registry.save(&registry_file)?;
            println!("Registered {}", name);
        }
        Some("list") => {
            registry.list();
        }
        _ => {
            println!("Git Sources - Centralized Repository Management");
            println!();
            println!("Usage:");
            println!("  git-sources ingest [list.txt]     - Ingest repos from list file");
            println!("  git-sources scan [directory]      - Scan directory for git repos");
            println!("  git-sources register <name> <path> - Register existing checkout");
            println!("  git-sources list                  - List all registered repos");
            println!();
            println!("Registry: {}", registry_file.display());
            println!("Symlinks: {}", links_dir.display());
        }
    }

    Ok(())
}
