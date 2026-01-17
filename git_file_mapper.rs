use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

fn find_git_root(file_path: &str) -> Option<String> {
    let mut path = Path::new(file_path);
    for _ in 0..10 {  // Limit depth
        if let Some(parent) = path.parent() {
            if parent.join(".git").exists() {
                return Some(parent.to_str()?.to_string());
            }
            path = parent;
        } else {
            break;
        }
    }
    None
}

fn get_git_info_cached(repo_path: &str, cache: &Arc<Mutex<HashMap<String, (String, String, String)>>>) -> (String, String, String) {
    {
        let c = cache.lock().unwrap();
        if let Some(info) = c.get(repo_path) {
            return info.clone();
        }
    }
    
    let commit = Command::new("git")
        .current_dir(repo_path)
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    let branch = Command::new("git")
        .current_dir(repo_path)
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    let remote = Command::new("git")
        .current_dir(repo_path)
        .args(&["config", "--get", "remote.origin.url"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    let info = (commit, branch, remote);
    cache.lock().unwrap().insert(repo_path.to_string(), info.clone());
    info
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(24).build_global().unwrap();
    
    println!("🔍 Associating files with git objects...\n");
    
    let file_list = "/mnt/data1/newfiles.txt";
    
    println!("📦 Reading {}...", file_list);
    let file = File::open(file_list).expect("Cannot open file");
    let reader = BufReader::new(file);
    
    let files: Vec<String> = reader.lines()
        .flatten()
        .filter(|l| !l.is_empty())
        .collect();
    
    println!("   Found {} files\n", files.len());
    
    // Group by git repo
    println!("📊 Finding git repositories...");
    let repo_cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let git_info_cache: Arc<Mutex<HashMap<String, (String, String, String)>>> = Arc::new(Mutex::new(HashMap::new()));
    let processed = Arc::new(AtomicUsize::new(0));
    let total = files.len();
    
    let results: Vec<_> = files.par_iter().map(|file_path| {
        let git_root = find_git_root(file_path);
        
        let (commit, branch, remote, url, tracked) = if let Some(ref root) = git_root {
            let (c, b, r) = get_git_info_cached(root, &git_info_cache);
            
            // Check if file is tracked
            let is_tracked = Command::new("git")
                .current_dir(root)
                .args(&["ls-files", "--error-unmatch", file_path])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            
            let url = if !r.is_empty() && !c.is_empty() && r.contains("github.com") && is_tracked {
                let repo = r.replace("git@github.com:", "https://github.com/").replace(".git", "");
                let rel_path = file_path.strip_prefix(root).unwrap_or(file_path).trim_start_matches('/');
                format!("{}/blob/{}/{}", repo, b, rel_path)
            } else {
                String::new()
            };
            (c, b, r, url, is_tracked)
        } else {
            (String::new(), String::new(), String::new(), String::new(), false)
        };
        
        let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
        if count % 50000 == 0 {
            println!("   Progress: {}/{} files ({:.1}%)", count, total, count as f64 / total as f64 * 100.0);
        }
        
        (file_path.clone(), git_root.unwrap_or_default(), commit, branch, remote, url, tracked)
    }).collect();
    
    println!("\n   ✓ Processed {} files\n", results.len());
    
    // Save to CSV and Parquet
    println!("💾 Saving to CSV and Parquet...");
    
    // Create output directory
    std::fs::create_dir_all("data/indexes").unwrap();
    
    // Save CSV (for backup)
    let mut csv = File::create("FILE_GIT_MAPPING.csv").unwrap();
    writeln!(csv, "file_path,git_repo,commit,branch,remote,url,tracked").unwrap();
    
    for (file_path, git_repo, commit, branch, remote, url, tracked) in &results {
        writeln!(csv, "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{}",
            file_path, git_repo, commit, branch, remote, url, tracked).unwrap();
    }
    
    println!("   ✓ Saved: FILE_GIT_MAPPING.csv");
    
    // Save to Parquet for HuggingFace
    use arrow::array::{StringArray, BooleanArray};
    use arrow::record_batch::RecordBatch;
    use arrow::datatypes::{DataType, Field, Schema};
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;
    
    let schema = Arc::new(Schema::new(vec![
        Field::new("file_path", DataType::Utf8, false),
        Field::new("git_repo", DataType::Utf8, false),
        Field::new("commit", DataType::Utf8, false),
        Field::new("branch", DataType::Utf8, false),
        Field::new("remote", DataType::Utf8, false),
        Field::new("url", DataType::Utf8, false),
        Field::new("tracked", DataType::Boolean, false),
    ]));
    
    let parquet_file = File::create("data/indexes/files.parquet").unwrap();
    let mut writer = ArrowWriter::try_new(parquet_file, schema.clone(), None).unwrap();
    
    // Write in batches of 100K rows
    for chunk in results.chunks(100_000) {
        let file_paths: Vec<&str> = chunk.iter().map(|(f, _, _, _, _, _, _)| f.as_str()).collect();
        let git_repos: Vec<&str> = chunk.iter().map(|(_, r, _, _, _, _, _)| r.as_str()).collect();
        let commits: Vec<&str> = chunk.iter().map(|(_, _, c, _, _, _, _)| c.as_str()).collect();
        let branches: Vec<&str> = chunk.iter().map(|(_, _, _, b, _, _, _)| b.as_str()).collect();
        let remotes: Vec<&str> = chunk.iter().map(|(_, _, _, _, r, _, _)| r.as_str()).collect();
        let urls: Vec<&str> = chunk.iter().map(|(_, _, _, _, _, u, _)| u.as_str()).collect();
        let tracked_flags: Vec<bool> = chunk.iter().map(|(_, _, _, _, _, _, t)| *t).collect();
        
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(file_paths)),
                Arc::new(StringArray::from(git_repos)),
                Arc::new(StringArray::from(commits)),
                Arc::new(StringArray::from(branches)),
                Arc::new(StringArray::from(remotes)),
                Arc::new(StringArray::from(urls)),
                Arc::new(BooleanArray::from(tracked_flags)),
            ],
        ).unwrap();
        
        writer.write(&batch).unwrap();
    }
    
    writer.close().unwrap();
    println!("   ✓ Saved: data/indexes/files.parquet\n");
    
    // Statistics
    let with_git = results.iter().filter(|(_, r, _, _, _, _, _)| !r.is_empty()).count();
    let with_commit = results.iter().filter(|(_, _, c, _, _, _, _)| !c.is_empty()).count();
    let with_url = results.iter().filter(|(_, _, _, _, _, u, _)| !u.is_empty()).count();
    let tracked = results.iter().filter(|(_, _, _, _, _, _, t)| *t).count();
    let untracked = results.iter().filter(|(_, r, _, _, _, _, t)| !r.is_empty() && !*t).count();
    
    println!("✅ Complete!");
    println!("   📊 Files with git repo: {} ({:.1}%)", with_git, with_git as f64 / total as f64 * 100.0);
    println!("   📊 Files tracked in git: {} ({:.1}%)", tracked, tracked as f64 / total as f64 * 100.0);
    println!("   📊 Files UNTRACKED in git: {} ({:.1}%)", untracked, untracked as f64 / total as f64 * 100.0);
    println!("   📊 Files with commit: {} ({:.1}%)", with_commit, with_commit as f64 / total as f64 * 100.0);
    println!("   📊 Files with URL: {} ({:.1}%)", with_url, with_url as f64 / total as f64 * 100.0);
}
