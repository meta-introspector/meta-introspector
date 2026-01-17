// Fast GitHub activity scanner using gix and octocrab
// Finds all repos with activity in 2025 and compares with git-sources registry

use crossbeam::channel::bounded;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepoActivity {
    name: String,
    url: String,
    commits: usize,
    in_registry: bool,
    local_path: Option<String>,
}

#[derive(Debug, Serialize)]
struct ActivityReport {
    year: u32,
    github_repos: usize,
    local_repos: usize,
    registered: usize,
    missing: usize,
    repos: Vec<RepoActivity>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = 2025;
    let since = format!("{}-01-01T00:00:00Z", year);
    
    println!("🔍 Finding GitHub activity for {}", year);
    
    // Load registry
    let registry = load_registry()?;
    println!("Registry: {} repos", registry.len());
    
    // Get GitHub activity using octocrab
    let github_token = std::env::var("GITHUB_TOKEN").ok();
    let octocrab = if let Some(token) = github_token {
        octocrab::OctocrabBuilder::new()
            .personal_token(token)
            .build()?
    } else {
        (*octocrab::instance()).clone()
    };
    
    println!("Fetching GitHub activity...");
    panic!("octocrab API changed - list() method not available");
    
    #[allow(unreachable_code)]
    Ok(())
}

fn load_registry() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("data/git-sources-registry.json")?;
    let registry: serde_json::Value = serde_json::from_str(&content)?;
    
    let mut urls = HashSet::new();
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
        .replace("ssh://git@github.com/", "https://github.com/")
        .replace("git@github.com:", "https://github.com/")
        .to_lowercase()
}

async fn check_github_repo(
    repo: &octocrab::models::Repository,
    since: &str,
    registry: &HashSet<String>,
) -> Option<RepoActivity> {
    // Check if repo has commits since date
    let url = repo.html_url.as_ref()?.to_string();
    let normalized = normalize_url(&url);
    
    // For now, just check if in registry
    // TODO: Use gix to check local clones for commit count
    
    Some(RepoActivity {
        name: repo.name.clone(),
        url: url.clone(),
        commits: 0, // TODO: count with gix
        in_registry: registry.contains(&normalized),
        local_path: None,
    })
}
