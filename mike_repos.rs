use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub date: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommitStats {
    pub author: String,
    pub total_commits: usize,
    pub repositories: Vec<String>,
    pub commits: Vec<CommitInfo>,
}

fn main() {
    let cache_file = "data/cache/commits_by_user.json";
    let content = fs::read_to_string(cache_file)
        .expect("Failed to read commits_by_user.json");
    
    let users: Vec<UserCommitStats> = serde_json::from_str(&content)
        .expect("Failed to parse JSON");
    
    if let Some(mike) = users.iter().find(|u| u.author == "mike dupont") {
        println!("Mike Dupont's repositories from last month:");
        println!("Total commits: {}", mike.total_commits);
        println!("Repositories ({}):", mike.repositories.len());
        
        for (i, repo) in mike.repositories.iter().enumerate() {
            let repo_commits = mike.commits.iter().filter(|c| c.message.contains(repo) || true).count();
            println!("{}. {}", i + 1, repo);
        }
        
        // Get repo paths from recent commits
        let recent_commits_file = "data/cache/recent_monthly_commits.json";
        if let Ok(recent_content) = fs::read_to_string(recent_commits_file) {
            if let Ok(repo_data) = serde_json::from_str::<Vec<serde_json::Value>>(&recent_content) {
                println!("\nRepository paths:");
                for repo_entry in repo_data {
                    if let (Some(name), Some(path)) = (repo_entry.get("repo_name"), repo_entry.get("repo_path")) {
                        if let (Some(name_str), Some(path_str)) = (name.as_str(), path.as_str()) {
                            if mike.repositories.contains(&name_str.to_string()) {
                                println!("{}: {}", name_str, path_str);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Mike Dupont not found in user stats");
    }
}
