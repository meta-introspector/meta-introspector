//! # LMFDB Types
//! 
//! Core data types for LMFDB mathematical structures.
//! Shared between client and server implementations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Core LMFDB Label Types
// ============================================================================

/// LMFDB Modular Form Label
/// Format: level.weight.character.orbit (e.g., "11.2.1a.a", "71.4.1b.c")
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LMFDBLabel {
    pub level: u32,
    pub weight: u32,
    pub character: String,
    pub orbit: char,
}

impl LMFDBLabel {
    pub fn new(level: u32, weight: u32, character: String, orbit: char) -> Self {
        Self { level, weight, character, orbit }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.level, self.weight, self.character, self.orbit)
    }
    
    pub fn from_string(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 4 {
            return Err(format!("Invalid LMFDB label format: {}", s));
        }
        
        Ok(Self {
            level: parts[0].parse().map_err(|e| format!("Invalid level: {}", e))?,
            weight: parts[1].parse().map_err(|e| format!("Invalid weight: {}", e))?,
            character: parts[2].to_string(),
            orbit: parts[3].chars().next().ok_or("Empty orbit")?,
        })
    }
}

/// LMFDB Orbit Classification
/// Based on the 71 pattern: 11 → 23 → 47 → 71
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrbitLevel {
    Genesis = 11,
    Trinity = 23,
    Completeness = 47,
    Return = 71,
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

// ============================================================================
// L-Function Types
// ============================================================================

/// L-Function representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LFunction {
    pub label: String,
    pub origin: String,
    pub degree: u32,
    pub conductor: u64,
    pub central_character: Option<String>,
    pub weight: Option<f64>,
    pub gamma_factors: Vec<GammaFactor>,
    pub sign: Option<f64>,
    pub order_of_vanishing: Option<u32>,
    pub leading_coeff: Option<f64>,
    pub algebraic_leading_coeff: Option<String>,
    pub dirichlet_coefficients: Option<Vec<DirichletCoefficient>>,
    pub euler_factors: Option<HashMap<u64, Vec<f64>>>,
    pub selberg_data: Option<SelbergData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaFactor {
    pub gamma_type: String,
    pub shift: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirichletCoefficient {
    pub n: u64,
    pub real: f64,
    pub imag: f64,
}

impl DirichletCoefficient {
    pub fn as_complex(&self) -> (f64, f64) {
        (self.real, self.imag)
    }
    
    pub fn abs(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelbergData {
    pub degree: u32,
    pub conductor: u64,
    pub gamma_factors: Vec<GammaFactor>,
    pub primitive: bool,
}

// ============================================================================
// Binary Mapping Types
// ============================================================================

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

/// Binary Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryAnalysis {
    pub binary_path: String,
    pub total_symbols: usize,
    pub symbol_mappings: Vec<SymbolMapping>,
    pub orbit_distribution: HashMap<OrbitLevel, usize>,
    pub conductor: u64,
}

/// Perf to LMFDB Mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfLMFDBMapping {
    pub symbol: String,
    pub samples: u64,
    pub lmfdb_label: LMFDBLabel,
    pub complexity_class: OrbitLevel,
}

// ============================================================================
// Query Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LFunctionQuery {
    pub origin: Option<String>,
    pub degree: Option<u32>,
    pub conductor: Option<u64>,
    pub conductor_min: Option<u64>,
    pub conductor_max: Option<u64>,
    pub sign: Option<f64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for LFunctionQuery {
    fn default() -> Self {
        Self {
            origin: None,
            degree: None,
            conductor: None,
            conductor_min: None,
            conductor_max: None,
            sign: None,
            limit: Some(100),
            offset: None,
        }
    }
}
