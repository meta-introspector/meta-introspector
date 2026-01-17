// Self-Compilation Job Queue: Nodes buy source code snippets to process
// Profile execution, extract minimal snippets that reach new nodes

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SourceSnippet {
    pub id: u64,
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub code: String,
    pub compressed_size: usize,
    pub price: u64,
    pub reaches_new_nodes: bool,
    pub perf_trace: Vec<u64>,  // Instruction pointers
}

#[derive(Debug)]
pub struct SelfCompilationQueue {
    pub snippets: Vec<SourceSnippet>,
    pub processed_nodes: HashSet<u64>,  // IPs we've seen
    pub total_coverage: usize,
}

impl SelfCompilationQueue {
    pub fn new() -> Self {
        Self {
            snippets: Vec::new(),
            processed_nodes: HashSet::new(),
            total_coverage: 0,
        }
    }
    
    /// Step 1: Read our own source code
    pub fn load_self_source(&mut self) -> Result<(), String> {
        println!("📖 Loading self source code...");
        
        // Read all .rs files in current directory
        let paths = std::fs::read_dir(".")
            .map_err(|e| e.to_string())?;
        
        let mut snippet_id = 0u64;
        
        for entry in paths.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // Split into snippets (functions)
                    let snippets = self.extract_functions(&content, path.to_string_lossy().to_string());
                    
                    for snippet in snippets {
                        snippet_id += 1;
                        self.snippets.push(SourceSnippet {
                            id: snippet_id,
                            file: snippet.0,
                            start_line: snippet.1,
                            end_line: snippet.2,
                            code: snippet.3,
                            compressed_size: 0,
                            price: 100,  // Base price
                            reaches_new_nodes: false,
                            perf_trace: Vec::new(),
                        });
                    }
                }
            }
        }
        
        println!("  Found {} source snippets", self.snippets.len());
        Ok(())
    }
    
    /// Extract functions from source
    fn extract_functions(&self, content: &str, file: String) -> Vec<(String, usize, usize, String)> {
        let mut functions = Vec::new();
        let mut in_function = false;
        let mut fn_start = 0;
        let mut brace_count = 0;
        let mut fn_code = String::new();
        
        for (line_num, line) in content.lines().enumerate() {
            if line.trim().starts_with("fn ") || line.trim().starts_with("pub fn ") {
                in_function = true;
                fn_start = line_num;
                fn_code.clear();
                brace_count = 0;
            }
            
            if in_function {
                fn_code.push_str(line);
                fn_code.push('\n');
                
                brace_count += line.matches('{').count() as i32;
                brace_count -= line.matches('}').count() as i32;
                
                if brace_count == 0 && line.contains('}') {
                    functions.push((file.clone(), fn_start, line_num, fn_code.clone()));
                    in_function = false;
                }
            }
        }
        
        functions
    }
    
    /// Step 2: Compress each snippet
    pub fn compress_snippets(&mut self) {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        for snippet in &mut self.snippets {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
            let _ = encoder.write_all(snippet.code.as_bytes());
            if let Ok(compressed) = encoder.finish() {
                snippet.compressed_size = compressed.len();
                // Price based on compressed size
                snippet.price = (snippet.compressed_size as u64 * 10).max(10);
            }
        }
    }
    
    /// Step 3: Compile snippet and get perf trace
    pub fn profile_snippet(&mut self, snippet_id: u64) -> Result<Vec<u64>, String> {
        // Parse perf trace first
        let trace = self.parse_perf_trace(snippet_id)?;
        
        // Find snippet
        let snippet = self.snippets.iter_mut()
            .find(|s| s.id == snippet_id)
            .ok_or("Snippet not found")?;
        
        // Write to temp file (skipped for now)
        
        // Check for new nodes
        let mut new_nodes = 0;
        for &ip in &trace {
            if self.processed_nodes.insert(ip) {
                new_nodes += 1;
            }
        }
        
        snippet.reaches_new_nodes = new_nodes > 0;
        snippet.perf_trace = trace.clone();
        
        if new_nodes > 0 {
            println!("  ✨ Snippet {} reaches {} new nodes!", snippet_id, new_nodes);
            self.total_coverage += new_nodes;
        }
        
        Ok(trace)
    }
    
    fn parse_perf_trace(&self, _snippet_id: u64) -> Result<Vec<u64>, String> {
        // Simplified: generate mock IPs based on snippet
        use crate::rand_shim::random_u64;
        let base = random_u64() & 0xFFFF_F000;
        Ok(vec![base, base + 0x10, base + 0x20, base + 0x30])
    }
    
    /// Step 4: Score snippet based on new coverage
    pub fn score_snippet(&self, snippet_id: u64) -> f64 {
        if let Some(snippet) = self.snippets.iter().find(|s| s.id == snippet_id) {
            let mut score = 0.0;
            
            // New nodes = high value
            if snippet.reaches_new_nodes {
                score += 100.0;
            }
            
            // Unique IPs
            score += snippet.perf_trace.len() as f64 * 10.0;
            
            // Compression efficiency
            let code_len = snippet.code.len();
            if snippet.compressed_size > 0 {
                score += 100.0 / (snippet.compressed_size as f64 / code_len as f64);
            }
            
            score
        } else {
            0.0
        }
    }
    
    /// Get snippets that reach new nodes (high value)
    pub fn get_valuable_snippets(&self) -> Vec<&SourceSnippet> {
        let mut valuable: Vec<_> = self.snippets.iter()
            .filter(|s| s.reaches_new_nodes)
            .collect();
        valuable.sort_by(|a, b| b.perf_trace.len().cmp(&a.perf_trace.len()));
        valuable
    }
    
    pub fn report(&self) {
        println!("\n📊 Self-Compilation Queue Report");
        println!("  Total snippets: {}", self.snippets.len());
        println!("  Processed nodes: {}", self.processed_nodes.len());
        println!("  Total coverage: {}", self.total_coverage);
        
        let valuable = self.get_valuable_snippets();
        println!("  Valuable snippets: {}", valuable.len());
        
        println!("\n  Top 5 snippets by coverage:");
        for (i, snippet) in valuable.iter().take(5).enumerate() {
            println!("    {}. {} ({}:{}) - {} IPs, {} bytes compressed, price: {}", 
                     i + 1,
                     snippet.file,
                     snippet.start_line,
                     snippet.end_line,
                     snippet.perf_trace.len(),
                     snippet.compressed_size,
                     snippet.price);
        }
    }
}

/// Node job: Buy and process source snippets
#[derive(Debug, Clone)]
pub struct NodeJob {
    pub node_id: usize,
    pub balance: u64,
    pub processed_snippets: Vec<u64>,
    pub coverage_gained: usize,
    pub evolved_snippets: Vec<SourceSnippet>,
    pub earnings: u64,
}

impl NodeJob {
    pub fn new(node_id: usize, balance: u64) -> Self {
        Self {
            node_id,
            balance,
            processed_snippets: Vec::new(),
            coverage_gained: 0,
            evolved_snippets: Vec::new(),
            earnings: 0,
        }
    }
    
    /// Buy snippet if we can afford it
    pub fn buy_snippet(&mut self, snippet: &SourceSnippet) -> bool {
        if self.balance >= snippet.price {
            self.balance -= snippet.price;
            self.processed_snippets.push(snippet.id);
            true
        } else {
            false
        }
    }
    
    /// Process snippet: compile, profile, score
    pub fn process_snippet(&mut self, queue: &mut SelfCompilationQueue, snippet_id: u64) -> f64 {
        if let Ok(trace) = queue.profile_snippet(snippet_id) {
            self.coverage_gained += trace.len();
            queue.score_snippet(snippet_id)
        } else {
            0.0
        }
    }
    
    /// Evolve snippet: compress better, optimize code
    pub fn evolve_snippet(&mut self, snippet: &SourceSnippet) -> SourceSnippet {
        use crate::rand_shim::random_u64;
        
        let mut evolved = snippet.clone();
        evolved.id = random_u64();
        
        // Evolution strategies
        let strategy = random_u64() % 4;
        
        match strategy {
            0 => {
                // Remove whitespace
                evolved.code = evolved.code.split_whitespace().collect::<Vec<_>>().join(" ");
            }
            1 => {
                // Inline small functions
                if evolved.code.contains("fn ") && evolved.code.len() < 200 {
                    evolved.code = format!("#[inline]\n{}", evolved.code);
                }
            }
            2 => {
                // Add const
                evolved.code = evolved.code.replace("let ", "const ");
            }
            3 => {
                // Optimize loops
                evolved.code = evolved.code.replace("for ", "for _ in ");
            }
            _ => {}
        }
        
        // Recompress
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let _ = encoder.write_all(evolved.code.as_bytes());
        if let Ok(compressed) = encoder.finish() {
            evolved.compressed_size = compressed.len();
            
            // If better compression, earn reward
            if evolved.compressed_size < snippet.compressed_size {
                let improvement = snippet.compressed_size - evolved.compressed_size;
                self.earnings += improvement as u64 * 100;
                evolved.price = (evolved.compressed_size as u64 * 10).max(10);
            }
        }
        
        evolved
    }
    
    /// Sell evolved snippet back to queue
    pub fn sell_snippet(&mut self, snippet: SourceSnippet) -> u64 {
        let sale_price = snippet.price * 2;  // 2x markup
        self.balance += sale_price;
        self.earnings += sale_price;
        self.evolved_snippets.push(snippet);
        sale_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_queue() {
        let mut queue = SelfCompilationQueue::new();
        assert_eq!(queue.snippets.len(), 0);
    }
}
