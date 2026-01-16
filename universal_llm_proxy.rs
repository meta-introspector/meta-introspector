// Universal LLM Proxy with Telemetry and Nix Store Integration
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Gemini { model: String, api_key: String },
    OpenAI { model: String, api_key: String },
    Anthropic { model: String, api_key: String },
    Ollama { model: String, endpoint: String },
    DeepSeek { model: String, api_key: String },
    Mistral { model: String, api_key: String },
    Local { model_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub provider: LLMProvider,
    pub prompt: String,
    pub context: Vec<String>,
    pub temperature: f64,
    pub max_tokens: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub provider: String,
    pub model: String,
    pub response: String,
    pub tokens_used: usize,
    pub latency_ms: u64,
    pub cost_usd: f64,
    pub timestamp: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryRecord {
    pub request: LLMRequest,
    pub response: LLMResponse,
    pub nix_store_path: Option<String>,
    pub parquet_path: Option<String>,
    pub huggingface_dataset: Option<String>,
}

pub struct UniversalLLMProxy {
    pub telemetry: Vec<TelemetryRecord>,
    pub nix_store_root: PathBuf,
    pub parquet_output: PathBuf,
}

impl UniversalLLMProxy {
    pub fn new(nix_store_root: PathBuf, parquet_output: PathBuf) -> Self {
        Self {
            telemetry: Vec::new(),
            nix_store_root,
            parquet_output,
        }
    }
    
    pub async fn query(&mut self, request: LLMRequest) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        // Route to appropriate provider
        let response = match &request.provider {
            LLMProvider::Gemini { model, api_key } => {
                self.query_gemini(model, api_key, &request.prompt).await?
            }
            LLMProvider::OpenAI { model, api_key } => {
                self.query_openai(model, api_key, &request.prompt).await?
            }
            LLMProvider::Anthropic { model, api_key } => {
                self.query_anthropic(model, api_key, &request.prompt).await?
            }
            LLMProvider::Ollama { model, endpoint } => {
                self.query_ollama(model, endpoint, &request.prompt).await?
            }
            LLMProvider::DeepSeek { model, api_key } => {
                self.query_deepseek(model, api_key, &request.prompt).await?
            }
            LLMProvider::Mistral { model, api_key } => {
                self.query_mistral(model, api_key, &request.prompt).await?
            }
            LLMProvider::Local { model_path } => {
                self.query_local(model_path, &request.prompt).await?
            }
        };
        
        let latency = start.elapsed().as_millis() as u64;
        
        // Create response with telemetry
        let llm_response = LLMResponse {
            provider: self.provider_name(&request.provider),
            model: self.model_name(&request.provider),
            response,
            tokens_used: 0, // TODO: Calculate from response
            latency_ms: latency,
            cost_usd: self.calculate_cost(&request.provider, 0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            content_hash: self.hash_content(&request.prompt),
        };
        
        // Store in nix store
        let nix_path = self.store_in_nix(&request, &llm_response).await?;
        
        // Save to parquet
        let parquet_path = self.save_to_parquet(&request, &llm_response).await?;
        
        // Record telemetry
        self.telemetry.push(TelemetryRecord {
            request,
            response: llm_response.clone(),
            nix_store_path: Some(nix_path),
            parquet_path: Some(parquet_path),
            huggingface_dataset: Some("introspector/rust/llm-telemetry".to_string()),
        });
        
        Ok(llm_response)
    }
    
    async fn query_gemini(&self, model: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Call Gemini API
        let client = reqwest::Client::new();
        let response = client
            .post(format!("https://generativelanguage.googleapis.com/v1/models/{}:generateContent", model))
            .header("x-goog-api-key", api_key)
            .json(&serde_json::json!({
                "contents": [{"parts": [{"text": prompt}]}]
            }))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
    
    async fn query_openai(&self, model: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "model": model,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
    
    async fn query_anthropic(&self, model: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 1024
            }))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
    
    async fn query_ollama(&self, model: &str, endpoint: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/api/generate", endpoint))
            .json(&serde_json::json!({
                "model": model,
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["response"].as_str().unwrap_or("").to_string())
    }
    
    async fn query_deepseek(&self, model: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // DeepSeek API (OpenAI-compatible)
        self.query_openai(model, api_key, prompt).await
    }
    
    async fn query_mistral(&self, model: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "model": model,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
    
    async fn query_local(&self, model_path: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Call local model via llama.cpp or similar
        let output = std::process::Command::new("llama-cli")
            .arg("-m")
            .arg(model_path)
            .arg("-p")
            .arg(prompt)
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    async fn store_in_nix(&self, request: &LLMRequest, response: &LLMResponse) -> Result<String, Box<dyn std::error::Error>> {
        // Store as content-addressable in nix store
        let hash = &response.content_hash;
        let nix_path = self.nix_store_root.join(&hash[..2]).join(hash);
        
        std::fs::create_dir_all(nix_path.parent().unwrap())?;
        
        let data = serde_json::json!({
            "request": request,
            "response": response,
        });
        
        std::fs::write(&nix_path, serde_json::to_string_pretty(&data)?)?;
        
        Ok(nix_path.to_string_lossy().to_string())
    }
    
    async fn save_to_parquet(&self, request: &LLMRequest, response: &LLMResponse) -> Result<String, Box<dyn std::error::Error>> {
        // Save to parquet for HuggingFace
        let parquet_path = self.parquet_output.join(format!("{}.parquet", response.content_hash));
        
        // TODO: Use arrow/parquet crate to write
        // For now, save as JSON
        let data = serde_json::json!({
            "provider": response.provider,
            "model": response.model,
            "prompt": request.prompt,
            "response": response.response,
            "tokens": response.tokens_used,
            "latency_ms": response.latency_ms,
            "cost_usd": response.cost_usd,
            "timestamp": response.timestamp,
            "hash": response.content_hash,
        });
        
        std::fs::write(&parquet_path, serde_json::to_string_pretty(&data)?)?;
        
        Ok(parquet_path.to_string_lossy().to_string())
    }
    
    fn provider_name(&self, provider: &LLMProvider) -> String {
        match provider {
            LLMProvider::Gemini { .. } => "gemini".to_string(),
            LLMProvider::OpenAI { .. } => "openai".to_string(),
            LLMProvider::Anthropic { .. } => "anthropic".to_string(),
            LLMProvider::Ollama { .. } => "ollama".to_string(),
            LLMProvider::DeepSeek { .. } => "deepseek".to_string(),
            LLMProvider::Mistral { .. } => "mistral".to_string(),
            LLMProvider::Local { .. } => "local".to_string(),
        }
    }
    
    fn model_name(&self, provider: &LLMProvider) -> String {
        match provider {
            LLMProvider::Gemini { model, .. } => model.clone(),
            LLMProvider::OpenAI { model, .. } => model.clone(),
            LLMProvider::Anthropic { model, .. } => model.clone(),
            LLMProvider::Ollama { model, .. } => model.clone(),
            LLMProvider::DeepSeek { model, .. } => model.clone(),
            LLMProvider::Mistral { model, .. } => model.clone(),
            LLMProvider::Local { model_path } => model_path.clone(),
        }
    }
    
    fn calculate_cost(&self, provider: &LLMProvider, tokens: usize) -> f64 {
        // Rough cost estimates per 1M tokens
        let cost_per_million = match provider {
            LLMProvider::Gemini { .. } => 0.50,
            LLMProvider::OpenAI { .. } => 2.00,
            LLMProvider::Anthropic { .. } => 3.00,
            LLMProvider::Ollama { .. } => 0.0,
            LLMProvider::DeepSeek { .. } => 0.14,
            LLMProvider::Mistral { .. } => 0.25,
            LLMProvider::Local { .. } => 0.0,
        };
        
        (tokens as f64 / 1_000_000.0) * cost_per_million
    }
    
    fn hash_content(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    pub async fn export_to_huggingface(&self, dataset: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📤 Exporting {} telemetry records to {}", self.telemetry.len(), dataset);
        
        // Export all parquet files to HuggingFace
        for record in &self.telemetry {
            if let Some(parquet_path) = &record.parquet_path {
                println!("  ✓ {}", parquet_path);
            }
        }
        
        println!("✅ Ready to upload to HuggingFace dataset: {}", dataset);
        Ok(())
    }
}
