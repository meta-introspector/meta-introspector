// Check which repos from activity dataset are in git-sources registry
// Scans activity dataset and reports coverage

use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct CoverageReport {
    total_repos_in_activity: usize,
    repos_in_registry: usize,
    repos_missing: usize,
    coverage_percent: f64,
    missing_repos: Vec<MissingRepo>,
}

#[derive(Debug, Clone, Serialize)]
struct MissingRepo {
    name: String,
    url: String,
    platform: String,
    commit_count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking activity dataset coverage");
    
    // Load registry
    let registry = load_registry()?;
    println!("Registry: {} repos", registry.len());
    
    // Scan activity dataset
    let activity_repos = scan_activity_dataset()?;
    println!("Activity dataset: {} unique repos", activity_repos.len());
    
    // Compare
    let mut missing = Vec::new();
    let mut found = 0;
    
    for (url, (name, platform, count)) in &activity_repos {
        let normalized = normalize_url(url);
        if registry.contains(&normalized) {
            found += 1;
        } else {
            missing.push(MissingRepo {
                name: name.clone(),
                url: url.clone(),
                platform: platform.clone(),
                commit_count: *count,
            });
        }
    }
    
    // Sort by commit count
    missing.sort_by(|a, b| b.commit_count.cmp(&a.commit_count));
    
    let coverage = (found as f64 / activity_repos.len() as f64) * 100.0;
    
    let report = CoverageReport {
        total_repos_in_activity: activity_repos.len(),
        repos_in_registry: found,
        repos_missing: missing.len(),
        coverage_percent: coverage,
        missing_repos: missing.clone(),
    };
    
    // Print summary
    println!("\n📊 Coverage Report");
    println!("==================");
    println!("Total repos in activity: {}", report.total_repos_in_activity);
    println!("✅ In registry: {} ({:.1}%)", report.repos_in_registry, coverage);
    println!("❌ Missing: {} ({:.1}%)", report.repos_missing, 100.0 - coverage);
    
    println!("\n❌ Top 30 missing repos by activity:");
    for (i, repo) in missing.iter().take(30).enumerate() {
        println!("  {}. {} ({} commits) - {} [{}]", 
            i + 1, repo.name, repo.commit_count, repo.url, repo.platform);
    }
    
    // Save report
    fs::write(
        "data/activity-coverage-report.json",
        serde_json::to_string_pretty(&report)?,
    )?;
    
    println!("\n📄 Full report: data/activity-coverage-report.json");
    
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

fn scan_activity_dataset() -> Result<HashMap<String, (String, String, usize)>, Box<dyn std::error::Error>> {
    let mut repos: HashMap<String, (String, String, usize)> = HashMap::new();
    
    // Walk activity directory
    walk_dir(Path::new("data/activity"), &mut repos)?;
    
    Ok(repos)
}

fn walk_dir(dir: &Path, repos: &mut HashMap<String, (String, String, usize)>) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.exists() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            walk_dir(&path, repos)?;
        } else if path.file_name().and_then(|n| n.to_str()) == Some("activity.json") {
            // Parse activity file
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(commits) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                    for commit in commits {
                        if let (Some(url), Some(name), Some(platform)) = (
                            commit["repo_url"].as_str(),
                            commit["repo_name"].as_str(),
                            commit["platform"].as_str(),
                        ) {
                            let entry = repos.entry(url.to_string())
                                .or_insert((name.to_string(), platform.to_string(), 0));
                            entry.2 += 1;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn normalize_url(url: &str) -> String {
    url.trim_end_matches(".git")
        .trim_end_matches('/')
        .replace("ssh://git@github.com/", "https://github.com/")
        .replace("git@github.com:", "https://github.com/")
        .to_lowercase()
}
