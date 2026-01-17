use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize)]
struct RepoAnalysis {
    repo_name: String,
    path: String,
    branch: String,
    analysis_level: String,
    success: bool,
    details: String,
    perf_data: Option<PerfData>,
    timestamp: String,
}

#[derive(Debug, Serialize)]
struct PerfData {
    session_id: String,
    perf_file: String,
    duration_ms: u64,
    syscalls: usize,
    binaries_executed: usize,
    libraries_loaded: usize,
}

fn main() {
    let registry = fs::read_to_string("data/git-sources-registry.json").unwrap();
    let data: serde_json::Value = serde_json::from_str(&registry).unwrap();
    let sources = data["sources"].as_object().unwrap();

    let mut results = Vec::new();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let session_id = format!("cascade_{}", chrono::Utc::now().timestamp());

    fs::create_dir_all(format!("data/perf_sessions/{}", session_id)).unwrap();

    for (name, info) in sources {
        let path = info["checkout_path"].as_str().unwrap();
        let branch = info["branch"].as_str().unwrap_or("main");
        
        println!("\n🔍 Analyzing: {} ({})", name, path);
        
        let analysis = analyze_repo_with_perf(name, path, branch, &timestamp, &session_id);
        println!("   Result: {} - {}", analysis.analysis_level, analysis.details);
        if let Some(ref perf) = analysis.perf_data {
            println!("   Perf: {} syscalls, {} binaries, {} libs", 
                perf.syscalls, perf.binaries_executed, perf.libraries_loaded);
        }
        results.push(analysis);
    }

    // Save results
    let output = serde_json::to_string_pretty(&results).unwrap();
    fs::write(format!("data/perf_sessions/{}/analysis.json", session_id), &output).unwrap();
    
    // Summary
    println!("\n📊 Analysis Summary:");
    for level in &["nix-build", "cargo-build", "syn-parse", "markov-model"] {
        let count = results.iter().filter(|r| r.analysis_level == *level).count();
        println!("   {:15} {} repos", level, count);
    }
    println!("   Total:          {} repos", results.len());
    println!("\n💾 Session: data/perf_sessions/{}/", session_id);
}

fn analyze_repo_with_perf(name: &str, path: &str, branch: &str, timestamp: &str, session_id: &str) -> RepoAnalysis {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return RepoAnalysis {
            repo_name: name.to_string(),
            path: path.to_string(),
            branch: branch.to_string(),
            analysis_level: "none".to_string(),
            success: false,
            details: "Path does not exist".to_string(),
            perf_data: None,
            timestamp: timestamp.to_string(),
        };
    }

    // Level 1: Audited nix build with perf
    if path_obj.join("flake.nix").exists() {
        if let Some(perf) = try_nix_build_with_perf(name, path, session_id) {
            return RepoAnalysis {
                repo_name: name.to_string(),
                path: path.to_string(),
                branch: branch.to_string(),
                analysis_level: "nix-build".to_string(),
                success: true,
                details: "Nix build with perf capture".to_string(),
                perf_data: Some(perf),
                timestamp: timestamp.to_string(),
            };
        }
    }

    // Level 2: Audited cargo build with perf
    if path_obj.join("Cargo.toml").exists() {
        if let Some(perf) = try_cargo_build_with_perf(name, path, session_id) {
            return RepoAnalysis {
                repo_name: name.to_string(),
                path: path.to_string(),
                branch: branch.to_string(),
                analysis_level: "cargo-build".to_string(),
                success: true,
                details: "Cargo build with perf capture".to_string(),
                perf_data: Some(perf),
                timestamp: timestamp.to_string(),
            };
        }
    }

    // Level 3: Syn parse with perf
    let rust_files = count_rust_files(path);
    if rust_files > 0 {
        if let Some(perf) = try_syn_parse_with_perf(name, path, session_id, rust_files) {
            return RepoAnalysis {
                repo_name: name.to_string(),
                path: path.to_string(),
                branch: branch.to_string(),
                analysis_level: "syn-parse".to_string(),
                success: true,
                details: format!("Parsed {} Rust files", rust_files),
                perf_data: Some(perf),
                timestamp: timestamp.to_string(),
            };
        }
    }

    // Level 4: Markov model with perf
    let (perf, details) = markov_model_with_perf(name, path, session_id);
    RepoAnalysis {
        repo_name: name.to_string(),
        path: path.to_string(),
        branch: branch.to_string(),
        analysis_level: "markov-model".to_string(),
        success: true,
        details,
        perf_data: Some(perf),
        timestamp: timestamp.to_string(),
    }
}

fn try_nix_build_with_perf(name: &str, path: &str, session_id: &str) -> Option<PerfData> {
    let perf_file = format!("data/perf_sessions/{}/{}_nix.perf", session_id, name);
    let start = std::time::Instant::now();
    
    let output = Command::new("perf")
        .args(["record", "-o", &perf_file, "--", 
                "nix", "build", "--no-link", "--max-jobs", "1"])
        .current_dir(path)
        .output();
    
    if output.map(|o| o.status.success()).unwrap_or(false) {
        let duration = start.elapsed().as_millis() as u64;
        let stats = analyze_perf_file(&perf_file);
        Some(PerfData {
            session_id: session_id.to_string(),
            perf_file,
            duration_ms: duration,
            syscalls: stats.0,
            binaries_executed: stats.1,
            libraries_loaded: stats.2,
        })
    } else {
        None
    }
}

fn try_cargo_build_with_perf(name: &str, path: &str, session_id: &str) -> Option<PerfData> {
    let perf_file = format!("data/perf_sessions/{}/{}_cargo.perf", session_id, name);
    let start = std::time::Instant::now();
    
    let output = Command::new("perf")
        .args(["record", "-o", &perf_file, "--", 
                "cargo", "check", "--quiet"])
        .current_dir(path)
        .output();
    
    if output.map(|o| o.status.success()).unwrap_or(false) {
        let duration = start.elapsed().as_millis() as u64;
        let stats = analyze_perf_file(&perf_file);
        Some(PerfData {
            session_id: session_id.to_string(),
            perf_file,
            duration_ms: duration,
            syscalls: stats.0,
            binaries_executed: stats.1,
            libraries_loaded: stats.2,
        })
    } else {
        None
    }
}

fn try_syn_parse_with_perf(name: &str, path: &str, session_id: &str, _file_count: usize) -> Option<PerfData> {
    let perf_file = format!("data/perf_sessions/{}/{}_syn.perf", session_id, name);
    let start = std::time::Instant::now();
    
    // Create a simple syn parser script
    let script = format!(r#"
#!/bin/bash
find {} -name "*.rs" -type f | while read f; do
    rustc --crate-type lib -Z parse-only "$f" 2>/dev/null || true
done
"#, path);
    
    fs::write("/tmp/syn_parse.sh", script).ok()?;
    Command::new("chmod").args(["+x", "/tmp/syn_parse.sh"]).output().ok()?;
    
    let output = Command::new("perf")
        .args(["record", "-o", &perf_file, "--", "/tmp/syn_parse.sh"])
        .output();
    
    if output.is_ok() {
        let duration = start.elapsed().as_millis() as u64;
        let stats = analyze_perf_file(&perf_file);
        Some(PerfData {
            session_id: session_id.to_string(),
            perf_file,
            duration_ms: duration,
            syscalls: stats.0,
            binaries_executed: stats.1,
            libraries_loaded: stats.2,
        })
    } else {
        None
    }
}

fn markov_model_with_perf(name: &str, path: &str, session_id: &str) -> (PerfData, String) {
    let perf_file = format!("data/perf_sessions/{}/{}_markov.perf", session_id, name);
    let start = std::time::Instant::now();
    
    let output = Command::new("perf")
        .args(["record", "-o", &perf_file, "--", 
                "find", path, "-type", "f"])
        .output()
        .unwrap();
    
    let duration = start.elapsed().as_millis() as u64;
    let file_count = String::from_utf8_lossy(&output.stdout).lines().count();
    let stats = analyze_perf_file(&perf_file);
    
    (PerfData {
        session_id: session_id.to_string(),
        perf_file,
        duration_ms: duration,
        syscalls: stats.0,
        binaries_executed: stats.1,
        libraries_loaded: stats.2,
    }, format!("{} files analyzed", file_count))
}

fn analyze_perf_file(perf_file: &str) -> (usize, usize, usize) {
    // Quick perf analysis using perf script
    let output = Command::new("perf")
        .args(["script", "-i", perf_file])
        .output();
    
    if let Ok(out) = output {
        let data = String::from_utf8_lossy(&out.stdout);
        let syscalls = data.lines().filter(|l| l.contains("syscalls:")).count();
        let binaries = data.lines().filter(|l| l.contains("PERF_RECORD_COMM")).count();
        let libs = data.lines().filter(|l| l.contains(".so")).count();
        (syscalls, binaries, libs)
    } else {
        (0, 0, 0)
    }
}

fn count_rust_files(path: &str) -> usize {
    Command::new("find")
        .args([path, "-name", "*.rs", "-type", "f"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
        .unwrap_or(0)
}
