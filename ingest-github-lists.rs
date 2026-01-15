// Ingest GitHub stars/forks lists and compare with git-sources registry

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    full_name: String,
    clone_url: String,
    fork: bool,
    html_url: String,
    #[serde(default)]
    stargazers_count: u32,
    #[serde(default)]
    forks_count: u32,
}

#[derive(Debug, Serialize)]
struct RepoAnalysis {
    total_starred: usize,
    total_forks: usize,
    total_originals: usize,
    registered_count: usize,
    missing_count: usize,
    missing_repos: Vec<String>,
    registered_repos: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stars_file = PathBuf::from(std::env::var("HOME")? + "/nix/index/stars.json");
    let registry_file = PathBuf::from("data/git-sources-registry.json");
    
    // Load stars
    let stars_content = fs::read_to_string(&stars_file)?;
    let starred_repos: Vec<GitHubRepo> = serde_json::from_str(&stars_content)?;
    
    println!("=== GitHub Stars Analysis ===");
    println!("Total starred repos: {}", starred_repos.len());
    
    let forks: Vec<_> = starred_repos.iter().filter(|r| r.fork).collect();
    let originals: Vec<_> = starred_repos.iter().filter(|r| !r.fork).collect();
    
    println!("Forks: {}", forks.len());
    println!("Originals: {}", originals.len());
    println!();
    
    // Load git-sources registry
    let registry_content = fs::read_to_string(&registry_file)?;
    let registry: serde_json::Value = serde_json::from_str(&registry_content)?;
    
    let registered_urls: HashSet<String> = registry["sources"]
        .as_object()
        .unwrap()
        .values()
        .filter_map(|v| v["url"].as_str().map(|s| s.to_string()))
        .collect();
    
    println!("Registered repos: {}", registered_urls.len());
    println!();
    
    // Find missing repos
    let mut missing = Vec::new();
    let mut registered = Vec::new();
    
    for repo in &starred_repos {
        let url_variants = vec![
            repo.clone_url.clone(),
            repo.html_url.clone(),
            repo.clone_url.replace("https://", "ssh://git@"),
            format!("ssh://git@github.com/{}.git", repo.full_name),
        ];
        
        if url_variants.iter().any(|u| registered_urls.contains(u)) {
            registered.push(repo.full_name.clone());
        } else {
            missing.push(repo.full_name.clone());
        }
    }
    
    println!("=== Comparison ===");
    println!("Registered from stars: {}", registered.len());
    println!("Missing from registry: {}", missing.len());
    println!();
    
    if !missing.is_empty() {
        println!("=== Missing Repos (Top 20) ===");
        for (i, name) in missing.iter().take(20).enumerate() {
            let repo = starred_repos.iter().find(|r| r.full_name == *name).unwrap();
            println!("{}. {} (fork: {}, stars: {})", 
                i + 1, name, repo.fork, repo.stargazers_count);
        }
        println!();
    }
    
    // Save analysis
    let analysis = RepoAnalysis {
        total_starred: starred_repos.len(),
        total_forks: forks.len(),
        total_originals: originals.len(),
        registered_count: registered.len(),
        missing_count: missing.len(),
        missing_repos: missing,
        registered_repos: registered,
    };
    
    let output_file = "data/github-stars-analysis.json";
    fs::write(output_file, serde_json::to_string_pretty(&analysis)?)?;
    println!("Analysis saved to: {}", output_file);
    
    Ok(())
}
