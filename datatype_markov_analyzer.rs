use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Expr, Lit, LitInt, LitBool};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataTypeMarkov {
    type_name: String,
    bit_width: usize,
    value_counts: HashMap<String, u32>,
    context_counts: HashMap<String, u32>,
    transitions: HashMap<String, HashMap<String, u32>>,
    total_instances: u32,
}

struct TypeAnalyzer {
    models: HashMap<String, DataTypeMarkov>,
    current_context: String,
}

impl TypeAnalyzer {
    fn new() -> Self {
        Self {
            models: HashMap::new(),
            current_context: "unknown".to_string(),
        }
    }

    fn add_value(&mut self, type_name: &str, bit_width: usize, value: &str, context: &str) {
        let model = self.models.entry(type_name.to_string()).or_insert_with(|| {
            DataTypeMarkov {
                type_name: type_name.to_string(),
                bit_width,
                value_counts: HashMap::new(),
                context_counts: HashMap::new(),
                transitions: HashMap::new(),
                total_instances: 0,
            }
        });

        model.total_instances += 1;
        *model.value_counts.entry(value.to_string()).or_default() += 1;
        *model.context_counts.entry(context.to_string()).or_default() += 1;

        // Build bit-level transitions for binary representation
        if let Ok(num_val) = value.parse::<u64>() {
            let binary = format!("{:0width$b}", num_val, width = bit_width);
            let bits: Vec<char> = binary.chars().collect();
            
            for i in 0..bits.len().saturating_sub(1) {
                let current_bit = bits[i].to_string();
                let next_bit = bits[i + 1].to_string();
                
                *model.transitions
                    .entry(current_bit)
                    .or_default()
                    .entry(next_bit)
                    .or_default() += 1;
            }
        }
    }
}

impl<'ast> Visit<'ast> for TypeAnalyzer {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(expr_lit) => {
                match &expr_lit.lit {
                    Lit::Bool(lit_bool) => {
                        let value = if lit_bool.value { "1" } else { "0" };
                        let context = self.current_context.clone();
                        self.add_value("bool", 1, value, &context);
                    }
                    Lit::Int(lit_int) => {
                        let digits = lit_int.base10_digits();
                        let suffix = lit_int.suffix();
                        
                        let (type_name, bit_width) = match suffix {
                            "u8" => ("u8", 8),
                            "u16" => ("u16", 16), 
                            "u32" => ("u32", 32),
                            "u64" => ("u64", 64),
                            "u128" => ("u128", 128),
                            "usize" => ("usize", 64), // assume 64-bit
                            "i8" => ("i8", 8),
                            "i16" => ("i16", 16),
                            "i32" => ("i32", 32),
                            "i64" => ("i64", 64),
                            "i128" => ("i128", 128),
                            "isize" => ("isize", 64),
                            _ => ("int", 32), // default
                        };
                        
                        let context = self.current_context.clone();
                        self.add_value(type_name, bit_width, digits, &context);
                    }
                    Lit::Byte(lit_byte) => {
                        let context = self.current_context.clone();
                        self.add_value("u8", 8, &lit_byte.value().to_string(), &context);
                    }
                    _ => {}
                }
            }
            Expr::Binary(binary_expr) => {
                self.current_context = "binary_op".to_string();
                syn::visit::visit_expr(self, &binary_expr.left);
                syn::visit::visit_expr(self, &binary_expr.right);
                self.current_context = "unknown".to_string();
                return;
            }
            Expr::Assign(assign_expr) => {
                self.current_context = "assignment".to_string();
                syn::visit::visit_expr(self, &assign_expr.right);
                self.current_context = "unknown".to_string();
                return;
            }
            Expr::Call(_) => {
                self.current_context = "function_call".to_string();
            }
            Expr::Array(_) => {
                self.current_context = "array_literal".to_string();
            }
            _ => {}
        }
        
        syn::visit::visit_expr(self, expr);
        self.current_context = "unknown".to_string();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 RUST DATA TYPE MARKOV MODELS");
    println!("===============================");

    let mut analyzer = TypeAnalyzer::new();
    let mut processed_files = 0;

    // Process zombie_driver2 files
    let base_dir = "/home/mdupont/zombie_driver2";
    
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rs") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(file) = parse_file(&content) {
                            analyzer.visit_file(&file);
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 DATA TYPE ANALYSIS:");
    println!("Type    | Bits | Instances | Unique | 0→1 | 1→0 | Most Common");
    println!("--------|------|-----------|--------|-----|-----|------------");

    let mut sorted_models: Vec<_> = analyzer.models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.total_instances.cmp(&a.1.total_instances));

    for (type_name, model) in &sorted_models {
        let unique_values = model.value_counts.len();
        
        // Bit transition analysis
        let zero_to_one = model.transitions.get("0")
            .and_then(|t| t.get("1"))
            .unwrap_or(&0);
        let one_to_zero = model.transitions.get("1")
            .and_then(|t| t.get("0"))
            .unwrap_or(&0);
            
        // Most common value
        let most_common = model.value_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(val, count)| format!("{}({})", val, count))
            .unwrap_or_else(|| "none".to_string());

        println!("{:7} | {:4} | {:9} | {:6} | {:3} | {:3} | {}", 
                 type_name, 
                 model.bit_width,
                 model.total_instances, 
                 unique_values,
                 zero_to_one,
                 one_to_zero,
                 most_common);
    }

    // Detailed bit analysis for bool type
    if let Some(bool_model) = analyzer.models.get("bool") {
        println!("\n🔍 BOOLEAN BIT ANALYSIS:");
        let zeros = bool_model.value_counts.get("0").unwrap_or(&0);
        let ones = bool_model.value_counts.get("1").unwrap_or(&0);
        let total = zeros + ones;
        
        if total > 0 {
            println!("0 (false): {} occurrences ({:.1}%)", zeros, *zeros as f64 / total as f64 * 100.0);
            println!("1 (true):  {} occurrences ({:.1}%)", ones, *ones as f64 / total as f64 * 100.0);
        }
        
        println!("\nContext distribution:");
        for (context, count) in &bool_model.context_counts {
            println!("  {}: {} times", context, count);
        }
    }

    // Save models
    for (type_name, model) in &analyzer.models {
        let filename = format!("datatype_markov_{}.json", type_name);
        let json = serde_json::to_string_pretty(model)?;
        fs::write(&filename, json)?;
    }

    println!("\n🎯 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Data type models: {}", analyzer.models.len());
    println!("Models saved to datatype_markov_*.json");

    Ok(())
}
