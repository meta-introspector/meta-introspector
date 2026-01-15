use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct DatasetIndex {
    huggingface_datasets: Vec<HFDataset>,
    local_datasets: Vec<LocalDataset>,
    untracked_datasets: Vec<UntrackedDataset>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HFDataset {
    name: String,
    org: String,
    url: String,
    purpose: String,
    size: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocalDataset {
    name: String,
    path: String,
    size_mb: u64,
    file_count: usize,
    purpose: String,
    hf_candidate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct UntrackedDataset {
    name: String,
    path: String,
    size_mb: u64,
    description: String,
    recommended_action: String,
}

fn main() {
    println!("🔍 Discovering datasets...\n");

    // Fetch HuggingFace datasets
    let hf_datasets = fetch_hf_datasets();
    
    // Scan local data directories
    let local_datasets = scan_local_datasets();
    
    // Find untracked datasets
    let untracked = find_untracked_datasets();

    let index = DatasetIndex {
        huggingface_datasets: hf_datasets,
        local_datasets,
        untracked_datasets: untracked,
    };

    // Save index
    let json = serde_json::to_string_pretty(&index).unwrap();
    fs::write("data/dataset-index.json", &json).unwrap();

    // Print summary
    println!("\n📊 Dataset Index Summary:");
    println!("   HuggingFace: {} datasets", index.huggingface_datasets.len());
    println!("   Local:       {} datasets", index.local_datasets.len());
    println!("   Untracked:   {} datasets", index.untracked_datasets.len());
    println!("\n✅ Index saved to: data/dataset-index.json");
}

fn fetch_hf_datasets() -> Vec<HFDataset> {
    let mut datasets = Vec::new();

    // introspector org datasets
    let introspector_datasets = vec![
        ("git-activity", "Git activity from 53 repos, 402K commits", "564 MB", "published"),
    ];

    for (name, purpose, size, status) in introspector_datasets {
        datasets.push(HFDataset {
            name: name.to_string(),
            org: "introspector".to_string(),
            url: format!("https://huggingface.co/datasets/introspector/{}", name),
            purpose: purpose.to_string(),
            size: size.to_string(),
            status: status.to_string(),
        });
    }

    // h4 org datasets (check what exists)
    let h4_check = Command::new("gh")
        .args(&["api", "/orgs/h4/repos", "--jq", ".[].name"])
        .output();
    
    if let Ok(output) = h4_check {
        let repos = String::from_utf8_lossy(&output.stdout);
        for repo in repos.lines().take(5) {
            datasets.push(HFDataset {
                name: repo.to_string(),
                org: "h4".to_string(),
                url: format!("https://huggingface.co/h4/{}", repo),
                purpose: "To be analyzed".to_string(),
                size: "Unknown".to_string(),
                status: "discovered".to_string(),
            });
        }
    }

    datasets
}

fn scan_local_datasets() -> Vec<LocalDataset> {
    let mut datasets = Vec::new();

    let data_dirs = vec![
        ("data/activity", "Git activity organized by platform/user/year/month", true),
        ("data/perf_sessions", "Perf capture sessions from builds", true),
        ("data/71_flakes_perf", "71 flakes performance analysis", true),
        ("data/build_analysis", "Real build analysis with strace", true),
        ("data/telemetry", "Build telemetry data", true),
    ];

    for (path, purpose, hf_candidate) in data_dirs {
        if let Ok(metadata) = get_dir_stats(path) {
            datasets.push(LocalDataset {
                name: path.split('/').last().unwrap().to_string(),
                path: path.to_string(),
                size_mb: metadata.0,
                file_count: metadata.1,
                purpose: purpose.to_string(),
                hf_candidate,
            });
        }
    }

    datasets
}

fn find_untracked_datasets() -> Vec<UntrackedDataset> {
    let mut untracked = Vec::new();

    // Check for data-* directories
    let output = Command::new("find")
        .args(&["/mnt/data1/meta-introspector", "-maxdepth", "1", "-type", "d", "-name", "data-*"])
        .output();

    if let Ok(out) = output {
        let dirs = String::from_utf8_lossy(&out.stdout);
        for dir in dirs.lines() {
            let name = dir.split('/').last().unwrap();
            if let Ok(metadata) = get_dir_stats(dir) {
                let recommendation = if metadata.0 > 100 {
                    format!("Create HF dataset: introspector/{}", name)
                } else if metadata.0 > 10 {
                    format!("Add to git-activity dataset as subdirectory")
                } else {
                    "Keep local or add to git repo".to_string()
                };

                untracked.push(UntrackedDataset {
                    name: name.to_string(),
                    path: dir.to_string(),
                    size_mb: metadata.0,
                    description: format!("{} files", metadata.1),
                    recommended_action: recommendation,
                });
            }
        }
    }

    // Check for other large data directories
    let other_dirs = vec![
        "/mnt/data1/nix/time",
        "/mnt/data1/downloads",
    ];

    for dir in other_dirs {
        if let Ok(metadata) = get_dir_stats(dir) {
            if metadata.0 > 1000 {
                untracked.push(UntrackedDataset {
                    name: dir.split('/').last().unwrap().to_string(),
                    path: dir.to_string(),
                    size_mb: metadata.0,
                    description: format!("{} files", metadata.1),
                    recommended_action: "Analyze and categorize for potential HF upload".to_string(),
                });
            }
        }
    }

    untracked
}

fn get_dir_stats(path: &str) -> Result<(u64, usize), std::io::Error> {
    let size_output = Command::new("du")
        .args(&["-sm", path])
        .output()?;
    
    let size_str = String::from_utf8_lossy(&size_output.stdout);
    let size_mb = size_str.split_whitespace().next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let count_output = Command::new("find")
        .args(&[path, "-type", "f"])
        .output()?;
    
    let file_count = String::from_utf8_lossy(&count_output.stdout).lines().count();

    Ok((size_mb, file_count))
}
