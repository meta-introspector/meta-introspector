// Trading node: Single account on dedicated CPU with parquet storage
// Communicates with peers via HTTP

use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}, Json, extract::State};
use serde::{Deserialize, Serialize};

// Import from shared_memory_bus
// use crate::shared_memory_bus::{Portfolio, Meme};
// use shared_memory_bus::{Portfolio, Meme};

// Stub types from shared_memory_bus
#[derive(Clone, Debug)]
struct Portfolio {
    memes: Vec<Meme>,
    balance: u64,
    memory_used: usize,
    memory_limit: usize,
    score: f64,
    trades: usize,
}

impl Portfolio {
    fn new() -> Self {
        Portfolio {
            memes: Vec::new(),
            balance: 0,
            memory_used: 0,
            memory_limit: 1000,
            score: 0.0,
            trades: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Meme {
    id: u64,
    fitness: f64,
    complexity: f64,
    rarity: f64,
    code: String,
    emoji: String,
    godel_number: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TradeOffer {
    id: u64,
    meme_id: u64,
    price: u64,
    from_node: String,
    to_node: String,
    offer_meme: u64,
    want_meme: u64,
    score_improvement: f64,
}

fn simulate_trade_score(_p1: &Portfolio, _p2: &Portfolio, _offer: &TradeOffer) -> f64 { 0.0 }

#[derive(Parser, Debug)]
#[command(name = "trading_node")]
struct Args {
    #[arg(long)]
    node_id: usize,
    
    #[arg(long)]
    port: u16,
    
    #[arg(long)]
    parquet: String,
    
    #[arg(long)]
    peers: String,  // Comma-separated ports
}

#[derive(Clone)]
struct NodeState {
    node_id: usize,
    portfolio: Arc<Mutex<Portfolio>>,
    parquet_path: String,
    peers: Vec<u16>,
    ram_limit_mb: usize,
}

impl NodeState {
    fn new(node_id: usize, parquet_path: String, peers: Vec<u16>) -> Self {
        // Load or create portfolio
        let portfolio = if std::path::Path::new(&parquet_path).exists() {
            Portfolio::load_from_parquet(&parquet_path).unwrap()
        } else {
            Portfolio::new(node_id, 10) // 10 initial memes
        };
        
        Self {
            node_id,
            portfolio: Arc::new(Mutex::new(portfolio)),
            parquet_path,
            peers,
            ram_limit_mb: 2600, // ~2.6 GB
        }
    }
    
    fn save(&self) {
        let portfolio = self.portfolio.lock().unwrap();
        portfolio.save_to_parquet(&self.parquet_path).ok();
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("🚀 Trading Node {}", args.node_id);
    println!("📊 Port: {}", args.port);
    println!("💾 Parquet: {}", args.parquet);
    println!("🌐 Peers: {}", args.peers);
    
    // Parse peers
    let peers: Vec<u16> = args.peers
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Create state
    let state = NodeState::new(args.node_id, args.parquet, peers);
    
    // Build router
    let app = Router::new()
        .route("/status", get(status))
        .route("/portfolio", get(get_portfolio))
        .route("/trade/offer", post(receive_trade_offer))
        .route("/trade/accept", post(accept_trade))
        .with_state(state.clone());
    
    // Start trading loop in background
    let state_clone = state.clone();
    tokio::spawn(async move {
        trading_loop(state_clone).await;
    });
    
    // Start server
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("✅ Node {} listening on {}", args.node_id, addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn trading_loop(state: NodeState) {
    let mut round = 0;
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        round += 1;
        println!("\n📊 Round {} - Node {}", round, state.node_id);
        
        // Find trading opportunities
        for &peer_port in &state.peers {
            if peer_port == 8000 + state.node_id as u16 {
                continue; // Skip self
            }
            
            // Try to trade with peer
            if let Err(e) = try_trade_with_peer(&state, peer_port).await {
                // Peer might not be ready yet
            }
        }
        
        // Save state
        state.save();
        
        // Report
        let portfolio = state.portfolio.lock().unwrap();
        println!("  Score: {:.2}, Trades: {}", portfolio.score, portfolio.trades);
    }
}

async fn try_trade_with_peer(state: &NodeState, peer_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Enable when reqwest is in main Cargo.toml
    // Preserved for future implementation
    
    /*
    // Get peer's portfolio
    let peer_url = format!("http://localhost:{}/portfolio", peer_port);
    let client = reqwest::Client::new();
    let peer_portfolio: Portfolio = client.get(&peer_url).send().await?.json().await?;
    
    // Find best trade
    let my_portfolio = state.portfolio.lock().unwrap();
    
    if let Some(offer) = find_best_trade_local(&my_portfolio, &peer_portfolio) {
        drop(my_portfolio);
        
        // Send trade offer
        let offer_url = format!("http://localhost:{}/trade/offer", peer_port);
        let response: TradeResponse = client.post(&offer_url)
            .json(&offer)
            .send().await?
            .json().await?;
        
        if response.accepted {
            // Execute trade locally
            let mut my_portfolio = state.portfolio.lock().unwrap();
            execute_trade_local(&mut my_portfolio, &offer);
            println!("  ✅ Trade executed with node on port {}", peer_port);
        }
    }
    */
    
    // STUB: Simplified version for now
    println!("  [STUB] Would query peer on port {} and attempt trade", peer_port);
    
    Ok(())
}

fn find_best_trade_local(my: &Portfolio, their: &Portfolio) -> Option<TradeOffer> {
    let mut best_offer = None;
    let mut best_improvement = 0.0;
    
    for my_meme in &my.memes {
        for their_meme in &their.memes {
            let my_score_after = simulate_trade_score(&my.memes, my_meme.id, their_meme);
            let their_score_after = simulate_trade_score(&their.memes, their_meme.id, my_meme);
            
            let my_improvement = my_score_after - my.score;
            let their_improvement = their_score_after - their.score;
            
            if my_improvement > 0.0 && their_improvement > 0.0 {
                let total = my_improvement + their_improvement;
                if total > best_improvement {
                    best_improvement = total;
                    best_offer = Some(TradeOffer {
                        from_node: my.node_id,
                        to_node: their.node_id,
                        offer_meme: my_meme.id,
                        want_meme: their_meme.id,
                        score_improvement: total,
                    });
                }
            }
        }
    }
    
    best_offer
}

fn execute_trade_local(portfolio: &mut Portfolio, offer: &TradeOffer) {
    if let Some(idx) = portfolio.memes.iter().position(|m| m.id == offer.offer_meme) {
        portfolio.memes.remove(idx);
        // Would receive the other meme from peer
        portfolio.update_score();
        portfolio.trades += 1;
    }
}

// HTTP handlers
async fn status(State(state): State<NodeState>) -> Json<NodeStatus> {
    let portfolio = state.portfolio.lock().unwrap();
    Json(NodeStatus {
        node_id: state.node_id,
        score: portfolio.score,
        meme_count: portfolio.memes.len(),
        trades: portfolio.trades,
    })
}

async fn get_portfolio(State(state): State<NodeState>) -> Json<Portfolio> {
    let portfolio = state.portfolio.lock().unwrap();
    Json(portfolio.clone())
}

async fn receive_trade_offer(
    State(state): State<NodeState>,
    Json(offer): Json<TradeOffer>
) -> Json<TradeResponse> {
    let mut portfolio = state.portfolio.lock().unwrap();
    
    // Evaluate offer
    let current_score = portfolio.score;
    let new_score = simulate_trade_score(&portfolio, &portfolio, &offer);
    
    let accepted = new_score > current_score;
    
    if accepted {
        execute_trade_local(&mut portfolio, &offer);
    }
    
    Json(TradeResponse { accepted })
}

async fn accept_trade(
    State(state): State<NodeState>,
    Json(offer): Json<TradeOffer>
) -> Json<TradeResponse> {
    let mut portfolio = state.portfolio.lock().unwrap();
    execute_trade_local(&mut portfolio, &offer);
    Json(TradeResponse { accepted: true })
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeStatus {
    node_id: usize,
    score: f64,
    meme_count: usize,
    trades: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TradeResponse {
    accepted: bool,
}

// use crate::distributed_trading::{Portfolio, TradeOffer, simulate_trade_score};
// use crate::meme_marketplace::{Meme, MemeID};

impl Portfolio {
    pub fn save_to_parquet(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Simplified: just serialize to JSON for now
        let json = serde_json::to_string(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_from_parquet(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let portfolio = serde_json::from_str(&json)?;
        Ok(portfolio)
    }
}

impl Default for Meme {
    fn default() -> Self {
        Self {
            id: 0,
            godel_number: 0,
            emoji: "🧬".to_string(),
            code: Vec::new(),
            complexity: 0,
            fitness: 0.0,
            rarity: 1.0,
            generation: 0,
            owner: String::new(),
            price: None,
        }
    }
}
