//! usage_meme_store - Turn git usage into P2P shareable memes
//! 
//! Tracks which git repos are used by which packages and creates
//! a parquet-based meme store for P2P distribution

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct UsageMeme {
    git_repo: String,
    used_by_nix: Vec<String>,
    used_by_apt: Vec<String>,
    usage_count: u64,
    domains: Vec<String>,
    meme_score: f64,
    first_seen: String,
    last_seen: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 Building Usage Meme Store");
    println!("============================\n");
    
    // Load Nix data
    let nix_data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("nix_store_all_sources.json")?
    )?;
    
    let nix_repos: Vec<String> = nix_data["git_repos"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    
    println!("📦 Loaded {} Nix git repos", nix_repos.len());
    
    // Load Apt data (if available)
    let apt_repos: Vec<String> = if let Ok(apt_data) = std::fs::read_to_string("apt_all_sources.json") {
        let apt_json: serde_json::Value = serde_json::from_str(&apt_data)?;
        apt_json["git_repos"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    } else {
        println!("⚠️  Apt data not ready yet, using Nix only");
        vec![]
    };
    
    println!("📦 Loaded {} Apt git repos", apt_repos.len());
    
    // Build usage map
    let mut usage_map: HashMap<String, UsageMeme> = HashMap::new();
    let now = Utc::now().to_rfc3339();
    
    // Process Nix repos
    for repo in &nix_repos {
        let clean_repo = normalize_git_url(repo);
        usage_map.entry(clean_repo.clone()).or_insert_with(|| UsageMeme {
            git_repo: clean_repo,
            used_by_nix: vec![],
            used_by_apt: vec![],
            usage_count: 0,
            domains: extract_domains(repo),
            meme_score: 0.0,
            first_seen: now.clone(),
            last_seen: now.clone(),
        }).used_by_nix.push("nix-store".to_string());
    }
    
    // Process Apt repos
    for repo in &apt_repos {
        let clean_repo = normalize_git_url(repo);
        usage_map.entry(clean_repo.clone()).or_insert_with(|| UsageMeme {
            git_repo: clean_repo.clone(),
            used_by_nix: vec![],
            used_by_apt: vec![],
            usage_count: 0,
            domains: extract_domains(repo),
            meme_score: 0.0,
            first_seen: now.clone(),
            last_seen: now.clone(),
        }).used_by_apt.push("apt-system".to_string());
    }
    
    // Calculate meme scores
    for meme in usage_map.values_mut() {
        meme.usage_count = (meme.used_by_nix.len() + meme.used_by_apt.len()) as u64;
        meme.meme_score = calculate_meme_score(meme);
    }
    
    println!("\n📊 Generated {} unique usage memes", usage_map.len());
    
    // Convert to sorted vec
    let mut memes: Vec<UsageMeme> = usage_map.into_values().collect();
    memes.sort_by(|a, b| b.meme_score.partial_cmp(&a.meme_score).unwrap());
    
    // Save as JSON (parquet requires arrow schema)
    let output = serde_json::json!({
        "generated": now,
        "total_memes": memes.len(),
        "nix_repos": nix_repos.len(),
        "apt_repos": apt_repos.len(),
        "memes": memes,
    });
    
    std::fs::write("usage_memes.json", serde_json::to_string_pretty(&output)?)?;
    println!("✅ Wrote: usage_memes.json");
    
    // Top memes
    println!("\n🔥 Top 10 Memes by Score:");
    for (i, meme) in memes.iter().take(10).enumerate() {
        println!("  {}. {} (score: {:.2})", i + 1, meme.git_repo, meme.meme_score);
    }
    
    // Create P2P manifest
    create_p2p_manifest(&memes)?;
    
    Ok(())
}

fn normalize_git_url(url: &str) -> String {
    url.trim_end_matches(".git")
       .trim_end_matches('/')
       .trim_end_matches('\\')
       .split('?').next().unwrap_or(url)
       .to_string()
}

fn extract_domains(url: &str) -> Vec<String> {
    let mut domains = vec![];
    
    if url.contains("llvm") || url.contains("clang") {
        domains.push("compiler".to_string());
    }
    if url.contains("rust") || url.contains("cargo") {
        domains.push("rust".to_string());
    }
    if url.contains("python") || url.contains("pypi") {
        domains.push("python".to_string());
    }
    if url.contains("kernel") || url.contains("linux") {
        domains.push("kernel".to_string());
    }
    if url.contains("systemd") || url.contains("glibc") {
        domains.push("system".to_string());
    }
    if url.contains("meson") || url.contains("cmake") || url.contains("ninja") {
        domains.push("build".to_string());
    }
    
    if domains.is_empty() {
        domains.push("other".to_string());
    }
    
    domains
}

fn calculate_meme_score(meme: &UsageMeme) -> f64 {
    let base_score = meme.usage_count as f64;
    let nix_bonus = if !meme.used_by_nix.is_empty() { 10.0 } else { 0.0 };
    let apt_bonus = if !meme.used_by_apt.is_empty() { 10.0 } else { 0.0 };
    let cross_system = if !meme.used_by_nix.is_empty() && !meme.used_by_apt.is_empty() { 50.0 } else { 0.0 };
    
    // Domain multipliers
    let domain_bonus: f64 = meme.domains.iter().map(|d| match d.as_str() {
        "compiler" => 20.0,
        "system" => 15.0,
        "kernel" => 15.0,
        "rust" => 10.0,
        "python" => 5.0,
        "build" => 5.0,
        _ => 1.0,
    }).sum();
    
    base_score + nix_bonus + apt_bonus + cross_system + domain_bonus
}

fn create_p2p_manifest(memes: &[UsageMeme]) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = serde_json::json!({
        "version": "1.0",
        "name": "meta-introspector-bootstrap-dataset",
        "description": "Complete git repository bootstrap dataset for system reproduction",
        "generated": Utc::now().to_rfc3339(),
        "stats": {
            "total_repos": memes.len(),
            "top_domains": count_domains(memes),
        },
        "files": [
            {
                "name": "nix_store_git_repos.txt",
                "description": "All git repos from Nix store",
                "format": "text/plain",
            },
            {
                "name": "apt_git_repos.txt",
                "description": "All git repos from apt packages",
                "format": "text/plain",
            },
            {
                "name": "usage_memes.json",
                "description": "Usage patterns and meme scores",
                "format": "application/json",
            }
        ],
        "p2p": {
            "ipfs": "Run: ipfs add -r . to generate CID",
            "torrent": "Run: transmission-create to generate torrent",
            "magnet": "Generated after torrent creation",
            "huggingface": "https://huggingface.co/datasets/introspector/meta-meme"
        }
    });
    
    std::fs::write("p2p_manifest.json", serde_json::to_string_pretty(&manifest)?)?;
    println!("✅ Wrote: p2p_manifest.json");
    
    Ok(())
}

fn count_domains(memes: &[UsageMeme]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for meme in memes {
        for domain in &meme.domains {
            *counts.entry(domain.clone()).or_insert(0) += 1;
        }
    }
    counts
}
