// GitHub activity scanner using cached data + incremental refresh
// Reads cached JSON, refreshes recent activity, stores in Parquet

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedRepo {
    name: String,
    full_name: String,
    html_url: String,
    clone_url: String,
    #[serde(default)]
    pushed_at: String,
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    fork: bool,
    #[serde(default)]
    owner: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepoStatus {
    name: String,
    full_name: String,
    url: String,
    owner: String,
    last_push: String,
    in_registry: bool,
    needs_refresh: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Loading cached GitHub data");
    
    // Load cached data
    let cached_repos = load_cached_repos()?;
    println!("Loaded {} repos from cache", cached_repos.len());
    
    // Load registry
    let registry = load_registry()?;
    println!("Registry: {} repos", registry.len());
    
    // Compare and identify missing
    let mut statuses = Vec::new();
    
    for repo in cached_repos {
        let normalized_url = normalize_url(&repo.html_url);
        let in_registry = registry.contains(&normalized_url);
        
        // Check if needs refresh (pushed in 2025)
        let needs_refresh = repo.pushed_at.starts_with("2025");
        
        // Extract owner
        let owner = repo.owner
            .as_ref()
            .and_then(|o| o["login"].as_str())
            .unwrap_or("unknown")
            .to_string();
        
        statuses.push(RepoStatus {
            name: repo.name.clone(),
            full_name: repo.full_name.clone(),
            url: repo.html_url.clone(),
            owner,
            last_push: repo.pushed_at.clone(),
            in_registry,
            needs_refresh,
        });
    }
    
    // Sort by last push
    statuses.sort_by(|a, b| b.last_push.cmp(&a.last_push));
    
    // Print summary
    let total = statuses.len();
    let registered = statuses.iter().filter(|s| s.in_registry).count();
    let missing = statuses.iter().filter(|s| !s.in_registry).count();
    let needs_refresh = statuses.iter().filter(|s| s.needs_refresh).count();
    
    println!("\n📊 GitHub Repository Status");
    println!("===========================");
    println!("Total repos: {}", total);
    println!("✅ Registered: {}", registered);
    println!("❌ Missing: {}", missing);
    println!("🔄 Needs refresh (2025 activity): {}", needs_refresh);
    
    println!("\n❌ MISSING from registry (top 30):");
    for status in statuses.iter().filter(|s| !s.in_registry).take(30) {
        println!("  {} by @{} - {} (pushed: {})", 
            status.full_name,
            status.owner,
            status.url,
            status.last_push.get(..10).unwrap_or(&status.last_push)
        );
    }
    
    // Save as JSON (will convert to Parquet next)
    fs::write(
        "data/github-repo-status.json",
        serde_json::to_string_pretty(&statuses)?,
    )?;
    
    println!("\n📄 Status saved: data/github-repo-status.json");
    println!("Next: Convert to Parquet and refresh {} repos", needs_refresh);
    
    Ok(())
}

fn load_cached_repos() -> Result<Vec<CachedRepo>, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let cache_files = vec![
        format!("{}/nix/index/github_meta-introspector_repos.json", home),
        format!("{}/nix/index/stars.json", home),
    ];
    
    let mut all_repos = Vec::new();
    
    for file in cache_files {
        if let Ok(content) = fs::read_to_string(&file) {
            println!("Reading: {}", file);
            
            // Try as array of repos
            if let Ok(repos) = serde_json::from_str::<Vec<CachedRepo>>(&content) {
                all_repos.extend(repos);
            }
            // Try as simple name/url format
            else if let Ok(simple) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                for item in simple {
                    if let (Some(name), Some(url)) = (item["name"].as_str(), item["url"].as_str()) {
                        all_repos.push(CachedRepo {
                            name: name.to_string(),
                            full_name: name.to_string(),
                            html_url: url.to_string(),
                            clone_url: url.to_string(),
                            owner: None,
                            pushed_at: String::new(),
                            updated_at: String::new(),
                            fork: false,
                        });
                    }
                }
            }
        }
    }
    
    // Deduplicate by URL
    let mut seen = std::collections::HashSet::new();
    all_repos.retain(|r| seen.insert(r.html_url.clone()));
    
    Ok(all_repos)
}

fn load_registry() -> Result<std::collections::HashSet<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("data/git-sources-registry.json")?;
    let registry: serde_json::Value = serde_json::from_str(&content)?;
    
    let mut urls = std::collections::HashSet::new();
    if let Some(sources) = registry["sources"].as_object() {
        for source in sources.values() {
            if let Some(url) = source["url"].as_str() {
                urls.insert(normalize_url(url));
            }
        }
    }
    
    Ok(urls)
}

fn normalize_url(url: &str) -> String {
    url.trim_end_matches(".git")
        .trim_end_matches('/')
        .replace("ssh://git@github.com/", "https://github.com/")
        .replace("git@github.com:", "https://github.com/")
        .to_lowercase()
}
