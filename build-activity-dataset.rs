// Canonical Git Activity Dataset Builder
// Extracts activity from all registered repos and saves to:
// data/activity/{platform}/{user}/{year}/{month}/activity.parquet

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitActivity {
    commit_hash: String,
    author_name: String,
    author_email: String,
    author_date: String,
    committer_name: String,
    committer_email: String,
    committer_date: String,
    message: String,
    repo_name: String,
    repo_url: String,
    platform: String,
    files_changed: i32,
    insertions: i32,
    deletions: i32,
}

#[derive(Debug)]
struct ActivityDataset {
    base_path: PathBuf,
    commits: Vec<CommitActivity>,
}

impl ActivityDataset {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
            commits: Vec::new(),
        }
    }

    fn extract_from_repo(&mut self, repo_path: &str, repo_name: &str, repo_url: &str) -> Result<usize, Box<dyn std::error::Error>> {
        println!("Extracting from: {}", repo_name);
        
        let platform = Self::detect_platform(repo_url);
        
        // Get commits with full info
        let output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                "--all",
                "--format=%H|%an|%ae|%aI|%cn|%ce|%cI|%s",
                "--numstat",
            ])
            .output()?;
        
        let log = String::from_utf8_lossy(&output.stdout);
        let mut current_commit: Option<CommitActivity> = None;
        let mut count = 0;
        
        for line in log.lines() {
            if line.contains('|') && line.len() > 40 {
                // Save previous commit
                if let Some(commit) = current_commit.take() {
                    self.commits.push(commit);
                    count += 1;
                }
                
                // Parse new commit
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 8 {
                    current_commit = Some(CommitActivity {
                        commit_hash: parts[0].to_string(),
                        author_name: parts[1].to_string(),
                        author_email: parts[2].to_string(),
                        author_date: parts[3].to_string(),
                        committer_name: parts[4].to_string(),
                        committer_email: parts[5].to_string(),
                        committer_date: parts[6].to_string(),
                        message: parts[7..].join("|"),
                        repo_name: repo_name.to_string(),
                        repo_url: repo_url.to_string(),
                        platform: platform.clone(),
                        files_changed: 0,
                        insertions: 0,
                        deletions: 0,
                    });
                }
            } else if let Some(ref mut commit) = current_commit {
                // Parse numstat line
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let (Ok(ins), Ok(del)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        commit.insertions += ins;
                        commit.deletions += del;
                        commit.files_changed += 1;
                    }
                }
            }
        }
        
        // Save last commit
        if let Some(commit) = current_commit {
            self.commits.push(commit);
            count += 1;
        }
        
        Ok(count)
    }

    fn detect_platform(url: &str) -> String {
        if url.is_empty() {
            return "local".to_string();
        }
        if url.contains("github.com") {
            "github".to_string()
        } else if url.contains("codeberg.org") {
            "codeberg".to_string()
        } else if url.contains("huggingface.co") {
            "huggingface".to_string()
        } else if url.contains("gitlab") {
            "gitlab".to_string()
        } else if url.contains("sr.ht") {
            "sourcehut".to_string()
        } else if url.starts_with('/') || url.starts_with("file://") {
            "local".to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn extract_user_from_email(email: &str) -> String {
        email.split('@').next().unwrap_or("unknown").to_string()
    }

    fn save_to_parquet(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Group by platform/user/year/month
        let mut grouped: HashMap<String, Vec<CommitActivity>> = HashMap::new();
        
        for commit in &self.commits {
            let user = Self::extract_user_from_email(&commit.author_email);
            let date = &commit.author_date;
            
            // Extract year-month (YYYY-MM)
            let year_month = if date.len() >= 7 {
                &date[..7]
            } else {
                "unknown"
            };
            
            let parts: Vec<&str> = year_month.split('-').collect();
            if parts.len() >= 2 {
                let year = parts[0];
                let month = parts[1];
                
                let key = format!("{}/{}/{}/{}", commit.platform, user, year, month);
                grouped.entry(key).or_insert_with(Vec::new).push(commit.clone());
            }
        }
        
        println!("\nSaving to parquet files...");
        
        for (path_key, commits) in grouped {
            let output_dir = self.base_path.join(&path_key);
            fs::create_dir_all(&output_dir)?;
            
            let json_path = output_dir.join("activity.json");
            fs::write(&json_path, serde_json::to_string_pretty(&commits)?)?;
            
            println!("  {} commits -> {}", commits.len(), path_key);
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Building Canonical Git Activity Dataset");
    println!("Format: data/activity/{{platform}}/{{user}}/{{year}}/{{month}}/activity.parquet");
    println!();
    
    let mut dataset = ActivityDataset::new("data/activity");
    
    // Load registry
    let registry_content = fs::read_to_string("data/git-sources-registry.json")?;
    let registry: serde_json::Value = serde_json::from_str(&registry_content)?;
    
    let sources = registry["sources"].as_object().ok_or("No sources")?;
    println!("Processing {} repositories...\n", sources.len());
    
    let mut total_commits = 0;
    
    for (name, source) in sources {
        let path = source["checkout_path"].as_str().ok_or("No path")?;
        let url = source["url"].as_str().unwrap_or("");
        
        match dataset.extract_from_repo(path, name, url) {
            Ok(count) => {
                total_commits += count;
                println!("  ✓ {} commits", count);
            }
            Err(e) => {
                eprintln!("  ✗ Error: {}", e);
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("Total commits extracted: {}", total_commits);
    
    dataset.save_to_parquet()?;
    
    println!("\n✅ Dataset saved to: data/activity/");
    println!("Next: Convert JSON to Parquet format");
    
    Ok(())
}
