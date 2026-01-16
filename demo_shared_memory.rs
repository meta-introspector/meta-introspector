// Demo: Run 24 nodes with shared memory bus
// Each node trades memes via direct memory queues

use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod shared_memory_bus;
mod distributed_trading;
mod meme_marketplace;
mod program_evolution;
mod rand_shim;

use shared_memory_bus::{SharedMemoryBus, SharedMemoryNode, Message};
use distributed_trading::Portfolio;
use meme_marketplace::Meme;
use rand_shim::{random_u64, random_usize, init_rand};

fn main() {
    init_rand();
    
    println!("🚀 Starting 24-node trading network with shared memory bus\n");
    
    let num_nodes = 24;
    let queue_size = 1000;
    
    // Create shared memory bus
    let bus = Arc::new(SharedMemoryBus::new(num_nodes, queue_size));
    
    // Create nodes with portfolios
    let mut nodes = Vec::new();
    for node_id in 0..num_nodes {
        let mut memes = Vec::new();
        for _ in 0..10 {
            memes.push(Meme::random());
        }
        
        let portfolio = Portfolio::new(node_id, memes);
        let node = SharedMemoryNode::new(node_id, portfolio, Arc::clone(&bus));
        nodes.push(Arc::new(std::sync::Mutex::new(node)));
    }
    
    println!("✅ Created {} nodes with portfolios\n", num_nodes);
    
    // Run trading rounds
    let rounds = 10;
    
    for round in 0..rounds {
        println!("📊 Round {}", round);
        
        // Each node sends trade offers
        for node in &nodes {
            let node = node.lock().unwrap();
            
            // Send offers to random peers
            for _ in 0..3 {
                let peer = random_usize() % num_nodes;
                if peer != node.node_id {
                    node.send_trade_offer(peer, 
                        random_u64(), 
                        random_u64(), 
                        10.0);
                }
            }
        }
        
        // Process messages
        thread::sleep(Duration::from_millis(100));
        
        let mut total_processed = 0;
        for node in &nodes {
            let mut node = node.lock().unwrap();
            let processed = node.process_messages();
            total_processed += processed;
        }
        
        println!("  Processed {} messages", total_processed);
        
        // Report scores
        let total_score: f64 = nodes.iter()
            .map(|n| n.lock().unwrap().portfolio.score)
            .sum();
        let avg_score = total_score / num_nodes as f64;
        
        println!("  Average score: {:.2}\n", avg_score);
    }
    
    println!("🏆 Final Results:");
    
    let mut results: Vec<_> = nodes.iter()
        .map(|n| {
            let node = n.lock().unwrap();
            (node.node_id, node.portfolio.score, node.portfolio.trades)
        })
        .collect();
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\nTop 10 nodes:");
    for (i, (node_id, score, trades)) in results.iter().take(10).enumerate() {
        println!("  {}. Node {}: score={:.2}, trades={}", 
                 i + 1, node_id, score, trades);
    }
}
