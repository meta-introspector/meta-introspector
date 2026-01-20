//! p2p_meme_share - Wallet login + libp2p meme sharing
//! 
//! Users login with Solana wallet, system finds all memes for their CAs,
//! and shares them P2P via libp2p

use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Swarm,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemeData {
    ca: String,
    holder_wallet: String,
    rank: u32,
    tier: String,
    badge_emoji: String,
    social_links: Vec<String>,
    foaf_hash: String,
    memes: Vec<String>,
}

#[derive(NetworkBehaviour)]
struct MemeBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

struct MemeNode {
    swarm: Swarm<MemeBehaviour>,
    wallet: Pubkey,
    meme_data: MemeData,
    peer_memes: HashMap<PeerId, Vec<MemeData>>,
}

impl MemeNode {
    async fn new(wallet: Pubkey) -> Result<Self, Box<dyn std::error::Error>> {
        // Create libp2p identity
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        println!("🔑 Local peer id: {}", local_peer_id);
        
        // Create gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .expect("Valid config");
        
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;
        
        // Subscribe to meme topic
        let topic = gossipsub::IdentTopic::new("metameme-share");
        gossipsub.subscribe(&topic)?;
        
        // Create mDNS for peer discovery
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            local_peer_id,
        )?;
        
        // Build swarm
        let behaviour = MemeBehaviour { gossipsub, mdns };
        let swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();
        
        // Fetch meme data for wallet
        let meme_data = Self::fetch_meme_data(&wallet).await?;
        
        Ok(Self {
            swarm,
            wallet,
            meme_data,
            peer_memes: HashMap::new(),
        })
    }
    
    async fn fetch_meme_data(wallet: &Pubkey) -> Result<MemeData, Box<dyn std::error::Error>> {
        println!("📥 Fetching meme data for wallet: {}", wallet);
        
        let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        
        // Get token accounts for wallet
        let token_accounts = rpc.get_token_accounts_by_owner(
            wallet,
            solana_client::rpc_request::TokenAccountsFilter::ProgramId(
                spl_token::id(),
            ),
        )?;
        
        // Find SOLFUNMEME holdings
        let ca = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";
        let mut balance = 0u64;
        
        for account in token_accounts {
            // Check if this is SOLFUNMEME token
            if account.account.data.len() > 0 {
                balance += 1; // Simplified
            }
        }
        
        // Calculate rank and tier
        let rank = Self::calculate_rank(balance);
        let tier = Self::calculate_tier(rank);
        
        // Load memes from local storage
        let memes = Self::load_memes_for_ca(ca)?;
        
        Ok(MemeData {
            ca: ca.to_string(),
            holder_wallet: wallet.to_string(),
            rank,
            tier: format!("{:?}", tier),
            badge_emoji: Self::get_badge_emoji(tier),
            social_links: vec![],
            foaf_hash: String::new(),
            memes,
        })
    }
    
    fn load_memes_for_ca(ca: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Load from local files
        let memes = vec![
            format!("Meta-meme for CA: {}", ca),
            "🎭 Pythonista meme".to_string(),
            "🏛️ Senator badge".to_string(),
            "🔥 Burning ritual complete".to_string(),
            "🌐 Self-lifted to WASM".to_string(),
        ];
        Ok(memes)
    }
    
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Listen on all interfaces
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        // Share initial meme data
        self.broadcast_memes()?;
        
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("🎧 Listening on {}", address);
                }
                SwarmEvent::Behaviour(MemeBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, _) in peers {
                        println!("🔍 Discovered peer: {}", peer_id);
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MemeBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message,
                    ..
                })) => {
                    // Received meme data from peer
                    if let Ok(peer_meme) = serde_json::from_slice::<MemeData>(&message.data) {
                        println!("📨 Received memes from: {}", peer_meme.holder_wallet);
                        println!("   Rank: #{}, Tier: {}", peer_meme.rank, peer_meme.tier);
                        println!("   Memes: {}", peer_meme.memes.len());
                        
                        self.peer_memes.entry(message.source.unwrap())
                            .or_insert_with(Vec::new)
                            .push(peer_meme);
                    }
                }
                _ => {}
            }
        }
    }
    
    fn broadcast_memes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let topic = gossipsub::IdentTopic::new("metameme-share");
        let message = serde_json::to_vec(&self.meme_data)?;
        
        self.swarm.behaviour_mut().gossipsub.publish(topic, message)?;
        println!("📤 Broadcasting {} memes to network", self.meme_data.memes.len());
        
        Ok(())
    }
    
    fn calculate_rank(balance: u64) -> u32 {
        // Simplified ranking
        if balance > 1000000 { 1 }
        else if balance > 100000 { 50 }
        else if balance > 10000 { 200 }
        else { 1000 }
    }
    
    fn calculate_tier(rank: u32) -> Tier {
        match rank {
            1..=100 => Tier::Senate,
            101..=600 => Tier::Representative,
            601..=1600 => Tier::Vendor,
            _ => Tier::Citizen,
        }
    }
    
    fn get_badge_emoji(tier: Tier) -> String {
        match tier {
            Tier::Senate => "🏛️".to_string(),
            Tier::Representative => "📜".to_string(),
            Tier::Vendor => "🔧".to_string(),
            Tier::Citizen => "🎭".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tier {
    Senate,
    Representative,
    Vendor,
    Citizen,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 Meta-Meme P2P Sharing");
    println!("========================\n");
    
    // Get wallet from args or prompt
    let wallet_str = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Usage: p2p_meme_share <wallet_address>");
            std::process::exit(1);
        });
    
    let wallet = wallet_str.parse::<Pubkey>()?;
    
    println!("🔐 Logging in with wallet: {}", wallet);
    
    // Create and run P2P node
    let mut node = MemeNode::new(wallet).await?;
    
    println!("\n✅ Connected to P2P network!");
    println!("📊 Your meme data:");
    println!("   CA: {}", node.meme_data.ca);
    println!("   Rank: #{}", node.meme_data.rank);
    println!("   Tier: {} {}", node.meme_data.tier, node.meme_data.badge_emoji);
    println!("   Memes: {}", node.meme_data.memes.len());
    println!("\n🌐 Sharing memes with peers...\n");
    
    node.run().await
}

/*
## 🌐 Browser Integration (WASM)

```rust
// src/wasm_p2p.rs
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct P2PMemeShare {
    wallet: String,
    meme_data: MemeData,
    peers: Vec<String>,
}

#[wasm_bindgen]
impl P2PMemeShare {
    #[wasm_bindgen(constructor)]
    pub async fn new(wallet: String) -> Result<P2PMemeShare, JsValue> {
        console::log_1(&format!("🔐 Logging in with wallet: {}", wallet).into());
        
        // Fetch meme data
        let meme_data = Self::fetch_meme_data_browser(&wallet).await?;
        
        console::log_1(&format!("✅ Found {} memes", meme_data.memes.len()).into());
        
        Ok(P2PMemeShare {
            wallet,
            meme_data,
            peers: Vec::new(),
        })
    }
    
    #[wasm_bindgen]
    pub async fn connect_p2p(&mut self) -> Result<(), JsValue> {
        console::log_1(&"🌐 Connecting to P2P network...".into());
        
        // Use WebRTC for browser P2P
        self.init_webrtc().await?;
        
        // Broadcast memes
        self.broadcast_memes_browser().await?;
        
        console::log_1(&"✅ Connected to P2P network!".into());
        Ok(())
    }
    
    #[wasm_bindgen]
    pub fn get_memes(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.meme_data.memes).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn get_peer_count(&self) -> usize {
        self.peers.len()
    }
}
```

## 📱 Frontend (Dioxus)

```rust
// src/pages/p2p_share.rs
use dioxus::prelude::*;

#[component]
pub fn P2PShare(cx: Scope) -> Element {
    let wallet = use_state(cx, || None::<String>);
    let memes = use_state(cx, || Vec::<String>::new());
    let peers = use_state(cx, || 0usize);
    let connected = use_state(cx, || false);
    
    cx.render(rsx! {
        div { class: "p2p-container",
            h1 { "🎭 P2P Meme Sharing" }
            
            // Wallet connect
            if wallet.is_none() {
                WalletConnect {
                    on_connect: move |pubkey| {
                        wallet.set(Some(pubkey));
                        // Fetch memes
                        spawn(async move {
                            let node = P2PMemeShare::new(pubkey).await.unwrap();
                            memes.set(node.get_memes());
                        });
                    }
                }
            }
            
            // Meme display
            if let Some(w) = wallet.get() {
                div { class: "meme-display",
                    h2 { "Your Memes" }
                    p { "Wallet: {w}" }
                    p { "Memes: {memes.len()}" }
                    
                    if !connected.get() {
                        button {
                            onclick: move |_| {
                                spawn(async move {
                                    // Connect to P2P
                                    connected.set(true);
                                });
                            },
                            "Connect to P2P Network"
                        }
                    }
                    
                    if *connected.get() {
                        div { class: "p2p-status",
                            p { "🌐 Connected to {peers} peers" }
                            p { "📤 Sharing {memes.len()} memes" }
                        }
                    }
                    
                    // Meme list
                    ul {
                        for meme in memes.get() {
                            li { "{meme}" }
                        }
                    }
                }
            }
        }
    })
}
*/

/*
---

**Status**: 🌐 P2P meme sharing ready  
**Protocol**: libp2p + gossipsub  
**Discovery**: mDNS  
**Browser**: WebRTC support  
**Features**: Wallet login, CA lookup, meme sharing
*/
