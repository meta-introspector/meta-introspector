// Ollama Introspector: Record LLM traces and map syn objects to weights

use std::process::Command;
use std::collections::HashMap;

#[derive(Clone)]
pub struct LlmTrace {
    pub prompt: String,
    pub response: String,
    pub tokens: Vec<String>,
    pub weights_accessed: Vec<(String, f64)>,
    pub execution_path: Vec<u64>,
}

#[derive(Clone)]
pub struct SynToWeightMapping {
    pub syn_type: String,
    pub syn_sample: String,
    pub llm_label: String,
    pub weight_activations: HashMap<String, f64>,
    pub trace_signature: String,
}

pub struct OllamaIntrospector {
    pub traces: Vec<LlmTrace>,
    pub mappings: Vec<SynToWeightMapping>,
}

impl OllamaIntrospector {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            mappings: Vec::new(),
        }
    }
    
    pub fn trace_ollama(&mut self, prompt: &str) -> Option<LlmTrace> {
        // Call ollama with tracing enabled
        let output = Command::new("ollama")
            .args(&["run", "codellama", prompt])
            .env("OLLAMA_DEBUG", "1")
            .output()
            .ok()?;
        
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        let debug_info = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Parse trace information from debug output
        let mut weights_accessed = Vec::new();
        let mut execution_path = Vec::new();
        
        for line in debug_info.lines() {
            // Extract weight access patterns
            if line.contains("weight") || line.contains("layer") {
                // Parse weight name and activation value
                // Format: "layer.0.weight: 0.123"
                if let Some((name, value)) = line.split_once(':') {
                    if let Ok(val) = value.trim().parse::<f64>() {
                        weights_accessed.push((name.trim().to_string(), val));
                    }
                }
            }
            
            // Extract execution path (instruction pointers)
            if line.contains("0x") {
                if let Some(hex) = line.split_whitespace()
                    .find(|s| s.starts_with("0x")) {
                    if let Ok(ip) = u64::from_str_radix(&hex[2..], 16) {
                        execution_path.push(ip);
                    }
                }
            }
        }
        
        let trace = LlmTrace {
            prompt: prompt.to_string(),
            response: response.clone(),
            tokens: response.split_whitespace().map(String::from).collect(),
            weights_accessed,
            execution_path,
        };
        
        self.traces.push(trace.clone());
        Some(trace)
    }
    
    pub fn map_syn_to_weights(&mut self, syn_type: &str, syn_sample: &str) -> Option<SynToWeightMapping> {
        // Ask LLM to label the syn code
        let prompt = format!("Analyze this Rust {} code and describe it: {}", syn_type, syn_sample);
        
        let trace = self.trace_ollama(&prompt)?;
        
        // Extract which weights were most activated
        let mut weight_activations = HashMap::new();
        for (weight_name, activation) in &trace.weights_accessed {
            *weight_activations.entry(weight_name.clone()).or_insert(0.0) += activation;
        }
        
        // Create signature from execution path
        let signature = format!("{:x}", trace.execution_path.iter().sum::<u64>());
        
        let mapping = SynToWeightMapping {
            syn_type: syn_type.to_string(),
            syn_sample: syn_sample.to_string(),
            llm_label: trace.response.clone(),
            weight_activations,
            trace_signature: signature,
        };
        
        self.mappings.push(mapping.clone());
        Some(mapping)
    }
    
    pub fn report(&self) {
        println!("\n📊 Ollama Introspector Report");
        println!("  Total traces: {}", self.traces.len());
        println!("  Syn → Weight mappings: {}", self.mappings.len());
        
        if !self.mappings.is_empty() {
            println!("\n  Sample mappings:");
            for mapping in self.mappings.iter().take(5) {
                println!("    {} → {} weights activated", 
                         mapping.syn_type, 
                         mapping.weight_activations.len());
                println!("      Label: {}", 
                         &mapping.llm_label[..50.min(mapping.llm_label.len())]);
            }
        }
    }
}
