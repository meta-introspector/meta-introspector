use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Debug)]
struct MarkovNode {
    state: String,
    transitions: HashMap<String, (usize, f64)>, // next_state -> (count, probability)
    total: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut markov: HashMap<String, MarkovNode> = HashMap::new();
    
    println!("🔄 Building Markov model...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let label = label_col.value(i);
            let chars: Vec<String> = label.chars().map(|c| c.to_string()).collect();
            
            for window in chars.windows(2) {
                let from = &window[0];
                let to = &window[1];
                
                let node = markov.entry(from.clone()).or_insert_with(|| MarkovNode {
                    state: from.clone(),
                    transitions: HashMap::new(),
                    total: 0,
                });
                
                let entry = node.transitions.entry(to.clone()).or_insert((0, 0.0));
                entry.0 += 1;
                node.total += 1;
            }
        }
    }
    
    // Calculate probabilities
    for node in markov.values_mut() {
        for (_, (count, prob)) in node.transitions.iter_mut() {
            *prob = *count as f64 / node.total as f64;
        }
    }
    
    println!("✅ Markov model built: {} states\n", markov.len());
    
    // Find top N starting nodes by total transitions
    let mut top_nodes: Vec<_> = markov.values().collect();
    top_nodes.sort_by(|a, b| b.total.cmp(&a.total));
    
    let depth = 5;
    let n_nodes = 10;
    
    println!("📊 Reconstructing probability trees (depth={}, top_n={})\n", depth, n_nodes);
    
    let mut visited = HashSet::new();
    
    for (idx, start_node) in top_nodes.iter().take(n_nodes).enumerate() {
        if visited.contains(&start_node.state) {
            continue;
        }
        
        println!("{}. Starting from '{}' (total: {})", idx + 1, start_node.state, start_node.total);
        reconstruct_tree(&markov, &start_node.state, depth, 0, 1.0, &mut visited);
        println!();
    }
    
    Ok(())
}

fn reconstruct_tree(
    markov: &HashMap<String, MarkovNode>,
    state: &str,
    max_depth: usize,
    current_depth: usize,
    cumulative_prob: f64,
    visited: &mut HashSet<String>,
) {
    if current_depth >= max_depth || cumulative_prob < 0.01 {
        return;
    }
    
    visited.insert(state.to_string());
    
    if let Some(node) = markov.get(state) {
        let mut trans: Vec<_> = node.transitions.iter().collect();
        trans.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());
        
        for (next_state, (count, prob)) in trans.iter().take(3) {
            let indent = "  ".repeat(current_depth + 1);
            let new_prob = cumulative_prob * prob;
            println!("{}→ '{}' (p={:.3}, count={}, cum_p={:.3})", 
                indent, next_state, prob, count, new_prob);
            
            reconstruct_tree(markov, next_state, max_depth, current_depth + 1, new_prob, visited);
        }
    }
}
