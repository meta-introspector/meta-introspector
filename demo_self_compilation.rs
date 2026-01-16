// Demo: 24 nodes buy and process source code snippets

mod self_compilation_queue;
mod rand_shim;

use self_compilation_queue::{SelfCompilationQueue, NodeJob};
use rand_shim::{init_rand, random_usize};

fn main() {
    init_rand();
    
    println!("🔄 Self-Compilation Job Queue\n");
    println!("System compiles itself, nodes buy snippets that reach new coverage\n");
    
    // Step 1: Load our own source
    let mut queue = SelfCompilationQueue::new();
    if let Err(e) = queue.load_self_source() {
        println!("Error loading source: {}", e);
        return;
    }
    
    // Step 2: Compress all snippets
    println!("\n🗜️  Compressing snippets...");
    queue.compress_snippets();
    
    let total_size: usize = queue.snippets.iter().map(|s| s.code.len()).sum();
    let compressed_size: usize = queue.snippets.iter().map(|s| s.compressed_size).sum();
    println!("  Original: {} bytes", total_size);
    println!("  Compressed: {} bytes", compressed_size);
    println!("  Ratio: {:.2}x", total_size as f64 / compressed_size as f64);
    
    // Step 3: Create 24 nodes with budgets
    println!("\n👥 Creating 24 nodes...");
    let mut nodes: Vec<NodeJob> = (0..24)
        .map(|i| NodeJob::new(i, 10000))
        .collect();
    
    // Step 4: Nodes buy and process snippets
    println!("\n💼 Nodes buying and processing snippets...\n");
    
    let rounds = 10.min(queue.snippets.len());
    for round in 0..rounds {
        println!("📊 Round {}", round);
        
        // Each node tries to buy a snippet
        for node in &mut nodes {
            if node.balance < 10 {
                continue;  // Can't afford anything
            }
            
            // Pick random snippet
            let snippet_idx = random_usize() % queue.snippets.len();
            let snippet_id = queue.snippets[snippet_idx].id;
            let snippet_price = queue.snippets[snippet_idx].price;
            
            // Try to buy
            if node.balance >= snippet_price {
                node.balance -= snippet_price;
                node.processed_snippets.push(snippet_id);
                
                // Process it
                let score = node.process_snippet(&mut queue, snippet_id);
                
                if score > 0.0 {
                    println!("  Node {} processed snippet {} - score: {:.2}", 
                             node.node_id, snippet_id, score);
                }
            }
        }
        
        println!("  Coverage: {} nodes", queue.processed_nodes.len());
    }
    
    // Final report
    queue.report();
    
    println!("\n👥 Node Results:");
    let mut sorted_nodes = nodes.clone();
    sorted_nodes.sort_by_key(|n| n.coverage_gained);
    sorted_nodes.reverse();
    
    for (i, node) in sorted_nodes.iter().take(5).enumerate() {
        println!("  {}. Node {}: {} snippets, {} coverage, {} coins left",
                 i + 1,
                 node.node_id,
                 node.processed_snippets.len(),
                 node.coverage_gained,
                 node.balance);
    }
    
    println!("\n✅ Self-compilation complete!");
    println!("\n💡 Key insights:");
    println!("  • System reads its own source in drips");
    println!("  • Nodes buy snippets that reach new coverage");
    println!("  • Perf trace = execution profile");
    println!("  • Minimal snippets that reach new nodes = valuable");
    println!("  • Job queue forces incremental processing");
}
