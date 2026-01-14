// 🔥 MODULE SIMILARITY DETECTOR
// Uses existing Markov/grammar tools to find reporting pattern overlaps

use std::collections::HashMap;
use crate::libreporting::*;

pub struct ModuleMarkovModel {
    pub module_name: String,
    pub line_transitions: HashMap<String, HashMap<String, u32>>,
    pub pattern_frequency: HashMap<String, u32>,
    pub reporting_density: f64,
}

impl ModuleMarkovModel {
    pub fn from_file(module_name: &str, content: &str) -> Self {
        let mut model = Self {
            module_name: module_name.to_string(),
            line_transitions: HashMap::new(),
            pattern_frequency: HashMap::new(),
            reporting_density: 0.0,
        };
        
        model.analyze_content(content);
        model
    }
    
    fn analyze_content(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        let mut reporting_lines = 0;
        
        // Build line-to-line transitions (Markov model)
        for window in lines.windows(2) {
            let from = self.normalize_line(window[0]);
            let to = self.normalize_line(window[1]);
            
            *self.line_transitions
                .entry(from)
                .or_insert_with(HashMap::new)
                .entry(to)
                .or_insert(0) += 1;
        }
        
        // Count reporting patterns
        for line in &lines {
            let normalized = self.normalize_line(line);
            *self.pattern_frequency.entry(normalized.clone()).or_insert(0) += 1;
            
            if self.is_reporting_line(line) {
                reporting_lines += 1;
            }
        }
        
        self.reporting_density = reporting_lines as f64 / lines.len() as f64;
    }
    
    fn normalize_line(&self, line: &str) -> String {
        // Normalize to pattern categories
        if line.contains("println!(\"🔥") { "HEADER".to_string() }
        else if line.contains("===") { "SEPARATOR".to_string() }
        else if line.contains("println!(\"📊") { "METRIC".to_string() }
        else if line.contains("println!(\"🔧") { "SECTION".to_string() }
        else if line.contains("println!(\"✅") { "FOOTER".to_string() }
        else if line.contains("println!()") { "BLANK".to_string() }
        else if line.trim().is_empty() { "EMPTY".to_string() }
        else if line.contains("println!") { "PRINT".to_string() }
        else { "CODE".to_string() }
    }
    
    fn is_reporting_line(&self, line: &str) -> bool {
        line.contains("🔥") || line.contains("===") || 
        line.contains("📊") || line.contains("🔧") || 
        line.contains("✅") || line.contains("Files processed") ||
        line.contains("Target:") || line.contains("SUMMARY")
    }
    
    pub fn similarity_to(&self, other: &ModuleMarkovModel) -> f64 {
        let mut common_transitions = 0;
        let mut total_transitions = 0;
        
        // Compare Markov transition patterns
        for (from, transitions) in &self.line_transitions {
            if let Some(other_transitions) = other.line_transitions.get(from) {
                for (to, count) in transitions {
                    total_transitions += count;
                    if let Some(other_count) = other_transitions.get(to) {
                        common_transitions += count.min(other_count);
                    }
                }
            }
        }
        
        if total_transitions == 0 { 0.0 }
        else { common_transitions as f64 / total_transitions as f64 }
    }
}

pub fn find_reporting_overlaps(modules: &[ModuleMarkovModel]) -> Vec<(String, String, f64)> {
    let mut overlaps = Vec::new();
    
    for i in 0..modules.len() {
        for j in i+1..modules.len() {
            let similarity = modules[i].similarity_to(&modules[j]);
            
            // Only report high similarity with reporting patterns
            if similarity > 0.3 && 
               (modules[i].reporting_density > 0.1 || modules[j].reporting_density > 0.1) {
                overlaps.push((
                    modules[i].module_name.clone(),
                    modules[j].module_name.clone(),
                    similarity
                ));
            }
        }
    }
    
    overlaps.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    overlaps
}
