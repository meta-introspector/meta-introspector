// Bits → Valid Rust: Force random bits into complexity orbits
// All bit patterns are valid programs at some complexity level

use libnix::rand_shim::random_u64;

/// Map N bits to valid Rust program at complexity Y
pub struct BitsToRust {
    complexity_templates: Vec<Vec<&'static str>>,
}

impl BitsToRust {
    pub fn new() -> Self {
        Self {
            complexity_templates: vec![
                // Complexity 0: Constants
                vec!["0", "1", "true", "false", "()"],
                
                // Complexity 1: Simple expressions
                vec!["x", "y", "n", "a + b", "x * 2", "n - 1"],
                
                // Complexity 2: Function calls
                vec!["f(x)", "g(a, b)", "h()", "x.len()", "v.iter()"],
                
                // Complexity 3: Control flow
                vec![
                    "if x { a } else { b }",
                    "match x { 0 => a, _ => b }",
                    "for i in 0..n { f(i) }",
                ],
                
                // Complexity 4: Functions
                vec![
                    "fn f(x: i32) -> i32 { x + 1 }",
                    "fn g(a: i32, b: i32) -> i32 { a * b }",
                    "|x| x * 2",
                ],
                
                // Complexity 5: Structs
                vec![
                    "struct Point { x: i32, y: i32 }",
                    "struct Node { val: i32, next: Option<Box<Node>> }",
                ],
                
                // Complexity 6: Traits
                vec![
                    "trait Add { fn add(&self, other: &Self) -> Self; }",
                    "impl Add for i32 { fn add(&self, other: &Self) -> Self { self + other } }",
                ],
                
                // Complexity 7: Generics
                vec![
                    "fn id<T>(x: T) -> T { x }",
                    "struct Pair<T, U> { first: T, second: U }",
                ],
            ],
        }
    }
    
    /// Convert bits to valid Rust at target complexity
    pub fn bits_to_rust(&self, bits: &[u8], target_complexity: usize) -> String {
        let complexity = target_complexity.min(self.complexity_templates.len() - 1);
        let templates = &self.complexity_templates[complexity];
        
        // Use bits to select template
        let template_idx = if !bits.is_empty() {
            bits[0] as usize % templates.len()
        } else {
            0
        };
        
        let template = templates[template_idx];
        
        // Use remaining bits to fill in variables
        let mut code = template.to_string();
        
        // Replace variables with bit-derived values
        if bits.len() > 1 {
            let val = bits[1] as i32;
            code = code.replace("x", &val.to_string());
            code = code.replace("n", &(val % 10).to_string());
        }
        
        code
    }
    
    /// Generate complete Rust program from bits
    pub fn generate_program(&self, bits: &[u8], complexity: usize) -> String {
        let mut program = String::new();
        
        // Add necessary imports based on complexity
        if complexity >= 2 {
            program.push_str("use std::collections::*;\n");
        }
        
        program.push('\n');
        
        // Generate main function
        program.push_str("fn main() {\n");
        
        // Generate body based on complexity
        let body = self.bits_to_rust(bits, complexity);
        program.push_str("    let result = ");
        program.push_str(&body);
        program.push_str(";\n");
        
        // Add print for verification
        if complexity <= 1 {
            program.push_str("    println!(\"Result: {:?}\", result);\n");
        }
        
        program.push_str("}\n");
        
        program
    }
    
    /// Calculate complexity from bit pattern
    pub fn bits_to_complexity(&self, bits: &[u8]) -> usize {
        if bits.is_empty() {
            return 0;
        }
        
        // Complexity = number of set bits / 8
        let set_bits: usize = bits.iter().map(|b| b.count_ones() as usize).sum();
        (set_bits / 8).min(self.complexity_templates.len() - 1)
    }
    
    /// Compile and test the generated code
    pub fn verify_compiles(&self, code: &str) -> bool {
        // Write to temp file
        let temp_file = format!("/tmp/meme_{}.rs", random_u64());
        if std::fs::write(&temp_file, code).is_err() {
            return false;
        }
        
        // Try to compile
        let output = std::process::Command::new("rustc")
            .arg("--crate-type=bin")
            .arg("--out-dir=/tmp")
            .arg(&temp_file)
            .output();
        
        // Clean up
        let _ = std::fs::remove_file(&temp_file);
        
        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }
    
    /// Compile to WASM for pure evaluation
    pub fn compile_to_wasm(&self, code: &str) -> Option<Vec<u8>> {
        let temp_file = format!("/tmp/meme_{}.rs", random_u64());
        let wasm_file = format!("/tmp/meme_{}.wasm", random_u64());
        
        if std::fs::write(&temp_file, code).is_err() {
            return None;
        }
        
        // Compile to WASM
        let output = std::process::Command::new("rustc")
            .arg("--target=wasm32-unknown-unknown")
            .arg("--crate-type=cdylib")
            .arg("-C")
            .arg("opt-level=z")
            .arg("-o")
            .arg(&wasm_file)
            .arg(&temp_file)
            .output();
        
        let wasm_bytes = if output.is_ok() {
            std::fs::read(&wasm_file).ok()
        } else {
            None
        };
        
        // Clean up
        let _ = std::fs::remove_file(&temp_file);
        let _ = std::fs::remove_file(&wasm_file);
        
        wasm_bytes
    }
}

/// Metrics for generated code
#[derive(Debug, Clone)]
pub struct CodeMetrics {
    pub complexity: usize,
    pub lines: usize,
    pub tokens: usize,
    pub compiles: bool,
    pub execution_time_ns: Option<u64>,
}

impl CodeMetrics {
    pub fn analyze(code: &str, compiles: bool) -> Self {
        let lines = code.lines().count();
        let tokens = code.split_whitespace().count();
        
        // Estimate complexity from AST depth
        let complexity = (tokens / 5).max(1);
        
        Self {
            complexity,
            lines,
            tokens,
            compiles,
            execution_time_ns: None,
        }
    }
    
    pub fn report(&self) {
        println!("📊 Code Metrics:");
        println!("   Complexity: {}", self.complexity);
        println!("   Lines: {}", self.lines);
        println!("   Tokens: {}", self.tokens);
        println!("   Compiles: {}", if self.compiles { "✅" } else { "❌" });
        if let Some(time) = self.execution_time_ns {
            println!("   Execution: {} ns", time);
        }
    }
}
