//! P2P GitHub Mirror Network
//! Distributed git object sharing with rate limit coordination

use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Swarm,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum GitMessage {
    RequestObject { repo: String, sha: String },
    ObjectData { repo: String, sha: String, data: Vec<u8> },
    RateLimitStatus { remaining: u32, reset_at: u64 },
    CacheHit { repo: String, sha: String, peer_id: String },
}

#[derive(NetworkBehaviour)]
struct GitP2PBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

struct P2PGitMirror {
    swarm: Swarm<GitP2PBehaviour>,
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    rate_limits: Arc<RwLock<HashMap<String, RateLimit>>>,
}

#[derive(Clone)]
struct RateLimit {
    remaining: u32,
    reset_at: u64,
}

impl P2PGitMirror {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        println!("🆔 Local peer id: {}", local_peer_id);

        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .build()?;
        
        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        let behaviour = GitP2PBehaviour { gossipsub, mdns };
        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id).build();

        Ok(Self {
            swarm,
            cache: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn request_object(&mut self, repo: &str, sha: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key = format!("{}:{}", repo, sha);
        
        // Check local cache
        if let Some(data) = self.cache.read().await.get(&key) {
            println!("💾 Cache HIT: {}", key);
            return Ok(data.clone());
        }

        // Try to read from local pack (no checkout)
        if let Ok(data) = self.read_from_pack(repo, sha).await {
            println!("📦 Pack HIT: {}", key);
            self.cache.write().await.insert(key.clone(), data.clone());
            return Ok(data);
        }

        // Check if we can fetch (rate limit)
        if !self.can_fetch(repo).await {
            println!("⏳ Rate limited, requesting from peers...");
            return self.request_from_peers(repo, sha).await;
        }

        // Fetch from GitHub
        println!("🌐 Fetching from GitHub: {}", key);
        let data = self.fetch_from_github(repo, sha).await?;
        
        // Cache it
        self.cache.write().await.insert(key.clone(), data.clone());
        
        // Broadcast cache hit to peers
        self.broadcast_cache_hit(repo, sha).await?;
        
        Ok(data)
    }

    async fn read_from_pack(&self, repo: &str, sha: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use git-sources canonical path
        let output = std::process::Command::new("./target/release/git-sources")
            .args(&["list"])
            .output()?;
        
        let registry = String::from_utf8_lossy(&output.stdout);
        
        // Find canonical path for this repo
        let mut canonical_path = None;
        for line in registry.lines() {
            if line.contains(&format!("URL: {}", repo)) {
                // Next line should be Path:
                canonical_path = Some(line.trim().to_string());
            }
            if line.contains("Path:") && canonical_path.is_some() {
                canonical_path = Some(line.split("Path:").nth(1).unwrap().trim().to_string());
                break;
            }
        }
        
        let path = canonical_path.ok_or("Repo not in registry")?;
        
        // Read from pack
        let output = tokio::process::Command::new("git")
            .args(&["-C", &path, "cat-file", "blob", sha])
            .output()
            .await?;
        
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err("Not in pack".into())
        }
    }

    async fn can_fetch(&self, repo: &str) -> bool {
        let limits = self.rate_limits.read().await;
        if let Some(limit) = limits.get(repo) {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            limit.remaining > 0 || now > limit.reset_at
        } else {
            true
        }
    }

    async fn request_from_peers(&mut self, repo: &str, sha: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let msg = GitMessage::RequestObject {
            repo: repo.to_string(),
            sha: sha.to_string(),
        };
        
        let topic = gossipsub::IdentTopic::new("git-objects");
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        
        let json = serde_json::to_vec(&msg)?;
        self.swarm.behaviour_mut().gossipsub.publish(topic, json)?;
        
        // Wait for response (simplified - should use proper async channel)
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        let key = format!("{}:{}", repo, sha);
        if let Some(data) = self.cache.read().await.get(&key) {
            Ok(data.clone())
        } else {
            Err("No peer had the object".into())
        }
    }

    async fn fetch_from_github(&self, repo: &str, sha: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use octocrab with rate limit tracking
        let octocrab = octocrab::instance();
        
        let parts: Vec<_> = repo.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid repo format".into());
        }
        
        let (owner, repo_name) = (parts[0], parts[1]);
        
        // Fetch blob via API
        let blob = octocrab
            .repos(owner, repo_name)
            .get_content()
            .path(&format!("blob/{}", sha))
            .send()
            .await?;
        
        // Update rate limits from response headers
        self.update_rate_limits(repo).await?;
        
        Ok(blob.content.unwrap_or_default().into_bytes())
    }
    
    async fn update_rate_limits(&self, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
        let octocrab = octocrab::instance();
        let rate_limit = octocrab.ratelimit().get().await?;
        
        let remaining = rate_limit.resources.core.remaining as u32;
        let reset_at = rate_limit.resources.core.reset as u64;
        
        self.rate_limits.write().await.insert(repo.to_string(), RateLimit {
            remaining,
            reset_at,
        });
        
        // Broadcast to P2P network
        self.broadcast_rate_limit(repo, remaining, reset_at).await?;
        
        Ok(())
    }

    async fn broadcast_cache_hit(&mut self, repo: &str, sha: &str) -> Result<(), Box<dyn std::error::Error>> {
        let msg = GitMessage::CacheHit {
            repo: repo.to_string(),
            sha: sha.to_string(),
            peer_id: self.swarm.local_peer_id().to_string(),
        };
        
        let topic = gossipsub::IdentTopic::new("git-cache");
        let json = serde_json::to_vec(&msg)?;
        self.swarm.behaviour_mut().gossipsub.publish(topic, json)?;
        
        Ok(())
    }

    async fn broadcast_rate_limit(&self, repo: &str, remaining: u32, reset_at: u64) -> Result<(), Box<dyn std::error::Error>> {
        let msg = GitMessage::RateLimitStatus {
            remaining,
            reset_at,
        };
        
        println!("📊 Rate limit: {} remaining, resets at {}", remaining, reset_at);
        Ok(())
    }

    async fn handle_message(&mut self, msg: GitMessage) -> Result<(), Box<dyn std::error::Error>> {
        match msg {
            GitMessage::RequestObject { repo, sha } => {
                let key = format!("{}:{}", repo, sha);
                if let Some(data) = self.cache.read().await.get(&key) {
                    println!("📤 Serving {} to peer", key);
                    // Send ObjectData response
                }
            }
            GitMessage::ObjectData { repo, sha, data } => {
                let key = format!("{}:{}", repo, sha);
                self.cache.write().await.insert(key, data);
                println!("📥 Received object from peer");
            }
            GitMessage::RateLimitStatus { remaining, reset_at } => {
                println!("⚠️  Peer rate limit: {} remaining", remaining);
            }
            GitMessage::CacheHit { repo, sha, peer_id } => {
                println!("✅ Peer {} has {}:{}", peer_id, repo, sha);
            }
        }
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        let topic = gossipsub::IdentTopic::new("git-objects");
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("🎧 Listening on {}", address);
                }
                SwarmEvent::Behaviour(event) => {
                    // Handle gossipsub/mdns events
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 P2P GitHub Mirror Network");
    
    let mut mirror = P2PGitMirror::new().await?;
    
    // Example: Request an object
    let data = mirror.request_object("meta-introspector/meta-introspector", "abc123").await?;
    println!("✅ Got {} bytes", data.len());
    
    // Run event loop
    mirror.run().await?;
    
    Ok(())
}
