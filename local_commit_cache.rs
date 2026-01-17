use std::process::Command;
use std::fs;
use std::path::Path;
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

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
    pub commits_past_month: Vec<CommitInfo>,
    pub total_commits: usize,
}

fn get_commits_last_month(repo_path: &str) -> Vec<CommitInfo> {
    let one_month_ago = Utc::now() - Duration::days(30);
    let since_date = one_month_ago.format("%Y-%m-%d").to_string();
    
    let output = Command::new("git")
        .args(["log", "--all", "--since", &since_date, "--pretty=format:%H|%ai|%s|%an"])
        .current_dir(repo_path)
        .output();
    
    if let Ok(output) = output {
        let log_output = String::from_utf8_lossy(&output.stdout);
        log_output.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    Some(CommitInfo {
                        hash: parts[0].to_string(),
                        date: parts[1].to_string(),
                        message: parts[2].to_string(),
                        author: parts[3].to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn process_repo(repo_path: String, sender: Sender<RepoCommits>) {
    let repo_name = Path::new(&repo_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let commits = get_commits_last_month(&repo_path);
    let total_commits = commits.len();
    
    let result = RepoCommits {
        repo_name,
        repo_path,
        commits_past_month: commits,
        total_commits,
    };
    
    let _ = sender.send(result);
}

fn main() {
    let (sender, receiver): (Sender<RepoCommits>, Receiver<RepoCommits>) = bounded(1000);
    
    // Find all git repositories
    let output = Command::new("find")
        .args([".", "-name", ".git", "-type", "d"])
        .output()
        .expect("Failed to find git repositories");
    
    let git_dirs = String::from_utf8_lossy(&output.stdout);
    let repo_paths: Vec<String> = git_dirs
        .lines()
        .map(|line| line.trim_end_matches("/.git").to_string())
        .collect();
    
    println!("Found {} repositories", repo_paths.len());
    
    // Spawn worker threads
    let mut handles = Vec::new();
    
    for repo_path in repo_paths {
        let sender_clone = sender.clone();
        let handle = thread::spawn(move || {
            process_repo(repo_path, sender_clone);
        });
        handles.push(handle);
    }
    
    drop(sender);
    
    // Collect results
    let mut all_results = Vec::new();
    while let Ok(result) = receiver.recv() {
        if result.total_commits > 0 {
            println!("Cached: {} ({} commits)", result.repo_name, result.total_commits);
        }
        all_results.push(result);
    }
    
    // Wait for all threads
    for handle in handles {
        let _ = handle.join();
    }
    
    // Save to structured cache
    fs::create_dir_all("data/cache").expect("Failed to create cache directory");
    let cache_file = "data/cache/local_commits.json";
    let json_data = serde_json::to_string_pretty(&all_results).unwrap();
    fs::write(cache_file, json_data).expect("Failed to write cache file");
    
    let total_commits: usize = all_results.iter().map(|r| r.total_commits).sum();
    println!("Cached {} repositories with {} total commits to {}", 
             all_results.len(), total_commits, cache_file);
}
