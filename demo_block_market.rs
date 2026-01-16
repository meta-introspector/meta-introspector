// Market maker fills deep order: "Buy all Rust source"
// 24 nodes compete to process blocks at best price

mod xz_to_syn_mapper;
mod rand_shim;
mod rustc_fuzzer;

use xz_to_syn_mapper::{XzToSynMapper, XzBlock};
use rand_shim::{init_rand, random_u64};
use rustc_fuzzer::SynToRustcSpectrum;
use std::collections::HashSet;

#[derive(Clone)]
struct Node {
    id: usize,
    balance: u64,
    blocks_processed: usize,
    total_paid: u64,
    coverage_found: HashSet<u64>,
    earnings: u64,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            balance: 10000,
            blocks_processed: 0,
            total_paid: 0,
            coverage_found: HashSet::new(),
            earnings: 0,
        }
    }
    
    fn bid_for_block(&self, block: &XzBlock) -> u64 {
        // Bid based on block size and balance
        let base_price = (block.compressed_size / 100) as u64;
        let noise = random_u64() % 50;
        base_price + noise
    }
    
    fn process_block(&mut self, block: XzBlock, price: u64, mapper: &mut XzToSynMapper, 
                     global_coverage: &HashSet<u64>) -> (bool, HashSet<u64>) {
        if self.balance < price {
            return (false, HashSet::new());
        }
        
        let source = String::from_utf8_lossy(&block.data).to_string();
        
        // Compile with rustc to get coverage
        let new_ips = if let Ok(spectrum) = SynToRustcSpectrum::from_source(source, 0) {
            spectrum.rustc_ips.difference(global_coverage).copied().collect()
        } else {
            HashSet::new()
        };
        
        if let Some(_syn_block) = mapper.map_to_syn(block) {
            self.balance -= price;
            self.total_paid += price;
            self.blocks_processed += 1;
            
            // Earn 100 coins per new IP
            let reward = new_ips.len() as u64 * 100;
            self.balance += reward;
            self.earnings += reward;
            self.coverage_found.extend(new_ips.iter());
            
            (true, new_ips)
        } else {
            (false, HashSet::new())
        }
    }
}

struct MarketMaker {
    nodes: Vec<Node>,
    blocks_sold: usize,
    total_revenue: u64,
    global_coverage: HashSet<u64>,
}

impl MarketMaker {
    fn new(num_nodes: usize) -> Self {
        Self {
            nodes: (0..num_nodes).map(Node::new).collect(),
            blocks_sold: 0,
            total_revenue: 0,
            global_coverage: HashSet::new(),
        }
    }
    
    fn fill_order(&mut self, blocks: Vec<XzBlock>, mapper: &mut XzToSynMapper) {
        println!("📦 Deep Order: Buy ALL Rust source ({} blocks)", blocks.len());
        println!("💰 Market maker distributing to {} nodes\n", self.nodes.len());
        
        for (i, block) in blocks.into_iter().enumerate() {
            let mut bids: Vec<(usize, u64)> = self.nodes.iter()
                .map(|node| (node.id, node.bid_for_block(&block)))
                .collect();
            
            bids.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (node_id, bid_price) in bids {
                let node = &mut self.nodes[node_id];
                
                let (success, new_ips) = node.process_block(
                    block.clone(), 
                    bid_price, 
                    mapper, 
                    &self.global_coverage
                );
                
                if success {
                    self.blocks_sold += 1;
                    self.total_revenue += bid_price;
                    self.global_coverage.extend(new_ips.iter());
                    
                    if i % 10 == 0 {
                        println!("  Block {}: Node {} won at {} coins, found {} new IPs, earned {} coins",
                                 i, node_id, bid_price, new_ips.len(), new_ips.len() * 100);
                    }
                    break;
                }
            }
        }
    }
    
    fn report(&self) {
        println!("\n📊 Market Report");
        println!("  Blocks sold: {}", self.blocks_sold);
        println!("  Total revenue: {} coins", self.total_revenue);
        println!("  Average price: {:.0} coins/block", 
                 self.total_revenue as f64 / self.blocks_sold as f64);
        println!("  Global coverage: {} IPs", self.global_coverage.len());
        
        let total_earnings: u64 = self.nodes.iter().map(|n| n.earnings).sum();
        println!("  Total earnings paid: {} coins", total_earnings);
        
        println!("\n🏆 Top Earners:");
        let mut sorted_nodes = self.nodes.clone();
        sorted_nodes.sort_by(|a, b| b.earnings.cmp(&a.earnings));
        
        for node in sorted_nodes.iter().take(5) {
            println!("  Node {}: {} blocks, {} IPs found, {} coins earned, {} balance",
                     node.id, node.blocks_processed, node.coverage_found.len(), 
                     node.earnings, node.balance);
        }
    }
}

fn main() {
    init_rand();
    
    println!("🏪 Block Market: Deep Order Execution\n");
    
    let rust_src = "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz";
    
    println!("📦 Loading blocks from nix store...\n");
    let blocks = XzToSynMapper::scan_xz_blocks(rust_src, 100);
    
    println!("Found {} blocks\n", blocks.len());
    
    let mut mapper = XzToSynMapper::new();
    let mut market = MarketMaker::new(24);
    
    market.fill_order(blocks, &mut mapper);
    
    market.report();
    mapper.report();
    
    println!("\n✅ Deep order filled!");
    println!("\n💡 Key insights:");
    println!("  • 24 nodes compete for blocks via bidding");
    println!("  • Market maker awards to highest bidder");
    println!("  • Nodes process blocks → compressed syn");
    println!("  • Economic incentives drive distribution");
    println!("  • Best price discovery through competition");
}
