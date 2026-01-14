//! # Markov Conformity Analysis
//! 
//! Build Markov models from:
//! 1. Binary instruction patterns (cmp/jmp sequences)
//! 2. Symbol label character sequences
//! 3. Directory/file name patterns
//! 
//! Then find conformity - where the same Markov structure appears across all three.

use lmfdb_types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Markov chain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkovState {
    pub state_id: String,
    pub transitions: HashMap<u8, f64>, // char -> probability
    pub modulus: u64,
}

/// Markov model extracted from different sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkovModel {
    pub source: MarkovSource,
    pub states: Vec<MarkovState>,
    pub lmfdb_label: LMFDBLabel,
    pub signature: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarkovSource {
    Instructions,  // From cmp/jmp bytecode
    Symbols,       // From symbol names
    Paths,         // From file/dir names
}

/// Conformity between Markov models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkovConformity {
    pub instruction_model: MarkovModel,
    pub symbol_model: MarkovModel,
    pub path_model: MarkovModel,
    pub conformity_score: f64,
    pub shared_lmfdb_label: Option<LMFDBLabel>,
}

pub struct MarkovAnalyzer {
    modulus: u64,
}

impl MarkovAnalyzer {
    pub fn new() -> Self {
        Self { modulus: 997 }
    }
    
    /// Build Markov model from instruction bytes (cmp/jmp patterns)
    pub fn model_from_instructions(&self, func_bytes: &[u8]) -> MarkovModel {
        let mut states: HashMap<String, MarkovState> = HashMap::new();
        
        for window in func_bytes.windows(3) {
            // Look for cmp/jmp patterns
            if self.is_control_flow(window) {
                let state_id = format!("{:02x}{:02x}", window[0], window[1]);
                let next_char = window[2];
                
                states.entry(state_id.clone())
                    .or_insert_with(|| MarkovState {
                        state_id: state_id.clone(),
                        transitions: HashMap::new(),
                        modulus: self.hash_state(&state_id),
                    })
                    .transitions.entry(next_char)
                    .and_modify(|p| *p += 1.0)
                    .or_insert(1.0);
            }
        }
        
        // Normalize probabilities
        for state in states.values_mut() {
            let total: f64 = state.transitions.values().sum();
            if total > 0.0 {
                for prob in state.transitions.values_mut() {
                    *prob /= total;
                }
            }
        }
        
        let signature = self.compute_signature(&states);
        let lmfdb_label = self.signature_to_lmfdb(signature);
        
        MarkovModel {
            source: MarkovSource::Instructions,
            states: states.into_values().collect(),
            lmfdb_label,
            signature,
        }
    }
    
    /// Build Markov model from symbol names
    pub fn model_from_symbols(&self, symbols: &[String]) -> MarkovModel {
        let mut states: HashMap<String, MarkovState> = HashMap::new();
        
        for symbol in symbols {
            let bytes = symbol.as_bytes();
            for window in bytes.windows(3) {
                let state_id = format!("{}{}", window[0] as char, window[1] as char);
                let next_char = window[2];
                
                states.entry(state_id.clone())
                    .or_insert_with(|| MarkovState {
                        state_id: state_id.clone(),
                        transitions: HashMap::new(),
                        modulus: self.hash_state(&state_id),
                    })
                    .transitions.entry(next_char)
                    .and_modify(|p| *p += 1.0)
                    .or_insert(1.0);
            }
        }
        
        // Normalize
        for state in states.values_mut() {
            let total: f64 = state.transitions.values().sum();
            if total > 0.0 {
                for prob in state.transitions.values_mut() {
                    *prob /= total;
                }
            }
        }
        
        let signature = self.compute_signature(&states);
        let lmfdb_label = self.signature_to_lmfdb(signature);
        
        MarkovModel {
            source: MarkovSource::Symbols,
            states: states.into_values().collect(),
            lmfdb_label,
            signature,
        }
    }
    
    /// Build Markov model from file/directory paths
    pub fn model_from_paths(&self, paths: &[&Path]) -> MarkovModel {
        let mut states: HashMap<String, MarkovState> = HashMap::new();
        
        for path in paths {
            if let Some(path_str) = path.to_str() {
                let bytes = path_str.as_bytes();
                for window in bytes.windows(3) {
                    let state_id = format!("{}{}", window[0] as char, window[1] as char);
                    let next_char = window[2];
                    
                    states.entry(state_id.clone())
                        .or_insert_with(|| MarkovState {
                            state_id: state_id.clone(),
                            transitions: HashMap::new(),
                            modulus: self.hash_state(&state_id),
                        })
                        .transitions.entry(next_char)
                        .and_modify(|p| *p += 1.0)
                        .or_insert(1.0);
                }
            }
        }
        
        // Normalize
        for state in states.values_mut() {
            let total: f64 = state.transitions.values().sum();
            if total > 0.0 {
                for prob in state.transitions.values_mut() {
                    *prob /= total;
                }
            }
        }
        
        let signature = self.compute_signature(&states);
        let lmfdb_label = self.signature_to_lmfdb(signature);
        
        MarkovModel {
            source: MarkovSource::Paths,
            states: states.into_values().collect(),
            lmfdb_label,
            signature,
        }
    }
    
    /// Find conformity between three models
    pub fn find_conformity(
        &self,
        instruction_model: MarkovModel,
        symbol_model: MarkovModel,
        path_model: MarkovModel,
    ) -> MarkovConformity {
        // Compare signatures
        let sig_diff_1 = (instruction_model.signature as i64 - symbol_model.signature as i64).abs();
        let sig_diff_2 = (symbol_model.signature as i64 - path_model.signature as i64).abs();
        let sig_diff_3 = (instruction_model.signature as i64 - path_model.signature as i64).abs();
        
        let avg_diff = (sig_diff_1 + sig_diff_2 + sig_diff_3) as f64 / 3.0;
        let conformity_score = 1.0 - (avg_diff / self.modulus as f64);
        
        // Check if all have same LMFDB label
        let shared_label = if instruction_model.lmfdb_label == symbol_model.lmfdb_label
            && symbol_model.lmfdb_label == path_model.lmfdb_label {
            Some(instruction_model.lmfdb_label.clone())
        } else {
            None
        };
        
        MarkovConformity {
            instruction_model,
            symbol_model,
            path_model,
            conformity_score,
            shared_lmfdb_label: shared_label,
        }
    }
    
    // Helper methods
    
    fn is_control_flow(&self, bytes: &[u8]) -> bool {
        matches!(bytes[0], 
            0x74 | 0x75 | // je/jne
            0x7c | 0x7d | // jl/jge
            0x80 | 0x81 | // cmp
            0x3c | 0x3d   // cmp al/ax
        )
    }
    
    fn hash_state(&self, state_id: &str) -> u64 {
        state_id.bytes()
            .enumerate()
            .map(|(i, b)| (b as u64).wrapping_mul(i as u64 + 1))
            .sum::<u64>() % self.modulus
    }
    
    fn compute_signature(&self, states: &HashMap<String, MarkovState>) -> u64 {
        states.values()
            .map(|s| s.modulus)
            .sum::<u64>() % self.modulus
    }
    
    fn signature_to_lmfdb(&self, signature: u64) -> LMFDBLabel {
        let level = (signature % 37) + 1;
        let weight = if signature % 3 == 0 { 2 } else { 4 };
        let character = if signature % 2 == 0 { "1a" } else { "1b" }.to_string();
        let orbit = ((signature % 26) as u8 + b'a') as char;
        
        LMFDBLabel::new(level as u32, weight, character, orbit)
    }
}

impl Default for MarkovAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_markov_from_symbols() {
        let analyzer = MarkovAnalyzer::new();
        let symbols = vec![
            "_ZN4core3ptr".to_string(),
            "_ZN4core3fmt".to_string(),
        ];
        
        let model = analyzer.model_from_symbols(&symbols);
        assert!(!model.states.is_empty());
        assert_eq!(model.source, MarkovSource::Symbols);
    }
    
    #[test]
    fn test_conformity() {
        let analyzer = MarkovAnalyzer::new();
        
        let inst_model = MarkovModel {
            source: MarkovSource::Instructions,
            states: vec![],
            lmfdb_label: LMFDBLabel::new(11, 2, "1a".to_string(), 'a'),
            signature: 100,
        };
        
        let sym_model = MarkovModel {
            source: MarkovSource::Symbols,
            states: vec![],
            lmfdb_label: LMFDBLabel::new(11, 2, "1a".to_string(), 'a'),
            signature: 105,
        };
        
        let path_model = MarkovModel {
            source: MarkovSource::Paths,
            states: vec![],
            lmfdb_label: LMFDBLabel::new(11, 2, "1a".to_string(), 'a'),
            signature: 102,
        };
        
        let conformity = analyzer.find_conformity(inst_model, sym_model, path_model);
        assert!(conformity.conformity_score > 0.9);
        assert!(conformity.shared_lmfdb_label.is_some());
    }
}
