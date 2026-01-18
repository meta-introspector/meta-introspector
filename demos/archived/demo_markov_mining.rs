// Demo: Markov Chain Mining - Map character transitions to rustc branches

#[path = "../../markov_chain_miner.rs"] mod markov_chain_miner;
use markov_chain_miner::*;

fn main() {
    println!("🔗 MARKOV CHAIN MINING MARKET");
    println!("==============================\n");
    println!("Character transitions → Grammar rules → Rustc branches");
    println!("Earn 25 coins per grammar→branch mapping!\n");
    
    // Create market with 24 miners
    let mut market = MarkovMarket::new(24);
    
    // Sample Rust source code
    let rust_sources = vec![
        r#"
fn parse_item(input: &str) -> Result<Item, Error> {
    match input.chars().next() {
        Some('f') if input.starts_with("fn ") => parse_fn(input),
        Some('s') if input.starts_with("struct ") => parse_struct(input),
        Some('i') if input.starts_with("impl ") => parse_impl(input),
        _ => Err(Error::UnknownItem),
    }
}
        "#.to_string(),
        
        r#"
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }
    
    fn parse_expr(&mut self) -> Expr {
        if self.peek() == "if" {
            self.parse_if()
        } else if self.peek() == "match" {
            self.parse_match()
        } else {
            self.parse_primary()
        }
    }
}
        "#.to_string(),
        
        r#"
fn compile<T: TypeCheck>(ast: Ast) -> Result<Mir, Error> {
    for node in ast.nodes {
        match node.kind {
            NodeKind::Fn => compile_fn(node),
            NodeKind::Struct => compile_struct(node),
            NodeKind::Impl => compile_impl(node),
            _ => continue,
        }
    }
    Ok(mir)
}
        "#.to_string(),
    ];
    
    println!("📋 Mining Configuration:");
    println!("  Miners: {}", market.miners.len());
    println!("  Source files: {}", rust_sources.len());
    println!("  Window size: 3 characters");
    println!("  Reward: 25 coins per grammar→branch mapping\n");
    
    // Run mining with 3-character window
    market.run_mining_round(&rust_sources, 3);
    
    // Show grammar to branch mappings
    market.show_grammar_to_branch_map();
    
    // Show top miners
    println!("\n🏆 Top Miners:");
    let mut sorted_miners = market.miners.clone();
    sorted_miners.sort_by_key(|m| std::cmp::Reverse(m.coins_earned));
    
    for (i, miner) in sorted_miners.iter().take(5).enumerate() {
        println!("  {}. Miner {} - {} coins ({} transitions, {} mappings)",
            i + 1,
            miner.miner_id,
            miner.coins_earned,
            miner.transitions.len(),
            miner.grammar_mappings.len()
        );
    }
    
    // Show sample transition
    if let Some(trans) = market.global_transitions.values().next() {
        println!("\n📊 Sample Markov Transition:");
        println!("  From: {:?}", trans.from_state);
        println!("  To: {:?}", trans.to_state);
        println!("  Probability: {:.4}", trans.probability);
        println!("  Grammar: {}", trans.grammar_rule);
        println!("  Rustc branch: {}", trans.rustc_branch);
    }
    
    // Export to HuggingFace
    if let Ok(()) = market.export_to_huggingface() {
        println!("\n✅ Markov transitions exported to:");
        println!("   https://huggingface.co/datasets/introspector/rust/markov");
    }
    
    println!("\n💡 The Insight:");
    println!("  Character transitions in source code reveal grammar");
    println!("  Grammar rules map directly to rustc compiler branches");
    println!("  'fn ' chars → parse_fn branch in rustc");
    println!("  'match ' chars → match_expr branch in rustc");
    println!("  'impl ' chars → check_impl branch in rustc");
    println!("\n  The Markov model of the code IS the compiler's control flow!");
}
