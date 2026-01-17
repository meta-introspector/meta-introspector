// 🔥 EXISTING CODE DOCUMENTATION COLLECTOR
// Structured collection of LMFDB/meme/godel files using locate

use std::collections::HashMap;
use std::process::Command;
use telemetry_lib::telemetry_lib::{TelemetryEntry, write_telemetry_entry, get_log_file};

pub struct ExistingCodeCollector {
    pub file_categories: HashMap<String, Vec<String>>,
    pub location_patterns: Vec<String>,
    pub total_files_found: usize,
}

impl ExistingCodeCollector {
    pub fn new() -> Self {
        Self {
            file_categories: HashMap::new(),
            location_patterns: vec![
                "lmfdb".to_string(),
                "meme".to_string(), 
                "godel".to_string(),
                "solfunmeme".to_string(),
                "golem".to_string(),
                "muse".to_string(),
                "metameme".to_string(),
                "conductor".to_string(),
                "weight".to_string(),
                "level".to_string(),
            ],
            total_files_found: 0,
        }
    }
    
    pub fn collect_all_existing_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let project = "existing_code_collection";
        let start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
        
        report_start!("Existing Code Documentation Collector", project);
        
        // Log telemetry start
        let entry = TelemetryEntry {
            r#type: "code_collection_start".to_string(),
            message: "Starting existing code collection".to_string(),
            timestamp: start_time,
            project: project.to_string(),
            binaries: 0,
            libraries: 0,
            symbols: 0,
        };
        let log_file = get_log_file(project, start_time);
        write_telemetry_entry(&entry, &log_file)?;
        
        // Collect files for each pattern
        for pattern in &self.location_patterns.clone() {
            report_section!(&format!("Collecting {} files", pattern));
            let files = self.locate_pattern(pattern)?;
            report_count!(&format!("{} files found", pattern), files.len());
            
            self.file_categories.insert(pattern.clone(), files);
        }
        
        // Calculate totals
        self.total_files_found = self.file_categories.values()
            .map(|files| files.len())
            .sum();
        
        // Generate documentation
        self.generate_documentation()?;
        
        // Log final telemetry
        let final_entry = TelemetryEntry {
            r#type: "code_collection_complete".to_string(),
            message: format!("Collected {} files across {} patterns", 
                           self.total_files_found, self.location_patterns.len()),
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            project: project.to_string(),
            binaries: self.location_patterns.len() as u32,
            libraries: self.file_categories.len() as u32,
            symbols: self.total_files_found as u32,
        };
        write_telemetry_entry(&final_entry, &log_file)?;
        
        report_summary!(
            "Total patterns" => self.location_patterns.len(),
            "Total files" => self.total_files_found,
            "Categories" => self.file_categories.len()
        );
        
        report_end!("Existing Code Documentation Collector", project, start_time);
        
        Ok(())
    }
    
    fn locate_pattern(&self, pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        // Use locate to find files
        let output = Command::new("locate")
            .arg("-i")  // Case insensitive
            .arg(pattern)
            .output()?;
            
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            for line in stdout.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    // Filter for relevant files
                    if self.is_relevant_file(line, pattern) {
                        files.push(line.to_string());
                    }
                }
            }
        }
        
        // Also search for Rust files containing the pattern
        let rust_files = self.locate_rust_files_with_pattern(pattern)?;
        files.extend(rust_files);
        
        // Deduplicate and sort
        files.sort();
        files.dedup();
        
        Ok(files)
    }
    
    fn locate_rust_files_with_pattern(&self, pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        // Search for .rs files containing pattern in name
        let pattern_query = format!("{}*.rs", pattern);
        let output = Command::new("locate")
            .arg("-i")
            .arg(&pattern_query)
            .output()?;
            
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            for line in stdout.lines() {
                let line = line.trim();
                if line.ends_with(".rs") && self.is_in_relevant_directory(line) {
                    files.push(line.to_string());
                }
            }
        }
        
        Ok(files)
    }
    
    fn is_relevant_file(&self, path: &str, pattern: &str) -> bool {
        // Filter criteria
        path.contains("meta-introspector") || 
        path.contains("zos") ||
        path.contains("solfunmeme") ||
        (path.ends_with(".rs") && path.contains(pattern))
    }
    
    fn is_in_relevant_directory(&self, path: &str) -> bool {
        path.contains("meta-introspector") ||
        path.contains("zos-qa") ||
        path.contains("zos-server") ||
        path.contains("solfunmeme") ||
        path.contains("/nix/time/")
    }
    
    fn generate_documentation(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = String::new();
        
        doc.push_str("# 🔥 EXISTING LMFDB/MEME CODE DOCUMENTATION\n\n");
        doc.push_str(&format!("**Total Files Found**: {}\n\n", self.total_files_found));
        
        for (pattern, files) in &self.file_categories {
            doc.push_str(&format!("## 📁 {} Files ({} found)\n\n", pattern.to_uppercase(), files.len()));
            
            // Group by directory
            let mut by_directory: HashMap<String, Vec<String>> = HashMap::new();
            
            for file in files {
                let dir = if let Some(pos) = file.rfind('/') {
                    file[..pos].to_string()
                } else {
                    ".".to_string()
                };
                
                by_directory.entry(dir).or_insert_with(Vec::new).push(file.clone());
            }
            
            for (dir, dir_files) in by_directory {
                doc.push_str(&format!("### 📂 {}\n\n", dir));
                
                for file in dir_files {
                    let filename = file.split('/').last().unwrap_or(&file);
                    doc.push_str(&format!("- `{}`\n", filename));
                }
                
                doc.push_str("\n");
            }
        }
        
        // Write documentation
        std::fs::write("EXISTING_CODE_DOCUMENTATION.md", doc)?;
        
        println!("📄 Documentation written to: EXISTING_CODE_DOCUMENTATION.md");
        
        Ok(())
    }
    
    pub fn print_summary(&self) {
        println!("📊 EXISTING CODE COLLECTION SUMMARY");
        println!("===================================");
        
        for (pattern, files) in &self.file_categories {
            println!("🔍 {}: {} files", pattern, files.len());
            
            // Show top 3 files as examples
            for (i, file) in files.iter().take(3).enumerate() {
                let filename = file.split('/').last().unwrap_or(file);
                println!("  {}. {}", i+1, filename);
            }
            
            if files.len() > 3 {
                println!("  ... and {} more", files.len() - 3);
            }
            
            println!();
        }
    }
}

// Import telemetry_lib module
#[path = "telemetry_lib.rs"]
mod telemetry_lib;

fn main() {
    println!("existing_code_collector - add usage here");
}
