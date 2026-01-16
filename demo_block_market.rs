// Market maker fills deep order: "Buy all Rust source"
// 24 nodes compete to process blocks at best price

mod xz_to_syn_mapper;
mod rand_shim;

use xz_to_syn_mapper::{XzToSynMapper, XzBlock};
use rand_shim::{init_rand, random_u64};

#[derive(Clone)]
struct Node {
    id: usize,
    balance: u64,
    blocks_processed: usize,
    total_paid: u64,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            balance: 10000,
            blocks_processed: 0,
            total_paid: 0,
        }
    }
    
    fn bid_for_block(&self, block: &XzBlock) -> u64 {
        // Bid based on block size and balance
        let base_price = (block.compressed_size / 100) as u64;
        let noise = random_u64() % 50;
        base_price + noise
    }
    
    fn process_block(&mut self, block: XzBlock, price: u64, mapper: &mut XzToSynMapper) -> bool {
        if self.balance < price {
            return false;
        }
        
        if let Some(_syn_block) = mapper.map_to_syn(block) {
            self.balance -= price;
            self.total_paid += price;
            self.blocks_processed += 1;
            true
        } else {
            false
        }
    }
}

struct MarketMaker {
    nodes: Vec<Node>,
    blocks_sold: usize,
    total_revenue: u64,
}

impl MarketMaker {
    fn new(num_nodes: usize) -> Self {
        Self {
            nodes: (0..num_nodes).map(Node::new).collect(),
            blocks_sold: 0,
            total_revenue: 0,
        }
    }
    
    fn fill_order(&mut self, blocks: Vec<XzBlock>, mapper: &mut XzToSynMapper) {
        println!("📦 Deep Order: Buy ALL Rust source ({} blocks)", blocks.len());
        println!("💰 Market maker distributing to {} nodes\n", self.nodes.len());
        
        for (i, block) in blocks.into_iter().enumerate() {
            // Collect bids from all nodes
            let mut bids: Vec<(usize, u64)> = self.nodes.iter()
                .map(|node| (node.id, node.bid_for_block(&block)))
                .collect();
            
            // Sort by price (highest first)
            bids.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Award to highest bidder who can afford it
            for (node_id, bid_price) in bids {
                let node = &mut self.nodes[node_id];
                
                if node.process_block(block.clone(), bid_price, mapper) {
                    self.blocks_sold += 1;
                    self.total_revenue += bid_price;
                    
                    if i % 10 == 0 {
                        println!("  Block {}: Node {} won at {} coins ({} bytes)",
                                 i, node_id, bid_price, block.compressed_size);
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
        
        println!("\n🏆 Top Processors:");
        let mut sorted_nodes = self.nodes.clone();
        sorted_nodes.sort_by(|a, b| b.blocks_processed.cmp(&a.blocks_processed));
        
        for node in sorted_nodes.iter().take(5) {
            println!("  Node {}: {} blocks, {} coins spent, {} balance",
                     node.id, node.blocks_processed, node.total_paid, node.balance);
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
