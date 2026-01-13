use std::process::Command;
use std::fs;
use chrono::Utc;

// Import from data_registry.rs
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRegistry {
    pub datasets: HashMap<String, DatasetInfo>,
    pub pipelines: HashMap<String, PipelineInfo>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineInfo {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub binary: String,
    pub last_run: Option<DateTime<Utc>>,
}

impl DataRegistry {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("data/registry.json")?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("data/registry.json", json)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PIPELINE RUNNER");
    
    let mut registry = DataRegistry::load()?;
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: pipeline_runner <pipeline_name>");
        println!("\nAvailable pipelines:");
        for name in registry.pipelines.keys() {
            println!("  {}", name);
        }
        return Ok(());
    }
    
    let pipeline_name = &args[1];
    
    if let Some(pipeline) = registry.pipelines.get_mut(pipeline_name) {
        println!("🔄 Running pipeline: {}", pipeline_name);
        println!("📦 Binary: {}", pipeline.binary);
        
        // Run the pipeline
        println!("🏃 Executing...");
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", &pipeline.binary]);
        
        let output = cmd.output()?;
        
        if output.status.success() {
            println!("✅ Pipeline completed successfully");
            pipeline.last_run = Some(Utc::now());
            registry.save()?;
        } else {
            println!("❌ Pipeline failed");
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        
    } else {
        println!("❌ Pipeline '{}' not found", pipeline_name);
        println!("\nAvailable pipelines:");
        for name in registry.pipelines.keys() {
            println!("  {}", name);
        }
    }
    
    Ok(())
}
