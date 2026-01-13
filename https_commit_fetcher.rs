use std::process::Command;
use std::fs;
use std::path::Path;
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use regex::Regex;

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
    pub remotes_converted: bool,
    pub fetch_success: bool,
    pub commits_past_month: Vec<CommitInfo>,
    pub total_commits: usize,
}

fn convert_ssh_to_https(repo_path: &str) -> bool {
    let output = Command::new("git")
        .args(&["remote", "-v"])
        .current_dir(repo_path)
        .output();
    
    if let Ok(output) = output {
        let remotes = String::from_utf8_lossy(&output.stdout);
        let ssh_regex = Regex::new(r"git@github\.com:([^/]+)/([^\.]+)\.git").unwrap();
        
        for line in remotes.lines() {
            if let Some(caps) = ssh_regex.captures(line) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let remote_name = parts[0];
                    let user = &caps[1];
                    let repo = &caps[2];
                    let https_url = format!("https://github.com/{}/{}.git", user, repo);
                    
                    let _ = Command::new("git")
                        .args(&["remote", "set-url", remote_name, &https_url])
                        .current_dir(repo_path)
                        .output();
                }
            }
        }
        true
    } else {
        false
    }
}

fn fetch_all_remotes(repo_path: &str) -> bool {
    let output = Command::new("git")
        .args(&["fetch", "--all"])
        .current_dir(repo_path)
        .output();
    
    output.is_ok() && output.unwrap().status.success()
}

fn get_commits_last_month(repo_path: &str) -> Vec<CommitInfo> {
    let one_month_ago = Utc::now() - Duration::days(30);
    let since_date = one_month_ago.format("%Y-%m-%d").to_string();
    
    let output = Command::new("git")
        .args(&["log", "--all", "--since", &since_date, "--pretty=format:%H|%ai|%s|%an"])
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
    
    let remotes_converted = convert_ssh_to_https(&repo_path);
    let fetch_success = fetch_all_remotes(&repo_path);
    let commits = get_commits_last_month(&repo_path);
    let total_commits = commits.len();
    
    let result = RepoCommits {
        repo_name,
        repo_path,
        remotes_converted,
        fetch_success,
        commits_past_month: commits,
        total_commits,
    };
    
    let _ = sender.send(result);
}

fn main() {
    let (sender, receiver): (Sender<RepoCommits>, Receiver<RepoCommits>) = bounded(1000);
    
    // Find all git repositories
    let output = Command::new("find")
        .args(&[".", "-name", ".git", "-type", "d"])
        .output()
        .expect("Failed to find git repositories");
    
    let git_dirs = String::from_utf8_lossy(&output.stdout);
    let repo_paths: Vec<String> = git_dirs
        .lines()
        .map(|line| line.trim_end_matches("/.git").to_string())
        .collect();
    
    println!("Found {} repositories", repo_paths.len());
    
    // Spawn worker threads
    let num_workers = 8;
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
        println!("Processed: {} ({} commits)", result.repo_name, result.total_commits);
        all_results.push(result);
    }
    
    // Wait for all threads
    for handle in handles {
        let _ = handle.join();
    }
    
    // Save to structured cache
    fs::create_dir_all("data/cache").expect("Failed to create cache directory");
    let cache_file = "data/cache/monthly_commits.json";
    let json_data = serde_json::to_string_pretty(&all_results).unwrap();
    fs::write(cache_file, json_data).expect("Failed to write cache file");
    
    println!("Processed {} repositories, saved to {}", all_results.len(), cache_file);
}
