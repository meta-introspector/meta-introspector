// Market for git packs: 24 nodes bid and buy packs to analyze

mod content_addressable_store;
mod rand_shim;

use content_addressable_store::ContentStore;
use rand_shim::{init_rand, random_u64};
use std::process::Command;
use std::collections::HashMap;

#[derive(Clone)]
struct GitPack {
    path: String,
    estimated_size: u64,
    estimated_objects: usize,
}

#[derive(Clone)]
struct Node {
    id: usize,
    balance: u64,
    packs_processed: usize,
    objects_found: usize,
    duplicates_found: usize,
    earnings: u64,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            balance: 10000,
            packs_processed: 0,
            objects_found: 0,
            duplicates_found: 0,
            earnings: 0,
        }
    }
    
    fn bid_for_pack(&self, pack: &GitPack) -> u64 {
        let base = (pack.estimated_size / 1000) as u64;
        base + (random_u64() % 50)
    }
    
    fn process_pack(&mut self, pack: GitPack, price: u64, 
                    global_oids: &HashMap<String, usize>,
                    store: &mut ContentStore) -> (bool, HashMap<String, usize>) {
        if self.balance < price {
            return (false, HashMap::new());
        }
        
        let mut local_oids = HashMap::new();
        
        // Scan pack with git verify-pack
        if let Ok(output) = Command::new("git")
            .args(&["verify-pack", "-v", &pack.path])
            .output() {
            
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines().take(100) {  // Limit per pack
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let oid = parts[0].to_string();
                    *local_oids.entry(oid.clone()).or_insert(0) += 1;
                    
                    // Store in content store
                    if let Ok(size) = parts[2].parse::<usize>() {
                        let synthetic = format!("git-{}-{}", oid, size);
                        store.store(&synthetic);
                    }
                }
            }
        }
        
        self.balance -= price;
        self.packs_processed += 1;
        self.objects_found += local_oids.len();
        
        // Count duplicates
        for oid in local_oids.keys() {
            if global_oids.contains_key(oid) {
                self.duplicates_found += 1;
            }
        }
        
        // Earn 10 coins per unique object
        let unique = local_oids.len() - self.duplicates_found;
        let reward = unique as u64 * 10;
        self.balance += reward;
        self.earnings += reward;
        
        (true, local_oids)
    }
}

struct PackMarket {
    nodes: Vec<Node>,
    packs_sold: usize,
    total_revenue: u64,
    global_oids: HashMap<String, usize>,
}

impl PackMarket {
    fn new(num_nodes: usize) -> Self {
        Self {
            nodes: (0..num_nodes).map(Node::new).collect(),
            packs_sold: 0,
            total_revenue: 0,
            global_oids: HashMap::new(),
        }
    }
    
    fn sell_packs(&mut self, packs: Vec<GitPack>, store: &mut ContentStore) {
        println!("💰 Market: {} packs available\n", packs.len());
        
        for (i, pack) in packs.into_iter().enumerate() {
            let pack_name = pack.path.split('/').last().unwrap_or(&pack.path);
            
            // Collect bids
            let mut bids: Vec<(usize, u64)> = self.nodes.iter()
                .map(|n| (n.id, n.bid_for_pack(&pack)))
                .collect();
            
            bids.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Award to highest bidder
            for (node_id, bid) in bids {
                let node = &mut self.nodes[node_id];
                
                let (success, oids) = node.process_pack(pack.clone(), bid, &self.global_oids, store);
                
                if success {
                    self.packs_sold += 1;
                    self.total_revenue += bid;
                    
                    let new_oids = oids.len();
                    self.global_oids.extend(oids);
                    
                    println!("  Pack {}: Node {} won at {} coins, {} objects, earned {} coins",
                             i, node_id, bid, new_oids, new_oids * 10);
                    break;
                }
            }
        }
    }
    
    fn report(&self) {
        println!("\n📊 Pack Market Report");
        println!("  Packs sold: {}", self.packs_sold);
        println!("  Total revenue: {} coins", self.total_revenue);
        println!("  Global unique OIDs: {}", self.global_oids.len());
        
        println!("\n🏆 Top Processors:");
        let mut sorted = self.nodes.clone();
        sorted.sort_by(|a, b| b.earnings.cmp(&a.earnings));
        
        for node in sorted.iter().take(5) {
            println!("  Node {}: {} packs, {} objects, {} dups, {} coins earned",
                     node.id, node.packs_processed, node.objects_found, 
                     node.duplicates_found, node.earnings);
        }
    }
}

fn main() {
    init_rand();
    
    println!("🏪 Git Pack Market\n");
    
    let submodules = std::env::home_dir()
        .map(|h| h.join("nix/vendor/rust/cargo2nix/submodules"))
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or_else(|| ".".to_string());
    
    println!("📦 Finding packs in {}\n", submodules);
    
    // Find packs
    let output = Command::new("find")
        .args(&[&submodules, "-name", "*.pack", "-type", "f"])
        .output();
    
    let packs: Vec<GitPack> = if let Ok(out) = output {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .take(20)
            .map(|path| GitPack {
                path: path.to_string(),
                estimated_size: random_u64() % 10000 + 1000,
                estimated_objects: 100,
            })
            .collect()
    } else {
        Vec::new()
    };
    
    println!("Found {} packs\n", packs.len());
    
    let mut market = PackMarket::new(24);
    let mut store = ContentStore::new("/tmp/git-pack-market");
    
    market.sell_packs(packs, &mut store);
    
    market.report();
    store.report();
    
    let parquet = "/tmp/git-pack-market/git_objects.parquet";
    if let Ok(_) = store.save_to_parquet(parquet) {
        if let Ok(meta) = std::fs::metadata(parquet) {
            println!("\n  ✓ Saved to {} ({} bytes)", parquet, meta.len());
        }
    }
    
    println!("\n✅ Pack market complete!");
    println!("\n💡 Key insights:");
    println!("  • 24 nodes compete for git packs");
    println!("  • Earn 10 coins per unique object");
    println!("  • Detect and track duplicates");
    println!("  • Store objects in content store");
}
