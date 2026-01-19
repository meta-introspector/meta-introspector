//! meta_meme_classifier - What's your meta meme?
//! Are you a js d00d or pythonista? Let your git repos tell the story.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MemeProfile {
    identity: String,
    tagline: String,
    score: f64,
    evidence: Vec<String>,
    repos: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 What's Your Meta Meme?");
    println!("========================\n");
    
    // Load usage memes
    let data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("usage_memes.json")?
    )?;
    
    let memes: Vec<serde_json::Value> = data["memes"]
        .as_array()
        .unwrap()
        .to_vec();
    
    // Classify by language/ecosystem
    let mut profiles: HashMap<String, MemeProfile> = HashMap::new();
    
    for meme in &memes {
        let repo = meme["git_repo"].as_str().unwrap_or("");
        let empty_vec = vec![];
        let domains = meme["domains"].as_array().unwrap_or(&empty_vec);
        
        // JavaScript d00d
        if repo.contains("node") || repo.contains("npm") || repo.contains("javascript") 
            || repo.contains("typescript") || repo.contains("deno") {
            add_to_profile(&mut profiles, "js_d00d", repo);
        }
        
        // Pythonista
        if repo.contains("python") || repo.contains("pypi") || repo.contains("pip")
            || repo.contains("pytest") || repo.contains("django") {
            add_to_profile(&mut profiles, "pythonista", repo);
        }
        
        // Rustacean
        if repo.contains("rust") || repo.contains("cargo") || domains.iter().any(|d| d.as_str() == Some("rust")) {
            add_to_profile(&mut profiles, "rustacean", repo);
        }
        
        // Gopher
        if repo.contains("golang") || repo.contains("/go/") || repo.contains("go-") {
            add_to_profile(&mut profiles, "gopher", repo);
        }
        
        // C/C++ Wizard
        if repo.contains("llvm") || repo.contains("gcc") || repo.contains("clang")
            || repo.contains("glibc") || domains.iter().any(|d| d.as_str() == Some("compiler")) {
            add_to_profile(&mut profiles, "c_wizard", repo);
        }
        
        // Kernel Hacker
        if repo.contains("kernel") || repo.contains("linux") 
            || domains.iter().any(|d| d.as_str() == Some("kernel")) {
            add_to_profile(&mut profiles, "kernel_hacker", repo);
        }
        
        // DevOps Ninja
        if repo.contains("docker") || repo.contains("kubernetes") || repo.contains("ansible")
            || repo.contains("terraform") || repo.contains("systemd") {
            add_to_profile(&mut profiles, "devops_ninja", repo);
        }
    }
    
    // Calculate scores and add taglines
    finalize_profiles(&mut profiles);
    
    // Sort by score
    let mut sorted: Vec<_> = profiles.into_iter().collect();
    sorted.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap());
    
    println!("🎯 Your System's Meta Meme Profile:\n");
    
    for (i, (_, profile)) in sorted.iter().enumerate() {
        println!("{}. {} (score: {:.0})", i + 1, profile.identity, profile.score);
        println!("   \"{}\"", profile.tagline);
        println!("   Evidence: {} repos", profile.repos.len());
        if !profile.evidence.is_empty() {
            println!("   Examples: {}", profile.evidence.join(", "));
        }
        println!();
    }
    
    // Save profile
    let output = serde_json::json!({
        "tagline": "What's your meta meme? Are you a js d00d or pythonista?",
        "profiles": sorted.iter().map(|(_, p)| p).collect::<Vec<_>>(),
    });
    
    std::fs::write("meta_meme_profile.json", serde_json::to_string_pretty(&output)?)?;
    println!("✅ Wrote: meta_meme_profile.json");
    
    Ok(())
}

fn add_to_profile(profiles: &mut HashMap<String, MemeProfile>, key: &str, repo: &str) {
    profiles.entry(key.to_string()).or_insert_with(|| MemeProfile {
        identity: key.to_string(),
        tagline: String::new(),
        score: 0.0,
        evidence: vec![],
        repos: vec![],
    }).repos.push(repo.to_string());
}

fn finalize_profiles(profiles: &mut HashMap<String, MemeProfile>) {
    for (key, profile) in profiles.iter_mut() {
        profile.score = profile.repos.len() as f64;
        profile.evidence = profile.repos.iter().take(3).cloned().collect();
        
        profile.identity = match key.as_str() {
            "js_d00d" => "JavaScript d00d".to_string(),
            "pythonista" => "Pythonista".to_string(),
            "rustacean" => "Rustacean".to_string(),
            "gopher" => "Gopher".to_string(),
            "c_wizard" => "C/C++ Wizard".to_string(),
            "kernel_hacker" => "Kernel Hacker".to_string(),
            "devops_ninja" => "DevOps Ninja".to_string(),
            _ => key.to_string(),
        };
        
        profile.tagline = match key.as_str() {
            "js_d00d" => "npm install universe".to_string(),
            "pythonista" => "import antigravity".to_string(),
            "rustacean" => "fearless concurrency, zero-cost abstractions".to_string(),
            "gopher" => "simplicity is complicated".to_string(),
            "c_wizard" => "segfault is a feature".to_string(),
            "kernel_hacker" => "I compile my own kernel".to_string(),
            "devops_ninja" => "it works on my machine... in production".to_string(),
            _ => "undefined behavior".to_string(),
        };
    }
}
