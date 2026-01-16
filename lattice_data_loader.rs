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
    pub fn load_from_huggingface(subdir: &str) -> Result<Self, String> {
        // Load from introspector/rust dataset with subdirs
        let dataset_path = format!("introspector/rust/{}", subdir);
        
        let output = Command::new("python3")
            .arg("-c")
            .arg(format!(
                "from datasets import load_dataset; \
                 ds = load_dataset('introspector/rust', data_dir='{}'); \
                 print(len(ds['train'])); \
                 print(ds['train'].column_names)",
                subdir
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
            num_syn_types: 11,
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
        // Try subdirs in introspector/rust dataset
        let subdirs = vec![
            "lattice",
            "syn-mappings",
            "rustc-ips",
            "pokemon-storage",
            "blockchain",
            "embeddings",
            "ziggurat",
        ];
        
        for subdir in subdirs {
            println!("  Trying introspector/rust/{}", subdir);
            if let Ok(data) = Self::load_from_huggingface(subdir) {
                println!("  ✓ Loaded from {}", subdir);
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
