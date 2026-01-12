use std::process::Command;
use std::fs;
use chrono::Utc;

mod repo_queue_manager;
use repo_queue_manager::{WorkQueue, JobStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 REPOSITORY WORKER STARTED");
    println!("============================");

    let mut queue = WorkQueue::load_state()?;
    
    while let Some(job) = queue.next_job() {
        println!("🔄 Processing: {} ({})", job.name, job.path);
        
        // Run the fixed complete_indexer on this repository
        let result = Command::new("cargo")
            .args(&["run", "--bin", "complete_indexer"])
            .current_dir("/home/mdupont/zombie_driver2")
            .env("REPO_PATH", &job.path)
            .output();
            
        match result {
            Ok(output) if output.status.success() => {
                println!("✅ Completed: {}", job.name);
                let result_path = format!("output_{}.json", job.name);
                queue.complete_job(job, Some(result_path));
            }
            Ok(output) => {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("❌ Failed: {} - {}", job.name, error);
                queue.fail_job(job, error.to_string());
            }
            Err(e) => {
                println!("❌ Error: {} - {}", job.name, e);
                queue.fail_job(job, e.to_string());
            }
        }
        
        // Save progress
        queue.save_state()?;
        
        let (pending, _, completed, failed) = queue.stats();
        println!("📊 Progress: {:.1}% ({} pending, {} completed, {} failed)", 
                queue.progress_percent(), pending, completed, failed);
    }
    
    println!("🎉 All jobs completed!");
    Ok(())
}
