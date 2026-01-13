use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommitStats {
    pub author: String,
    pub total_commits: usize,
    pub repositories: Vec<String>,
    pub commits: Vec<CommitInfo>,
}

fn main() {
    let cache_file = "data/cache/recent_monthly_commits.json";
    let content = fs::read_to_string(cache_file)
        .expect("Failed to read recent_monthly_commits.json");
    
    let repo_commits: Vec<RepoCommits> = serde_json::from_str(&content)
        .expect("Failed to parse JSON");
    
    let mut user_stats: HashMap<String, UserCommitStats> = HashMap::new();
    
    for repo in repo_commits {
        for commit in repo.commits_past_month {
            let author = commit.author.clone();
            let entry = user_stats.entry(author.clone()).or_insert(UserCommitStats {
                author: author.clone(),
                total_commits: 0,
                repositories: Vec::new(),
                commits: Vec::new(),
            });
            
            entry.total_commits += 1;
            if !entry.repositories.contains(&repo.repo_name) {
                entry.repositories.push(repo.repo_name.clone());
            }
            entry.commits.push(commit);
        }
    }
    
    let mut sorted_users: Vec<UserCommitStats> = user_stats.into_values().collect();
    sorted_users.sort_by(|a, b| b.total_commits.cmp(&a.total_commits));
    
    println!("Top contributors in the last month:");
    for (i, user) in sorted_users.iter().take(20).enumerate() {
        println!("{}. {} - {} commits across {} repos", 
                 i + 1, user.author, user.total_commits, user.repositories.len());
    }
    
    fs::create_dir_all("data/cache").expect("Failed to create cache directory");
    let output_file = "data/cache/commits_by_user.json";
    let json_data = serde_json::to_string_pretty(&sorted_users).unwrap();
    fs::write(output_file, json_data).expect("Failed to write user stats");
    
    println!("\nSaved {} users to {}", sorted_users.len(), output_file);
}
