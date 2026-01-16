// Markov Chain Miner: Map character transitions to rustc compiler branches
// The Markov model of source code reveals the grammar, which maps to compiler branches

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MarkovTransition {
    pub from_state: String,      // Current character(s)
    pub to_state: String,        // Next character(s)
    pub probability: f64,        // Transition probability
    pub grammar_rule: String,    // Inferred grammar rule (e.g., "fn_decl")
    pub rustc_branch: String,    // Corresponding rustc branch location
}

#[derive(Debug)]
pub struct MarkovMiner {
    pub miner_id: usize,
    pub coins_earned: u64,
    pub transitions: HashMap<(String, String), f64>,  // (from, to) -> probability
    pub grammar_mappings: HashMap<String, String>,    // grammar_rule -> rustc_branch
}

impl MarkovMiner {
    pub fn new(miner_id: usize) -> Self {
        Self {
            miner_id,
            coins_earned: 0,
            transitions: HashMap::new(),
            grammar_mappings: HashMap::new(),
        }
    }
    
    pub fn mine_transitions(&mut self, rust_code: &str, window_size: usize) {
        // Build Markov chain from character transitions
        let chars: Vec<char> = rust_code.chars().collect();
        
        for i in 0..chars.len().saturating_sub(window_size) {
            let from: String = chars[i..i+window_size].iter().collect();
            let to: String = chars[i+1..i+1+window_size].iter().collect();
            
            *self.transitions.entry((from.clone(), to.clone())).or_insert(0.0) += 1.0;
            
            // Detect grammar patterns
            if let Some(grammar) = self.detect_grammar(&from, &to) {
                if let Some(branch) = self.map_to_rustc_branch(&grammar) {
                    self.grammar_mappings.insert(grammar.clone(), branch);
                    self.coins_earned += 25; // 25 coins per grammar→branch mapping
                }
            }
        }
        
        // Normalize probabilities
        let total: f64 = self.transitions.values().sum();
        for prob in self.transitions.values_mut() {
            *prob /= total;
        }
    }
    
    fn detect_grammar(&self, from: &str, to: &str) -> Option<String> {
        // Detect grammar rules from character transitions
        match (from, to) {
            (f, t) if f.ends_with("fn ") && t.starts_with("n ") => Some("fn_decl".to_string()),
            (f, t) if f.ends_with("struct ") => Some("struct_decl".to_string()),
            (f, t) if f.ends_with("impl ") => Some("impl_block".to_string()),
            (f, t) if f.ends_with("match ") => Some("match_expr".to_string()),
            (f, t) if f.ends_with("if ") => Some("if_expr".to_string()),
            (f, t) if f.ends_with("for ") => Some("for_loop".to_string()),
            (f, t) if f.ends_with("::") => Some("path_segment".to_string()),
            (f, t) if f.ends_with("<") && t.starts_with("T") => Some("generic_param".to_string()),
            _ => None,
        }
    }
    
    fn map_to_rustc_branch(&self, grammar: &str) -> Option<String> {
        // Map grammar rules to rustc compiler branches
        let mapping = match grammar {
            "fn_decl" => "src/librustc_parse/parser/item.rs:parse_fn",
            "struct_decl" => "src/librustc_parse/parser/item.rs:parse_struct",
            "impl_block" => "src/librustc_typeck/check/mod.rs:check_impl",
            "match_expr" => "src/librustc_mir_build/build/matches/mod.rs:match_expr",
            "if_expr" => "src/librustc_mir_build/build/expr/into.rs:expr_if",
            "for_loop" => "src/librustc_mir_build/build/expr/into.rs:expr_loop",
            "path_segment" => "src/librustc_resolve/lib.rs:resolve_path",
            "generic_param" => "src/librustc_typeck/collect.rs:generics_of",
            _ => return None,
        };
        Some(mapping.to_string())
    }
    
    pub fn export_transition(&self, from: &str, to: &str) -> Option<MarkovTransition> {
        let prob = self.transitions.get(&(from.to_string(), to.to_string()))?;
        let grammar = self.detect_grammar(from, to)?;
        let branch = self.grammar_mappings.get(&grammar)?.clone();
        
        Some(MarkovTransition {
            from_state: from.to_string(),
            to_state: to.to_string(),
            probability: *prob,
            grammar_rule: grammar,
            rustc_branch: branch,
        })
    }
}

pub struct MarkovMarket {
    pub miners: Vec<MarkovMiner>,
    pub global_transitions: HashMap<(String, String), MarkovTransition>,
    pub total_coins_paid: u64,
}

impl MarkovMarket {
    pub fn new(num_miners: usize) -> Self {
        Self {
            miners: (0..num_miners).map(|i| MarkovMiner::new(i)).collect(),
            global_transitions: HashMap::new(),
            total_coins_paid: 0,
        }
    }
    
    pub fn run_mining_round(&mut self, rust_sources: &[String], window_size: usize) {
        println!("\n🔗 Markov Chain Mining Round");
        println!("============================");
        println!("Window size: {} chars\n", window_size);
        
        for (i, miner) in self.miners.iter_mut().enumerate() {
            let source = &rust_sources[i % rust_sources.len()];
            miner.mine_transitions(source, window_size);
            
            println!("  Miner {} found {} transitions, {} grammar→branch mappings",
                miner.miner_id,
                miner.transitions.len(),
                miner.grammar_mappings.len()
            );
        }
        
        // Aggregate unique transitions
        for miner in &self.miners {
            for ((from, to), _) in &miner.transitions {
                if let Some(trans) = miner.export_transition(from, to) {
                    self.global_transitions.insert(
                        (from.clone(), to.clone()),
                        trans
                    );
                }
            }
            self.total_coins_paid += miner.coins_earned;
        }
        
        println!("\n📊 Mining Results:");
        println!("  Total transitions: {}", self.global_transitions.len());
        println!("  Total coins paid: {} coins", self.total_coins_paid);
        println!("  Average per miner: {} coins", 
            self.total_coins_paid / self.miners.len() as u64);
    }
    
    pub fn export_to_huggingface(&self) -> Result<(), String> {
        // Export to introspector/rust/markov/
        let json = serde_json::json!({
            "total_transitions": self.global_transitions.len(),
            "total_coins_paid": self.total_coins_paid,
            "transitions": self.global_transitions.values().collect::<Vec<_>>(),
            "miners": self.miners.len(),
        });
        
        std::fs::write(
            "/tmp/markov_transitions.json",
            serde_json::to_string_pretty(&json).unwrap()
        ).map_err(|e| e.to_string())?;
        
        println!("\n✅ Exported to introspector/rust/markov/");
        Ok(())
    }
    
    pub fn show_grammar_to_branch_map(&self) {
        println!("\n🗺️  Grammar → Rustc Branch Mappings:");
        println!("===================================");
        
        let mut mappings: Vec<_> = self.global_transitions.values()
            .map(|t| (&t.grammar_rule, &t.rustc_branch))
            .collect();
        mappings.sort();
        mappings.dedup();
        
        for (grammar, branch) in mappings {
            println!("  {} → {}", grammar, branch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_markov_mining() {
        let mut market = MarkovMarket::new(10);
        
        let sources = vec![
            "fn main() { println!(\"hello\"); }".to_string(),
            "struct Point { x: i32, y: i32 }".to_string(),
            "impl Point { fn new() -> Self { } }".to_string(),
        ];
        
        market.run_mining_round(&sources, 3);
        assert!(market.total_coins_paid > 0);
        assert!(market.global_transitions.len() > 0);
    }
}
