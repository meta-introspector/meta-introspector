use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use serde_json::json;

#[derive(Debug, Clone)]
struct RepoInfo {
    path: String,
    name: String,
    remote_url: Option<String>,
    is_fork: bool,
    branch: Option<String>,
    status: String,
    last_commit: Option<String>,
    is_local: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 GLOBAL REPOSITORY INDEX BUILDER");
    println!("==================================");
    
    let mut repos = HashMap::new();
    
    // Read our discovered git repositories
    let git_repos_file = "/mnt/data1/meta-introspector/data/raw/git_repos_by_date.txt";
    if let Ok(content) = fs::read_to_string(git_repos_file) {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let repo_path = parts[4..].join(" ");
                if let Some(repo_info) = analyze_repository(&repo_path) {
                    repos.insert(repo_path.clone(), repo_info);
                }
            }
        }
    }
    
    println!("📊 Analyzed {} repositories", repos.len());
    
    // Generate JSON index
    let mut json_repos = Vec::new();
    for (_, repo) in repos.iter() {
        json_repos.push(json!({
            "path": repo.path,
            "name": repo.name,
            "remote_url": repo.remote_url,
            "is_fork": repo.is_fork,
            "is_local": repo.is_local,
            "branch": repo.branch,
            "status": repo.status,
            "last_commit": repo.last_commit
        }));
    }
    
    let index = json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "total_repos": repos.len(),
        "repositories": json_repos
    });
    
    // Save to file
    let output_file = "/mnt/data1/meta-introspector/data/raw/global_repo_index.json";
    fs::write(output_file, serde_json::to_string_pretty(&index)?)?;
    
    println!("✅ Global repository index saved to: {}", output_file);
    
    // Print summary
    let local_count = repos.values().filter(|r| r.is_local).count();
    let fork_count = repos.values().filter(|r| r.is_fork).count();
    let github_count = repos.values().filter(|r| 
        r.remote_url.as_ref().is_some_and(|url| url.contains("github.com"))
    ).count();
    
    println!("\n📈 SUMMARY:");
    println!("  Total repositories: {}", repos.len());
    println!("  Local repositories: {}", local_count);
    println!("  Forks: {}", fork_count);
    println!("  GitHub repositories: {}", github_count);
    
    Ok(())
}

fn analyze_repository(repo_path: &str) -> Option<RepoInfo> {
    let path = Path::new(repo_path);
    if !path.exists() || !path.join(".git").exists() {
        return None;
    }
    
    let name = path.file_name()?.to_string_lossy().to_string();
    
    // Get remote URL
    let remote_url = get_remote_url(repo_path);
    
    // Check if it's a fork
    let is_fork = check_if_fork(repo_path, &remote_url);
    
    // Determine if local (no remote or local remote)
    let is_local = remote_url.as_ref().is_none_or(|url| 
        url.starts_with("file://") || url.starts_with("/") || !url.contains("://")
    );
    
    // Get git status
    let status = get_git_status(repo_path);
    
    // Get current branch
    let branch = get_current_branch(repo_path);
    
    // Get last commit
    let last_commit = get_last_commit(repo_path);
    
    Some(RepoInfo {
        path: repo_path.to_string(),
        name,
        remote_url,
        is_fork,
        branch,
        status,
        last_commit,
        is_local,
    })
}

fn get_remote_url(repo_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", repo_path, "remote", "get-url", "origin"])
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn check_if_fork(repo_path: &str, remote_url: &Option<String>) -> bool {
    // Check if it's a fork by looking for upstream remote
    let output = Command::new("git")
        .args(["-C", repo_path, "remote"])
        .output();
    
    if let Ok(output) = output {
        let remotes = String::from_utf8_lossy(&output.stdout);
        if remotes.contains("upstream") {
            return true;
        }
    }
    
    // Check if remote URL suggests it's a fork (contains your username)
    if let Some(url) = remote_url {
        url.contains("meta-introspector") || url.contains("mdupont")
    } else {
        false
    }
}

fn get_git_status(repo_path: &str) -> String {
    let output = Command::new("git")
        .args(["-C", repo_path, "status", "--porcelain"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let lines = String::from_utf8_lossy(&output.stdout).lines().count();
            if lines == 0 {
                "clean".to_string()
            } else {
                format!("{} modified files", lines)
            }
        }
        _ => "unknown".to_string()
    }
}

fn get_current_branch(repo_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", repo_path, "branch", "--show-current"])
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn get_last_commit(repo_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", repo_path, "log", "-1", "--format=%ci [%h] %s"])
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
