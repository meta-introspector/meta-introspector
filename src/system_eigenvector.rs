// 🔥 SYSTEM EIGENVECTOR CALCULATOR
// Topological + behavioral flow graphs → system eigenvector V

use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;
use crate::label_reach::*;

pub struct SystemEigenvector {
    pub flow_matrix: DMatrix<f64>,           // Topological structure
    pub transaction_matrix: DMatrix<f64>,    // Behavioral patterns
    pub combined_matrix: DMatrix<f64>,       // Topology + behavior
    pub eigenvector_v: DVector<f64>,         // System eigenvector V
    pub eigenvalue: f64,                     // Dominant eigenvalue
    pub node_importance: HashMap<String, f64>, // Node centrality scores
}

impl SystemEigenvector {
    pub fn from_label_reach(program: &LabelReachProgram) -> Self {
        let n = program.label_vectors.len() + program.action_vectors.len();
        
        let mut system = Self {
            flow_matrix: DMatrix::zeros(n, n),
            transaction_matrix: DMatrix::zeros(n, n),
            combined_matrix: DMatrix::zeros(n, n),
            eigenvector_v: DVector::zeros(n),
            eigenvalue: 0.0,
            node_importance: HashMap::new(),
        };
        
        // Build topological flow matrix
        system.build_topological_matrix(program);
        
        // Build behavioral transaction matrix
        system.build_behavioral_matrix(program);
        
        // Combine topology + behavior
        system.combine_matrices();
        
        // Calculate system eigenvector V
        system.calculate_eigenvector();
        
        // Calculate node importance scores
        system.calculate_node_importance(program);
        
        system
    }
    
    fn build_topological_matrix(&mut self, program: &LabelReachProgram) {
        // Map reach paths to matrix indices
        let labels: Vec<_> = program.label_vectors.keys().cloned().collect();
        let actions: Vec<_> = program.action_vectors.keys().cloned().collect();
        
        // Labels occupy indices 0..label_count
        // Actions occupy indices label_count..total
        let label_count = labels.len();
        
        for path in &program.reach_paths {
            if let (Some(label_idx), Some(action_idx)) = (
                labels.iter().position(|l| l == &path.label),
                actions.iter().position(|a| a == &path.action)
            ) {
                // Topological connection: label → action
                self.flow_matrix[(label_idx, label_count + action_idx)] = path.path_strength;
            }
        }
    }
    
    fn build_behavioral_matrix(&mut self, program: &LabelReachProgram) {
        let labels: Vec<_> = program.label_vectors.keys().cloned().collect();
        let actions: Vec<_> = program.action_vectors.keys().cloned().collect();
        let label_count = labels.len();
        
        // Behavioral patterns: frequency-weighted transactions
        for (i, label_key) in labels.iter().enumerate() {
            if let Some(label) = program.label_vectors.get(label_key) {
                // Self-reinforcement based on frequency
                self.transaction_matrix[(i, i)] = (label.frequency as f64).ln();
                
                // Cross-label behavioral connections
                for (j, other_label_key) in labels.iter().enumerate() {
                    if i != j {
                        if let Some(other_label) = program.label_vectors.get(other_label_key) {
                            // Behavioral similarity creates transactions
                            let behavioral_strength = self.calculate_behavioral_similarity(label, other_label);
                            self.transaction_matrix[(i, j)] = behavioral_strength;
                        }
                    }
                }
            }
        }
        
        // Action-to-action behavioral patterns
        for (i, action_key) in actions.iter().enumerate() {
            if let Some(action) = program.action_vectors.get(action_key) {
                let action_idx = label_count + i;
                
                // Self-reinforcement based on execution count
                self.transaction_matrix[(action_idx, action_idx)] = (action.execution_count as f64).ln();
                
                // Sequential action patterns (behavioral chains)
                for (j, other_action_key) in actions.iter().enumerate() {
                    if i != j {
                        if let Some(other_action) = program.action_vectors.get(other_action_key) {
                            let other_action_idx = label_count + j;
                            let sequence_strength = self.calculate_sequence_probability(action, other_action);
                            self.transaction_matrix[(action_idx, other_action_idx)] = sequence_strength;
                        }
                    }
                }
            }
        }
    }
    
    fn calculate_behavioral_similarity(&self, label1: &LabelVector, label2: &LabelVector) -> f64 {
        // Behavioral similarity based on domain and frequency patterns
        let domain_match = if std::mem::discriminant(&label1.domain) == std::mem::discriminant(&label2.domain) {
            0.5
        } else {
            0.1
        };
        
        let frequency_similarity = 1.0 / (1.0 + ((label1.frequency as f64) - (label2.frequency as f64)).abs());
        
        domain_match * frequency_similarity
    }
    
    fn calculate_sequence_probability(&self, action1: &ActionVector, action2: &ActionVector) -> f64 {
        // Behavioral sequence probability based on instruction patterns
        match (&action1.action.as_str(), &action2.action.as_str()) {
            ("PUSH", "CALL") => 0.8,  // Common sequence
            ("CALL", "POP") => 0.7,   // Return cleanup
            ("MOV", "MOV") => 0.6,    // Data movement chains
            ("PUSH", "PUSH") => 0.5,  // Stack building
            _ => 0.1,                 // Default low probability
        }
    }
    
    fn combine_matrices(&mut self) {
        // Combine topological structure + behavioral patterns
        // V = α * Topology + β * Behavior
        let alpha = 0.6;  // Topological weight
        let beta = 0.4;   // Behavioral weight
        
        self.combined_matrix = alpha * &self.flow_matrix + beta * &self.transaction_matrix;
        
        // Add small diagonal for numerical stability
        for i in 0..self.combined_matrix.nrows() {
            self.combined_matrix[(i, i)] += 0.01;
        }
    }
    
    fn calculate_eigenvector(&mut self) {
        // Calculate dominant eigenvector using power iteration
        let n = self.combined_matrix.nrows();
        let mut v = DVector::from_element(n, 1.0 / (n as f64)); // Initial uniform vector
        
        // Power iteration to find dominant eigenvector
        for _ in 0..100 {
            let v_new = &self.combined_matrix * &v;
            let norm = v_new.norm();
            
            if norm > 0.0 {
                v = v_new / norm;
                self.eigenvalue = (&self.combined_matrix * &v).dot(&v) / v.dot(&v);
            }
        }
        
        self.eigenvector_v = v;
    }
    
    fn calculate_node_importance(&mut self, program: &LabelReachProgram) {
        let labels: Vec<_> = program.label_vectors.keys().cloned().collect();
        let actions: Vec<_> = program.action_vectors.keys().cloned().collect();
        let label_count = labels.len();
        
        // Map eigenvector components to node importance
        for (i, label) in labels.iter().enumerate() {
            self.node_importance.insert(label.clone(), self.eigenvector_v[i]);
        }
        
        for (i, action) in actions.iter().enumerate() {
            self.node_importance.insert(action.clone(), self.eigenvector_v[label_count + i]);
        }
    }
    
    pub fn print_system_analysis(&self) {
        println!("🎯 SYSTEM EIGENVECTOR ANALYSIS");
        println!("==============================");
        
        println!("📊 Matrix size: {}x{}", self.combined_matrix.nrows(), self.combined_matrix.ncols());
        println!("🔢 Dominant eigenvalue: {:.6}", self.eigenvalue);
        println!("📈 Eigenvector norm: {:.6}", self.eigenvector_v.norm());
        
        println!("\n🏆 Top Important Nodes (by eigenvector centrality):");
        let mut importance_pairs: Vec<_> = self.node_importance.iter().collect();
        importance_pairs.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        for (i, (node, importance)) in importance_pairs.iter().take(10).enumerate() {
            println!("  {}. {} (importance: {:.6})", i+1, node, importance);
        }
        
        println!("\n🌊 System Properties:");
        println!("  Spectral radius: {:.6}", self.eigenvalue);
        println!("  System stability: {}", if self.eigenvalue < 1.0 { "Stable" } else { "Unstable" });
        println!("  Behavioral coherence: {:.3}", self.calculate_coherence());
    }
    
    fn calculate_coherence(&self) -> f64 {
        // Measure how well the system components align
        let variance = self.eigenvector_v.iter()
            .map(|&x| {
                let mean = self.eigenvector_v.mean();
                (x - mean).powi(2)
            })
            .sum::<f64>() / self.eigenvector_v.len() as f64;
            
        1.0 / (1.0 + variance) // Higher coherence = lower variance
    }
}
