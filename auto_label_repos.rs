//! Auto-Label Repos - Introspective Classification
//! Analyzes code to auto-label repos by keyword and taxonomy
//! Stores in time-based structure

use std::collections::HashMap;
use std::path::PathBuf;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use arrow::array::StringArray;
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct RepoLabel {
    url: String,
    timestamp: String,
    keywords: Vec<String>,
    taxonomy: Vec<String>,
    language: String,
    framework: String,
    purpose: String,
}

struct AutoLabeler {
    // Introspective model: programs that process this code
    processors: HashMap<String, Vec<String>>,
}

impl AutoLabeler {
    fn new() -> Self {
        let mut processors = HashMap::new();
        
        // Model: What programs process what code?
        processors.insert("rust".to_string(), vec!["rustc".to_string(), "cargo".to_string()]);
        processors.insert("nix".to_string(), vec!["nix-build".to_string(), "nix-shell".to_string()]);
        processors.insert("python".to_string(), vec!["python".to_string(), "pip".to_string()]);
        
        Self { processors }
    }

    fn label_repo(&self, url: &str, content: &str) -> RepoLabel {
        let keywords = self.extract_keywords(content);
        let taxonomy = self.classify_taxonomy(&keywords);
        let language = self.detect_language(content);
        let framework = self.detect_framework(content);
        let purpose = self.infer_purpose(&keywords, &taxonomy);
        
        RepoLabel {
            url: url.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            keywords,
            taxonomy,
            language,
            framework,
            purpose,
        }
    }

    fn extract_keywords(&self, content: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Introspective: What does the code say about itself?
        if content.contains("blockchain") { keywords.push("blockchain".to_string()); }
        if content.contains("compiler") { keywords.push("compiler".to_string()); }
        if content.contains("p2p") { keywords.push("p2p".to_string()); }
        if content.contains("git") { keywords.push("git".to_string()); }
        if content.contains("mirror") { keywords.push("mirror".to_string()); }
        if content.contains("telemetry") { keywords.push("telemetry".to_string()); }
        
        keywords
    }

    fn classify_taxonomy(&self, keywords: &[String]) -> Vec<String> {
        let mut taxonomy = Vec::new();
        
        // Hierarchical classification
        if keywords.contains(&"blockchain".to_string()) {
            taxonomy.push("distributed-systems".to_string());
            taxonomy.push("cryptocurrency".to_string());
        }
        if keywords.contains(&"compiler".to_string()) {
            taxonomy.push("programming-languages".to_string());
            taxonomy.push("tooling".to_string());
        }
        if keywords.contains(&"git".to_string()) {
            taxonomy.push("version-control".to_string());
            taxonomy.push("infrastructure".to_string());
        }
        
        taxonomy
    }

    fn detect_language(&self, content: &str) -> String {
        if content.contains("fn main()") { return "rust".to_string(); }
        if content.contains("def ") { return "python".to_string(); }
        if content.contains("mkDerivation") { return "nix".to_string(); }
        "unknown".to_string()
    }

    fn detect_framework(&self, content: &str) -> String {
        if content.contains("actix") { return "actix-web".to_string(); }
        if content.contains("tokio") { return "tokio".to_string(); }
        if content.contains("libp2p") { return "libp2p".to_string(); }
        "none".to_string()
    }

    fn infer_purpose(&self, keywords: &[String], taxonomy: &[String]) -> String {
        // Introspective inference from keywords and taxonomy
        if keywords.contains(&"mirror".to_string()) && keywords.contains(&"git".to_string()) {
            return "git-mirror-service".to_string();
        }
        if taxonomy.contains(&"compiler".to_string()) {
            return "language-tooling".to_string();
        }
        "general-purpose".to_string()
    }

    fn save_to_time_repo(&self, label: &RepoLabel) -> Result<(), Box<dyn std::error::Error>> {
        // Store in time-based structure: ~/nix/time/YYYY/MM/DD/
        let now = Utc::now();
        let time_path = format!(
            "{}/nix/time/{}/{:02}/{:02}",
            std::env::var("HOME")?,
            now.year(),
            now.month(),
            now.day()
        );
        
        std::fs::create_dir_all(&time_path)?;
        
        // Save as JSON
        let json = serde_json::to_string_pretty(label)?;
        let file_path = format!("{}/{}.json", time_path, sanitize_url(&label.url));
        std::fs::write(file_path, json)?;
        
        Ok(())
    }
}

fn sanitize_url(url: &str) -> String {
    url.replace("https://", "")
       .replace("http://", "")
       .replace("/", "_")
       .replace(":", "_")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏷️  Auto-labeling repos with introspective model");
    
    let labeler = AutoLabeler::new();
    
    // Read URLs from extracted list
    let urls = std::fs::read_to_string("data/extracted_git_urls.parquet")?;
    
    let mut labels = Vec::new();
    
    for url in urls.lines().take(10) {
        println!("  Labeling: {}", url);
        
        // Fetch content from git mirror
        let content = reqwest::blocking::get(
            format!("http://localhost:9418/content?url={}", url)
        )?.text()?;
        
        let label = labeler.label_repo(url, &content);
        
        println!("    Keywords: {:?}", label.keywords);
        println!("    Taxonomy: {:?}", label.taxonomy);
        println!("    Purpose: {}", label.purpose);
        
        // Save to time repo
        labeler.save_to_time_repo(&label)?;
        
        labels.push(label);
    }
    
    println!("\n✅ Labeled {} repos", labels.len());
    println!("📁 Saved to ~/nix/time/YYYY/MM/DD/");
    
    Ok(())
}
