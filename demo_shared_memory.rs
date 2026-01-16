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
    let rounds = 100;
    
    for round in 0..rounds {
        println!("📊 Round {}", round);
        
        // Auction phase: nodes bid on best memes
        if round % 10 == 0 && round > 0 {
            println!("  💰 Auction phase!");
            for node in &nodes {
                let node = node.lock().unwrap();
                // Find best meme in portfolio
                if let Some(best_meme) = node.portfolio.memes.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()) {
                    // Broadcast auction bid
                    for peer in 0..num_nodes {
                        if peer != node.node_id {
                            let bid = (best_meme.fitness * 10.0) as u64;
                            if node.portfolio.balance >= bid {
                                let msg = shared_memory_bus::Message::AuctionBid {
                                    meme_id: best_meme.id,
                                    bid_amount: bid,
                                    bidder_id: node.node_id,
                                };
                                let _ = node.bus.send(node.node_id, peer, msg);
                            }
                        }
                    }
                }
            }
        }
        
        // Replication phase: best memes spread
        if round % 5 == 0 {
            for node in &nodes {
                let node = node.lock().unwrap();
                for meme in &node.portfolio.memes {
                    if meme.fitness > 50.0 && meme.code.len() < 100 {
                        // Lean, high-fitness meme - replicate!
                        for peer in 0..num_nodes {
                            if peer != node.node_id {
                                let msg = shared_memory_bus::Message::MemeReplicate {
                                    meme_id: meme.id,
                                    fitness: meme.fitness,
                                };
                                let _ = node.bus.send(node.node_id, peer, msg);
                            }
                        }
                    }
                }
            }
        }
        
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
    
    // Meme statistics
    println!("\n🧬 Top 3 Memes by Rarity:");
    let mut meme_details = std::collections::HashMap::new();
    for node in &nodes {
        let node = node.lock().unwrap();
        for meme in &node.portfolio.memes {
            let entry = meme_details.entry(meme.id).or_insert((0, meme.clone()));
            entry.0 += 1;
        }
    }
    
    let mut meme_stats: Vec<_> = meme_details.iter()
        .map(|(&id, (count, meme))| {
            let rarity = 1.0 / *count as f64;
            (id, *count, rarity, meme.clone())
        })
        .collect();
    meme_stats.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    for (i, (id, count, rarity, meme)) in meme_stats.iter().take(3).enumerate() {
        println!("  {}. Meme {} {}", i + 1, id, meme.emoji);
        println!("     Held by {} nodes, rarity={:.4}", count, rarity);
        println!("     Gödel: {}, complexity: {}, fitness: {:.2}, size: {} bytes", 
                 meme.godel_number, meme.complexity, meme.fitness, meme.code.len());
    }
    
    // Economic stats
    println!("\n💰 Economic Stats:");
    let total_balance: u64 = nodes.iter().map(|n| n.lock().unwrap().portfolio.balance).sum();
    let avg_balance = total_balance / num_nodes as u64;
    let total_memory: usize = nodes.iter().map(|n| n.lock().unwrap().portfolio.memory_used).sum();
    println!("  Total balance: {} coins", total_balance);
    println!("  Average balance: {} coins", avg_balance);
    println!("  Total memory used: {} bytes", total_memory);
}
