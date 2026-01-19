use std::collections::{HashMap, HashSet};
use std::fs;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DepGraph {
    nodes: Vec<String>,  // crate paths
    edges: Vec<(usize, usize)>,  // (from, to) indices
    topo_order: Vec<usize>,  // topological order
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Building dependency graph...");
    
    let deps_groups: HashMap<String, Vec<String>> = 
        serde_json::from_str(&fs::read_to_string("cargo_deps_groups.json")?)?;
    
    let mut graph = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut nodes = Vec::new();
    
    // Add all crates as nodes
    for repos in deps_groups.values() {
        for repo in repos {
            if !node_map.contains_key(repo) {
                let idx = graph.add_node(repo.clone());
                node_map.insert(repo.clone(), idx);
                nodes.push(repo.clone());
            }
        }
    }
    
    println!("✅ Added {} nodes", nodes.len());
    
    // Add edges based on dependencies
    for repos in deps_groups.values() {
        for repo in repos {
            let cargo_toml = format!("{}/Cargo.toml", repo);
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                // Parse dependencies
                for line in content.lines() {
                    if line.contains("path = ") {
                        // Extract path dependency
                        if let Some(path) = line.split("path = \"").nth(1) {
                            if let Some(dep_path) = path.split('"').next() {
                                let full_path = format!("{}/{}", repo, dep_path);
                                if let Some(&dep_idx) = node_map.get(&full_path) {
                                    if let Some(&repo_idx) = node_map.get(repo) {
                                        graph.add_edge(repo_idx, dep_idx, ());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("✅ Added {} edges", graph.edge_count());
    
    // Topological sort
    let topo = match toposort(&graph, None) {
        Ok(order) => order.iter().map(|&idx| idx.index()).collect(),
        Err(_) => {
            println!("⚠️  Cycle detected in dependency graph!");
            Vec::new()
        }
    };
    
    let dep_graph = DepGraph {
        nodes,
        edges: graph.edge_indices()
            .map(|e| {
                let (a, b) = graph.edge_endpoints(e).unwrap();
                (a.index(), b.index())
            })
            .collect(),
        topo_order: topo,
    };
    
    fs::write("dep_graph.json", serde_json::to_string_pretty(&dep_graph)?)?;
    println!("💾 Saved to dep_graph.json");
    
    Ok(())
}
