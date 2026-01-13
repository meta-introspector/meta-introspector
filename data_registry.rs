use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRegistry {
    pub datasets: HashMap<String, DatasetInfo>,
    pub pipelines: HashMap<String, PipelineInfo>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub path: PathBuf,
    pub format: DataFormat,
    pub size_bytes: u64,
    pub created: DateTime<Utc>,
    pub description: String,
    pub schema: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineInfo {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub binary: String,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataFormat {
    Json,
    Csv,
    Binary,
    Archive,
}

impl DataRegistry {
    pub fn new() -> Self {
        Self {
            datasets: HashMap::new(),
            pipelines: HashMap::new(),
            last_updated: Utc::now(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let registry_path = "data/registry.json";
        if Path::new(registry_path).exists() {
            let content = fs::read_to_string(registry_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.last_updated = Utc::now();
        fs::create_dir_all("data")?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write("data/registry.json", json)?;
        Ok(())
    }

    pub fn register_dataset(&mut self, name: &str, path: PathBuf, format: DataFormat, description: &str) -> Result<(), Box<dyn std::error::Error>> {
        let size_bytes = if path.exists() {
            fs::metadata(&path)?.len()
        } else {
            0
        };

        let dataset = DatasetInfo {
            name: name.to_string(),
            path,
            format,
            size_bytes,
            created: Utc::now(),
            description: description.to_string(),
            schema: None,
        };

        self.datasets.insert(name.to_string(), dataset);
        self.save()?;
        Ok(())
    }

    pub fn register_pipeline(&mut self, name: &str, binary: &str, inputs: Vec<String>, outputs: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let pipeline = PipelineInfo {
            name: name.to_string(),
            inputs,
            outputs,
            binary: binary.to_string(),
            last_run: None,
        };

        self.pipelines.insert(name.to_string(), pipeline);
        self.save()?;
        Ok(())
    }

    pub fn get_dataset_path(&self, name: &str) -> Option<&PathBuf> {
        self.datasets.get(name).map(|d| &d.path)
    }

    pub fn list_datasets(&self) -> Vec<&str> {
        self.datasets.keys().map(|s| s.as_str()).collect()
    }

    pub fn list_pipelines(&self) -> Vec<&str> {
        self.pipelines.keys().map(|s| s.as_str()).collect()
    }
}

// Standard data paths
pub struct DataPaths;

impl DataPaths {
    pub const REGISTRY: &'static str = "data/registry.json";
    pub const RAW: &'static str = "data/raw";
    pub const PROCESSED: &'static str = "data/processed";
    pub const CACHE: &'static str = "data/cache";
    pub const ANALYSIS: &'static str = "data/analysis";
    pub const COMPRESSED: &'static str = "data/compressed";
    pub const BUILD_ORDER: &'static str = "data/build_order";
    
    pub fn ensure_dirs() -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(Self::RAW)?;
        fs::create_dir_all(Self::PROCESSED)?;
        fs::create_dir_all(Self::CACHE)?;
        fs::create_dir_all(Self::ANALYSIS)?;
        fs::create_dir_all(Self::COMPRESSED)?;
        fs::create_dir_all(Self::BUILD_ORDER)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗂️  CENTRAL DATA REGISTRY");
    
    DataPaths::ensure_dirs()?;
    let mut registry = DataRegistry::load()?;
    
    // Register core datasets
    registry.register_dataset(
        "rust_source_archives",
        PathBuf::from("/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz"),
        DataFormat::Archive,
        "Rust 1.92.0 source code archive"
    )?;
    
    registry.register_dataset(
        "rust_source_nightly",
        PathBuf::from("/nix/store/xp98ag7yvxjk13a3yan8qilb97wsavgy-rust-src-nightly.tar.xz"),
        DataFormat::Archive,
        "Rust nightly source code archive"
    )?;
    
    // Register pipelines
    registry.register_pipeline(
        "extract_rust_sources",
        "extract_rust_sources",
        vec!["rust_source_archives".to_string(), "rust_source_nightly".to_string()],
        vec!["rust_source_analysis.json".to_string()]
    )?;
    
    registry.register_pipeline(
        "build_order_capture",
        "rustc_interceptor.rs",
        vec!["rust_project".to_string()],
        vec!["rustc_intercept_compression.json".to_string()]
    )?;
    
    registry.register_pipeline(
        "build_order_analysis",
        "build_order_pipeline",
        vec!["rust_source_archives".to_string(), "rustc_intercept_compression.json".to_string()],
        vec!["build_order_analysis.json".to_string()]
    )?;
    
    println!("📊 Registered {} datasets", registry.datasets.len());
    println!("🔄 Registered {} pipelines", registry.pipelines.len());
    
    println!("\n📁 DATASETS:");
    for (name, dataset) in &registry.datasets {
        println!("  {} -> {} ({:.2} MB)", name, dataset.path.display(), dataset.size_bytes as f64 / 1_000_000.0);
    }
    
    println!("\n🔄 PIPELINES:");
    for (name, pipeline) in &registry.pipelines {
        println!("  {} -> {}", name, pipeline.binary);
        println!("    Inputs: {:?}", pipeline.inputs);
        println!("    Outputs: {:?}", pipeline.outputs);
    }
    
    println!("\n💾 Registry saved to: {}", DataPaths::REGISTRY);
    Ok(())
}
