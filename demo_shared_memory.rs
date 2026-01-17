// Demo: Run 24 nodes with shared memory bus
// Each node trades memes via direct memory queues

use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod shared_memory_bus;
// mod distributed_trading;
// mod meme_marketplace;
// mod program_evolution;
// mod rand_shim;
// mod market_maker;
// mod meme_evolver;
// mod bits_to_rust;
// mod wasm_runner;

use shared_memory_bus::{SharedMemoryBus, SharedMemoryNode, Message, Portfolio, Meme};
use libnix::rand_shim::{random_u64, random_usize, init_rand};

fn main() {
    init_rand();
    
    println!("🚀 Starting 24-node trading network with shared memory bus\n");
    
    let num_nodes = 24;
    let queue_size = 1000;
    
    // Create shared memory bus
    let bus = Arc::new(SharedMemoryBus::new(num_nodes, queue_size));
    
    // Create market maker with large balance
    // let mut market_maker = market_maker::MarketMaker::new(0, 100000);
    
    // Track profitable trades
    let mut trade_sequences = Vec::new();
    
    // Create nodes with portfolios and evolvers
    let mut nodes = Vec::new();
    let mut evolvers = Vec::new();
    for node_id in 0..num_nodes {
        let mut memes = Vec::new();
        for _ in 0..10 {
            memes.push(Meme::random());
        }
        
        let portfolio = Portfolio::new(node_id, memes);
        let node = SharedMemoryNode::new(node_id, portfolio, Arc::clone(&bus));
        nodes.push(Arc::new(std::sync::Mutex::new(node)));
        // evolvers.push(// meme_evolver::MemeEvolver::new(node_id));
    }
    
    println!("✅ Created {} nodes with portfolios\n", num_nodes);
    
    // Run trading rounds
    let rounds = 100;
    
    for round in 0..rounds {
        println!("📊 Round {}", round);
        
        // Evolution phase: buy, evolve, sell
        if round % 7 == 0 && round > 0 {
            println!("  🧬 Evolution phase!");
            for i in 0..nodes.len() {
                let mut node = nodes[i].lock().unwrap();
                let evolver = &evolvers[i];
                
                // Find a meme to evolve
                if let Some(meme_idx) = (0..node.portfolio.memes.len())
                    .max_by_key(|&idx| (node.portfolio.memes[idx].fitness * 100.0) as u64) {
                    
                    let meme = node.portfolio.memes[meme_idx].clone();
                    let buy_price = (meme.fitness * 100.0) as u64;
                    
                    // Evolve it
                    let evolved = evolver.evolve(&meme);
                    let sell_price = (evolved.fitness * 100.0) as u64;
                    let profit = sell_price as i64 - buy_price as i64;
                    
                    // Replace original with evolved
                    node.portfolio.memes[meme_idx] = evolved.clone();
                    
                    // Record profitable trade
                    if profit > 0 {
                        trade_sequences.push(// meme_evolver::TradeSequence {
                            node_id: node.node_id,
                            bought_meme_id: meme.id,
                            buy_price,
                            evolved_meme_id: evolved.id,
                            sell_price,
                            profit,
                            strategy: "evolve".to_string(),
                        });
                    }
                }
                
                // Try combining two rare memes
                if node.portfolio.memes.len() >= 2 {
                    let rare_memes: Vec<_> = node.portfolio.memes.iter()
                        .enumerate()
                        .filter(|(_, m)| m.rarity > 0.8)
                        .map(|(idx, m)| (idx, m.clone()))
                        .collect();
                    
                    if rare_memes.len() >= 2 {
                        let meme1 = &rare_memes[0].1;
                        let meme2 = &rare_memes[1].1;
                        let cost = ((meme1.fitness + meme2.fitness) * 100.0) as u64;
                        
                        let hybrid = evolver.combine(meme1, meme2);
                        let sell_price = (hybrid.fitness * 100.0) as u64;
                        let profit = sell_price as i64 - cost as i64;
                        
                        // Add hybrid to portfolio
                        node.portfolio.memes.push(hybrid.clone());
                        
                        if profit > 0 {
                            trade_sequences.push(// meme_evolver::TradeSequence {
                                node_id: node.node_id,
                                bought_meme_id: meme1.id,
                                buy_price: cost,
                                evolved_meme_id: hybrid.id,
                                sell_price,
                                profit,
                                strategy: format!("combine {} + {}", meme1.id, meme2.id),
                            });
                        }
                    }
                }
            }
        }
        
        // Market maker quotes: provide liquidity
        if round % 5 == 0 {
            for node in &nodes {
                let node = node.lock().unwrap();
                for meme in &node.portfolio.memes {
                    let (bid, ask) = // market_maker.quote(meme);
                    // Market maker stands ready to buy at bid, sell at ask
                }
            }
        }
        
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
        
        // Each node sends trade offers with REAL meme IDs from peers
        for i in 0..nodes.len() {
            let node = nodes[i].lock().unwrap();
            
            // Send offers to random peers using actual memes
            if !node.portfolio.memes.is_empty() {
                for _ in 0..2 {
                    let peer_idx = random_usize() % num_nodes;
                    if peer_idx != node.node_id {
                        // Get a meme from peer's portfolio
                        let peer = nodes[peer_idx].lock().unwrap();
                        if !peer.portfolio.memes.is_empty() {
                            let want_idx = random_usize() % peer.portfolio.memes.len();
                            let want_meme = peer.portfolio.memes[want_idx].id;
                            drop(peer);  // Release lock
                            
                            // Offer a random meme from our portfolio
                            let offer_idx = random_usize() % node.portfolio.memes.len();
                            let offer_meme = node.portfolio.memes[offer_idx].id;
                            
                            node.send_trade_offer(peer_idx, offer_meme, want_meme, 10.0);
                        }
                    }
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
    
    // Market maker report
    // market_maker.report();
    println!();
    
    // Report profitable trades
    if !trade_sequences.is_empty() {
        println!("\n💎 Top 5 Profitable Trades:");
        trade_sequences.sort_by_key(|t| -t.profit);
        for trade in trade_sequences.iter().take(5) {
            trade.report();
        }
    }
    
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
        
        // Show generated Rust code for top meme
        if i == 0 {
            println!("\n     📝 Generated Rust Code:");
            let code = meme.to_rust_code();
            for line in code.lines() {
                println!("        {}", line);
            }
            
            // Show metrics
            println!();
            let metrics = meme.metrics();
            println!("     📊 Code Metrics:");
            println!("        Complexity: {}", metrics.complexity);
            println!("        Lines: {}", metrics.lines);
            println!("        Tokens: {}", metrics.tokens);
            println!("        Compiles: {}", if metrics.compiles { "✅" } else { "❌" });
            
            // Try WASM compilation and trace
            if let Some((wasm, trace)) = meme.compile_and_trace() {
                println!("\n     🔍 WASM Execution Trace:");
                println!("        WASM size: {} bytes", wasm.len());
                println!("        Instructions: {}", trace.instructions.len());
                println!("        Gödel number: {}", trace.godel_number);
                if !trace.instructions.is_empty() {
                    println!("        First instruction: {}", trace.instructions[0].opcode);
                }
            }
        }
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
