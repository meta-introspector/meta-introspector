use std::collections::{HashMap, VecDeque};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct DepGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph: DepGraph = serde_json::from_str(&fs::read_to_string("dep_graph.json")?)?;
    
    // Build adjacency list
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for (from, to) in &graph.edges {
        adj.entry(*from).or_insert_with(Vec::new).push(*to);
    }
    
    // Find meta-introspector and rustc nodes
    let meta_nodes: Vec<_> = graph.nodes.iter().enumerate()
        .filter(|(_, n)| n.contains("meta-introspector"))
        .collect();
    
    let rustc_nodes: Vec<_> = graph.nodes.iter().enumerate()
        .filter(|(_, n)| n.contains("rustc") || n.contains("rust/compiler"))
        .collect();
    
    println!("📍 Meta-introspector nodes: {}", meta_nodes.len());
    for (idx, node) in meta_nodes.iter().take(10) {
        let depth = bfs_depth(&adj, *idx);
        println!("  [{}] depth={} {}", idx, depth, node);
    }
    
    println!("\n📍 Rustc nodes: {}", rustc_nodes.len());
    for (idx, node) in rustc_nodes.iter().take(10) {
        let depth = bfs_depth(&adj, *idx);
        println!("  [{}] depth={} {}", idx, depth, node);
    }
    
    Ok(())
}

fn bfs_depth(adj: &HashMap<usize, Vec<usize>>, start: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    let mut max_depth = 0;
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((node, depth)) = queue.pop_front() {
        max_depth = max_depth.max(depth);
        
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                if visited.insert(next) {
                    queue.push_back((next, depth + 1));
                }
            }
        }
    }
    
    max_depth
}
