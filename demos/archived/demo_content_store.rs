// Demo: Content addressable storage for source snippets

#[path = "../../content_addressable_store.rs"] mod content_addressable_store;
#[path = "../../rand_shim.rs"] mod rand_shim;

use content_addressable_store::ContentStore;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("📦 Content Addressable Store\n");
    
    let mut store = ContentStore::new("/tmp/meta-introspector-store");
    
    println!("Storing snippets from our code...\n");
    
    // Extract snippets from our files
    let files = vec![
        "/mnt/data1/meta-introspector/distributed_trading.rs",
        "/mnt/data1/meta-introspector/compression_memes.rs",
        "/mnt/data1/meta-introspector/meme_evolver.rs",
    ];
    
    let mut hashes = Vec::new();
    
    for file_path in files {
        if let Ok(source) = std::fs::read_to_string(file_path) {
            let file_name = file_path.split('/').next_back().unwrap_or(file_path);
            
            // Split into functions (simple: split on "fn ")
            let snippets: Vec<&str> = source.split("fn ").collect();
            
            println!("📄 {}: {} snippets", file_name, snippets.len());
            
            for (i, snippet) in snippets.iter().enumerate() {
                if snippet.len() > 50 {
                    let hash = store.store(snippet);
                    hashes.push(hash.clone());
                    
                    if i < 3 {
                        let display_hash = if hash.len() > 20 { &hash[..20] } else { &hash };
                        println!("  ✓ Snippet {} → {}", i, display_hash);
                    }
                }
            }
        }
    }
    
    println!();
    store.report();
    
    // Save to parquet
    println!("\n💾 Saving to parquet...\n");
    
    let parquet_path = "/tmp/meta-introspector-store/snippets.parquet";
    if store.save_to_parquet(parquet_path).is_ok() {
        if let Ok(metadata) = std::fs::metadata(parquet_path) {
            println!("  ✓ Saved to {}", parquet_path);
            println!("  ✓ Parquet size: {} bytes", metadata.len());
        }
    }
    
    // Test deduplication
    println!("\n🔄 Testing deduplication...\n");
    
    let test_snippet = "pub fn test() { println!(\"hello\"); }";
    let hash1 = store.store(test_snippet);
    let hash2 = store.store(test_snippet);
    
    println!("  First store: {}", hash1);
    println!("  Second store: {}", hash2);
    println!("  Same hash: {}", hash1 == hash2);
    
    store.report();
    
    // Test retrieval
    println!("\n🔍 Testing retrieval...\n");
    
    if !hashes.is_empty() {
        if let Some(content) = store.load(&hashes[0]) {
            let display_hash = if hashes[0].len() > 20 { &hashes[0][..20] } else { &hashes[0] };
            println!("  Retrieved {} bytes from {}", content.len(), display_hash);
        }
    }
    
    println!("\n✅ Content store working!");
    println!("\n💡 Key insights:");
    println!("  • Hash-based storage: objects/ab/cdef...");
    println!("  • Automatic deduplication");
    println!("  • Reference counting");
    println!("  • Fast lookup by hash");
    println!("  • Reusable snippets across projects");
}
