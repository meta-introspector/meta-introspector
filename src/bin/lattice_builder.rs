// Complexity Lattice Builder
// Constructs partial order from Galois field analysis

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaloisResult {
    pub language: String,
    pub galois_field: u32,  // n in GF(2^n)
    pub coverage: f64,
    pub samples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeNode {
    pub language: String,
    pub complexity: u32,
    pub less_than: Vec<String>,    // Languages simpler than this
    pub greater_than: Vec<String>, // Languages more complex than this
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityLattice {
    pub nodes: HashMap<String, LatticeNode>,
    pub levels: HashMap<u32, Vec<String>>,
    pub partial_order: Vec<(String, String)>, // (simpler, more_complex)
}

impl ComplexityLattice {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            levels: HashMap::new(),
            partial_order: Vec::new(),
        }
    }
    
    pub fn add_result(&mut self, result: GaloisResult) {
        let node = LatticeNode {
            language: result.language.clone(),
            complexity: result.galois_field,
            less_than: Vec::new(),
            greater_than: Vec::new(),
        };
        
        self.nodes.insert(result.language.clone(), node);
        self.levels.entry(result.galois_field)
            .or_insert_with(Vec::new)
            .push(result.language);
    }
    
    pub fn build_partial_order(&mut self) {
        // For each pair of languages, if A has lower complexity than B, A < B
        let languages: Vec<_> = self.nodes.keys().cloned().collect();
        
        for i in 0..languages.len() {
            for j in 0..languages.len() {
                if i == j { continue; }
                
                let lang_a = &languages[i];
                let lang_b = &languages[j];
                
                let complexity_a = self.nodes[lang_a].complexity;
                let complexity_b = self.nodes[lang_b].complexity;
                
                if complexity_a < complexity_b {
                    // A is simpler than B
                    self.nodes.get_mut(lang_a).unwrap()
                        .greater_than.push(lang_b.clone());
                    self.nodes.get_mut(lang_b).unwrap()
                        .less_than.push(lang_a.clone());
                    self.partial_order.push((lang_a.clone(), lang_b.clone()));
                }
            }
        }
    }
    
    pub fn to_graphviz(&self) -> String {
        let mut dot = String::from("digraph ComplexityLattice {\n");
        dot.push_str("  rankdir=BT;\n");
        dot.push_str("  node [shape=box];\n\n");
        
        // Group by complexity level
        for (level, langs) in &self.levels {
            dot.push_str(&format!("  // Level GF(2^{})\n", level));
            dot.push_str("  { rank=same; ");
            for lang in langs {
                dot.push_str(&format!("\"{}\" ", lang));
            }
            dot.push_str("}\n\n");
        }
        
        // Add edges (only direct relationships, not transitive)
        let direct_edges = self.compute_direct_edges();
        for (from, to) in direct_edges {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
        }
        
        dot.push_str("}\n");
        dot
    }
    
    fn compute_direct_edges(&self) -> Vec<(String, String)> {
        // Remove transitive edges to show only direct relationships
        let mut direct = Vec::new();
        
        for (simpler, complex) in &self.partial_order {
            let is_direct = !self.has_intermediate(simpler, complex);
            if is_direct {
                direct.push((simpler.clone(), complex.clone()));
            }
        }
        
        direct
    }
    
    fn has_intermediate(&self, from: &str, to: &str) -> bool {
        // Check if there's a language between from and to
        let from_complexity = self.nodes[from].complexity;
        let to_complexity = self.nodes[to].complexity;
        
        self.nodes.values().any(|node| {
            node.complexity > from_complexity && 
            node.complexity < to_complexity
        })
    }
    
    pub fn print_summary(&self) {
        println!("🔬 Complexity Lattice Summary");
        println!("==============================\n");
        
        println!("📊 Total languages: {}", self.nodes.len());
        println!("📊 Complexity levels: {}\n", self.levels.len());
        
        println!("🏔️  Complexity Hierarchy:");
        let mut sorted_levels: Vec<_> = self.levels.keys().collect();
        sorted_levels.sort();
        
        for level in sorted_levels {
            let langs = &self.levels[level];
            println!("  GF(2^{:2}) ({:5} states): {} languages", 
                level, 1u64 << level, langs.len());
            for lang in langs {
                println!("    - {}", lang);
            }
        }
        
        println!("\n📈 Partial Order Relationships: {}", self.partial_order.len());
    }
}

fn parse_analysis_file(path: &str) -> Option<GaloisResult> {
    let content = fs::read_to_string(path).ok()?;
    
    // Extract language from filename
    let filename = std::path::Path::new(path).file_stem()?.to_str()?;
    let language = filename.strip_suffix("_analysis")?.to_string();
    
    // Parse Galois field (look for "GF(2^N): 100.000000%")
    let mut galois_field = 0;
    let mut samples = 0;
    
    for line in content.lines() {
        if line.contains("samples") && !line.contains("TOP") {
            if let Some(num) = line.split_whitespace().nth(1) {
                samples = num.parse().unwrap_or(0);
            }
        }
        if line.contains("GF(2^") && line.contains("100.000000%") {
            if let Some(start) = line.find("GF(2^") {
                if let Some(end) = line[start..].find(')') {
                    let num_str = &line[start+5..start+end];
                    galois_field = num_str.parse().unwrap_or(0);
                }
            }
        }
    }
    
    if galois_field > 0 {
        Some(GaloisResult {
            language,
            galois_field,
            coverage: 100.0,
            samples,
        })
    } else {
        None
    }
}

fn main() {
    println!("🔬 Building Complexity Lattice from Galois Analysis\n");
    
    let mut lattice = ComplexityLattice::new();
    
    // Load all analysis files
    let results_dir = "data/71_results";
    if let Ok(entries) = fs::read_dir(results_dir) {
        for entry in entries.flatten() {
            if let Some(path) = entry.path().to_str() {
                if path.ends_with("_analysis.txt") {
                    if let Some(result) = parse_analysis_file(path) {
                        println!("✅ Loaded: {} -> GF(2^{})", result.language, result.galois_field);
                        lattice.add_result(result);
                    }
                }
            }
        }
    }
    
    // Build partial order
    lattice.build_partial_order();
    
    // Print summary
    println!();
    lattice.print_summary();
    
    // Save as JSON
    let json = serde_json::to_string_pretty(&lattice).unwrap();
    fs::write("data/complexity_lattice.json", json).unwrap();
    println!("\n✅ Saved to data/complexity_lattice.json");
    
    // Generate GraphViz
    let dot = lattice.to_graphviz();
    fs::write("data/complexity_lattice.dot", dot).unwrap();
    println!("✅ Saved to data/complexity_lattice.dot");
    
    println!("\n🎨 Visualize with:");
    println!("  dot -Tpng data/complexity_lattice.dot -o complexity_lattice.png");
}
