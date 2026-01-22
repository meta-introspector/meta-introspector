// Topological Function Matrix: Build order creates orthogonal hierarchy
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;

/// A function's unique position in the topological matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionPosition {
    /// Topological layer (build depth from base)
    pub layer: usize,
    
    /// Index within layer
    pub index: usize,
    
    /// Instruction pointer
    pub ip: u64,
    
    /// Galois field size for this layer
    pub gf_size: usize,
    
    /// Orthogonal projection from previous layer
    pub projection: f64,
    
    /// Harmonic resonance with base layer (fundamental frequency)
    /// Base = 1.0, higher layers = integer multiples (harmonics)
    pub harmonic: f64,
    
    /// Harmonic number: 1 = fundamental, 2 = first overtone, 3 = second overtone, etc.
    /// Layer 0 (base) = harmonic 1
    /// Layer 1 = harmonic 2
    /// Layer 2 = harmonic 3
    pub harmonic_number: usize,
}

/// Build node in topological graph
#[derive(Debug, Clone)]
pub struct BuildNode {
    pub name: String,
    pub functions: Vec<u64>,  // IPs from perf data
    pub gf_size: usize,
}

/// Topological Function Matrix
pub struct FunctionMatrix {
    /// Build dependency graph
    graph: DiGraph<BuildNode, ()>,
    
    /// Topological ordering (Mes → ... → High-level)
    topo_order: Vec<NodeIndex>,
    
    /// Function IP → Matrix position
    positions: HashMap<u64, FunctionPosition>,
    
    /// Matrix dimensions
    num_layers: usize,
    max_functions_per_layer: usize,
}

impl FunctionMatrix {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            topo_order: Vec::new(),
            positions: HashMap::new(),
            num_layers: 0,
            max_functions_per_layer: 0,
        }
    }
    
    /// Add build node to topology
    pub fn add_node(&mut self, node: BuildNode) -> NodeIndex {
        self.graph.add_node(node)
    }
    
    /// Add dependency edge (from → to)
    pub fn add_dependency(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }
    
    /// Compute topological ordering and assign matrix positions
    pub fn compute_matrix(&mut self) -> Result<(), String> {
        // Topological sort (Mes first, high-level last)
        self.topo_order = toposort(&self.graph, None)
            .map_err(|_| "Cycle detected in build graph")?;
        
        self.num_layers = self.topo_order.len();
        
        println!("🔬 Computing topological function matrix...");
        println!("   Layers: {}", self.num_layers);
        
        // Assign each function to matrix position
        for (layer, &node_idx) in self.topo_order.iter().enumerate() {
            let node = &self.graph[node_idx];
            
            println!("   Layer {}: {} ({} functions, GF(2^{}))", 
                layer, node.name, node.functions.len(), 
                (node.gf_size as f64).log2() as usize);
            
            self.max_functions_per_layer = self.max_functions_per_layer.max(node.functions.len());
            
            // Assign position to each function
            for (index, &ip) in node.functions.iter().enumerate() {
                // Compute orthogonal projection from previous layer
                let projection = if layer > 0 {
                    self.compute_projection(ip, layer - 1)
                } else {
                    1.0  // Base layer (Mes)
                };
                
                self.positions.insert(ip, FunctionPosition {
                    layer,
                    index,
                    ip,
                    gf_size: node.gf_size,
                    projection,
                    harmonic: (layer + 1) as f64,  // Harmonic = layer + 1
                    harmonic_number: layer + 1,
                });
            }
        }
        
        println!("   Matrix size: {} layers × {} functions", 
            self.num_layers, self.max_functions_per_layer);
        println!("   Total functions: {}", self.positions.len());
        
        Ok(())
    }
    
    /// Compute orthogonal projection from previous layer
    fn compute_projection(&self, ip: u64, prev_layer: usize) -> f64 {
        let prev_node_idx = self.topo_order[prev_layer];
        let prev_node = &self.graph[prev_node_idx];
        
        // Find nearest function in previous layer
        let min_dist = prev_node.functions.iter()
            .map(|&prev_ip| (ip as i64 - prev_ip as i64).abs())
            .min()
            .unwrap_or(i64::MAX);
        
        // Projection strength inversely proportional to distance
        1.0 / (1.0 + min_dist as f64 / 1000.0)
    }
    
    /// Get function position in matrix
    pub fn position_of(&self, ip: u64) -> Option<&FunctionPosition> {
        self.positions.get(&ip)
    }
    
    /// Get all functions at a layer
    pub fn functions_at_layer(&self, layer: usize) -> Vec<&FunctionPosition> {
        self.positions.values()
            .filter(|pos| pos.layer == layer)
            .collect()
    }
    
    /// Export matrix as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct MatrixExport {
            num_layers: usize,
            max_functions_per_layer: usize,
            total_functions: usize,
            positions: Vec<FunctionPosition>,
        }
        
        let export = MatrixExport {
            num_layers: self.num_layers,
            max_functions_per_layer: self.max_functions_per_layer,
            total_functions: self.positions.len(),
            positions: self.positions.values().cloned().collect(),
        };
        
        serde_json::to_string_pretty(&export)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topological_matrix() {
        let mut matrix = FunctionMatrix::new();
        
        // Create simple topology: mes → gcc → rust
        let mes = matrix.add_node(BuildNode {
            name: "mes".to_string(),
            functions: vec![0x400000, 0x400010],
            gf_size: 1 << 19,
        });
        
        let gcc = matrix.add_node(BuildNode {
            name: "gcc".to_string(),
            functions: vec![0x500000, 0x500010],
            gf_size: 1 << 21,
        });
        
        let rust = matrix.add_node(BuildNode {
            name: "rust".to_string(),
            functions: vec![0x600000],
            gf_size: 1 << 22,
        });
        
        matrix.add_dependency(mes, gcc);
        matrix.add_dependency(gcc, rust);
        
        matrix.compute_matrix().unwrap();
        
        // Check positions
        assert_eq!(matrix.position_of(0x400000).unwrap().layer, 0);
        assert_eq!(matrix.position_of(0x500000).unwrap().layer, 1);
        assert_eq!(matrix.position_of(0x600000).unwrap().layer, 2);
        
        assert_eq!(matrix.num_layers, 3);
    }
}
