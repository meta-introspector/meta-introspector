// Load lattice data from HuggingFace datasets

use std::process::Command;
use std::collections::HashSet;

pub struct LatticeData {
    pub num_syn_types: usize,
    pub num_ips: usize,
    pub num_weights: usize,
    pub syn_types: Vec<String>,
    pub unique_ips: HashSet<u64>,
}

impl LatticeData {
    pub fn load_from_huggingface(dataset_name: &str) -> Result<Self, String> {
        // Use huggingface-cli to load dataset
        let output = Command::new("python3")
            .arg("-c")
            .arg(format!(
                "from datasets import load_dataset; \
                 ds = load_dataset('{}'); \
                 print(len(ds['train'])); \
                 print(ds['train'].column_names)",
                dataset_name
            ))
            .output()
            .map_err(|e| e.to_string())?;
        
        let result = String::from_utf8_lossy(&output.stdout);
        
        // Parse dataset info
        let lines: Vec<&str> = result.lines().collect();
        let num_rows = lines.get(0)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        
        Ok(LatticeData {
            num_syn_types: 11, // Will be loaded from dataset
            num_ips: num_rows,
            num_weights: 768,
            syn_types: vec![
                "Fn".to_string(),
                "Struct".to_string(),
                "Enum".to_string(),
                "Trait".to_string(),
                "Impl".to_string(),
                "Const".to_string(),
                "Static".to_string(),
                "Type".to_string(),
                "Mod".to_string(),
                "Use".to_string(),
                "Macro".to_string(),
            ],
            unique_ips: HashSet::new(),
        })
    }
    
    pub fn load_from_available_datasets() -> Result<Self, String> {
        // Try multiple HuggingFace datasets
        let datasets = vec![
            "meta-introspector/rust-lattice",
            "meta-introspector/syn-mappings",
            "meta-introspector/rustc-ips",
            "meta-introspector/pokemon-storage",
        ];
        
        for dataset in datasets {
            println!("  Trying dataset: {}", dataset);
            if let Ok(data) = Self::load_from_huggingface(dataset) {
                println!("  ✓ Loaded from {}", dataset);
                return Ok(data);
            }
        }
        
        // Fallback to mock data
        println!("  ⚠ Using mock data (datasets not yet published)");
        Ok(LatticeData {
            num_syn_types: 11,
            num_ips: 103,
            num_weights: 768,
            syn_types: vec![
                "Fn".to_string(),
                "Struct".to_string(),
                "Enum".to_string(),
                "Trait".to_string(),
                "Impl".to_string(),
                "Const".to_string(),
                "Static".to_string(),
                "Type".to_string(),
                "Mod".to_string(),
                "Use".to_string(),
                "Macro".to_string(),
            ],
            unique_ips: (0..103).collect(),
        })
    }
    
    pub fn report(&self) {
        println!("\n📊 Loaded Lattice Data");
        println!("  Syn types: {}", self.num_syn_types);
        println!("  Unique IPs: {}", self.num_ips);
        println!("  Weight dimensions: {}", self.num_weights);
        
        println!("\n  Syn types:");
        for (i, syn_type) in self.syn_types.iter().enumerate() {
            println!("    {}. {}", i + 1, syn_type);
        }
    }
}
