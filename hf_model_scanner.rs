// Scan HuggingFace for small Rust coding models that fit in GPU

use std::process::Command;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HfModel {
    pub name: String,
    pub size_mb: u64,
    pub tags: Vec<String>,
    pub downloads: u64,
    pub fits_in_gpu: bool,
}

pub struct HfModelScanner {
    pub gpu_memory_mb: u64,
    pub models: Vec<HfModel>,
}

impl HfModelScanner {
    pub fn new(gpu_memory_mb: u64) -> Self {
        Self {
            gpu_memory_mb,
            models: Vec::new(),
        }
    }
    
    pub fn scan_rust_models(&mut self) -> Option<()> {
        // Use huggingface-cli to search for Rust coding models
        let output = Command::new("huggingface-cli")
            .args(&[
                "scan-cache",
                "--filter", "rust",
                "--filter", "code",
            ])
            .output()
            .ok()?;
        
        let result = String::from_utf8_lossy(&output.stdout);
        
        // Parse model information
        for line in result.lines() {
            if line.contains("rust") || line.contains("code") {
                // Extract model name and size
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let size_str = parts.get(1).unwrap_or(&"0");
                    let size_mb = size_str.parse::<u64>().unwrap_or(0);
                    
                    let fits = size_mb <= self.gpu_memory_mb;
                    
                    self.models.push(HfModel {
                        name,
                        size_mb,
                        tags: vec!["rust".to_string(), "code".to_string()],
                        downloads: 0,
                        fits_in_gpu: fits,
                    });
                }
            }
        }
        
        Some(())
    }
    
    pub fn search_api(&mut self) -> Option<()> {
        // Use HuggingFace API to search
        let output = Command::new("curl")
            .args(&[
                "-s",
                "https://huggingface.co/api/models?search=rust+code&sort=downloads&limit=50"
            ])
            .output()
            .ok()?;
        
        let json = String::from_utf8_lossy(&output.stdout);
        
        // Parse JSON response (simplified)
        for line in json.lines() {
            if line.contains("modelId") {
                // Extract model ID
                if let Some(start) = line.find("\"modelId\":\"") {
                    let rest = &line[start + 11..];
                    if let Some(end) = rest.find("\"") {
                        let model_name = rest[..end].to_string();
                        
                        // Estimate size (would need actual API call)
                        let estimated_size = 500; // MB
                        
                        self.models.push(HfModel {
                            name: model_name,
                            size_mb: estimated_size,
                            tags: vec!["rust".to_string()],
                            downloads: 0,
                            fits_in_gpu: estimated_size <= self.gpu_memory_mb,
                        });
                    }
                }
            }
        }
        
        Some(())
    }
    
    pub fn filter_small_models(&self) -> Vec<HfModel> {
        self.models.iter()
            .filter(|m| m.fits_in_gpu)
            .cloned()
            .collect()
    }
    
    pub fn sample_models(&self, count: usize) -> Vec<HfModel> {
        let small_models = self.filter_small_models();
        small_models.into_iter().take(count).collect()
    }
    
    pub fn report(&self) {
        println!("\n📊 HuggingFace Model Scan Report");
        println!("  GPU Memory: {} MB", self.gpu_memory_mb);
        println!("  Total models found: {}", self.models.len());
        
        let fitting = self.models.iter().filter(|m| m.fits_in_gpu).count();
        println!("  Models that fit: {}", fitting);
        
        if fitting > 0 {
            println!("\n  Small models (fit in GPU):");
            for model in self.filter_small_models().iter().take(10) {
                println!("    {} - {} MB", model.name, model.size_mb);
            }
        }
    }
}

pub fn get_gpu_memory() -> u64 {
    // Try to get GPU memory with nvidia-smi
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output() {
        
        let mem_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(mem_mb) = mem_str.trim().parse::<u64>() {
            return mem_mb;
        }
    }
    
    // Default to 8GB if can't detect
    8192
}
