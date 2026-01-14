// 🔥 LABEL REACH PROGRAM
// Flow vectors from labels → actions through holistic project mapping

use std::collections::HashMap;
use crate::holistic_mapper::*;

pub struct LabelReachProgram {
    pub label_vectors: HashMap<String, LabelVector>,
    pub action_vectors: HashMap<String, ActionVector>,
    pub flow_matrix: Vec<Vec<f64>>,
    pub reach_paths: Vec<ReachPath>,
}

pub struct LabelVector {
    pub label: String,
    pub domain: LabelDomain,
    pub embedding: Vec<f64>,
    pub frequency: u32,
}

pub struct ActionVector {
    pub action: String,
    pub domain: ActionDomain,
    pub embedding: Vec<f64>,
    pub execution_count: u32,
}

#[derive(Debug, Clone)]
pub enum LabelDomain {
    Directory,      // src/, docs/, target/
    Documentation,  // README headers, doc comments
    Source,         // function names, struct names
    Symbol,         // exported symbols
}

#[derive(Debug, Clone)]
pub enum ActionDomain {
    FileSystem,     // file operations
    Memory,         // malloc, free
    Network,        // socket operations
    Computation,    // CPU instructions
}

pub struct ReachPath {
    pub label: String,
    pub action: String,
    pub path_strength: f64,
    pub intermediate_nodes: Vec<String>,
}

impl LabelReachProgram {
    pub fn from_holistic_map(map: &HolisticProjectMap) -> Self {
        let mut program = Self {
            label_vectors: HashMap::new(),
            action_vectors: HashMap::new(),
            flow_matrix: Vec::new(),
            reach_paths: Vec::new(),
        };
        
        // Extract labels from all domains
        program.extract_directory_labels(&map.directory_structure);
        program.extract_doc_labels(&map.documentation_model);
        program.extract_source_labels(&map.source_models);
        
        // Extract actions from binary analysis
        program.extract_binary_actions(&map.binary_models);
        
        // Build flow matrix
        program.build_flow_matrix();
        
        // Calculate reach paths
        program.calculate_reach_paths();
        
        program
    }
    
    fn extract_directory_labels(&mut self, dir_model: &DirectoryModel) {
        for (pattern, count) in &dir_model.structure_patterns {
            let embedding = self.encode_directory_name(pattern);
            
            self.label_vectors.insert(pattern.clone(), LabelVector {
                label: pattern.clone(),
                domain: LabelDomain::Directory,
                embedding,
                frequency: *count,
            });
        }
    }
    
    fn extract_doc_labels(&mut self, doc_model: &DocumentationModel) {
        for (pattern, count) in &doc_model.readme_patterns {
            let embedding = self.encode_doc_pattern(pattern);
            
            self.label_vectors.insert(format!("doc_{}", pattern), LabelVector {
                label: pattern.clone(),
                domain: LabelDomain::Documentation,
                embedding,
                frequency: *count,
            });
        }
    }
    
    fn extract_source_labels(&mut self, source_models: &HashMap<String, SourceMarkovModel>) {
        for (file_path, model) in source_models {
            // Extract function-like tokens as labels
            for (token, count) in &model.token_patterns {
                if token.len() > 2 && !token.chars().all(|c| c.is_numeric()) {
                    let embedding = self.encode_source_token(token);
                    
                    self.label_vectors.insert(format!("src_{}", token), LabelVector {
                        label: token.clone(),
                        domain: LabelDomain::Source,
                        embedding,
                        frequency: *count,
                    });
                }
            }
        }
    }
    
    fn extract_binary_actions(&mut self, binary_models: &HashMap<String, crate::binary_markov::BinaryMarkovModel>) {
        for (binary_path, model) in binary_models {
            // Extract instruction patterns as actions
            for (instruction, count) in &model.instruction_patterns {
                let embedding = self.encode_instruction(instruction);
                
                self.action_vectors.insert(format!("{}_{}", binary_path, instruction), ActionVector {
                    action: instruction.clone(),
                    domain: self.classify_action_domain(instruction),
                    embedding,
                    execution_count: *count,
                });
            }
        }
    }
    
    fn encode_directory_name(&self, name: &str) -> Vec<f64> {
        // Simple character-based encoding
        name.chars().take(8).map(|c| (c as u8 as f64) / 255.0).collect()
    }
    
    fn encode_doc_pattern(&self, pattern: &str) -> Vec<f64> {
        pattern.chars().take(8).map(|c| (c as u8 as f64) / 255.0).collect()
    }
    
    fn encode_source_token(&self, token: &str) -> Vec<f64> {
        token.chars().take(8).map(|c| (c as u8 as f64) / 255.0).collect()
    }
    
    fn encode_instruction(&self, instruction: &str) -> Vec<f64> {
        instruction.chars().take(8).map(|c| (c as u8 as f64) / 255.0).collect()
    }
    
    fn classify_action_domain(&self, instruction: &str) -> ActionDomain {
        match instruction {
            "MOV" | "PUSH" | "POP" => ActionDomain::Memory,
            "CALL" | "RET" => ActionDomain::Computation,
            "NOP" => ActionDomain::Computation,
            _ => ActionDomain::Computation,
        }
    }
    
    fn build_flow_matrix(&mut self) {
        let label_count = self.label_vectors.len();
        let action_count = self.action_vectors.len();
        
        // Initialize flow matrix: labels → actions
        self.flow_matrix = vec![vec![0.0; action_count]; label_count];
        
        let labels: Vec<_> = self.label_vectors.keys().cloned().collect();
        let actions: Vec<_> = self.action_vectors.keys().cloned().collect();
        
        // Calculate flow strengths based on semantic similarity
        for (i, label_key) in labels.iter().enumerate() {
            for (j, action_key) in actions.iter().enumerate() {
                if let (Some(label), Some(action)) = (
                    self.label_vectors.get(label_key),
                    self.action_vectors.get(action_key)
                ) {
                    let flow_strength = self.calculate_flow_strength(label, action);
                    self.flow_matrix[i][j] = flow_strength;
                }
            }
        }
    }
    
    fn calculate_flow_strength(&self, label: &LabelVector, action: &ActionVector) -> f64 {
        // Cosine similarity between embeddings
        if label.embedding.len() != action.embedding.len() {
            return 0.0;
        }
        
        let dot_product: f64 = label.embedding.iter()
            .zip(&action.embedding)
            .map(|(a, b)| a * b)
            .sum();
            
        let norm_a: f64 = label.embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = action.embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
    
    fn calculate_reach_paths(&mut self) {
        let labels: Vec<_> = self.label_vectors.keys().cloned().collect();
        let actions: Vec<_> = self.action_vectors.keys().cloned().collect();
        
        for (i, label_key) in labels.iter().enumerate() {
            for (j, action_key) in actions.iter().enumerate() {
                let strength = self.flow_matrix[i][j];
                
                if strength > 0.1 {  // Threshold for significant reach
                    self.reach_paths.push(ReachPath {
                        label: label_key.clone(),
                        action: action_key.clone(),
                        path_strength: strength,
                        intermediate_nodes: vec![], // Could be expanded with graph traversal
                    });
                }
            }
        }
        
        // Sort by path strength
        self.reach_paths.sort_by(|a, b| b.path_strength.partial_cmp(&a.path_strength).unwrap());
    }
    
    pub fn print_label_reach_analysis(&self) {
        println!("🎯 LABEL REACH ANALYSIS");
        println!("=======================");
        
        println!("📊 Labels: {}, Actions: {}", 
                self.label_vectors.len(), self.action_vectors.len());
        
        println!("\n🔥 Top Reach Paths:");
        for (i, path) in self.reach_paths.iter().take(10).enumerate() {
            println!("  {}. {} → {} (strength: {:.3})", 
                    i+1, path.label, path.action, path.path_strength);
        }
        
        println!("\n📈 Domain Distribution:");
        let mut label_domains = HashMap::new();
        let mut action_domains = HashMap::new();
        
        for label in self.label_vectors.values() {
            *label_domains.entry(format!("{:?}", label.domain)).or_insert(0) += 1;
        }
        
        for action in self.action_vectors.values() {
            *action_domains.entry(format!("{:?}", action.domain)).or_insert(0) += 1;
        }
        
        for (domain, count) in label_domains {
            println!("  Labels {}: {}", domain, count);
        }
        
        for (domain, count) in action_domains {
            println!("  Actions {}: {}", domain, count);
        }
    }
}
