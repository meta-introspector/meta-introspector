// Vector embed code snippets using HuggingFace tokenizers

use std::process::Command;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CodeEmbedding {
    pub snippet: String,
    pub syn_type: String,
    pub tokens: Vec<String>,
    pub token_ids: Vec<u32>,
    pub embedding: Vec<f32>,
    pub embedding_dim: usize,
}

pub struct EmbeddingEngine {
    pub model_name: String,
    pub embeddings: Vec<CodeEmbedding>,
    pub embedding_index: HashMap<String, Vec<usize>>,
}

impl EmbeddingEngine {
    pub fn new(model_name: String) -> Self {
        Self {
            model_name,
            embeddings: Vec::new(),
            embedding_index: HashMap::new(),
        }
    }
    
    pub fn tokenize(&self, snippet: &str) -> Option<(Vec<String>, Vec<u32>)> {
        // Use transformers tokenizer
        let output = Command::new("python3")
            .arg("-c")
            .arg(format!(
                "from transformers import AutoTokenizer; \
                 tokenizer = AutoTokenizer.from_pretrained('{}'); \
                 result = tokenizer('{}'); \
                 print(result['input_ids'])",
                self.model_name,
                snippet.replace("'", "\\'")
            ))
            .output()
            .ok()?;
        
        let result = String::from_utf8_lossy(&output.stdout);
        
        // Parse token IDs
        let token_ids: Vec<u32> = result
            .trim()
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        
        // Mock tokens for now
        let tokens: Vec<String> = token_ids.iter()
            .map(|id| format!("tok_{}", id))
            .collect();
        
        Some((tokens, token_ids))
    }
    
    pub fn embed(&self, snippet: &str) -> Option<Vec<f32>> {
        // Use sentence-transformers for embeddings
        let output = Command::new("python3")
            .arg("-c")
            .arg(format!(
                "from sentence_transformers import SentenceTransformer; \
                 model = SentenceTransformer('{}'); \
                 embedding = model.encode('{}'); \
                 print(embedding.tolist())",
                self.model_name,
                snippet.replace("'", "\\'")
            ))
            .output()
            .ok()?;
        
        let result = String::from_utf8_lossy(&output.stdout);
        
        // Parse embedding vector
        let embedding: Vec<f32> = result
            .trim()
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        
        Some(embedding)
    }
    
    pub fn embed_snippet(&mut self, snippet: &str, syn_type: &str) -> Option<CodeEmbedding> {
        let (tokens, token_ids) = self.tokenize(snippet)?;
        let embedding = self.embed(snippet)?;
        
        let code_embedding = CodeEmbedding {
            snippet: snippet.to_string(),
            syn_type: syn_type.to_string(),
            tokens,
            token_ids,
            embedding: embedding.clone(),
            embedding_dim: embedding.len(),
        };
        
        // Index by syn type
        let idx = self.embeddings.len();
        self.embedding_index
            .entry(syn_type.to_string())
            .or_insert_with(Vec::new)
            .push(idx);
        
        self.embeddings.push(code_embedding.clone());
        
        Some(code_embedding)
    }
    
    pub fn cosine_similarity(&self, emb1: &[f32], emb2: &[f32]) -> f32 {
        if emb1.len() != emb2.len() {
            return 0.0;
        }
        
        let dot: f32 = emb1.iter().zip(emb2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = emb1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = emb2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot / (norm1 * norm2)
        }
    }
    
    pub fn find_similar(&self, embedding: &[f32], top_k: usize) -> Vec<(usize, f32)> {
        let mut similarities: Vec<(usize, f32)> = self.embeddings
            .iter()
            .enumerate()
            .map(|(idx, emb)| {
                let sim = self.cosine_similarity(embedding, &emb.embedding);
                (idx, sim)
            })
            .collect();
        
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.into_iter().take(top_k).collect()
    }
    
    pub fn report(&self) {
        println!("\n📊 Embedding Engine Report");
        println!("  Model: {}", self.model_name);
        println!("  Total embeddings: {}", self.embeddings.len());
        
        if !self.embeddings.is_empty() {
            println!("  Embedding dimension: {}", self.embeddings[0].embedding_dim);
        }
        
        println!("\n  Embeddings by syn type:");
        for (syn_type, indices) in &self.embedding_index {
            println!("    {}: {} embeddings", syn_type, indices.len());
        }
    }
}
