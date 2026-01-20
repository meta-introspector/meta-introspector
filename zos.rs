use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
struct Repo {
    name: String,
    path: String,
    priority: u32,
    dependencies: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "cargo" if args.len() > 2 && args[2] == "audit" => {
            cargo_audit_all();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("ZOS - Zero Operating System");
    println!("\nUsage:");
    println!("  zos cargo audit    - Audit all repos in topological order");
}

fn cargo_audit_all() {
    println!("🌐 ZOS Cargo Audit - Scheduling all repos");
    
    // Load repos from master_url_list.txt and submodules
    let repos = discover_repos();
    println!("📊 Discovered {} repos", repos.len());
    
    // Build dependency graph
    let graph = build_dependency_graph(&repos);
    
    // Topological sort with priority
    let order = topological_sort_with_priority(&graph);
    println!("📋 Audit order: {} repos", order.len());
    
    // Execute audits
    let mut results = HashMap::new();
    for (idx, repo_name) in order.iter().enumerate() {
        println!("\n[{}/{}] 🔍 Auditing: {}", idx + 1, order.len(), repo_name);
        
        if let Some(repo) = repos.iter().find(|r| r.name == *repo_name) {
            let result = audit_repo(&repo);
            results.insert(repo_name.clone(), result);
        }
    }
    
    // Generate summary
    generate_audit_summary(&results);
}

fn discover_repos() -> Vec<Repo> {
    let mut repos = Vec::new();
    
    // Current repo
    repos.push(Repo {
        name: "meta-introspector".to_string(),
        path: ".".to_string(),
        priority: 100,
        dependencies: vec![],
    });
    
    // Submodules
    if let Ok(output) = Command::new("git").args(&["submodule", "status"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let path = parts[1];
                repos.push(Repo {
                    name: Path::new(path).file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.to_string(),
                    priority: 50,
                    dependencies: vec![],
                });
            }
        }
    }
    
    // From master_url_list.txt
    if let Ok(content) = fs::read_to_string("data/master_url_list.txt") {
        for (idx, url) in content.lines().enumerate() {
            if let Some(name) = url.split('/').last() {
                let name = name.trim_end_matches(".git");
                repos.push(Repo {
                    name: name.to_string(),
                    path: format!("/mnt/data1/git/{}", name),
                    priority: (idx / 100) as u32,
                    dependencies: vec![],
                });
            }
        }
    }
    
    repos
}

fn build_dependency_graph(repos: &[Repo]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    
    for repo in repos {
        let deps = extract_dependencies(&repo.path);
        graph.insert(repo.name.clone(), deps);
    }
    
    graph
}

fn extract_dependencies(repo_path: &str) -> Vec<String> {
    let mut deps = Vec::new();
    
    // Check Cargo.toml for dependencies
    let cargo_path = format!("{}/Cargo.toml", repo_path);
    if let Ok(content) = fs::read_to_string(&cargo_path) {
        let mut in_deps = false;
        for line in content.lines() {
            if line.starts_with("[dependencies]") {
                in_deps = true;
            } else if line.starts_with('[') {
                in_deps = false;
            } else if in_deps && !line.trim().is_empty() {
                if let Some(dep) = line.split('=').next() {
                    deps.push(dep.trim().to_string());
                }
            }
        }
    }
    
    deps
}

fn topological_sort_with_priority(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build in-degree map
    for (node, deps) in graph {
        in_degree.entry(node.clone()).or_insert(0);
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
            adj_list.entry(node.clone()).or_insert_vec![]).push(dep.clone());
        }
    }
    
    // Priority queue (higher priority first)
    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(node, _)| node.clone())
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        
        if let Some(neighbors) = adj_list.get(&node) {
            for neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }
    
    result
}

fn audit_repo(repo: &Repo) -> AuditResult {
    let cargo_toml = format!("{}/Cargo.toml", repo.path);
    
    if !Path::new(&cargo_toml).exists() {
        return AuditResult::NoCargo;
    }
    
    // Run cargo audit tool
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&repo.path)
        .output();
    
    match output {
        Ok(out) if out.status.success() => AuditResult::Success,
        Ok(out) => AuditResult::Failed(String::from_utf8_lossy(&out.stderr).to_string()),
        Err(e) => AuditResult::Error(e.to_string()),
    }
}

#[derive(Debug)]
enum AuditResult {
    Success,
    Failed(String),
    Error(String),
    NoCargo,
}

fn generate_audit_summary(results: &HashMap<String, AuditResult>) {
    let mut summary = String::from("# ZOS Cargo Audit Summary\n\n");
    
    let success = results.values().filter(|r| matches!(r, AuditResult::Success)).count();
    let failed = results.values().filter(|r| matches!(r, AuditResult::Failed(_))).count();
    
    summary.push_str(&format!("✅ Success: {}\n", success));
    summary.push_str(&format!("❌ Failed: {}\n", failed));
    summary.push_str("\n## Failed Repos\n\n");
    
    for (name, result) in results {
        if let AuditResult::Failed(err) = result {
            summary.push_str(&format!("### {}\n```\n{}\n```\n\n", name, err));
        }
    }
    
    fs::write("ZOS_AUDIT_SUMMARY.md", summary).unwrap();
    println!("\n📋 Summary: ZOS_AUDIT_SUMMARY.md");
}
