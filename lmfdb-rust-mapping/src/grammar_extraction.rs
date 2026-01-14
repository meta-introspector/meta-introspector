//! # Binary Grammar Extraction
//! 
//! Extract lexer/parser DFA patterns from binary code using modular arithmetic.
//! Each parser state transition is a modular operation - we can reverse-engineer the grammar.

use lmfdb_types::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// DFA State extracted from binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DFAState {
    pub state_id: u64,
    pub transitions: HashMap<u8, u64>, // char -> next_state
    pub is_accepting: bool,
    pub modulus: u64, // Modular signature of this state
}

/// Grammar pattern extracted from binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarPattern {
    pub pattern_id: String,
    pub states: Vec<DFAState>,
    pub alphabet: HashSet<u8>,
    pub lmfdb_label: LMFDBLabel,
    pub modular_signature: u64,
}

/// Binary Grammar Extractor
pub struct GrammarExtractor {
    /// Modulus for state extraction
    state_modulus: u64,
    /// Extracted patterns
    patterns: Vec<GrammarPattern>,
}

impl GrammarExtractor {
    pub fn new() -> Self {
        Self {
            state_modulus: 251, // Prime for state space
            patterns: Vec::new(),
        }
    }
    
    /// Extract DFA states from function bytes
    pub fn extract_dfa(&mut self, func_bytes: &[u8]) -> Vec<DFAState> {
        let mut states = Vec::new();
        let mut state_map: HashMap<u64, DFAState> = HashMap::new();
        
        // Scan for state transition patterns - looking for character checks
        for window in func_bytes.windows(4) {
            // Pattern: cmp + jmp = state transition
            if self.is_transition_pattern(window) {
                let state_id = self.compute_state_id(window);
                let char_check = window[1]; // THE ACTUAL CHARACTER BEING CHECKED
                let next_state = self.compute_next_state(&window[2..]);
                
                state_map.entry(state_id)
                    .or_insert_with(|| DFAState {
                        state_id,
                        transitions: HashMap::new(),
                        is_accepting: false,
                        modulus: state_id % self.state_modulus,
                    })
                    .transitions.insert(char_check, next_state);
            }
        }
        
        states.extend(state_map.into_values());
        states
    }
    
    /// Check if byte pattern represents a state transition
    fn is_transition_pattern(&self, bytes: &[u8]) -> bool {
        // Common x86_64 patterns for char comparison + jump
        matches!(bytes[0], 
            0x80 | 0x81 | // cmp
            0x3c | 0x3d | // cmp al/ax
            0x38 | 0x39   // cmp
        ) && matches!(bytes[2],
            0x74 | 0x75 | // je/jne
            0x7c | 0x7d | // jl/jge
            0xeb          // jmp
        )
    }
    
    /// Compute state ID from bytes
    fn compute_state_id(&self, bytes: &[u8]) -> u64 {
        let mut id = 0u64;
        for (i, &b) in bytes.iter().enumerate() {
            id = id.wrapping_add((b as u64).wrapping_mul(i as u64 + 1));
        }
        id % self.state_modulus
    }
    
    /// Compute next state from jump offset
    fn compute_next_state(&self, bytes: &[u8]) -> u64 {
        let offset = bytes[0] as i8; // Relative jump
        ((offset as i64).abs() as u64) % self.state_modulus
    }
    
    /// Extract grammar pattern from DFA states
    pub fn extract_grammar(&mut self, states: Vec<DFAState>) -> GrammarPattern {
        let mut alphabet = HashSet::new();
        for state in &states {
            alphabet.extend(state.transitions.keys());
        }
        
        // Compute modular signature of entire grammar
        let signature = states.iter()
            .map(|s| s.modulus)
            .sum::<u64>() % 997; // Large prime
        
        // Map to LMFDB label
        let level = (signature % 37) + 1;
        let weight = if signature % 3 == 0 { 2 } else { 4 };
        let character = if signature % 2 == 0 { "1a" } else { "1b" }.to_string();
        let orbit = ((signature % 26) as u8 + b'a') as char;
        
        let lmfdb_label = LMFDBLabel::new(level as u32, weight, character, orbit);
        let pattern_id = format!("grammar_{}", signature);
        
        GrammarPattern {
            pattern_id,
            states,
            alphabet,
            lmfdb_label,
            modular_signature: signature,
        }
    }
    
    /// Find similar grammars by modular signature
    pub fn find_similar(&self, target: &GrammarPattern, threshold: f64) -> Vec<&GrammarPattern> {
        self.patterns.iter()
            .filter(|p| {
                let diff = (p.modular_signature as i64 - target.modular_signature as i64).abs();
                let similarity = 1.0 - (diff as f64 / 997.0);
                similarity >= threshold
            })
            .collect()
    }
    
    /// Store pattern for later matching
    pub fn store_pattern(&mut self, pattern: GrammarPattern) {
        self.patterns.push(pattern);
    }
    
    /// Get all patterns with same LMFDB label (same grammar class)
    pub fn patterns_by_label(&self, label: &LMFDBLabel) -> Vec<&GrammarPattern> {
        self.patterns.iter()
            .filter(|p| &p.lmfdb_label == label)
            .collect()
    }
}

impl Default for GrammarExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Grammar-based code labeling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarLabel {
    pub symbol_name: String,
    pub grammar_pattern: String,
    pub lmfdb_label: LMFDBLabel,
    pub confidence: f64,
}

/// Label code using extracted grammar keys
pub fn label_with_grammar(
    func_bytes: &[u8],
    known_grammars: &[GrammarPattern]
) -> Option<GrammarLabel> {
    let mut extractor = GrammarExtractor::new();
    let states = extractor.extract_dfa(func_bytes);
    
    if states.is_empty() {
        return None;
    }
    
    let pattern = extractor.extract_grammar(states);
    
    // Find best matching known grammar
    for known in known_grammars {
        let diff = (pattern.modular_signature as i64 - known.modular_signature as i64).abs();
        let confidence = 1.0 - (diff as f64 / 997.0);
        
        if confidence > 0.8 {
            return Some(GrammarLabel {
                symbol_name: String::new(),
                grammar_pattern: known.pattern_id.clone(),
                lmfdb_label: known.lmfdb_label.clone(),
                confidence,
            });
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dfa_extraction() {
        let mut extractor = GrammarExtractor::new();
        
        // Simulate lexer bytecode: cmp + jmp pattern
        let bytes = vec![
            0x80, 0x61, 0x74, 0x05, // cmp [reg], 'a'; je +5
            0x80, 0x62, 0x74, 0x03, // cmp [reg], 'b'; je +3
        ];
        
        let states = extractor.extract_dfa(&bytes);
        assert!(!states.is_empty());
    }
    
    #[test]
    fn test_grammar_pattern() {
        let mut extractor = GrammarExtractor::new();
        let states = vec![
            DFAState {
                state_id: 0,
                transitions: [(b'a', 1)].iter().cloned().collect(),
                is_accepting: false,
                modulus: 0,
            }
        ];
        
        let pattern = extractor.extract_grammar(states);
        assert!(!pattern.pattern_id.is_empty());
    }
}
