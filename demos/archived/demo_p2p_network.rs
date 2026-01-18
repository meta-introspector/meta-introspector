// Demo: P2P network sharing findings to HuggingFace

#[path = "../../p2p_network.rs"] mod p2p_network;
#[path = "../../content_addressable_store.rs"] mod content_addressable_store;
#[path = "../../rand_shim.rs"] mod rand_shim;

use p2p_network::{P2PNetwork, Finding};
use content_addressable_store::ContentStore;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🌐 P2P Network: Share Findings via libp2p + HuggingFace\n");
    
    // Create network
    let mut network = P2PNetwork::new(24);
    
    println!("📦 Simulating local discoveries...\n");
    
    // Simulate each node making discoveries
    for i in 0..network.nodes.len() {
        let node = &mut network.nodes[i];
        
        // Add some findings
        for j in 0..5 {
            let finding = Finding {
                hash: format!("hash_{}_{}", i, j),
                data_type: if j % 3 == 0 { "rustc_ip" } else if j % 3 == 1 { "git_oid" } else { "snippet" }.to_string(),
                compression_ratio: 2.5 + (i as f64 * 0.1),
                source: format!("node_{}", i),
                timestamp: 1000 + i as u64,
            };
            
            node.add_finding(finding);
        }
        
        if i < 3 {
            node.report();
        }
    }
    
    println!("\n📡 Broadcasting findings across network...\n");
    
    network.broadcast_findings();
    
    network.report();
    
    // Export to HuggingFace format
    println!("\n💾 Exporting to HuggingFace dataset format...\n");
    
    let hf_path = "/tmp/huggingface-datasets";
    std::fs::create_dir_all(hf_path).ok();
    
    network.export_all_to_huggingface(hf_path);
    
    println!("  ✓ Exported to {}", hf_path);
    
    // Show sample
    if let Ok(sample) = std::fs::read_to_string(format!("{}/node_0.json", hf_path)) {
        println!("\n  Sample (node_0.json):");
        for line in sample.lines().take(8) {
            println!("    {}", line);
        }
    }
    
    // Integrate with content store
    println!("\n📦 Integrating with content store...\n");
    
    let mut store = ContentStore::new("/tmp/p2p-findings");
    
    for node in &network.nodes {
        for finding in node.local_findings.values() {
            let content = format!("{}-{}-{}", finding.hash, finding.data_type, finding.compression_ratio);
            store.store(&content);
        }
    }
    
    store.report();
    
    let parquet = "/tmp/p2p-findings/findings.parquet";
    if store.save_to_parquet(parquet).is_ok() {
        if let Ok(meta) = std::fs::metadata(parquet) {
            println!("\n  ✓ Saved to {} ({} bytes)", parquet, meta.len());
        }
    }
    
    println!("\n✅ P2P network ready!");
    println!("\n💡 Next steps:");
    println!("  • Add real libp2p for network transport");
    println!("  • Push datasets to HuggingFace Hub");
    println!("  • Add IPFS for content distribution");
    println!("  • Implement gossip protocol for findings");
    println!("  • Add reputation system for nodes");
    println!("  • Enable cross-internet discovery");
}
