use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Debug)]
struct MarkovNode {
    state: String,
    transitions: HashMap<String, (usize, f64)>,
    total: usize,
}

struct PathStats {
    total_paths: usize,
    total_leaves: usize,
    max_depth_reached: usize,
    paths_by_depth: HashMap<usize, usize>,
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
    
    for node in markov.values_mut() {
        for (_, (count, prob)) in node.transitions.iter_mut() {
            *prob = *count as f64 / node.total as f64;
        }
    }
    
    println!("✅ Markov model: {} states\n", markov.len());
    
    let depth = 10;
    let start = ".";
    
    println!("🌳 Full traversal from '{}' (depth={})\n", start, depth);
    
    let mut visited = HashSet::new();
    let mut stats = PathStats {
        total_paths: 0,
        total_leaves: 0,
        max_depth_reached: 0,
        paths_by_depth: HashMap::new(),
    };
    
    traverse_all(&markov, start, depth, 0, 1.0, String::new(), &mut visited, &mut stats);
    
    println!("\n📊 Traversal Summary:");
    println!("   Total paths explored: {}", stats.total_paths);
    println!("   Total leaf nodes: {}", stats.total_leaves);
    println!("   Max depth reached: {}", stats.max_depth_reached);
    println!("   Unique states visited: {}", visited.len());
    println!("\n   Paths by depth:");
    
    let mut depths: Vec<_> = stats.paths_by_depth.iter().collect();
    depths.sort_by_key(|&(d, _)| d);
    for (depth, count) in depths {
        println!("     Depth {}: {} paths", depth, count);
    }
    
    Ok(())
}

fn traverse_all(
    markov: &HashMap<String, MarkovNode>,
    state: &str,
    max_depth: usize,
    current_depth: usize,
    cumulative_prob: f64,
    path: String,
    visited: &mut HashSet<String>,
    stats: &mut PathStats,
) {
    let current_path = if path.is_empty() {
        state.to_string()
    } else {
        format!("{} → {}", path, state)
    };
    
    stats.total_paths += 1;
    *stats.paths_by_depth.entry(current_depth).or_insert(0) += 1;
    
    if current_depth > stats.max_depth_reached {
        stats.max_depth_reached = current_depth;
    }
    
    if current_depth >= max_depth || cumulative_prob < 0.001 {
        stats.total_leaves += 1;
        println!("🍃 [Depth {}] {} (p={:.4})", current_depth, current_path, cumulative_prob);
        return;
    }
    
    visited.insert(state.to_string());
    
    if let Some(node) = markov.get(state) {
        let mut trans: Vec<_> = node.transitions.iter().collect();
        trans.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());
        
        for (next_state, (_, prob)) in trans {
            let new_prob = cumulative_prob * prob;
            traverse_all(markov, next_state, max_depth, current_depth + 1, new_prob, current_path.clone(), visited, stats);
        }
    } else {
        stats.total_leaves += 1;
        println!("🍃 [Depth {}] {} (p={:.4}) [TERMINAL]", current_depth, current_path, cumulative_prob);
    }
}
