// 🔥 MULTI-LEVEL BINARY MARKOV ANALYZER
// Bit-level, byte-level, word-level, and instruction-level similarity matching

use std::collections::HashMap;
use goblin::elf::Elf;
use crate::telemetry_lib::telemetry_lib::*;

pub struct BinaryMarkovModel {
    pub binary_path: String,
    pub bit_transitions: HashMap<u8, HashMap<u8, u32>>,      // 8-bit transitions
    pub byte_transitions: HashMap<u8, HashMap<u8, u32>>,     // Byte-level
    pub word_transitions: HashMap<u16, HashMap<u16, u32>>,   // 16-bit word level
    pub instruction_patterns: HashMap<String, u32>,          // Disassembly patterns
    pub entropy: f64,
}

impl BinaryMarkovModel {
    pub fn from_binary(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer = std::fs::read(path)?;
        let elf = Elf::parse(&buffer)?;
        
        let mut model = Self {
            binary_path: path.to_string(),
            bit_transitions: HashMap::new(),
            byte_transitions: HashMap::new(),
            word_transitions: HashMap::new(),
            instruction_patterns: HashMap::new(),
            entropy: 0.0,
        };
        
        // Analyze .text section for executable code
        if let Some(text_section) = elf.section_headers.iter()
            .find(|s| elf.shdr_strtab.get_at(s.sh_name).unwrap_or("") == ".text") {
            
            let start = text_section.sh_offset as usize;
            let size = text_section.sh_size as usize;
            let code = &buffer[start..start + size];
            
            model.analyze_bit_level(code);
            model.analyze_byte_level(code);
            model.analyze_word_level(code);
            model.analyze_instruction_patterns(code);
            model.calculate_entropy(code);
        }
        
        Ok(model)
    }
    
    fn analyze_bit_level(&mut self, data: &[u8]) {
        // Bit-level transitions (inspired by zombie_driver2 bit analysis)
        for window in data.windows(2) {
            let from = window[0];
            let to = window[1];
            
            // Analyze bit patterns within bytes
            for bit_pos in 0..8 {
                let from_bit = (from >> bit_pos) & 1;
                let to_bit = (to >> bit_pos) & 1;
                let transition_key = (from_bit << 1) | to_bit;
                
                *self.bit_transitions
                    .entry(transition_key)
                    .or_insert_with(HashMap::new)
                    .entry(bit_pos)
                    .or_insert(0) += 1;
            }
        }
    }
    
    fn analyze_byte_level(&mut self, data: &[u8]) {
        // Byte-level Markov chains (from zos-server binary analysis)
        for window in data.windows(2) {
            let from = window[0];
            let to = window[1];
            
            *self.byte_transitions
                .entry(from)
                .or_insert_with(HashMap::new)
                .entry(to)
                .or_insert(0) += 1;
        }
    }
    
    fn analyze_word_level(&mut self, data: &[u8]) {
        // 16-bit word level analysis
        let words: Vec<u16> = data.chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
            
        for window in words.windows(2) {
            let from = window[0];
            let to = window[1];
            
            *self.word_transitions
                .entry(from)
                .or_insert_with(HashMap::new)
                .entry(to)
                .or_insert(0) += 1;
        }
    }
    
    fn analyze_instruction_patterns(&mut self, data: &[u8]) {
        // Simple x86_64 instruction pattern detection
        for window in data.windows(4) {
            let pattern = format!("{:02x}{:02x}{:02x}{:02x}", 
                                window[0], window[1], window[2], window[3]);
            
            // Detect common instruction prefixes
            let instruction_type = match window[0] {
                0x48..=0x4f => "REX",      // REX prefixes
                0x50..=0x57 => "PUSH",     // PUSH registers
                0x58..=0x5f => "POP",      // POP registers
                0x89 => "MOV",             // MOV r/m32, r32
                0x8b => "MOV",             // MOV r32, r/m32
                0xe8 => "CALL",            // CALL rel32
                0xc3 => "RET",             // RET
                0x90 => "NOP",             // NOP
                _ => "OTHER",
            };
            
            *self.instruction_patterns.entry(instruction_type.to_string()).or_insert(0) += 1;
        }
    }
    
    fn calculate_entropy(&mut self, data: &[u8]) {
        let mut byte_counts = [0u32; 256];
        for &byte in data {
            byte_counts[byte as usize] += 1;
        }
        
        let total = data.len() as f64;
        self.entropy = byte_counts.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.log2()
            })
            .sum();
    }
    
    pub fn similarity_to(&self, other: &BinaryMarkovModel) -> BinarySimilarity {
        BinarySimilarity {
            bit_similarity: self.compare_bit_transitions(&other.bit_transitions),
            byte_similarity: self.compare_byte_transitions(&other.byte_transitions),
            word_similarity: self.compare_word_transitions(&other.word_transitions),
            instruction_similarity: self.compare_instruction_patterns(&other.instruction_patterns),
            entropy_similarity: 1.0 - (self.entropy - other.entropy).abs() / 8.0, // Max entropy is 8
        }
    }
    
    fn compare_bit_transitions(&self, other: &HashMap<u8, HashMap<u8, u32>>) -> f64 {
        // Jaccard similarity on bit transition patterns
        let self_keys: std::collections::HashSet<_> = self.bit_transitions.keys().collect();
        let other_keys: std::collections::HashSet<_> = other.keys().collect();
        
        let intersection = self_keys.intersection(&other_keys).count();
        let union = self_keys.union(&other_keys).count();
        
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }
    
    fn compare_byte_transitions(&self, other: &HashMap<u8, HashMap<u8, u32>>) -> f64 {
        // Cosine similarity on byte transition frequencies
        cosine_similarity_u8(&self.byte_transitions, other)
    }
    
    fn compare_word_transitions(&self, other: &HashMap<u16, HashMap<u16, u32>>) -> f64 {
        cosine_similarity_u16(&self.word_transitions, other)
    }
    
    fn compare_instruction_patterns(&self, other: &HashMap<String, u32>) -> f64 {
        cosine_similarity_string(&self.instruction_patterns, other)
    }
}

#[derive(Debug)]
pub struct BinarySimilarity {
    pub bit_similarity: f64,
    pub byte_similarity: f64,
    pub word_similarity: f64,
    pub instruction_similarity: f64,
    pub entropy_similarity: f64,
}

impl BinarySimilarity {
    pub fn overall_similarity(&self) -> f64 {
        // Weighted average - instruction patterns are most important
        self.instruction_similarity * 0.4 +
        self.byte_similarity * 0.3 +
        self.word_similarity * 0.2 +
        self.bit_similarity * 0.05 +
        self.entropy_similarity * 0.05
    }
}

fn cosine_similarity_u8(map1: &HashMap<u8, HashMap<u8, u32>>, map2: &HashMap<u8, HashMap<u8, u32>>) -> f64 {
    // Flatten to single dimension for cosine similarity
    let mut vec1 = HashMap::new();
    let mut vec2 = HashMap::new();
    
    for (k1, inner) in map1 {
        for (k2, count) in inner {
            vec1.insert((*k1 as u16) << 8 | (*k2 as u16), *count);
        }
    }
    
    for (k1, inner) in map2 {
        for (k2, count) in inner {
            vec2.insert((*k1 as u16) << 8 | (*k2 as u16), *count);
        }
    }
    
    cosine_similarity_flat(&vec1, &vec2)
}

fn cosine_similarity_u16(map1: &HashMap<u16, HashMap<u16, u32>>, map2: &HashMap<u16, HashMap<u16, u32>>) -> f64 {
    let mut vec1 = HashMap::new();
    let mut vec2 = HashMap::new();
    
    for (k1, inner) in map1 {
        for (k2, count) in inner {
            vec1.insert((*k1 as u32) << 16 | (*k2 as u32), *count);
        }
    }
    
    for (k1, inner) in map2 {
        for (k2, count) in inner {
            vec2.insert((*k1 as u32) << 16 | (*k2 as u32), *count);
        }
    }
    
    cosine_similarity_flat(&vec1, &vec2)
}

fn cosine_similarity_string(map1: &HashMap<String, u32>, map2: &HashMap<String, u32>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    let all_keys: std::collections::HashSet<_> = map1.keys().chain(map2.keys()).collect();
    
    for key in all_keys {
        let v1 = *map1.get(key).unwrap_or(&0) as f64;
        let v2 = *map2.get(key).unwrap_or(&0) as f64;
        
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }
    
    if norm1 == 0.0 || norm2 == 0.0 { 0.0 }
    else { dot_product / (norm1.sqrt() * norm2.sqrt()) }
}

fn cosine_similarity_flat<K: std::hash::Hash + Eq>(map1: &HashMap<K, u32>, map2: &HashMap<K, u32>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    let all_keys: std::collections::HashSet<_> = map1.keys().chain(map2.keys()).collect();
    
    for key in all_keys {
        let v1 = *map1.get(key).unwrap_or(&0) as f64;
        let v2 = *map2.get(key).unwrap_or(&0) as f64;
        
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }
    
    if norm1 == 0.0 || norm2 == 0.0 { 0.0 }
    else { dot_product / (norm1.sqrt() * norm2.sqrt()) }
}
