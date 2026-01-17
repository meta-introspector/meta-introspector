use std::collections::VecDeque;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoJob {
    pub path: String,
    pub name: String,
    pub priority: u64,  // timestamp for ordering (higher = more recent)
    pub status: JobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub result_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQueue {
    pub jobs: VecDeque<RepoJob>,
    pub completed: Vec<RepoJob>,
    pub failed: Vec<RepoJob>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WorkQueue {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
            completed: Vec::new(),
            failed: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn load_from_recent_repos() -> Result<Self, Box<dyn std::error::Error>> {
        let mut queue = Self::new();
        
        // Load recent repos file
        let content = fs::read_to_string("/mnt/data1/meta-introspector/data/raw/recent_repos_3months.txt")?;
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let timestamp = parts[0].parse::<u64>().unwrap_or(0);
                let repo_path = parts[4..].join(" ");
                let repo_name = std::path::Path::new(&repo_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let job = RepoJob {
                    path: repo_path,
                    name: repo_name,
                    priority: timestamp,
                    status: JobStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    result_path: None,
                };

                queue.jobs.push_back(job);
            }
        }

        // Sort by priority (most recent first)
        let mut jobs_vec: Vec<_> = queue.jobs.into_iter().collect();
        jobs_vec.sort_by(|a, b| b.priority.cmp(&a.priority));
        queue.jobs = jobs_vec.into();

        println!("📋 Loaded {} repositories into work queue", queue.jobs.len());
        Ok(queue)
    }

    pub fn next_job(&mut self) -> Option<RepoJob> {
        if let Some(mut job) = self.jobs.pop_front() {
            job.status = JobStatus::Processing;
            job.started_at = Some(Utc::now());
            self.updated_at = Utc::now();
            Some(job)
        } else {
            None
        }
    }

    pub fn complete_job(&mut self, mut job: RepoJob, result_path: Option<String>) {
        job.status = JobStatus::Completed;
        job.completed_at = Some(Utc::now());
        job.result_path = result_path;
        self.completed.push(job);
        self.updated_at = Utc::now();
    }

    pub fn fail_job(&mut self, mut job: RepoJob, error: String) {
        job.status = JobStatus::Failed;
        job.completed_at = Some(Utc::now());
        job.error = Some(error);
        self.failed.push(job);
        self.updated_at = Utc::now();
    }

    pub fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("/mnt/data1/meta-introspector/data/raw/queue_status.json", json)?;
        Ok(())
    }

    pub fn load_state() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("/mnt/data1/meta-introspector/data/raw/queue_status.json")?;
        let queue: WorkQueue = serde_json::from_str(&content)?;
        Ok(queue)
    }

    pub fn stats(&self) -> (usize, usize, usize, usize) {
        (
            self.jobs.len(),           // pending
            0,                         // processing (would need separate tracking)
            self.completed.len(),      // completed
            self.failed.len(),         // failed
        )
    }

    pub fn progress_percent(&self) -> f64 {
        let total = self.jobs.len() + self.completed.len() + self.failed.len();
        if total == 0 {
            100.0
        } else {
            ((self.completed.len() + self.failed.len()) as f64 / total as f64) * 100.0
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 REPOSITORY WORK QUEUE MANAGER");
    println!("================================");

    // Try to load existing state, or create new queue
    let queue = WorkQueue::load_state()
        .or_else(|_| WorkQueue::load_from_recent_repos())?;

    let (pending, processing, completed, failed) = queue.stats();
    println!("📊 Queue Status:");
    println!("   Pending: {}", pending);
    println!("   Processing: {}", processing);
    println!("   Completed: {}", completed);
    println!("   Failed: {}", failed);
    println!("   Progress: {:.1}%", queue.progress_percent());

    // Save initial state
    queue.save_state()?;
    println!("💾 Queue state saved to queue_status.json");

    Ok(())
}
