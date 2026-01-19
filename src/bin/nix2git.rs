//! nix2git - Extract git upstreams from Nix packages
//! 
//! Reads Nix derivations, finds git repositories, and enables reproducible builds from source

use std::process::Command;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct NixPackage {
    name: String,
    pname: Option<String>,
    version: Option<String>,
    src_url: Option<String>,
    git_repos: Vec<String>,
    dependencies: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let build_mode = args.contains(&"--build".to_string());
    let recursive = args.contains(&"--recursive".to_string());
    let all_mode = args.contains(&"--all".to_string());
    let depth = args.iter()
        .position(|a| a == "--depth")
        .and_then(|i| args.get(i + 1))
        .and_then(|d| d.parse::<usize>().ok())
        .unwrap_or(1);
    let threads = args.iter()
        .position(|a| a == "-j")
        .and_then(|i| args.get(i + 1))
        .and_then(|d| d.parse::<usize>().ok())
        .unwrap_or(1);
    
    if all_mode {
        println!("🔍 Analyzing ALL packages in /nix/store (using {} threads)", threads);
        return process_all_store_packages(build_mode, recursive, depth, threads);
    }
    
    let package = args.iter()
        .find(|a| !a.starts_with("--") && !a.ends_with("nix2git") && !a.parse::<usize>().is_ok())
        .ok_or("Usage: nix2git [--build] [--recursive] [--depth N] [--all] <package>")?;
    
    println!("🔍 Analyzing Nix package: {}", package);
    if recursive {
        println!("🔄 Recursive mode: depth {}", depth);
    }
    
    // Get derivation info
    let output = Command::new("nix")
        .args(&["derivation", "show", package])
        .output()?;
    
    let drv_json = String::from_utf8_lossy(&output.stdout);
    let pkg = parse_derivation(&drv_json, package)?;
    
    println!("📦 Package: {}", pkg.name);
    if let Some(ref v) = pkg.version {
        println!("   Version: {}", v);
    }
    
    if !pkg.git_repos.is_empty() {
        println!("\n🔗 Git repositories:");
        for repo in &pkg.git_repos {
            println!("  {}", repo);
        }
    }
    
    // Get dependencies based on mode
    let deps = if build_mode {
        println!("\n🔨 Build mode: analyzing build dependencies");
        if recursive {
            get_build_deps_recursive(package, depth)?
        } else {
            get_build_deps(package)?
        }
    } else {
        get_runtime_deps(package)?
    };
    
    let dep_type = if build_mode { "Build" } else { "Runtime" };
    println!("\n📋 {} dependencies: {}", dep_type, deps.len());
    
    let mut all_git_repos: HashSet<String> = pkg.git_repos.iter().cloned().collect();
    
    // Find git repos for dependencies
    for dep in &deps {
        if let Ok(dep_pkg) = analyze_package(dep) {
            all_git_repos.extend(dep_pkg.git_repos);
        }
        
        // Extract git URLs from the dependency path/name itself
        let urls = extract_git_urls(dep);
        all_git_repos.extend(urls);
    }
    
    println!("\n🎯 Total git repositories needed: {}", all_git_repos.len());
    for repo in &all_git_repos {
        println!("  {}", repo);
    }
    
    // Output JSON
    let output = serde_json::json!({
        "package": pkg,
        "runtime_deps": deps,
        "all_git_repos": all_git_repos.iter().collect::<Vec<_>>(),
    });
    
    let output_file = format!("{}_sources.json", package.replace("nixpkgs#", "").replace("/", "_"));
    std::fs::write(&output_file, serde_json::to_string_pretty(&output)?)?;
    println!("\n✅ Wrote: {}", output_file);
    
    Ok(())
}

fn parse_derivation(json: &str, name: &str) -> Result<NixPackage, Box<dyn std::error::Error>> {
    let parsed: serde_json::Value = serde_json::from_str(json)?;
    
    let mut pkg = NixPackage {
        name: name.to_string(),
        pname: None,
        version: None,
        src_url: None,
        git_repos: Vec::new(),
        dependencies: Vec::new(),
    };
    
    // Extract from first derivation
    if let Some(drv) = parsed.as_object().and_then(|o| o.values().next()) {
        if let Some(env) = drv.get("env").and_then(|e| e.as_object()) {
            pkg.pname = env.get("pname").and_then(|v| v.as_str()).map(String::from);
            pkg.version = env.get("version").and_then(|v| v.as_str()).map(String::from);
            
            // Check for git sources
            if let Some(src) = env.get("src").and_then(|v| v.as_str()) {
                if src.contains("git") || src.contains("github") || src.contains("gitlab") {
                    pkg.src_url = Some(src.to_string());
                }
            }
            
            // Extract git URLs from various fields
            for (key, value) in env {
                if let Some(s) = value.as_str() {
                    if (key.contains("url") || key.contains("src") || key.contains("repo")) 
                        && (s.starts_with("https://github.com") 
                            || s.starts_with("https://gitlab.com")
                            || s.starts_with("https://git.")
                            || s.contains(".git")) {
                        pkg.git_repos.push(s.to_string());
                    }
                }
            }
        }
    }
    
    Ok(pkg)
}

fn get_runtime_deps(package: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("nix")
        .args(&["path-info", "-r", package])
        .output()?;
    
    let paths = String::from_utf8_lossy(&output.stdout);
    let deps: Vec<String> = paths
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    
    Ok(deps)
}

fn get_build_deps(package: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Get the derivation file path
    let output = Command::new("nix")
        .args(&["derivation", "show", package])
        .output()?;
    
    let drv_json = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&drv_json)?;
    
    let mut deps = Vec::new();
    
    // New format: derivations -> <hash> -> inputs -> drvs
    if let Some(derivations) = parsed.get("derivations").and_then(|d| d.as_object()) {
        for (_drv_name, drv) in derivations {
            if let Some(inputs) = drv.get("inputs").and_then(|i| i.as_object()) {
                // Get build derivations
                if let Some(drvs) = inputs.get("drvs").and_then(|d| d.as_object()) {
                    for (drv_path, _) in drvs {
                        deps.push(format!("/nix/store/{}.drv", drv_path));
                    }
                }
                
                // Get source inputs
                if let Some(srcs) = inputs.get("srcs").and_then(|s| s.as_array()) {
                    for src in srcs {
                        if let Some(s) = src.as_str() {
                            deps.push(format!("/nix/store/{}", s));
                        }
                    }
                }
            }
        }
    }
    
    Ok(deps)
}

fn get_build_deps_recursive(package: &str, max_depth: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut all_deps = HashSet::new();
    let mut to_process = vec![(package.to_string(), 0)];
    let mut processed = HashSet::new();
    
    while let Some((pkg, depth)) = to_process.pop() {
        if depth >= max_depth || processed.contains(&pkg) {
            continue;
        }
        
        processed.insert(pkg.clone());
        println!("  [depth {}] {}", depth, pkg);
        
        let deps = get_build_deps(&pkg).unwrap_or_default();
        
        for dep in deps {
            all_deps.insert(dep.clone());
            
            // Only recurse on .drv files
            if dep.ends_with(".drv") && depth + 1 < max_depth {
                to_process.push((dep, depth + 1));
            }
        }
    }
    
    Ok(all_deps.into_iter().collect())
}

fn analyze_package(path: &str) -> Result<NixPackage, Box<dyn std::error::Error>> {
    let output = Command::new("nix")
        .args(&["derivation", "show", path])
        .output()?;
    
    let drv_json = String::from_utf8_lossy(&output.stdout);
    parse_derivation(&drv_json, path)
}

fn extract_git_urls(text: &str) -> Vec<String> {
    let url_pattern = Regex::new(
        r#"(?:https?://|git@|git://|ssh://git@)(?:github\.com|gitlab\.com|git\.savannah\.gnu\.org|git\.kernel\.org|salsa\.debian\.org|git\.launchpad\.net|git\.hadrons\.org)[:/][^\s"'<>)}\]]+\.git|(?:https?://|git@|git://|ssh://git@)(?:github\.com|gitlab\.com|git\.savannah\.gnu\.org|git\.kernel\.org|salsa\.debian\.org|git\.launchpad\.net|git\.hadrons\.org)[:/][^\s"'<>)}\]]+"#
    ).unwrap();
    
    url_pattern
        .captures_iter(text)
        .filter_map(|cap| cap.get(0).map(|m| m.as_str().trim_end_matches(&['"', '\'', ')', '}', ']'][..]).to_string()))
        .collect()
}

fn process_all_store_packages(build_mode: bool, recursive: bool, depth: usize, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use crossbeam::thread;
    
    let store_path = Path::new("/nix/store");
    let all_git_repos = Arc::new(Mutex::new(HashSet::new()));
    let processed = Arc::new(Mutex::new(0usize));
    
    println!("📂 Scanning /nix/store for .drv files...");
    
    // Collect all .drv files
    let mut drv_files = Vec::new();
    for entry in fs::read_dir(store_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("drv") {
            drv_files.push(path);
        }
    }
    
    let total = drv_files.len();
    println!("📦 Found {} derivations to process", total);
    
    // Process in parallel with crossbeam
    let drv_files = Arc::new(drv_files);
    
    thread::scope(|s| {
        let mut handles = vec![];
        
        for thread_id in 0..threads {
            let drv_files = Arc::clone(&drv_files);
            let all_git_repos = Arc::clone(&all_git_repos);
            let processed = Arc::clone(&processed);
            
            let handle = s.spawn(move |_| {
                for (idx, path) in drv_files.iter().enumerate() {
                    if idx % threads != thread_id {
                        continue;
                    }
                    
                    let path_str = path.to_str().unwrap();
                    
                    // Get dependencies
                    let deps = if build_mode {
                        if recursive {
                            get_build_deps_recursive(path_str, depth).unwrap_or_default()
                        } else {
                            get_build_deps(path_str).unwrap_or_default()
                        }
                    } else {
                        get_runtime_deps(path_str).unwrap_or_default()
                    };
                    
                    // Extract git URLs
                    let mut local_urls = HashSet::new();
                    
                    if let Ok(content) = fs::read_to_string(&path) {
                        let urls = extract_git_urls(&content);
                        local_urls.extend(urls);
                    }
                    
                    for dep in &deps {
                        let urls = extract_git_urls(dep);
                        local_urls.extend(urls);
                    }
                    
                    // Update shared state
                    if !local_urls.is_empty() {
                        let mut repos = all_git_repos.lock().unwrap();
                        repos.extend(local_urls);
                    }
                    
                    let mut p = processed.lock().unwrap();
                    *p += 1;
                    
                    if *p % 100 == 0 {
                        let repo_count = all_git_repos.lock().unwrap().len();
                        println!("  [Thread {}] Processed: {}/{} packages, found {} git repos", 
                                 thread_id, *p, total, repo_count);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }).unwrap();
    
    let final_processed = *processed.lock().unwrap();
    let final_repos = all_git_repos.lock().unwrap();
    
    println!("\n✅ Processed {} derivations", final_processed);
    println!("🎯 Total git repositories needed: {}", final_repos.len());
    
    // Save to file
    let output = serde_json::json!({
        "total_derivations": final_processed,
        "git_repos": final_repos.iter().collect::<Vec<_>>(),
    });
    
    let output_file = "nix_store_all_sources.json";
    fs::write(output_file, serde_json::to_string_pretty(&output)?)?;
    println!("📝 Wrote: {}", output_file);
    
    // Also save as plain text list
    let mut repos: Vec<_> = final_repos.iter().cloned().collect();
    repos.sort();
    fs::write("nix_store_git_repos.txt", repos.join("\n"))?;
    println!("📝 Wrote: nix_store_git_repos.txt");
    
    Ok(())
}
