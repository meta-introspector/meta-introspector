// 🔥 MEMEPLEX ANALYZER
// Track meme propagation: "rust", "python", "emacslisp", "npm"
// Collect all usages to build memeplex eigenvector

use std::collections::HashMap;
use crate::system_eigenvector::*;
use crate::label_reach::*;

pub struct Memeplex {
    pub meme_name: String,
    pub occurrences: Vec<MemeOccurrence>,
    pub propagation_graph: HashMap<String, Vec<String>>,
    pub meme_eigenvector: Vec<f64>,
    pub influence_score: f64,
}

pub struct MemeOccurrence {
    pub location: MemeLocation,
    pub context: String,
    pub frequency: u32,
}

#[derive(Debug, Clone)]
pub enum MemeLocation {
    DirectoryName(String),      // rust-overlay-test/
    FileName(String),           // rustc_analyzer.rs
    FunctionName(String),       // fn rust_compile()
    DocComment(String),         // /// Rust implementation
    BinarySymbol(String),       // _ZN4rust...
    CargoToml(String),          // name = "rust-..."
}

impl Memeplex {
    pub fn analyze_meme(meme: &str, project_map: &crate::holistic_mapper::HolisticProjectMap) -> Self {
        let mut memeplex = Self {
            meme_name: meme.to_string(),
            occurrences: Vec::new(),
            propagation_graph: HashMap::new(),
            meme_eigenvector: Vec::new(),
            influence_score: 0.0,
        };
        
        // Collect all occurrences of the meme
        memeplex.collect_directory_occurrences(meme, &project_map.directory_structure);
        memeplex.collect_doc_occurrences(meme, &project_map.documentation_model);
        memeplex.collect_source_occurrences(meme, &project_map.source_models);
        memeplex.collect_binary_occurrences(meme, &project_map.binary_models);
        
        // Build propagation graph
        memeplex.build_propagation_graph();
        
        // Calculate meme eigenvector
        memeplex.calculate_meme_eigenvector();
        
        memeplex
    }
    
    fn collect_directory_occurrences(&mut self, meme: &str, dir_model: &crate::holistic_mapper::DirectoryModel) {
        for (pattern, count) in &dir_model.structure_patterns {
            if pattern.to_lowercase().contains(&meme.to_lowercase()) {
                self.occurrences.push(MemeOccurrence {
                    location: MemeLocation::DirectoryName(pattern.clone()),
                    context: format!("Directory structure: {}", pattern),
                    frequency: *count,
                });
            }
        }
        
        // Check file names
        for (ext, files) in &dir_model.file_type_clusters {
            for file_path in files {
                if file_path.to_lowercase().contains(&meme.to_lowercase()) {
                    self.occurrences.push(MemeOccurrence {
                        location: MemeLocation::FileName(file_path.clone()),
                        context: format!("File: {}", file_path),
                        frequency: 1,
                    });
                }
            }
        }
    }
    
    fn collect_doc_occurrences(&mut self, meme: &str, doc_model: &crate::holistic_mapper::DocumentationModel) {
        // Check Cargo.toml
        for (key, value) in &doc_model.cargo_toml {
            if key.to_lowercase().contains(&meme.to_lowercase()) || 
               value.to_lowercase().contains(&meme.to_lowercase()) {
                self.occurrences.push(MemeOccurrence {
                    location: MemeLocation::CargoToml(format!("{} = {}", key, value)),
                    context: format!("Cargo.toml: {} = {}", key, value),
                    frequency: 1,
                });
            }
        }
    }
    
    fn collect_source_occurrences(&mut self, meme: &str, source_models: &HashMap<String, crate::source_binary_mapper::SourceMarkovModel>) {
        for (file_path, model) in source_models {
            // Check tokens (function names, struct names, etc.)
            for (token, count) in &model.token_patterns {
                if token.to_lowercase().contains(&meme.to_lowercase()) {
                    self.occurrences.push(MemeOccurrence {
                        location: MemeLocation::FunctionName(token.clone()),
                        context: format!("Token in {}: {}", file_path, token),
                        frequency: *count,
                    });
                }
            }
            
            // Check word transitions (comments, strings)
            for (word, transitions) in &model.word_transitions {
                if word.to_lowercase().contains(&meme.to_lowercase()) {
                    let total_transitions: u32 = transitions.values().sum();
                    self.occurrences.push(MemeOccurrence {
                        location: MemeLocation::DocComment(word.clone()),
                        context: format!("Word in {}: {}", file_path, word),
                        frequency: total_transitions,
                    });
                }
            }
        }
    }
    
    fn collect_binary_occurrences(&mut self, meme: &str, binary_models: &HashMap<String, crate::binary_markov::BinaryMarkovModel>) {
        for (binary_path, _model) in binary_models {
            if binary_path.to_lowercase().contains(&meme.to_lowercase()) {
                self.occurrences.push(MemeOccurrence {
                    location: MemeLocation::BinarySymbol(binary_path.clone()),
                    context: format!("Binary: {}", binary_path),
                    frequency: 1,
                });
            }
        }
    }
    
    fn build_propagation_graph(&mut self) {
        // Build graph showing how meme propagates through system
        for occurrence in &self.occurrences {
            let source = self.location_to_node(&occurrence.location);
            
            // Find related occurrences (propagation edges)
            for other in &self.occurrences {
                if !std::ptr::eq(occurrence, other) {
                    let target = self.location_to_node(&other.location);
                    
                    if self.are_related(&occurrence.location, &other.location) {
                        self.propagation_graph
                            .entry(source.clone())
                            .or_insert_with(Vec::new)
                            .push(target);
                    }
                }
            }
        }
    }
    
    fn location_to_node(&self, location: &MemeLocation) -> String {
        match location {
            MemeLocation::DirectoryName(s) => format!("dir:{}", s),
            MemeLocation::FileName(s) => format!("file:{}", s),
            MemeLocation::FunctionName(s) => format!("fn:{}", s),
            MemeLocation::DocComment(s) => format!("doc:{}", s),
            MemeLocation::BinarySymbol(s) => format!("bin:{}", s),
            MemeLocation::CargoToml(s) => format!("cargo:{}", s),
        }
    }
    
    fn are_related(&self, loc1: &MemeLocation, loc2: &MemeLocation) -> bool {
        // Simple heuristic: same file or directory
        match (loc1, loc2) {
            (MemeLocation::FileName(f1), MemeLocation::FunctionName(_)) => true,
            (MemeLocation::DirectoryName(d), MemeLocation::FileName(f)) => f.starts_with(d),
            (MemeLocation::FunctionName(_), MemeLocation::BinarySymbol(_)) => true,
            _ => false,
        }
    }
    
    fn calculate_meme_eigenvector(&mut self) {
        // Calculate eigenvector centrality for meme propagation
        let nodes: Vec<_> = self.propagation_graph.keys().cloned().collect();
        let n = nodes.len();
        
        if n == 0 {
            return;
        }
        
        // Build adjacency matrix
        let mut adj_matrix = vec![vec![0.0; n]; n];
        
        for (i, node) in nodes.iter().enumerate() {
            if let Some(neighbors) = self.propagation_graph.get(node) {
                for neighbor in neighbors {
                    if let Some(j) = nodes.iter().position(|n| n == neighbor) {
                        adj_matrix[i][j] = 1.0;
                    }
                }
            }
        }
        
        // Power iteration for eigenvector
        let mut v = vec![1.0 / (n as f64); n];
        
        for _ in 0..50 {
            let mut v_new = vec![0.0; n];
            
            for i in 0..n {
                for j in 0..n {
                    v_new[i] += adj_matrix[j][i] * v[j];
                }
            }
            
            let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                v = v_new.iter().map(|x| x / norm).collect();
            }
        }
        
        self.meme_eigenvector = v;
        self.influence_score = self.meme_eigenvector.iter().sum::<f64>() / n as f64;
    }
    
    pub fn print_memeplex_analysis(&self) {
        println!("🧬 MEMEPLEX ANALYSIS: {}", self.meme_name.to_uppercase());
        println!("=====================================");
        
        println!("📊 Total occurrences: {}", self.occurrences.len());
        println!("🌐 Propagation nodes: {}", self.propagation_graph.len());
        println!("💪 Influence score: {:.6}", self.influence_score);
        
        println!("\n📍 Occurrence Distribution:");
        let mut by_type: HashMap<String, u32> = HashMap::new();
        for occ in &self.occurrences {
            let type_name = match &occ.location {
                MemeLocation::DirectoryName(_) => "Directory",
                MemeLocation::FileName(_) => "File",
                MemeLocation::FunctionName(_) => "Function",
                MemeLocation::DocComment(_) => "Documentation",
                MemeLocation::BinarySymbol(_) => "Binary",
                MemeLocation::CargoToml(_) => "Cargo.toml",
            };
            *by_type.entry(type_name.to_string()).or_insert(0) += 1;
        }
        
        for (type_name, count) in by_type {
            println!("  {}: {}", type_name, count);
        }
        
        println!("\n🔥 Top Propagation Hubs:");
        let nodes: Vec<_> = self.propagation_graph.keys().collect();
        let mut hub_scores: Vec<_> = nodes.iter()
            .enumerate()
            .map(|(i, node)| (node, if i < self.meme_eigenvector.len() { self.meme_eigenvector[i] } else { 0.0 }))
            .collect();
        hub_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (i, (node, score)) in hub_scores.iter().take(5).enumerate() {
            println!("  {}. {} (score: {:.6})", i+1, node, score);
        }
    }
}

pub fn compare_memeplexes(memes: &[Memeplex]) {
    println!("\n🔬 MEMEPLEX COMPARISON");
    println!("======================");
    
    for meme in memes {
        println!("{:12} | Occurrences: {:4} | Influence: {:.4} | Nodes: {:3}",
                meme.meme_name,
                meme.occurrences.len(),
                meme.influence_score,
                meme.propagation_graph.len());
    }
}
