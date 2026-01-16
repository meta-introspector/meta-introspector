// Distributed meme trading: 24 CPUs, each optimizing portfolio
// Nodes swap memes with partners to maximize score

use crossbeam::channel::{bounded, Sender, Receiver};
use crossbeam::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Portfolio {
    pub node_id: usize,
    pub memes: Vec<Meme>,
    pub score: f64,
    pub trades: usize,
    pub balance: u64,  // Money for auctions
    pub memory_used: usize,  // Track memory consumption
    pub memory_limit: usize,  // Max memory allowed
}

impl Portfolio {
    pub fn new(node_id: usize, initial_memes: Vec<Meme>) -> Self {
        let score = Self::compute_score(&initial_memes);
        let memory_used = initial_memes.iter().map(|m| m.code.len()).sum();
        Self {
            node_id,
            memes: initial_memes,
            score,
            trades: 0,
            balance: 10000,  // Start with 10k coins
            memory_used,
            memory_limit: 100000,  // 100KB limit
        }
    }
    
    pub fn compute_score(memes: &[Meme]) -> f64 {
        let mut score = 0.0;
        
        // Diversity bonus
        let unique_complexities: std::collections::HashSet<_> = 
            memes.iter().map(|m| m.complexity).collect();
        score += unique_complexities.len() as f64 * 10.0;
        
        // Fitness sum
        score += memes.iter().map(|m| m.fitness).sum::<f64>();
        
        // Rarity bonus
        score += memes.iter().map(|m| m.rarity).sum::<f64>() * 5.0;
        
        // Memory efficiency: penalize large memes
        let total_size: usize = memes.iter().map(|m| m.code.len()).sum();
        let efficiency_bonus = 1000.0 / (total_size as f64 + 1.0);
        score += efficiency_bonus;
        
        // Prime orbit bonus
        for meme in memes {
            if is_prime(meme.godel_number % 71) {
                score += 50.0;
            }
        }
        
        score
    }
    
    pub fn update_score(&mut self) {
        self.score = Self::compute_score(&self.memes);
    }
}

#[derive(Debug, Clone)]
pub struct TradeOffer {
    pub from_node: usize,
    pub to_node: usize,
    pub offer_meme: MemeID,
    pub want_meme: MemeID,
    pub score_improvement: f64,
}

pub struct TradingNetwork {
    pub nodes: Vec<Arc<Mutex<Portfolio>>>,
    pub marketplace: Arc<Mutex<Marketplace>>,
    pub trade_channel: (Sender<TradeOffer>, Receiver<TradeOffer>),
}

impl TradingNetwork {
    pub fn new(num_nodes: usize, memes_per_node: usize) -> Self {
        let mut nodes = Vec::new();
        
        // Distribute memes to nodes
        for node_id in 0..num_nodes {
            let mut memes = Vec::new();
            for _ in 0..memes_per_node {
                memes.push(Meme::random());
            }
            nodes.push(Arc::new(Mutex::new(Portfolio::new(node_id, memes))));
        }
        
        let (tx, rx) = bounded(1000);
        
        Self {
            nodes,
            marketplace: Arc::new(Mutex::new(Marketplace::new("network".to_string()))),
            trade_channel: (tx, rx),
        }
    }
    
    pub fn run(&mut self, rounds: usize) {
        println!("🚀 Starting trading network with {} nodes", self.nodes.len());
        
        for round in 0..rounds {
            println!("\n📊 Round {}", round);
            
            // Each node runs in parallel
            thread::scope(|s| {
                for node in &self.nodes {
                    let node = Arc::clone(node);
                    let tx = self.trade_channel.0.clone();
                    let all_nodes = self.nodes.clone();
                    
                    s.spawn(move |_| {
                        node_trading_loop(node, all_nodes, tx);
                    });
                }
            }).unwrap();
            
            // Process trades
            self.process_trades();
            
            // Report
            self.report_round(round);
        }
        
        println!("\n🏆 Final Results:");
        self.report_final();
    }
    
    fn process_trades(&mut self) {
        let mut trades_executed = 0;
        
        while let Ok(offer) = self.trade_channel.1.try_recv() {
            if self.execute_trade(&offer) {
                trades_executed += 1;
            }
        }
        
        println!("  ✅ Executed {} trades", trades_executed);
    }
    
    fn execute_trade(&mut self, offer: &TradeOffer) -> bool {
        let from_node = &self.nodes[offer.from_node];
        let to_node = &self.nodes[offer.to_node];
        
        let mut from = from_node.lock().unwrap();
        let mut to = to_node.lock().unwrap();
        
        // Find memes
        let from_meme_idx = from.memes.iter().position(|m| m.id == offer.offer_meme);
        let to_meme_idx = to.memes.iter().position(|m| m.id == offer.want_meme);
        
        if let (Some(from_idx), Some(to_idx)) = (from_meme_idx, to_meme_idx) {
            // Swap memes
            let from_meme = from.memes.remove(from_idx);
            let to_meme = to.memes.remove(to_idx);
            
            from.memes.push(to_meme);
            to.memes.push(from_meme);
            
            // Update scores
            from.update_score();
            to.update_score();
            
            from.trades += 1;
            to.trades += 1;
            
            true
        } else {
            false
        }
    }
    
    fn report_round(&self, round: usize) {
        let total_score: f64 = self.nodes.iter()
            .map(|n| n.lock().unwrap().score)
            .sum();
        
        let avg_score = total_score / self.nodes.len() as f64;
        
        println!("  📈 Average score: {:.2}", avg_score);
    }
    
    fn report_final(&self) {
        let mut scores: Vec<_> = self.nodes.iter()
            .map(|n| {
                let node = n.lock().unwrap();
                (node.node_id, node.score, node.trades)
            })
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        println!("\nTop 10 nodes:");
        for (i, (node_id, score, trades)) in scores.iter().take(10).enumerate() {
            println!("  {}. Node {}: score={:.2}, trades={}", 
                     i + 1, node_id, score, trades);
        }
    }
}

fn node_trading_loop(
    node: Arc<Mutex<Portfolio>>,
    all_nodes: Vec<Arc<Mutex<Portfolio>>>,
    tx: Sender<TradeOffer>
) {
    let node_id = node.lock().unwrap().node_id;
    
    // Try to find beneficial trades
    for _ in 0..10 {
        // Pick random partner
        use crate::rand_shim::random_usize;
        let partner_idx = random_usize() % all_nodes.len();
        if partner_idx == node_id {
            continue;
        }
        
        let partner = &all_nodes[partner_idx];
        
        // Find best trade
        if let Some(offer) = find_best_trade(&node, partner) {
            tx.send(offer).ok();
        }
    }
}

fn find_best_trade(
    node: &Arc<Mutex<Portfolio>>,
    partner: &Arc<Mutex<Portfolio>>
) -> Option<TradeOffer> {
    let node_lock = node.lock().unwrap();
    let partner_lock = partner.lock().unwrap();
    
    let mut best_offer = None;
    let mut best_improvement = 0.0;
    
    // Try all combinations
    for my_meme in &node_lock.memes {
        for their_meme in &partner_lock.memes {
            // Simulate trade
            let my_score_after = simulate_trade_score(&node_lock.memes, my_meme.id, their_meme);
            let their_score_after = simulate_trade_score(&partner_lock.memes, their_meme.id, my_meme);
            
            let my_improvement = my_score_after - node_lock.score;
            let their_improvement = their_score_after - partner_lock.score;
            
            // Both must benefit
            if my_improvement > 0.0 && their_improvement > 0.0 {
                let total_improvement = my_improvement + their_improvement;
                
                if total_improvement > best_improvement {
                    best_improvement = total_improvement;
                    best_offer = Some(TradeOffer {
                        from_node: node_lock.node_id,
                        to_node: partner_lock.node_id,
                        offer_meme: my_meme.id,
                        want_meme: their_meme.id,
                        score_improvement: total_improvement,
                    });
                }
            }
        }
    }
    
    best_offer
}

fn simulate_trade_score(memes: &[Meme], remove_id: MemeID, add: &Meme) -> f64 {
    let mut new_memes: Vec<_> = memes.iter()
        .filter(|m| m.id != remove_id)
        .cloned()
        .collect();
    new_memes.push(add.clone());
    
    Portfolio::compute_score(&new_memes)
}

use crate::meme_marketplace::{Meme, MemeID, Marketplace};
use crate::program_evolution::is_prime;

impl Meme {
    pub fn random() -> Self {
        use crate::rand_shim::random_u64;
        let id = random_u64();
        let godel = random_u64();
        
        // Generate random bits
        let bits: Vec<u8> = (0..32).map(|_| (random_u64() & 0xFF) as u8).collect();
        let complexity = (random_u64() % 100) as usize;
        
        Self {
            id,
            godel_number: godel,
            emoji: "🧬".to_string(),
            code: bits,
            complexity,
            fitness: (random_u64() as f64 / u64::MAX as f64) * 100.0,
            rarity: 1.0,
            generation: 0,
            owner: "network".to_string(),
            price: None,
        }
    }
    
    /// Generate Rust code from meme bits
    pub fn to_rust_code(&self) -> String {
        use crate::bits_to_rust::BitsToRust;
        let converter = BitsToRust::new();
        converter.generate_program(&self.code, self.complexity.min(7))
    }
    
    /// Compile to WASM and get trace using existing wasm_runner
    pub fn compile_and_trace(&self) -> Option<(Vec<u8>, crate::wasm_runner::WasmTrace)> {
        use crate::bits_to_rust::BitsToRust;
        
        let converter = BitsToRust::new();
        let code = self.to_rust_code();
        
        if let Some(wasm) = converter.compile_to_wasm(&code) {
            let mut runner = crate::wasm_runner::WasmRunner::new();
            if let Ok(trace) = runner.eval_with_trace(&wasm) {
                return Some((wasm, trace));
            }
        }
        None
    }
    
    /// Get code metrics
    pub fn metrics(&self) -> crate::bits_to_rust::CodeMetrics {
        use crate::bits_to_rust::{BitsToRust, CodeMetrics};
        
        let converter = BitsToRust::new();
        let code = self.to_rust_code();
        let compiles = converter.verify_compiles(&code);
        
        CodeMetrics::analyze(&code, compiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_portfolio() {
        let memes = vec![Meme::random(), Meme::random()];
        let portfolio = Portfolio::new(0, memes);
        assert!(portfolio.score > 0.0);
    }
    
    #[test]
    fn test_trading_network() {
        let mut network = TradingNetwork::new(4, 5);
        network.run(2);
    }
}
