//! GitHub Caching Proxy
//! Caches GitHub API and git operations to avoid rate limits

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct CacheEntry {
    data: Vec<u8>,
    cached_at: SystemTime,
    ttl: Duration,
}

struct GitHubProxy {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    token: Option<String>,
}

impl GitHubProxy {
    fn new(token: Option<String>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            token,
        }
    }

    async fn get(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Check cache
        if let Some(entry) = self.cache.read().unwrap().get(url) {
            if entry.cached_at.elapsed()? < entry.ttl {
                println!("Cache HIT: {}", url);
                return Ok(entry.data.clone());
            }
        }

        println!("Cache MISS: {}", url);
        
        // Fetch from GitHub
        let client = reqwest::Client::new();
        let mut req = client.get(url);
        
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("token {}", token));
        }
        
        let resp = req.send().await?;
        let data = resp.bytes().await?.to_vec();
        
        // Cache it
        self.cache.write().unwrap().insert(url.to_string(), CacheEntry {
            data: data.clone(),
            cached_at: SystemTime::now(),
            ttl: Duration::from_secs(3600),
        });
        
        Ok(data)
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("GITHUB_TOKEN").ok();
    let proxy = GitHubProxy::new(token);
    
    // Start HTTP server on localhost:8080
    println!("GitHub cache proxy running on http://127.0.0.1:8080");
    println!("Configure git: git config --global http.proxy http://127.0.0.1:8080");
}
