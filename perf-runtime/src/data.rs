// perf-runtime/src/data.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfData {
    pub name: String,
    pub timestamp: u64,
    pub duration_secs: f64,
    pub cycles: u64,
    pub instructions: u64,
    pub cache_references: u64,
    pub cache_misses: u64,
    pub branches: u64,
    pub branch_misses: u64,
}

impl Default for PerfData {
    fn default() -> Self {
        Self {
            name: String::new(),
            timestamp: 0,
            duration_secs: 0.0,
            cycles: 0,
            instructions: 0,
            cache_references: 0,
            cache_misses: 0,
            branches: 0,
            branch_misses: 0,
        }
    }
}

impl PerfData {
    pub fn ipc(&self) -> f64 {
        if self.cycles > 0 {
            self.instructions as f64 / self.cycles as f64
        } else {
            0.0
        }
    }
    
    pub fn cache_miss_rate(&self) -> f64 {
        if self.cache_references > 0 {
            self.cache_misses as f64 / self.cache_references as f64
        } else {
            0.0
        }
    }
    
    pub fn branch_miss_rate(&self) -> f64 {
        if self.branches > 0 {
            self.branch_misses as f64 / self.branches as f64
        } else {
            0.0
        }
    }
}
