// Demo: Branch Prediction Mining Market

mod branch_prediction_miner;
use branch_prediction_miner::*;

fn main() {
    println!("🪙 BRANCH PREDICTION MINING MARKET");
    println!("==================================\n");
    println!("Miners extract rustc branch predictions from LLM models");
    println!("Earn 50 coins per unique branch location discovered!\n");
    
    // Create market with 24 miners
    let mut market = BranchMarket::new(24);
    
    // Sample rustc code snippets with branches
    let rustc_snippets = vec![
        "if tcx.sess.opts.debugging_opts.print_type_sizes { ... }".to_string(),
        "match self.kind { TyKind::Int(_) => true, _ => false }".to_string(),
        "if let Some(def_id) = trait_ref.def_id() { ... }".to_string(),
        "for item in items.iter() { if item.is_fn() { ... } }".to_string(),
        "match mir.basic_blocks()[bb].terminator().kind { ... }".to_string(),
    ];
    
    // LLM models to query
    let llm_models = vec![
        "codellama:7b".to_string(),
        "deepseek-coder:6.7b".to_string(),
        "starcoder:3b".to_string(),
    ];
    
    println!("📋 Mining Configuration:");
    println!("  Miners: {}", market.miners.len());
    println!("  Rustc snippets: {}", rustc_snippets.len());
    println!("  LLM models: {}", llm_models.len());
    println!("  Reward: 50 coins per unique branch\n");
    
    // Run mining round
    market.run_mining_round(&rustc_snippets, &llm_models);
    
    // Show top miners
    println!("\n🏆 Top Miners:");
    let mut sorted_miners = market.miners.clone();
    sorted_miners.sort_by_key(|m| std::cmp::Reverse(m.coins_earned));
    
    for (i, miner) in sorted_miners.iter().take(5).enumerate() {
        println!("  {}. Miner {} - {} coins ({} branches)",
            i + 1,
            miner.miner_id,
            miner.coins_earned,
            miner.branches_found.len()
        );
    }
    
    // Show sample branch predictions
    if let Some(miner) = sorted_miners.first() {
        if let Some(branch) = miner.branches_found.first() {
            println!("\n📊 Sample Branch Prediction:");
            println!("  Location: {}", branch.source_location);
            println!("  Type: {}", branch.branch_type);
            println!("  Taken probability: {:.2}%", branch.taken_probability * 100.0);
            println!("  LLM confidence: {:.2}%", branch.llm_confidence * 100.0);
        }
    }
    
    // Export to HuggingFace
    if let Ok(()) = market.export_to_huggingface() {
        println!("\n✅ Branch predictions exported to:");
        println!("   https://huggingface.co/datasets/introspector/rust/branch-predictions");
    }
    
    println!("\n💡 Use Case:");
    println!("  • Profile-guided optimization without profiling");
    println!("  • Predict hot paths in rustc compilation");
    println!("  • Optimize branch layout based on LLM predictions");
    println!("  • Train better code generation models");
    println!("  • Discover common compilation patterns");
}
