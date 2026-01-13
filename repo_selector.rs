use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RepoSelector {
    github_stars: u32,
    rust_percentage: f32,
    recent_activity: bool,
    complexity_tier: Tier,
}

#[derive(Debug, Serialize, Deserialize)]
enum Tier {
    Basic,      // CLI tools, simple data structures
    Intermediate, // Web frameworks, async systems  
    Advanced,   // Compilers, databases, OS components
    Expert,     // rustc, LLVM bindings, formal verification
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoCandidate {
    name: String,
    url: String,
    stars: u32,
    rust_percentage: f32,
    last_commit_days: u32,
    tier: Tier,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 REPOSITORY SELECTOR FOR COMPREHENSIVE ANALYSIS");
    println!("================================================");

    // Predefined high-quality Rust repositories by tier
    let candidates = vec![
        // BASIC TIER - CLI tools, simple libraries
        RepoCandidate { name: "ripgrep".to_string(), url: "https://github.com/BurntSushi/ripgrep".to_string(), stars: 45000, rust_percentage: 98.0, last_commit_days: 30, tier: Tier::Basic },
        RepoCandidate { name: "fd".to_string(), url: "https://github.com/sharkdp/fd".to_string(), stars: 32000, rust_percentage: 95.0, last_commit_days: 15, tier: Tier::Basic },
        RepoCandidate { name: "bat".to_string(), url: "https://github.com/sharkdp/bat".to_string(), stars: 47000, rust_percentage: 92.0, last_commit_days: 20, tier: Tier::Basic },
        RepoCandidate { name: "exa".to_string(), url: "https://github.com/ogham/exa".to_string(), stars: 23000, rust_percentage: 98.0, last_commit_days: 60, tier: Tier::Basic },
        RepoCandidate { name: "starship".to_string(), url: "https://github.com/starship/starship".to_string(), stars: 42000, rust_percentage: 96.0, last_commit_days: 5, tier: Tier::Basic },
        
        // INTERMEDIATE TIER - Web frameworks, async systems
        RepoCandidate { name: "tokio".to_string(), url: "https://github.com/tokio-rs/tokio".to_string(), stars: 25000, rust_percentage: 89.0, last_commit_days: 2, tier: Tier::Intermediate },
        RepoCandidate { name: "actix-web".to_string(), url: "https://github.com/actix/actix-web".to_string(), stars: 20000, rust_percentage: 94.0, last_commit_days: 10, tier: Tier::Intermediate },
        RepoCandidate { name: "serde".to_string(), url: "https://github.com/serde-rs/serde".to_string(), stars: 8500, rust_percentage: 87.0, last_commit_days: 7, tier: Tier::Intermediate },
        RepoCandidate { name: "hyper".to_string(), url: "https://github.com/hyperium/hyper".to_string(), stars: 13000, rust_percentage: 91.0, last_commit_days: 5, tier: Tier::Intermediate },
        RepoCandidate { name: "warp".to_string(), url: "https://github.com/seanmonstar/warp".to_string(), stars: 9000, rust_percentage: 95.0, last_commit_days: 30, tier: Tier::Intermediate },
        
        // ADVANCED TIER - Compilers, databases, OS components
        RepoCandidate { name: "tikv".to_string(), url: "https://github.com/tikv/tikv".to_string(), stars: 14000, rust_percentage: 85.0, last_commit_days: 1, tier: Tier::Advanced },
        RepoCandidate { name: "servo".to_string(), url: "https://github.com/servo/servo".to_string(), stars: 26000, rust_percentage: 78.0, last_commit_days: 3, tier: Tier::Advanced },
        RepoCandidate { name: "swc".to_string(), url: "https://github.com/swc-project/swc".to_string(), stars: 30000, rust_percentage: 82.0, last_commit_days: 1, tier: Tier::Advanced },
        RepoCandidate { name: "deno".to_string(), url: "https://github.com/denoland/deno".to_string(), stars: 93000, rust_percentage: 65.0, last_commit_days: 1, tier: Tier::Advanced },
        RepoCandidate { name: "polkadot".to_string(), url: "https://github.com/paritytech/polkadot".to_string(), stars: 7000, rust_percentage: 88.0, last_commit_days: 1, tier: Tier::Advanced },
        
        // EXPERT TIER - rustc, formal verification, LLVM
        RepoCandidate { name: "rust".to_string(), url: "https://github.com/rust-lang/rust".to_string(), stars: 94000, rust_percentage: 75.0, last_commit_days: 1, tier: Tier::Expert },
        RepoCandidate { name: "miri".to_string(), url: "https://github.com/rust-lang/miri".to_string(), stars: 4000, rust_percentage: 92.0, last_commit_days: 2, tier: Tier::Expert },
        RepoCandidate { name: "chalk".to_string(), url: "https://github.com/rust-lang/chalk".to_string(), stars: 1700, rust_percentage: 95.0, last_commit_days: 15, tier: Tier::Expert },
        RepoCandidate { name: "prusti-dev".to_string(), url: "https://github.com/viperproject/prusti-dev".to_string(), stars: 2000, rust_percentage: 89.0, last_commit_days: 5, tier: Tier::Expert },
    ];

    // Filter and select repositories by tier
    let mut selected_repos: HashMap<String, Vec<RepoCandidate>> = HashMap::new();
    
    for candidate in candidates {
        if candidate.stars >= 1000 && candidate.rust_percentage >= 75.0 && candidate.last_commit_days <= 90 {
            let tier_name = format!("{:?}", candidate.tier);
            selected_repos.entry(tier_name).or_default().push(candidate);
        }
    }

    // Output selection results
    println!("\n📊 REPOSITORY SELECTION RESULTS:");
    for (tier, repos) in &selected_repos {
        println!("\n🎯 {} TIER ({} repositories):", tier.to_uppercase(), repos.len());
        for repo in repos {
            println!("  ✅ {} - {} stars, {:.1}% Rust, {} days ago", 
                     repo.name, repo.stars, repo.rust_percentage, repo.last_commit_days);
        }
    }

    // Save selection to JSON files
    for (tier, repos) in &selected_repos {
        let filename = format!("selected_repos_{}.json", tier.to_lowercase());
        let json_data = serde_json::to_string_pretty(repos)?;
        fs::write(&filename, json_data)?;
        println!("\n💾 Saved {} repositories to {}", repos.len(), filename);
    }

    println!("\n🚀 READY FOR PARALLEL ANALYSIS DEPLOYMENT");
    println!("Total repositories selected: {}", 
             selected_repos.values().map(|v| v.len()).sum::<usize>());

    Ok(())
}
