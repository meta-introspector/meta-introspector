// Logistical Graph Builder - Order symbols by dependencies, build order, call chains
// Creates DAG from Nix/Cargo/linker data for topological analysis

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{toposort, is_cyclic_directed};
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use serde_json::Value;

/// Node in the logistical graph
#[derive(Debug, Clone)]
pub struct LogisticalNode {
    pub symbol: String,
    pub crate_name: String,
    pub nix_derivation: String,
    pub build_order: usize,
    pub call_count: u64,
    pub lmfdb_conductor: u32,
}

/// Edge types in the graph
#[derive(Debug, Clone)]
pub enum LogisticalEdge {
    BuildDependency,    // A must be built before B
    LinkDependency,     // A links against B
    CallDependency,     // A calls B at runtime
    NixDependency,      // A is in B's buildInputs
}

/// Build logistical graph from captured callbacks
pub fn build_logistical_graph(
    callbacks: &[Value],
    symbols: &[Value],
) -> DiGraph<LogisticalNode, LogisticalEdge> {
    let mut graph = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    
    // Phase 1: Add nodes from symbols
    for symbol_data in symbols {
        let symbol = symbol_data["symbol"].as_str().unwrap_or("unknown");
        let node = LogisticalNode {
            symbol: symbol.to_string(),
            crate_name: symbol_data["build_context"]["cargo"]["cargo_pkg_name"]
                .as_str().unwrap_or("unknown").to_string(),
            nix_derivation: symbol_data["build_context"]["nix"]["name"]
                .as_str().unwrap_or("unknown").to_string(),
            build_order: 0, // Will be computed
            call_count: 0,
            lmfdb_conductor: symbol_data["lmfdb_conductor"].as_u64().unwrap_or(3000) as u32,
        };
        
        let idx = graph.add_node(node);
        node_map.insert(symbol.to_string(), idx);
    }
    
    // Phase 2: Add build dependencies from Cargo
    for callback in callbacks.iter().filter(|c| c["callback"] == "cargo") {
        let crate_name = callback["cargo_pkg_name"].as_str().unwrap_or("");
        
        // Parse DEP_ environment variables for dependencies
        if let Some(deps) = callback["dependencies"].as_array() {
            for dep in deps {
                let dep_name = dep.as_str().unwrap_or("");
                add_edge_between_crates(&mut graph, &node_map, crate_name, dep_name, 
                    LogisticalEdge::BuildDependency);
            }
        }
    }
    
    // Phase 3: Add link dependencies from linker
    for callback in callbacks.iter().filter(|c| c["callback"] == "linker") {
        let binary = callback["binary"].as_str().unwrap_or("");
        
        if let Some(libs) = callback["libraries"].as_array() {
            for lib in libs {
                let lib_name = lib.as_str().unwrap_or("");
                add_edge_for_library(&mut graph, &node_map, binary, lib_name,
                    LogisticalEdge::LinkDependency);
            }
        }
    }
    
    // Phase 4: Add Nix dependencies
    for callback in callbacks.iter().filter(|c| c["callback"] == "nix") {
        let derivation = callback["name"].as_str().unwrap_or("");
        
        if let Some(inputs) = callback["build_inputs"].as_str() {
            for input in inputs.split_whitespace() {
                add_edge_for_nix(&mut graph, &node_map, derivation, input,
                    LogisticalEdge::NixDependency);
            }
        }
    }
    
    graph
}

/// Compute topological order (build order)
pub fn compute_build_order(graph: &mut DiGraph<LogisticalNode, LogisticalEdge>) -> Vec<String> {
    if is_cyclic_directed(&*graph) {
        eprintln!("⚠️  Warning: Cycle detected in dependency graph");
        return vec![];
    }
    
    match toposort(&*graph, None) {
        Ok(order) => {
            // Update build_order in nodes
            for (build_order, &node_idx) in order.iter().enumerate() {
                graph[node_idx].build_order = build_order;
            }
            
            order.iter()
                .map(|&idx| graph[idx].symbol.clone())
                .collect()
        }
        Err(_) => vec![],
    }
}

/// Export graph to DOT format for visualization
pub fn export_dot(graph: &DiGraph<LogisticalNode, LogisticalEdge>) -> String {
    format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]))
}

/// Analyze critical path (longest dependency chain)
pub fn find_critical_path(_graph: &DiGraph<LogisticalNode, LogisticalEdge>) -> Vec<String> {
    // TODO: Implement longest path algorithm
    vec![]
}

/// Cluster by LMFDB conductor
pub fn cluster_by_conductor(
    graph: &DiGraph<LogisticalNode, LogisticalEdge>
) -> HashMap<u32, Vec<String>> {
    let mut clusters: HashMap<u32, Vec<String>> = HashMap::new();
    
    for node in graph.node_weights() {
        let conductor_tier = (node.lmfdb_conductor / 1000) * 1000; // Round to nearest 1000
        clusters.entry(conductor_tier)
            .or_default()
            .push(node.symbol.clone());
    }
    
    clusters
}

// Helper functions
fn add_edge_between_crates(
    graph: &mut DiGraph<LogisticalNode, LogisticalEdge>,
    node_map: &HashMap<String, NodeIndex>,
    from_crate: &str,
    to_crate: &str,
    edge_type: LogisticalEdge,
) {
    // Find nodes in these crates and add edges
    for (_from_sym, &from_idx) in node_map.iter() {
        if graph[from_idx].crate_name == from_crate {
            for (_to_sym, &to_idx) in node_map.iter() {
                if graph[to_idx].crate_name == to_crate {
                    graph.add_edge(from_idx, to_idx, edge_type.clone());
                }
            }
        }
    }
}

fn add_edge_for_library(
    _graph: &mut DiGraph<LogisticalNode, LogisticalEdge>,
    _node_map: &HashMap<String, NodeIndex>,
    _binary: &str,
    _library: &str,
    _edge_type: LogisticalEdge,
) {
    // Add edges from binary symbols to library symbols
    // TODO: Parse binary and library to find actual symbol dependencies
}

fn add_edge_for_nix(
    graph: &mut DiGraph<LogisticalNode, LogisticalEdge>,
    node_map: &HashMap<String, NodeIndex>,
    derivation: &str,
    input: &str,
    edge_type: LogisticalEdge,
) {
    // Add edges based on Nix derivation dependencies
    for (_sym, &idx) in node_map.iter() {
        if graph[idx].nix_derivation == derivation {
            // Find symbols from input derivation
            for (_input_sym, &input_idx) in node_map.iter() {
                if graph[input_idx].nix_derivation.contains(input) {
                    graph.add_edge(input_idx, idx, edge_type.clone());
                }
            }
        }
    }
}


fn main() {
    println!("logistical_graph - library, add usage here");
}
