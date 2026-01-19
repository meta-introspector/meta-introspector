//! apt2git - Extract git upstreams from apt source packages
//! 
//! Reads apt sources, finds git repositories, and enables reproducible builds from source

use std::process::Command;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    source_url: Option<String>,
    git_upstream: Option<String>,
    vcs_git: Option<String>,
    vcs_browser: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let all_mode = args.contains(&"--all".to_string());
    
    if all_mode {
        println!("🔍 Analyzing ALL apt packages");
        return process_all_apt_packages();
    }
    
    let package = args.get(1).map(|s| s.as_str()).unwrap_or("plocate");
    
    println!("🔍 Analyzing package: {}", package);
    
    // Get source package info
    let output = Command::new("apt-cache")
        .args(&["showsrc", package])
        .output()?;
    
    let info = String::from_utf8_lossy(&output.stdout);
    let pkg = parse_source_info(&info, package)?;
    
    println!("📦 Package: {} {}", pkg.name, pkg.version);
    
    if let Some(vcs) = &pkg.vcs_git {
        println!("🔗 VCS-Git: {}", vcs);
    }
    
    if let Some(browser) = &pkg.vcs_browser {
        println!("🌐 VCS-Browser: {}", browser);
    }
    
    // Get build dependencies
    let deps = get_build_deps(package)?;
    println!("\n📋 Build dependencies: {}", deps.len());
    
    let mut all_git_repos = HashSet::new();
    if let Some(ref git) = pkg.vcs_git {
        all_git_repos.insert(git.clone());
    }
    
    // Find git repos for all dependencies
    for dep in &deps {
        if let Ok(dep_pkg) = get_git_upstream(dep) {
            if let Some(ref git) = dep_pkg.vcs_git {
                all_git_repos.insert(git.clone());
            }
        }
    }
    
    println!("\n🎯 Total git repositories needed: {}", all_git_repos.len());
    for repo in &all_git_repos {
        println!("  {}", repo);
    }
    
    // Output JSON
    let output = serde_json::json!({
        "package": pkg,
        "build_deps": deps,
        "git_repos": all_git_repos.iter().collect::<Vec<_>>(),
    });
    
    let output_file = format!("{}_sources.json", package);
    std::fs::write(&output_file, serde_json::to_string_pretty(&output)?)?;
    println!("\n✅ Wrote: {}", output_file);
    
    Ok(())
}

fn parse_source_info(info: &str, name: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let mut pkg = Package {
        name: name.to_string(),
        version: String::new(),
        source_url: None,
        git_upstream: None,
        vcs_git: None,
        vcs_browser: None,
    };
    
    for line in info.lines() {
        if line.starts_with("Version:") {
            pkg.version = line.split_whitespace().nth(1).unwrap_or("").to_string();
        } else if line.starts_with("Vcs-Git:") {
            pkg.vcs_git = Some(line.split_whitespace().skip(1).collect::<Vec<_>>().join(" "));
        } else if line.starts_with("Vcs-Browser:") {
            pkg.vcs_browser = Some(line.split_whitespace().nth(1).unwrap_or("").to_string());
        }
    }
    
    Ok(pkg)
}

fn get_build_deps(package: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("apt-cache")
        .args(&["showsrc", package])
        .output()?;
    
    let info = String::from_utf8_lossy(&output.stdout);
    let mut deps = HashSet::new();
    
    for line in info.lines() {
        if line.starts_with("Build-Depends:") {
            let dep_line = line.strip_prefix("Build-Depends:").unwrap_or("");
            for dep in dep_line.split(',') {
                let name = dep.trim().split_whitespace().next().unwrap_or("");
                if !name.is_empty() {
                    deps.insert(name.to_string());
                }
            }
        }
    }
    
    Ok(deps.into_iter().collect())
}

fn get_git_upstream(package: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let output = Command::new("apt-cache")
        .args(&["showsrc", package])
        .output()?;
    
    let info = String::from_utf8_lossy(&output.stdout);
    parse_source_info(&info, package)
}

fn process_all_apt_packages() -> Result<(), Box<dyn std::error::Error>> {
    println!("📂 Getting list of all installed packages...");
    
    // Get all installed packages
    let output = Command::new("dpkg-query")
        .args(&["-W", "-f=${Package}\\n"])
        .output()?;
    
    let packages = String::from_utf8_lossy(&output.stdout);
    let package_list: Vec<&str> = packages.lines().collect();
    
    println!("📦 Found {} installed packages", package_list.len());
    
    let mut all_git_repos = HashSet::new();
    let mut processed = 0;
    
    for (idx, package) in package_list.iter().enumerate() {
        if idx % 100 == 0 {
            println!("  Processed: {}/{} packages, found {} git repos", 
                     idx, package_list.len(), all_git_repos.len());
        }
        
        // Get source package info
        if let Ok(pkg) = get_git_upstream(package) {
            if let Some(git) = pkg.vcs_git {
                all_git_repos.insert(git);
            }
        }
        
        // Get build deps
        if let Ok(deps) = get_build_deps(package) {
            for dep in deps {
                if let Ok(dep_pkg) = get_git_upstream(&dep) {
                    if let Some(git) = dep_pkg.vcs_git {
                        all_git_repos.insert(git);
                    }
                }
            }
        }
        
        processed += 1;
    }
    
    println!("\n✅ Processed {} packages", processed);
    println!("🎯 Total git repositories needed: {}", all_git_repos.len());
    
    // Save to file
    let output = serde_json::json!({
        "total_packages": processed,
        "git_repos": all_git_repos.iter().collect::<Vec<_>>(),
    });
    
    let output_file = "apt_all_sources.json";
    std::fs::write(output_file, serde_json::to_string_pretty(&output)?)?;
    println!("📝 Wrote: {}", output_file);
    
    // Also save as plain text list
    let mut repos: Vec<_> = all_git_repos.into_iter().collect();
    repos.sort();
    std::fs::write("apt_git_repos.txt", repos.join("\n"))?;
    println!("📝 Wrote: apt_git_repos.txt");
    
    Ok(())
}
