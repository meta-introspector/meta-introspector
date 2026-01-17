use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serde_json::json;

mod repo_queue_manager;
use repo_queue_manager::{WorkQueue, RepoJob};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 MULTI-WORKER REPOSITORY PROCESSOR");
    println!("====================================");

    let queue = Arc::new(Mutex::new(WorkQueue::load_state()?));
    let mut handles = vec![];

    // Spawn 20 workers
    for worker_id in 0..20 {
        let queue_clone = Arc::clone(&queue);
        
        let handle = thread::spawn(move || {
            worker_loop(worker_id, queue_clone);
        });
        
        handles.push(handle);
    }

    // Wait for all workers to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("🎉 All workers completed!");
    Ok(())
}

fn worker_loop(worker_id: usize, queue: Arc<Mutex<WorkQueue>>) {
    println!("👷 Worker {} started", worker_id);
    
    loop {
        let job = {
            let mut q = queue.lock().unwrap();
            q.next_job()
        };
        
        match job {
            Some(job) => {
                println!("👷 Worker {} processing: {}", worker_id, job.name);
                
                let result = process_repository(&job);
                
                {
                    let mut q = queue.lock().unwrap();
                    match result {
                        Ok(data) => {
                            println!("✅ Worker {} completed: {}", worker_id, job.name);
                            
                            // Save individual result to structured file
                            let result_file = format!("/mnt/data1/meta-introspector/data/processed/repo_results/{}.json", job.name);
                            if let Some(parent) = std::path::Path::new(&result_file).parent() {
                                std::fs::create_dir_all(parent).ok();
                            }
                            std::fs::write(&result_file, &data).ok();
                            
                            q.complete_job(job, Some(data));
                        }
                        Err(error) => {
                            println!("❌ Worker {} failed: {} - {}", worker_id, job.name, error);
                            q.fail_job(job, error);
                        }
                    }
                    
                    // Save progress every job
                    if let Err(e) = q.save_state() {
                        eprintln!("Failed to save state: {}", e);
                    }
                    
                    let (pending, _, completed, failed) = q.stats();
                    if (completed + failed) % 10 == 0 {
                        println!("📊 Progress: {:.1}% ({} pending, {} completed, {} failed)", 
                                q.progress_percent(), pending, completed, failed);
                    }
                }
            }
            None => {
                println!("👷 Worker {} finished - no more jobs", worker_id);
                break;
            }
        }
        
        // Small delay to prevent overwhelming the system
        thread::sleep(Duration::from_millis(100));
    }
}

fn process_repository(job: &RepoJob) -> Result<String, String> {
    let repo_path = &job.path;
    
    // Check if path exists and is a git repo
    if !std::path::Path::new(repo_path).exists() {
        return Err("Path does not exist".to_string());
    }
    
    if !std::path::Path::new(repo_path).join(".git").exists() {
        return Err("Not a git repository".to_string());
    }
    
    // Get git status (ignore submodules as requested)
    let status_output = Command::new("git")
        .args(["-C", repo_path, "status", "--porcelain", "--ignore-submodules"])
        .output()
        .map_err(|e| format!("Git status failed: {}", e))?;
    
    let status_lines = String::from_utf8_lossy(&status_output.stdout);
    let modified_files = status_lines.lines().count();
    let is_clean = modified_files == 0;
    
    // Get current branch
    let branch_output = Command::new("git")
        .args(["-C", repo_path, "branch", "--show-current"])
        .output()
        .map_err(|e| format!("Git branch failed: {}", e))?;
    
    let current_branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
    
    // Get remote URLs
    let remote_output = Command::new("git")
        .args(["-C", repo_path, "remote", "-v"])
        .output()
        .map_err(|e| format!("Git remote failed: {}", e))?;
    
    let remotes = String::from_utf8_lossy(&remote_output.stdout);
    let mut remote_urls = Vec::new();
    for line in remotes.lines() {
        if line.contains("(fetch)") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                remote_urls.push(json!({
                    "name": parts[0],
                    "url": parts[1]
                }));
            }
        }
    }
    
    // Get last commit info
    let commit_output = Command::new("git")
        .args(["-C", repo_path, "log", "-1", "--format=%ci|%h|%s"])
        .output()
        .map_err(|e| format!("Git log failed: {}", e))?;
    
    let commit_string = String::from_utf8_lossy(&commit_output.stdout);
    let commit_info = commit_string.trim();
    let commit_parts: Vec<&str> = commit_info.split('|').collect();
    
    // Build result JSON
    let result = json!({
        "path": repo_path,
        "name": job.name,
        "status": {
            "is_clean": is_clean,
            "modified_files": modified_files,
            "status_output": status_lines.trim()
        },
        "branch": current_branch,
        "remotes": remote_urls,
        "last_commit": {
            "date": commit_parts.first().unwrap_or(&""),
            "hash": commit_parts.get(1).unwrap_or(&""),
            "message": commit_parts.get(2).unwrap_or(&"")
        },
        "processed_at": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(serde_json::to_string_pretty(&result).unwrap())
}
