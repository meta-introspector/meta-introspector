//! # LMFDB Rust Mapping Library
//! 
//! Unified library for mapping Rust binaries, symbols, and performance data to LMFDB mathematical structures.
//! 
//! ## Core Concepts
//! 
//! - **Binary → LMFDB**: Map ELF binaries to modular forms
//! - **Symbol → LMFDB**: Map function symbols to LMFDB labels
//! - **Perf → LMFDB**: Map performance data to complexity classes
//! - **Orbit Classification**: 11, 23, 47, 71 levels
//! 
//! ## Usage
//! 
//! ```rust
//! use lmfdb_rust_mapping::*;
//! 
//! // Map a binary symbol to LMFDB
//! let mapping = LMFDBMapper::new();
//! let label = mapping.symbol_to_lmfdb("_ZN4core3ptr...", &function_bytes)?;
//! 
//! // Classify complexity
//! let orbit = mapping.classify_orbit(sample_count, complexity_score);
//! ```

use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

// ============================================================================
// Core Data Structures
// ============================================================================

/// LMFDB Modular Form Label
/// Format: level.weight.character.orbit (e.g., "11.2.1a.a", "71.4.1b.c")
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LMFDBLabel {
    pub level: u32,        // Prime level (11, 23, 47, 71, ...)
    pub weight: u32,       // Weight (2, 4, 6, ...)
    pub character: String, // Character ("1a", "1b", ...)
    pub orbit: char,       // Orbit letter (a, b, c, ...)
}

impl LMFDBLabel {
    pub fn new(level: u32, weight: u32, character: String, orbit: char) -> Self {
        Self { level, weight, character, orbit }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.level, self.weight, self.character, self.orbit)
    }
    
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 4 {
            anyhow::bail!("Invalid LMFDB label format: {}", s);
        }
        
        Ok(Self {
            level: parts[0].parse()?,
            weight: parts[1].parse()?,
            character: parts[2].to_string(),
            orbit: parts[3].chars().next().unwrap_or('a'),
        })
    }
}

/// LMFDB Orbit Classification
/// Based on the 71 pattern: 11 → 23 → 47 → 71
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrbitLevel {
    Genesis = 11,      // Core/Foundation
    Trinity = 23,      // Stability/Structure
    Completeness = 47, // Advanced/Complete
    Return = 71,       // Mastery/Transcendence
}

impl OrbitLevel {
    pub fn from_level(level: u32) -> Self {
        match level {
            0..=16 => OrbitLevel::Genesis,
            17..=34 => OrbitLevel::Trinity,
            35..=58 => OrbitLevel::Completeness,
            _ => OrbitLevel::Return,
        }
    }
    
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

/// Symbol to LMFDB Mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMapping {
    pub symbol_name: String,
    pub address: u64,
    pub size: u64,
    pub lmfdb_label: LMFDBLabel,
    pub modular_signature: u64,
    pub orbit_level: OrbitLevel,
}

/// Binary to LMFDB Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryAnalysis {
    pub binary_path: String,
    pub total_symbols: usize,
    pub symbol_mappings: Vec<SymbolMapping>,
    pub orbit_distribution: HashMap<OrbitLevel, usize>,
    pub conductor: u64,
}

// ============================================================================
// LMFDB Mapper
// ============================================================================

pub struct LMFDBMapper {
    /// Cache of computed signatures
    signature_cache: HashMap<String, u64>,
}

impl LMFDBMapper {
    pub fn new() -> Self {
        Self {
            signature_cache: HashMap::new(),
        }
    }
    
    /// Compute modular signature from function bytes
    pub fn compute_modular_signature(&mut self, func_bytes: &[u8]) -> u64 {
        let mut signature = 0u64;
        
        for (i, &byte) in func_bytes.iter().enumerate() {
            signature = signature.wrapping_add((byte as u64).wrapping_mul(i as u64 + 1));
        }
        
        // Apply modular reduction
        signature % 37 // Prime modulus
    }
    
    /// Map symbol to LMFDB label
    pub fn symbol_to_lmfdb(&mut self, symbol_name: &str, func_bytes: &[u8]) -> Result<LMFDBLabel> {
        let signature = self.compute_modular_signature(func_bytes);
        
        // Map signature to LMFDB parameters
        let level = (signature % 37) + 1;
        let weight = match signature % 3 {
            0 => 2,
            1 => 4,
            _ => 6,
        };
        let character = if signature % 2 == 0 { "1a" } else { "1b" }.to_string();
        let orbit = ((signature % 26) as u8 + b'a') as char;
        
        Ok(LMFDBLabel::new(level as u32, weight, character, orbit))
    }
    
    /// Classify orbit level based on complexity
    pub fn classify_orbit(&self, sample_count: u64, complexity_score: f64) -> OrbitLevel {
        let combined = (sample_count as f64 * complexity_score) as u32;
        OrbitLevel::from_level(combined % 100)
    }
    
    /// Analyze entire binary
    pub fn analyze_binary(&mut self, binary_path: &str) -> Result<BinaryAnalysis> {
        let binary_data = std::fs::read(binary_path)?;
        let elf = Elf::parse(&binary_data)?;
        
        let mut symbol_mappings = Vec::new();
        let mut orbit_distribution: HashMap<OrbitLevel, usize> = HashMap::new();
        
        // Find .text section
        let text_section = elf.section_headers.iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text");
        
        if let Some(text_sec) = text_section {
            let text_start = text_sec.sh_offset as usize;
            let text_bytes = &binary_data[text_start..];
            
            for sym in elf.syms.iter() {
                if sym.st_size == 0 || sym.st_value == 0 {
                    continue;
                }
                
                let symbol_name = elf.strtab.get_at(sym.st_name).unwrap_or("").to_string();
                
                // Extract function bytes
                if sym.st_value >= text_sec.sh_addr {
                    let func_start = (sym.st_value - text_sec.sh_addr) as usize;
                    let func_size = (sym.st_size as usize).min(64);
                    
                    if func_start < text_bytes.len() {
                        let func_end = (func_start + func_size).min(text_bytes.len());
                        let func_bytes = &text_bytes[func_start..func_end];
                        
                        let signature = self.compute_modular_signature(func_bytes);
                        let label = self.symbol_to_lmfdb(&symbol_name, func_bytes)?;
                        let orbit = OrbitLevel::from_level(label.level);
                        
                        *orbit_distribution.entry(orbit).or_insert(0) += 1;
                        
                        symbol_mappings.push(SymbolMapping {
                            symbol_name,
                            address: sym.st_value,
                            size: sym.st_size,
                            lmfdb_label: label,
                            modular_signature: signature,
                            orbit_level: orbit,
                        });
                    }
                }
            }
        }
        
        // Compute conductor (sum of all signatures mod large prime)
        let conductor = symbol_mappings.iter()
            .map(|m| m.modular_signature)
            .sum::<u64>() % 997; // Large prime
        
        Ok(BinaryAnalysis {
            binary_path: binary_path.to_string(),
            total_symbols: symbol_mappings.len(),
            symbol_mappings,
            orbit_distribution,
            conductor,
        })
    }
}

impl Default for LMFDBMapper {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Perf Data Integration
// ============================================================================

/// Map perf sample counts to LMFDB complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfLMFDBMapping {
    pub symbol: String,
    pub samples: u64,
    pub lmfdb_label: LMFDBLabel,
    pub complexity_class: OrbitLevel,
}

impl LMFDBMapper {
    /// Map perf data to LMFDB
    pub fn perf_to_lmfdb(&mut self, symbol: &str, samples: u64, func_bytes: &[u8]) -> Result<PerfLMFDBMapping> {
        let label = self.symbol_to_lmfdb(symbol, func_bytes)?;
        let complexity = self.classify_orbit(samples, func_bytes.len() as f64);
        
        Ok(PerfLMFDBMapping {
            symbol: symbol.to_string(),
            samples,
            lmfdb_label: label,
            complexity_class: complexity,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lmfdb_label() {
        let label = LMFDBLabel::new(11, 2, "1a".to_string(), 'a');
        assert_eq!(label.to_string(), "11.2.1a.a");
        
        let parsed = LMFDBLabel::from_string("71.4.1b.c").unwrap();
        assert_eq!(parsed.level, 71);
        assert_eq!(parsed.orbit, 'c');
    }
    
    #[test]
    fn test_orbit_classification() {
        assert_eq!(OrbitLevel::from_level(5), OrbitLevel::Genesis);
        assert_eq!(OrbitLevel::from_level(23), OrbitLevel::Trinity);
        assert_eq!(OrbitLevel::from_level(47), OrbitLevel::Completeness);
        assert_eq!(OrbitLevel::from_level(71), OrbitLevel::Return);
    }
    
    #[test]
    fn test_modular_signature() {
        let mut mapper = LMFDBMapper::new();
        let bytes = b"test function bytes";
        let sig1 = mapper.compute_modular_signature(bytes);
        let sig2 = mapper.compute_modular_signature(bytes);
        assert_eq!(sig1, sig2); // Deterministic
    }
}
