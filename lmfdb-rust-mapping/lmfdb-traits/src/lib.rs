//! # LMFDB Traits
//! 
//! Trait definitions for LMFDB client and server implementations.

use lmfdb_types::*;
use async_trait::async_trait;

// ============================================================================
// Client Trait
// ============================================================================

#[async_trait]
pub trait LMFDBClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Get an L-function by label
    async fn get_lfunction(&self, label: &str) -> Result<LFunction, Self::Error>;
    
    /// Search for L-functions
    async fn search(&self, query: LFunctionQuery) -> Result<Vec<LFunction>, Self::Error>;
    
    /// Get Dirichlet coefficients
    async fn get_coefficients(&self, label: &str, count: usize) 
        -> Result<Vec<DirichletCoefficient>, Self::Error>;
    
    /// Compute L-function value at point
    async fn compute_value(&self, label: &str, s_real: f64, s_imag: f64) 
        -> Result<(f64, f64), Self::Error>;
}

// ============================================================================
// Server Trait
// ============================================================================

#[async_trait]
pub trait LMFDBServer: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Store an L-function
    async fn store_lfunction(&self, lfunction: LFunction) -> Result<(), Self::Error>;
    
    /// Query L-functions
    async fn query(&self, query: LFunctionQuery) -> Result<Vec<LFunction>, Self::Error>;
    
    /// Delete an L-function
    async fn delete_lfunction(&self, label: &str) -> Result<(), Self::Error>;
    
    /// Update L-function data
    async fn update_lfunction(&self, label: &str, lfunction: LFunction) 
        -> Result<(), Self::Error>;
}

// ============================================================================
// Mapper Trait
// ============================================================================

pub trait LMFDBMapper: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Compute modular signature from bytes
    fn compute_modular_signature(&mut self, func_bytes: &[u8]) -> u64;
    
    /// Map symbol to LMFDB label
    fn symbol_to_lmfdb(&mut self, symbol_name: &str, func_bytes: &[u8]) 
        -> Result<LMFDBLabel, Self::Error>;
    
    /// Classify orbit level
    fn classify_orbit(&self, sample_count: u64, complexity_score: f64) -> OrbitLevel;
    
    /// Analyze binary
    fn analyze_binary(&mut self, binary_path: &str) -> Result<BinaryAnalysis, Self::Error>;
    
    /// Map perf data to LMFDB
    fn perf_to_lmfdb(&mut self, symbol: &str, samples: u64, func_bytes: &[u8]) 
        -> Result<PerfLMFDBMapping, Self::Error>;
}

// ============================================================================
// Storage Trait
// ============================================================================

#[async_trait]
pub trait LMFDBStorage: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Store binary analysis
    async fn store_binary_analysis(&self, analysis: BinaryAnalysis) 
        -> Result<(), Self::Error>;
    
    /// Get binary analysis
    async fn get_binary_analysis(&self, binary_path: &str) 
        -> Result<Option<BinaryAnalysis>, Self::Error>;
    
    /// Store perf mapping
    async fn store_perf_mapping(&self, mapping: PerfLMFDBMapping) 
        -> Result<(), Self::Error>;
    
    /// Query perf mappings
    async fn query_perf_mappings(&self, symbol_pattern: &str) 
        -> Result<Vec<PerfLMFDBMapping>, Self::Error>;
}
