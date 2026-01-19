// Project Hierarchy Model: projects -> git -> forks -> branches -> commits -> uncommitted/unpushed
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectHierarchy {
    projects: HashMap<String, Project>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    abs_path: PathBuf,
    git_repos: Vec<GitRepo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitRepo {
    path: PathBuf,
    is_fork: bool,
    upstream: Option<String>,
    branches: Vec<Branch>,
    uncommitted_files: usize,
    unpushed_commits: HashMap<String, usize>, // branch -> count
}

#[derive(Debug, Serialize, Deserialize)]
struct Branch {
    name: String,
    current: bool,
    commits_ahead: usize,
    commits_behind: usize,
    last_commit: String,
    last_commit_date: String,
}

fn get_git_info(repo_path: &Path) -> Option<GitRepo> {
    if !repo_path.join(".git").exists() {
        return None;
    }
    
    // Get branches
    let branches_output = Command::new("git")
        .current_dir(repo_path)
        .args(["branch", "-vv"])
        .output()
        .ok()?;
    
    let mut branches = Vec::new();
    let mut current_branch = String::new();
    
    for line in String::from_utf8_lossy(&branches_output.stdout).lines() {
        let is_current = line.starts_with('*');
        let parts: Vec<&str> = line.trim_start_matches('*').trim().split_whitespace().collect();
        
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let commit = parts[1].to_string();
            
            if is_current {
                current_branch = name.clone();
            }
            
            branches.push(Branch {
                name: name.clone(),
                current: is_current,
                commits_ahead: parse_commits_ahead(&output),
                commits_behind: 0,
                last_commit: commit,
                last_commit_date: String::new(),
            });
        }
    }
    
    // Check for uncommitted files
    let status_output = Command::new("git")
        .current_dir(repo_path)
        .args(["status", "--porcelain"])
        .output()
        .ok()?;
    
    let uncommitted_files = String::from_utf8_lossy(&status_output.stdout)
        .lines()
        .count();
    
    // Check if fork
    let remote_output = Command::new("git")
        .current_dir(repo_path)
        .args(["remote", "-v"])
        .output()
        .ok()?;
    
    let remotes = String::from_utf8_lossy(&remote_output.stdout);
    let is_fork = remotes.contains("upstream");
    let upstream = if is_fork {
        remotes.lines()
            .find(|l| l.starts_with("upstream"))
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.to_string())
    } else {
        None
    };
    
    // Check unpushed commits per branch
    let mut unpushed_commits = HashMap::new();
    for branch in &branches {
        if let Ok(output) = Command::new("git")
            .current_dir(repo_path)
            .args(["rev-list", &format!("origin/{}..{}", branch.name, branch.name)])
            .output()
        {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                unpushed_commits.insert(branch.name.clone(), count);
            }
        }
    }
    
    Some(GitRepo {
        path: repo_path.to_path_buf(),
        is_fork,
        upstream,
        branches,
        uncommitted_files,
        unpushed_commits,
    })
}

fn scan_projects(base_path: &Path) -> ProjectHierarchy {
    let mut projects = HashMap::new();
    
    // Scan for git repositories
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                
                // Check if it's a git repo or contains git repos
                let mut git_repos = Vec::new();
                
                if path.join(".git").exists() {
                    if let Some(repo) = get_git_info(&path) {
                        git_repos.push(repo);
                    }
                }
                
                if !git_repos.is_empty() {
                    projects.insert(name.clone(), Project {
                        name,
                        abs_path: path.canonicalize().unwrap_or(path),
                        git_repos,
                    });
                }
            }
        }
    }
    
    ProjectHierarchy { projects }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 PROJECT HIERARCHY ANALYZER\n");
    println!("Scanning /mnt/data1 for projects...\n");
    
    let hierarchy = scan_projects(Path::new("/mnt/data1"));
    
    println!("Found {} projects\n", hierarchy.projects.len());
    
    // Generate report
    let mut report = String::from("# Project Hierarchy Report\n\n");
    report.push_str(&format!("**Projects found**: {}\n\n", hierarchy.projects.len()));
    
    for (name, project) in hierarchy.projects.iter() {
        report.push_str(&format!("## {} ({})\n\n", name, project.abs_path.display()));
        
        for repo in &project.git_repos {
            report.push_str(&format!("### Git Repository\n\n"));
            report.push_str(&format!("- **Path**: `{}`\n", repo.path.display()));
            report.push_str(&format!("- **Is Fork**: {}\n", repo.is_fork));
            
            if let Some(upstream) = &repo.upstream {
                report.push_str(&format!("- **Upstream**: {}\n", upstream));
            }
            
            report.push_str(&format!("- **Uncommitted files**: {}\n", repo.uncommitted_files));
            
            if !repo.unpushed_commits.is_empty() {
                report.push_str("- **Unpushed commits**:\n");
                for (branch, count) in &repo.unpushed_commits {
                    report.push_str(&format!("  - `{}`: {} commits\n", branch, count));
                }
            }
            
            report.push_str("\n**Branches**:\n\n");
            report.push_str("| Branch | Current | Last Commit |\n");
            report.push_str("|--------|---------|-------------|\n");
            
            for branch in &repo.branches {
                let current = if branch.current { "✓" } else { "" };
                report.push_str(&format!("| {} | {} | {} |\n", 
                    branch.name, current, branch.last_commit));
            }
            
            report.push_str("\n");
        }
        
        report.push_str("---\n\n");
    }
    
    // Save JSON
    let json = serde_json::to_string_pretty(&hierarchy)?;
    fs::write("project_hierarchy.json", json)?;
    
    // Save report
    fs::write("PROJECT_HIERARCHY_REPORT.md", report)?;
    
    println!("✅ Saved project_hierarchy.json");
    println!("✅ Saved PROJECT_HIERARCHY_REPORT.md");
    
    Ok(())
}


fn parse_commits_ahead(output: &str) -> usize {
    output.lines()
        .find(|l| l.contains("ahead"))
        .and_then(|l| l.split("ahead").nth(1))
        .and_then(|s| s.trim().split_whitespace().next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}
