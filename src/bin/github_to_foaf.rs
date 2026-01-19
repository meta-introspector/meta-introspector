//! github_to_foaf - Ingest GitHub stars and forks, export to FOAF
//! 
//! Uses octocrab to fetch GitHub activity and convert to semantic web format

use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GitHubActivity {
    username: String,
    stars: Vec<Repo>,
    forks: Vec<Repo>,
    repos: Vec<Repo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repo {
    name: String,
    full_name: String,
    url: String,
    description: Option<String>,
    language: Option<String>,
    topics: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⭐ GitHub to FOAF Converter");
    println!("===========================\n");
    
    let username = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "meta-introspector".to_string());
    
    println!("👤 Fetching GitHub activity for: {}", username);
    
    // Initialize octocrab
    let token = std::env::var("GITHUB_TOKEN").ok();
    let octocrab = if let Some(token) = token {
        Octocrab::builder().personal_token(token).build()?
    } else {
        println!("💡 Set GITHUB_TOKEN for higher rate limits");
        Octocrab::builder().build()?
    };
    
    // Fetch starred repos
    println!("\n⭐ Fetching starred repositories...");
    let stars = octocrab
        .users(&username)
        .starred()
        .per_page(100)
        .send()
        .await?;
    
    let starred_repos: Vec<Repo> = stars
        .items
        .iter()
        .map(|repo| Repo {
            name: repo.name.clone(),
            full_name: repo.full_name.clone().unwrap_or_default(),
            url: repo.html_url.clone().map(|u| u.to_string()).unwrap_or_default(),
            description: repo.description.clone(),
            language: repo.language.as_ref().and_then(|v| v.as_str()).map(String::from),
            topics: vec![],
        })
        .collect();
    
    println!("   Found {} starred repos", starred_repos.len());
    
    // Fetch user repos
    println!("\n📦 Fetching repositories...");
    let repos = octocrab
        .users(&username)
        .repos()
        .per_page(100)
        .send()
        .await?;
    
    let user_repos: Vec<Repo> = repos
        .items
        .iter()
        .map(|repo| Repo {
            name: repo.name.clone(),
            full_name: repo.full_name.clone().unwrap_or_default(),
            url: repo.html_url.clone().map(|u| u.to_string()).unwrap_or_default(),
            description: repo.description.clone(),
            language: repo.language.as_ref().and_then(|v| v.as_str()).map(String::from),
            topics: vec![],
        })
        .collect();
    
    let forks: Vec<Repo> = user_repos
        .iter()
        .filter(|r| r.name.contains("fork") || r.full_name.contains("/"))
        .cloned()
        .collect();
    
    println!("   Found {} repositories", user_repos.len());
    println!("   Found {} forks", forks.len());
    
    // Save activity
    let activity = GitHubActivity {
        username: username.clone(),
        stars: starred_repos.clone(),
        forks: forks.clone(),
        repos: user_repos.clone(),
    };
    
    let json = serde_json::to_string_pretty(&activity)?;
    std::fs::write("github_activity.json", json)?;
    println!("\n✅ Saved: github_activity.json");
    
    // Generate FOAF
    println!("\n🕸️  Generating FOAF document...");
    generate_foaf(&username, &activity)?;
    
    println!("\n✅ Complete! Files created:");
    println!("   - github_activity.json");
    println!("   - github_foaf.ttl");
    
    Ok(())
}

fn generate_foaf(username: &str, activity: &GitHubActivity) -> Result<(), Box<dyn std::error::Error>> {
    let mut foaf = String::new();
    
    // Header
    foaf.push_str(&format!(r#"@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix doap: <http://usefulinc.com/ns/doap#> .
@prefix gh: <https://github.com/> .

<gh:{}> a foaf:Person ;
    foaf:name "{}" ;
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://github.com> ;
        foaf:accountName "{}"
    ] ;

"#, username, username, username));
    
    // Add starred repos as interests
    foaf.push_str("    # Starred Repositories (Interests)\n");
    for (i, repo) in activity.stars.iter().take(50).enumerate() {
        if i > 0 { foaf.push_str(" ;\n"); }
        foaf.push_str(&format!("    foaf:interest <{}>", repo.url));
    }
    foaf.push_str(" ;\n\n");
    
    // Add own repos as projects
    foaf.push_str("    # Own Repositories (Projects)\n");
    for (i, repo) in activity.repos.iter().enumerate() {
        if i > 0 { foaf.push_str(" ;\n"); }
        foaf.push_str(&format!("    foaf:currentProject <{}>", repo.url));
    }
    foaf.push_str(" .\n\n");
    
    // Add repo details as DOAP projects
    foaf.push_str("# Repository Details\n\n");
    for repo in &activity.repos {
        foaf.push_str(&format!(r#"<{}> a doap:Project ;
    doap:name "{}" ;
    doap:homepage <{}> ;
"#, repo.url, repo.name, repo.url));
        
        if let Some(desc) = &repo.description {
            foaf.push_str(&format!("    doap:description \"{}\" ;\n", desc.replace('"', "'")));
        }
        
        if let Some(lang) = &repo.language {
            foaf.push_str(&format!("    doap:programming-language \"{}\" ;\n", lang));
        }
        
        foaf.push_str(&format!("    doap:repository <{}.git> .\n\n", repo.url));
    }
    
    // Add starred repos as interests with details
    foaf.push_str("# Starred Repositories (Interests)\n\n");
    for repo in activity.stars.iter().take(50) {
        foaf.push_str(&format!(r#"<{}> a doap:Project ;
    doap:name "{}" ;
    rdfs:label "{}" ;
"#, repo.url, repo.name, repo.full_name));
        
        if let Some(desc) = &repo.description {
            foaf.push_str(&format!("    doap:description \"{}\" ;\n", desc.replace('"', "'")));
        }
        
        if let Some(lang) = &repo.language {
            foaf.push_str(&format!("    doap:programming-language \"{}\" ;\n", lang));
        }
        
        if !repo.topics.is_empty() {
            foaf.push_str("    doap:category ");
            for (i, topic) in repo.topics.iter().enumerate() {
                if i > 0 { foaf.push_str(", "); }
                foaf.push_str(&format!("\"{}\"", topic));
            }
            foaf.push_str(" ;\n");
        }
        
        foaf.push_str(&format!("    doap:homepage <{}> .\n\n", repo.url));
    }
    
    std::fs::write("github_foaf.ttl", foaf)?;
    println!("✅ Generated: github_foaf.ttl");
    
    // Generate stats
    let mut stats = String::new();
    stats.push_str(&format!("# GitHub Activity Stats for {}\n\n", username));
    stats.push_str(&format!("Total starred: {}\n", activity.stars.len()));
    stats.push_str(&format!("Total repos: {}\n", activity.repos.len()));
    stats.push_str(&format!("Total forks: {}\n\n", activity.forks.len()));
    
    // Language breakdown
    let mut lang_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for repo in &activity.stars {
        if let Some(lang) = &repo.language {
            *lang_count.entry(lang.clone()).or_insert(0) += 1;
        }
    }
    
    stats.push_str("Top languages in starred repos:\n");
    let mut langs: Vec<_> = lang_count.iter().collect();
    langs.sort_by(|a, b| b.1.cmp(a.1));
    for (lang, count) in langs.iter().take(10) {
        stats.push_str(&format!("  {}: {}\n", lang, count));
    }
    
    std::fs::write("github_stats.txt", stats)?;
    println!("✅ Generated: github_stats.txt");
    
    Ok(())
}
