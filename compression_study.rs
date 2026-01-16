// Compression Tool Study: Analyze source, complexity, and conformal fields
// Use librustc driver to analyze compression tool source code

use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CompressionSourceAnalysis {
    pub tool_name: String,
    pub source_path: PathBuf,
    pub is_pure_rust: bool,
    pub total_lines: usize,
    pub rust_lines: usize,
    pub c_lines: usize,
    pub functions: Vec<FunctionAnalysis>,
    pub complexity_score: usize,
}

#[derive(Debug, Clone)]
pub struct FunctionAnalysis {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub cyclomatic_complexity: usize,
    pub instruction_count: usize,
    pub calls_unsafe: bool,
}

pub struct CompressionStudy {
    tools: Vec<CompressionSourceAnalysis>,
}

impl CompressionStudy {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }
    
    /// Find compression tools in nix store
    pub fn discover_tools(&mut self) -> Result<(), String> {
        let nix_store = PathBuf::from("/nix/store");
        
        // Look for compression libraries
        let tool_patterns = vec![
            ("lz4", "lz4"),
            ("zstd", "zstd"),
            ("brotli", "brotli"),
            ("xz", "xz"),
            ("zlib", "zlib"),
        ];
        
        for (name, pattern) in tool_patterns {
            if let Ok(entries) = std::fs::read_dir(&nix_store) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.to_string_lossy().contains(pattern) {
                        println!("📦 Found {}: {:?}", name, path);
                        
                        // Analyze this tool
                        if let Ok(analysis) = self.analyze_tool(name, &path) {
                            self.tools.push(analysis);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze a compression tool's source
    fn analyze_tool(&self, name: &str, path: &PathBuf) -> Result<CompressionSourceAnalysis, String> {
        let mut analysis = CompressionSourceAnalysis {
            tool_name: name.to_string(),
            source_path: path.clone(),
            is_pure_rust: false,
            total_lines: 0,
            rust_lines: 0,
            c_lines: 0,
            functions: Vec::new(),
            complexity_score: 0,
        };
        
        // Scan for source files
        self.scan_directory(path, &mut analysis)?;
        
        // Calculate complexity
        analysis.complexity_score = analysis.functions.iter()
            .map(|f| f.cyclomatic_complexity)
            .sum();
        
        // Determine if pure Rust
        analysis.is_pure_rust = analysis.c_lines == 0 && analysis.rust_lines > 0;
        
        Ok(analysis)
    }
    
    fn scan_directory(&self, dir: &PathBuf, analysis: &mut CompressionSourceAnalysis) -> Result<(), String> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    let _ = self.scan_directory(&path, analysis);
                } else if let Some(ext) = path.extension() {
                    match ext.to_str() {
                        Some("rs") => {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                analysis.rust_lines += content.lines().count();
                                analysis.total_lines += content.lines().count();
                                
                                // Parse functions
                                self.parse_rust_functions(&content, &path, analysis);
                            }
                        }
                        Some("c") | Some("h") => {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                analysis.c_lines += content.lines().count();
                                analysis.total_lines += content.lines().count();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_rust_functions(&self, content: &str, path: &PathBuf, analysis: &mut CompressionSourceAnalysis) {
        for (line_num, line) in content.lines().enumerate() {
            if line.trim().starts_with("fn ") || line.trim().starts_with("pub fn ") {
                // Extract function name
                if let Some(name_start) = line.find("fn ") {
                    let after_fn = &line[name_start + 3..];
                    if let Some(paren) = after_fn.find('(') {
                        let name = after_fn[..paren].trim().to_string();
                        
                        analysis.functions.push(FunctionAnalysis {
                            name,
                            file: path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            cyclomatic_complexity: 1, // Simplified
                            instruction_count: 0,
                            calls_unsafe: line.contains("unsafe"),
                        });
                    }
                }
            }
        }
    }
    
    /// Generate report comparing all tools
    pub fn report(&self) {
        println!("\n🔬 Compression Tool Analysis Report\n");
        println!("{:<15} {:<10} {:<10} {:<10} {:<10}", 
                 "Tool", "Pure Rust", "Lines", "Functions", "Complexity");
        println!("{}", "=".repeat(65));
        
        for tool in &self.tools {
            println!("{:<15} {:<10} {:<10} {:<10} {:<10}",
                     tool.tool_name,
                     if tool.is_pure_rust { "✅" } else { "❌" },
                     tool.total_lines,
                     tool.functions.len(),
                     tool.complexity_score);
        }
        
        println!("\n📊 Complexity Ranking:");
        let mut sorted = self.tools.clone();
        sorted.sort_by_key(|t| t.complexity_score);
        
        for (i, tool) in sorted.iter().enumerate() {
            println!("  {}. {} (complexity: {})", 
                     i + 1, tool.tool_name, tool.complexity_score);
        }
    }
    
    /// Get tool for detailed analysis
    pub fn get_tool(&self, name: &str) -> Option<&CompressionSourceAnalysis> {
        self.tools.iter().find(|t| t.tool_name == name)
    }
}

/// Markov model: Bits → Compression operations → Code
#[derive(Debug)]
pub struct CompressionMarkovModel {
    /// State transitions: (current_state, input_bits) → (next_state, operation)
    transitions: HashMap<(u8, u8), (u8, String)>,
    current_state: u8,
}

impl CompressionMarkovModel {
    pub fn new() -> Self {
        let mut model = Self {
            transitions: HashMap::new(),
            current_state: 0,
        };
        
        // Define transitions for compression operations
        // State 0: Initial
        model.transitions.insert((0, 0b00), (1, "LiteralCopy".to_string()));
        model.transitions.insert((0, 0b01), (2, "LZ77Match".to_string()));
        model.transitions.insert((0, 0b10), (3, "HuffmanEncode".to_string()));
        model.transitions.insert((0, 0b11), (4, "RangeEncode".to_string()));
        
        // State 1: After literal
        model.transitions.insert((1, 0b00), (1, "LiteralCopy".to_string()));
        model.transitions.insert((1, 0b01), (2, "LZ77Match".to_string()));
        
        // State 2: After match
        model.transitions.insert((2, 0b00), (1, "LiteralCopy".to_string()));
        model.transitions.insert((2, 0b01), (2, "LZ77Match".to_string()));
        
        model
    }
    
    /// Process bits and generate compression operations
    pub fn process_bits(&mut self, bits: &[u8]) -> Vec<String> {
        let mut operations = Vec::new();
        
        for &byte in bits {
            // Process 2 bits at a time
            for shift in (0..8).step_by(2) {
                let two_bits = (byte >> shift) & 0b11;
                
                if let Some((next_state, operation)) = self.transitions.get(&(self.current_state, two_bits)) {
                    operations.push(operation.clone());
                    self.current_state = *next_state;
                }
            }
        }
        
        operations
    }
    
    /// Generate Rust code from operations
    pub fn operations_to_code(&self, operations: &[String]) -> String {
        let mut code = String::from("fn compress(input: &[u8]) -> Vec<u8> {\n");
        code.push_str("    let mut output = Vec::new();\n");
        
        for op in operations {
            match op.as_str() {
                "LiteralCopy" => code.push_str("    output.push(input[pos]);\n"),
                "LZ77Match" => code.push_str("    output.extend_from_slice(&match_data);\n"),
                "HuffmanEncode" => code.push_str("    output.push(huffman_code);\n"),
                "RangeEncode" => code.push_str("    output.extend(range_encode(symbol));\n"),
                _ => {}
            }
        }
        
        code.push_str("    output\n");
        code.push_str("}\n");
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_markov_model() {
        let mut model = CompressionMarkovModel::new();
        let bits = vec![0b00110011];
        let ops = model.process_bits(&bits);
        assert!(!ops.is_empty());
        
        let code = model.operations_to_code(&ops);
        assert!(code.contains("fn compress"));
    }
}
