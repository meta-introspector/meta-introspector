use std::process::Command;
use std::fs;
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub date: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoCommits {
    pub repo_name: String,
    pub repo_path: String,
    pub last_commit_author: String,
    pub is_your_repo: bool,
    pub commits_past_month: Vec<CommitInfo>,
    pub total_commits: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitCollectionJob {
    pub repo_name: String,
    pub repo_path: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitQueue {
    pub jobs: VecDeque<CommitCollectionJob>,
    pub completed: Vec<RepoCommits>,
    pub failed: Vec<String>,
}

impl CommitQueue {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
            completed: Vec::new(),
            failed: Vec::new(),
        }
    }

    pub fn load_from_repo_results() -> Result<Self, Box<dyn std::error::Error>> {
        let mut queue = Self::new();
        let results_dir = "/mnt/data1/meta-introspector/data/processed/repo_results";
        
        println!("📋 Loading repositories from results...");
        
        if let Ok(entries) = fs::read_dir(results_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".json") {
                            let repo_name = name.trim_end_matches(".json").to_string();
                            
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                if let Ok(repo_data) = serde_json::from_str::<serde_json::Value>(&content) {
                                    if let Some(repo_path) = repo_data["path"].as_str() {
                                        queue.jobs.push_back(CommitCollectionJob {
                                            repo_name,
                                            repo_path: repo_path.to_string(),
                                            status: "pending".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("📊 Loaded {} repositories for commit collection", queue.jobs.len());
        Ok(queue)
    }

    pub fn process_next(&mut self, target_author: &str) -> Option<()> {
        if let Some(mut job) = self.jobs.pop_front() {
            job.status = "processing".to_string();
            
            match collect_repo_commits(&job.repo_path, &job.repo_name, target_author) {
                Ok(repo_commits) => {
                    if repo_commits.is_your_repo {
                        println!("✅ Found your repo: {} ({} commits)", 
                                repo_commits.repo_name, repo_commits.total_commits);
                    }
                    self.completed.push(repo_commits);
                }
                Err(e) => {
                    println!("❌ Failed: {} - {}", job.repo_name, e);
                    self.failed.push(format!("{}: {}", job.repo_name, e));
                }
            }
            
            Some(())
        } else {
            None
        }
    }

    pub fn save_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Save individual results
        for repo_commits in &self.completed {
            if repo_commits.is_your_repo && repo_commits.total_commits > 0 {
                let result_file = format!(
                    "/mnt/data1/meta-introspector/data/processed/commit_results/{}.json",
                    repo_commits.repo_name
                );
                
                if let Some(parent) = std::path::Path::new(&result_file).parent() {
                    fs::create_dir_all(parent)?;
                }
                
                let json = serde_json::to_string_pretty(repo_commits)?;
                fs::write(&result_file, json)?;
            }
        }
        
        // Save summary
        let summary = serde_json::json!({
            "generated_at": Utc::now().to_rfc3339(),
            "total_repos_processed": self.completed.len(),
            "your_repos_found": self.completed.iter().filter(|r| r.is_your_repo).count(),
            "total_commits_found": self.completed.iter()
                .filter(|r| r.is_your_repo)
                .map(|r| r.total_commits)
                .sum::<usize>(),
            "failed_repos": self.failed.len()
        });
        
        fs::write(
            "/mnt/data1/meta-introspector/data/processed/commit_collection_summary.json",
            serde_json::to_string_pretty(&summary)?
        )?;
        
        Ok(())
    }
}

fn collect_repo_commits(
    repo_path: &str,
    repo_name: &str,
    target_author: &str,
) -> Result<RepoCommits, Box<dyn std::error::Error>> {
    
    if !std::path::Path::new(repo_path).exists() {
        return Err("Repository path does not exist".into());
    }
    
    if !std::path::Path::new(&format!("{}/.git", repo_path)).exists() {
        return Err("Not a git repository".into());
    }
    
    // Get last commit author
    let last_author_output = Command::new("git")
        .args(&["-C", repo_path, "log", "-1", "--format=%an"])
        .output()?;
    
    let last_commit_author = String::from_utf8_lossy(&last_author_output.stdout).trim().to_string();
    let is_your_repo = last_commit_author.to_lowercase().contains(&target_author.to_lowercase());
    
    let mut commits = Vec::new();
    
    if is_your_repo {
        // Get your commits from past month
        let commits_output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                &format!("--author={}", target_author),
                "--since=1 month ago",
                "--format=%H|%ci|%s|%an"
            ])
            .output()?;
        
        let commits_text = String::from_utf8_lossy(&commits_output.stdout);
        
        for line in commits_text.lines() {
            if !line.is_empty() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    commits.push(CommitInfo {
                        hash: parts[0].to_string(),
                        date: parts[1].to_string(),
                        message: parts[2].to_string(),
                        author: parts[3].to_string(),
                    });
                }
            }
        }
    }
    
    Ok(RepoCommits {
        repo_name: repo_name.to_string(),
        repo_path: repo_path.to_string(),
        last_commit_author,
        is_your_repo,
        total_commits: commits.len(),
        commits_past_month: commits,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 COMMIT COLLECTION SYSTEM");
    println!("===========================");
    
    let mut queue = CommitQueue::load_from_repo_results()?;
    let target_author = "mdupont";
    
    println!("🔍 Processing {} repositories...", queue.jobs.len());
    
    while queue.process_next(target_author).is_some() {
        if queue.completed.len() % 50 == 0 {
            println!("📊 Processed: {} repos", queue.completed.len());
        }
    }
    
    let your_repos = queue.completed.iter().filter(|r| r.is_your_repo).count();
    let total_commits: usize = queue.completed.iter()
        .filter(|r| r.is_your_repo)
        .map(|r| r.total_commits)
        .sum();
    
    println!("\n🎉 COLLECTION COMPLETE!");
    println!("📊 Results:");
    println!("   - Total repos processed: {}", queue.completed.len());
    println!("   - Your repos found: {}", your_repos);
    println!("   - Your commits (past month): {}", total_commits);
    println!("   - Failed repos: {}", queue.failed.len());
    
    queue.save_results()?;
    println!("💾 Results saved to data/processed/commit_results/");
    
    Ok(())
}
