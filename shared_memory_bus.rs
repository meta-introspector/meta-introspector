// Shared memory bus: Direct memory queues between nodes
// Each node has queue to every other node via shared memory

use std::sync::Arc;
use crossbeam::queue::ArrayQueue;
use std::collections::HashMap;

// Stub type until we properly organize modules
#[derive(Debug, Clone)]
pub struct Portfolio {
    pub memes: Vec<Meme>,
    pub balance: u64,
    pub memory_used: usize,
    pub memory_limit: usize,
    pub score: f64,
    pub trades: usize,
}

impl Portfolio {
    pub fn update_score(&mut self) {
        self.score = self.memes.iter().map(|m| m.fitness).sum();
    }
}

#[derive(Clone, Debug)]
pub struct Meme {
    pub id: u64,
    pub fitness: f64,
    pub complexity: f64,
    pub rarity: f64,
    pub code: String,
    pub emoji: String,
    pub godel_number: u64,
}

#[derive(Clone, Debug)]
pub struct MemeMetrics {
    pub complexity: f64,
    pub lines: usize,
    pub tokens: usize,
    pub compiles: bool,
}

impl Meme {
    pub fn to_rust_code(&self) -> String { self.code.clone() }
    pub fn metrics(&self) -> MemeMetrics { 
        MemeMetrics { 
            complexity: self.complexity, 
            lines: 0, 
            tokens: 0, 
            compiles: true 
        } 
    }
    pub fn compile_and_trace(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> { Ok(vec![]) }
}

/// Shared memory bus connecting all nodes
pub struct SharedMemoryBus {
    /// Queues: (from_node, to_node) -> Queue
    queues: HashMap<(usize, usize), Arc<ArrayQueue<Message>>>,
    num_nodes: usize,
}

impl SharedMemoryBus {
    pub fn new(num_nodes: usize, queue_size: usize) -> Self {
        let mut queues = HashMap::new();
        
        // Create queue from each node to every other node
        for from in 0..num_nodes {
            for to in 0..num_nodes {
                if from != to {
                    let queue = Arc::new(ArrayQueue::new(queue_size));
                    queues.insert((from, to), queue);
                }
            }
        }
        
        println!("🚌 Created shared memory bus with {} queues", queues.len());
        
        Self {
            queues,
            num_nodes,
        }
    }
    
    /// Get queue for sending from node A to node B
    pub fn get_queue(&self, from: usize, to: usize) -> Option<Arc<ArrayQueue<Message>>> {
        self.queues.get(&(from, to)).cloned()
    }
    
    /// Send message from one node to another
    pub fn send(&self, from: usize, to: usize, msg: Message) -> Result<(), Message> {
        if let Some(queue) = self.get_queue(from, to) {
            queue.push(msg)
        } else {
            Err(msg)
        }
    }
    
    /// Receive message at node
    pub fn receive(&self, from: usize, to: usize) -> Option<Message> {
        if let Some(queue) = self.get_queue(from, to) {
            queue.pop()
        } else {
            None
        }
    }
    
    /// Broadcast message to all other nodes
    pub fn broadcast(&self, from: usize, msg: Message) {
        for to in 0..self.num_nodes {
            if to != from {
                let _ = self.send(from, to, msg.clone());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TradeOffer {
        offer_meme: u64,
        want_meme: u64,
        score_improvement: f64,
    },
    TradeAccept {
        offer_meme: u64,
        want_meme: u64,
    },
    TradeReject {
        offer_meme: u64,
    },
    AuctionBid {
        meme_id: u64,
        bid_amount: u64,
        bidder_id: usize,
    },
    AuctionWin {
        meme_id: u64,
        price: u64,
    },
    MemeReplicate {
        meme_id: u64,
        fitness: f64,
    },
    PortfolioQuery,
    PortfolioResponse {
        memes: Vec<MemeInfo>,
        score: f64,
    },
    Ping,
    Pong,
}

#[derive(Debug, Clone)]
pub struct MemeInfo {
    pub id: u64,
    pub complexity: usize,
    pub fitness: f64,
    pub rarity: f64,
}

/// Node with shared memory communication
pub struct SharedMemoryNode {
    pub node_id: usize,
    pub portfolio: Portfolio,
    pub bus: Arc<SharedMemoryBus>,
}

impl SharedMemoryNode {
    pub fn new(node_id: usize, portfolio: Portfolio, bus: Arc<SharedMemoryBus>) -> Self {
        Self {
            node_id,
            portfolio,
            bus,
        }
    }
    
    /// Send trade offer to another node
    pub fn send_trade_offer(&self, to_node: usize, offer_meme: u64, want_meme: u64, improvement: f64) {
        let msg = Message::TradeOffer {
            offer_meme,
            want_meme,
            score_improvement: improvement,
        };
        
        if let Err(_) = self.bus.send(self.node_id, to_node, msg) {
            println!("  [Node {}] Queue full to node {}", self.node_id, to_node);
        }
    }
    
    /// Process incoming messages
    pub fn process_messages(&mut self) -> usize {
        let mut processed = 0;
        
        // Check messages from all other nodes
        for from_node in 0..self.bus.num_nodes {
            if from_node == self.node_id {
                continue;
            }
            
            while let Some(msg) = self.bus.receive(from_node, self.node_id) {
                self.handle_message(from_node, msg);
                processed += 1;
            }
        }
        
        processed
    }
    
    fn handle_message(&mut self, from_node: usize, msg: Message) {
        match msg {
            Message::TradeOffer { offer_meme, want_meme, score_improvement } => {
                // Evaluate trade
                if self.should_accept_trade(offer_meme, want_meme) {
                    // Accept trade
                    let response = Message::TradeAccept { offer_meme, want_meme };
                    let _ = self.bus.send(self.node_id, from_node, response);
                    
                    // Execute trade locally
                    self.execute_trade(offer_meme, want_meme);
                    
                    println!("  [Node {}] ✅ Accepted trade from node {}", self.node_id, from_node);
                } else {
                    // Reject trade
                    let response = Message::TradeReject { offer_meme };
                    let _ = self.bus.send(self.node_id, from_node, response);
                }
            }
            
            Message::TradeAccept { offer_meme, want_meme } => {
                // Trade accepted! Execute locally
                self.execute_trade(want_meme, offer_meme);
                println!("  [Node {}] ✅ Trade accepted by node {}", self.node_id, from_node);
            }
            
            Message::TradeReject { .. } => {
                // Trade rejected, try another
            }
            
            Message::AuctionBid { meme_id, bid_amount, bidder_id } => {
                // Check if we have this meme and want to sell
                if let Some(meme) = self.portfolio.memes.iter().find(|m| m.id == meme_id) {
                    let min_price = (meme.fitness * 100.0) as u64;
                    if bid_amount >= min_price {
                        // Accept bid
                        let response = Message::AuctionWin { meme_id, price: bid_amount };
                        let _ = self.bus.send(self.node_id, bidder_id, response);
                        self.portfolio.balance += bid_amount;
                    }
                }
            }
            
            Message::AuctionWin { meme_id, price } => {
                // Won auction! Pay and receive meme
                if self.portfolio.balance >= price {
                    self.portfolio.balance -= price;
                    println!("  [Node {}] 💰 Won auction for meme {} at {}", self.node_id, meme_id, price);
                }
            }
            
            Message::MemeReplicate { meme_id, fitness } => {
                // High-fitness meme spreading
                if fitness > 50.0 && self.portfolio.memory_used < self.portfolio.memory_limit {
                    println!("  [Node {}] 🧬 Replicating high-fitness meme {}", self.node_id, meme_id);
                }
            }
            
            Message::PortfolioQuery => {
                // Send portfolio info
                let memes: Vec<MemeInfo> = self.portfolio.memes.iter()
                    .map(|m| MemeInfo {
                        id: m.id,
                        complexity: m.complexity as usize,
                        fitness: m.fitness,
                        rarity: m.rarity,
                    })
                    .collect();
                
                let response = Message::PortfolioResponse {
                    memes,
                    score: self.portfolio.score,
                };
                
                let _ = self.bus.send(self.node_id, from_node, response);
            }
            
            Message::PortfolioResponse { .. } => {
                // Store peer portfolio info for trading decisions
            }
            
            Message::Ping => {
                let _ = self.bus.send(self.node_id, from_node, Message::Pong);
            }
            
            Message::Pong => {
                // Peer is alive
            }
        }
    }
    
    fn should_accept_trade(&self, offer_meme: u64, want_meme: u64) -> bool {
        // Check if we have the wanted meme
        let have_meme = self.portfolio.memes.iter().find(|m| m.id == want_meme);
        if have_meme.is_none() {
            return false;
        }
        
        // Accept if we don't have the offered meme (diversity)
        let already_have = self.portfolio.memes.iter().any(|m| m.id == offer_meme);
        if !already_have {
            return true;  // Always accept new memes for diversity
        }
        
        // Accept if offered meme has higher fitness than what we're giving
        let want_fitness = have_meme.unwrap().fitness;
        // Assume offered meme has reasonable fitness (we'd need to query)
        // For now, accept 30% of trades randomly to create liquidity
        use crate::rand_shim::random_f64;
        random_f64() < 0.3
    }
    
    fn execute_trade(&mut self, give_meme: u64, _receive_meme: u64) {
        // Remove given meme
        if let Some(idx) = self.portfolio.memes.iter().position(|m| m.id == give_meme) {
            self.portfolio.memes.remove(idx);
        }
        
        // Add received meme (would come from peer)
        // TODO: Get actual meme data
        
        self.portfolio.update_score();
        self.portfolio.trades += 1;
    }
}

// use crate::distributed_trading::Portfolio;
// use crate::meme_marketplace::Meme;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shared_memory_bus() {
        let bus = SharedMemoryBus::new(4, 100);
        
        // Send message from node 0 to node 1
        let msg = Message::Ping;
        assert!(bus.send(0, 1, msg).is_ok());
        
        // Receive at node 1
        let received = bus.receive(0, 1);
        assert!(received.is_some());
    }
    
    #[test]
    fn test_broadcast() {
        let bus = SharedMemoryBus::new(4, 100);
        
        // Broadcast from node 0
        bus.broadcast(0, Message::Ping);
        
        // All other nodes should receive
        assert!(bus.receive(0, 1).is_some());
        assert!(bus.receive(0, 2).is_some());
        assert!(bus.receive(0, 3).is_some());
    }
}
