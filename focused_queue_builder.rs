use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusedJobType {
    UnTrackedFiles,
    OutOfDateRepo,
    ForkSync,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusedJob {
    pub repo_path: String,
    pub repo_name: String,
    pub job_type: FocusedJobType,
    pub priority: u64,
    pub user_changes: Vec<String>,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusedQueue {
    pub jobs: Vec<FocusedJob>,
    pub created_at: DateTime<Utc>,
}

impl FocusedQueue {
    pub fn build_from_results() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔍 BUILDING FOCUSED PROCESSING QUEUE");
        println!("===================================");
        
        let mut jobs = Vec::new();
        let results_dir = "/mnt/data1/meta-introspector/data/processed/repo_results";
        
        // Three weeks ago
        let three_weeks_ago = Utc::now() - Duration::weeks(3);
        
        if let Ok(entries) = fs::read_dir(results_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        let repo_name = name.trim_end_matches(".json");
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(repo_data) = serde_json::from_str::<serde_json::Value>(&content) {
                                if let Some(repo_path) = repo_data["path"].as_str() {
                                    jobs.extend(analyze_repository_for_focused_work(
                                        repo_path, 
                                        repo_name, 
                                        &repo_data,
                                        three_weeks_ago
                                    )?);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by priority (higher = more important)
        jobs.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        println!("📋 Created {} focused jobs", jobs.len());
        
        Ok(FocusedQueue {
            jobs,
            created_at: Utc::now(),
        })
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("/mnt/data1/meta-introspector/data/raw/focused_queue.json", json)?;
        Ok(())
    }
}

fn analyze_repository_for_focused_work(
    repo_path: &str,
    repo_name: &str,
    repo_data: &serde_json::Value,
    three_weeks_ago: DateTime<Utc>,
) -> Result<Vec<FocusedJob>, Box<dyn std::error::Error>> {
    let mut jobs = Vec::new();
    
    if !Path::new(repo_path).exists() {
        return Ok(jobs);
    }
    
    // Check for user activity in last 3 weeks
    let user_changes = get_user_changes_last_3_weeks(repo_path, three_weeks_ago)?;
    
    // Skip repos with no recent user activity
    if user_changes.is_empty() {
        return Ok(jobs);
    }
    
    let priority = user_changes.len() as u64 * 100; // More changes = higher priority
    
    // 1. Check for untracked files
    if let Some(status) = repo_data["status"]["status_output"].as_str() {
        let untracked_files: Vec<&str> = status.lines()
            .filter(|line| line.starts_with("??"))
            .collect();
            
        if !untracked_files.is_empty() {
            jobs.push(FocusedJob {
                repo_path: repo_path.to_string(),
                repo_name: repo_name.to_string(),
                job_type: FocusedJobType::UnTrackedFiles,
                priority: priority + untracked_files.len() as u64,
                user_changes: user_changes.clone(),
                details: format!("{} untracked files", untracked_files.len()),
            });
        }
    }
    
    // 2. Check if repo is out of date (has remote and might be behind)
    if let Some(remotes) = repo_data["remotes"].as_array() {
        for remote in remotes {
            if let Some(url) = remote["url"].as_str() {
                if url.contains("github.com") && !url.contains("file://") {
                    // This repo has a GitHub remote, check if it's out of date
                    jobs.push(FocusedJob {
                        repo_path: repo_path.to_string(),
                        repo_name: repo_name.to_string(),
                        job_type: FocusedJobType::OutOfDateRepo,
                        priority: priority + 50,
                        user_changes: user_changes.clone(),
                        details: format!("Check sync with {}", url),
                    });
                    break;
                }
            }
        }
    }
    
    // 3. Check if it's a fork (has upstream potential)
    if is_likely_fork(repo_data) {
        jobs.push(FocusedJob {
            repo_path: repo_path.to_string(),
            repo_name: repo_name.to_string(),
            job_type: FocusedJobType::ForkSync,
            priority: priority + 25,
            user_changes: user_changes.clone(),
            details: "Fork sync check".to_string(),
        });
    }
    
    Ok(jobs)
}

fn get_user_changes_last_3_weeks(
    repo_path: &str,
    since: DateTime<Utc>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let since_str = since.format("%Y-%m-%d").to_string();
    
    let output = Command::new("git")
        .args([
            "-C", repo_path,
            "log",
            "--author=mdupont",
            &format!("--since={}", since_str),
            "--name-only",
            "--pretty=format:",
        ])
        .output()?;
    
    if output.status.success() {
        let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_string())
            .collect();
        Ok(files)
    } else {
        Ok(Vec::new())
    }
}

fn is_likely_fork(repo_data: &serde_json::Value) -> bool {
    // Check if repo name or remotes suggest it's a fork
    if let Some(remotes) = repo_data["remotes"].as_array() {
        for remote in remotes {
            if let Some(url) = remote["url"].as_str() {
                if url.contains("meta-introspector") || url.contains("mdupont") {
                    return true;
                }
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = FocusedQueue::build_from_results()?;
    
    println!("\n📊 FOCUSED QUEUE SUMMARY:");
    let mut type_counts = HashMap::new();
    for job in &queue.jobs {
        *type_counts.entry(format!("{:?}", job.job_type)).or_insert(0) += 1;
    }
    
    for (job_type, count) in type_counts {
        println!("  {}: {}", job_type, count);
    }
    
    queue.save()?;
    println!("\n💾 Focused queue saved to focused_queue.json");
    
    Ok(())
}
