use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Type, Expr, Lit, Item};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MarkovModel {
    data_type: String,
    transitions: HashMap<String, HashMap<String, u32>>,
    total_instances: u32,
    unique_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TypeInstance {
    value: String,
    context: String,
    file_path: String,
}

struct TypeCollector {
    models: HashMap<String, MarkovModel>,
    current_file: String,
}

impl TypeCollector {
    fn new() -> Self {
        Self {
            models: HashMap::new(),
            current_file: String::new(),
        }
    }

    fn add_instance(&mut self, type_name: &str, value: &str, context: &str) {
        let model = self.models.entry(type_name.to_string()).or_insert_with(|| {
            MarkovModel {
                data_type: type_name.to_string(),
                transitions: HashMap::new(),
                total_instances: 0,
                unique_values: Vec::new(),
            }
        });

        model.total_instances += 1;
        
        if !model.unique_values.contains(&value.to_string()) {
            model.unique_values.push(value.to_string());
        }

        // Build Markov transitions from character sequences
        let chars: Vec<char> = value.chars().collect();
        for i in 0..chars.len().saturating_sub(1) {
            let current = chars[i].to_string();
            let next = chars[i + 1].to_string();
            
            *model.transitions
                .entry(current)
                .or_default()
                .entry(next)
                .or_default() += 1;
        }
    }

    fn extract_type_name(ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                type_path.path.segments.last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            }
            Type::Reference(type_ref) => {
                format!("&{}", Self::extract_type_name(&type_ref.elem))
            }
            Type::Slice(type_slice) => {
                format!("[{}]", Self::extract_type_name(&type_slice.elem))
            }
            Type::Array(type_array) => {
                format!("[{}; N]", Self::extract_type_name(&type_array.elem))
            }
            _ => "complex_type".to_string(),
        }
    }
}

impl<'ast> Visit<'ast> for TypeCollector {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(expr_lit) => {
                let (type_name, value) = match &expr_lit.lit {
                    Lit::Str(lit_str) => ("String", lit_str.value()),
                    Lit::ByteStr(lit_byte_str) => ("&[u8]", format!("{:?}", lit_byte_str.value())),
                    Lit::Byte(lit_byte) => ("u8", lit_byte.value().to_string()),
                    Lit::Char(lit_char) => ("char", lit_char.value().to_string()),
                    Lit::Int(lit_int) => ("integer", lit_int.base10_digits().to_string()),
                    Lit::Float(lit_float) => ("float", lit_float.base10_digits().to_string()),
                    Lit::Bool(lit_bool) => ("bool", lit_bool.value.to_string()),
                    Lit::Verbatim(_) => ("verbatim", "complex".to_string()),
                };
                
                self.add_instance(type_name, &value, "literal");
            }
            _ => {}
        }
        
        syn::visit::visit_expr(self, expr);
    }

    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Struct(item_struct) => {
                self.add_instance("struct", &item_struct.ident.to_string(), "definition");
            }
            Item::Enum(item_enum) => {
                self.add_instance("enum", &item_enum.ident.to_string(), "definition");
            }
            Item::Fn(item_fn) => {
                self.add_instance("fn", &item_fn.sig.ident.to_string(), "definition");
            }
            Item::Type(item_type) => {
                self.add_instance("type_alias", &item_type.ident.to_string(), "definition");
            }
            _ => {}
        }
        
        syn::visit::visit_item(self, item);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 RUST TYPE MARKOV MODEL GENERATOR");
    println!("==================================");

    let mut collector = TypeCollector::new();
    let mut processed_files = 0;

    // Process Rust files from zombie_driver2
    let base_dir = "/home/mdupont/zombie_driver2";
    
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rs") {
                    collector.current_file = name.to_string();
                    
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(file) = parse_file(&content) {
                            collector.visit_file(&file);
                            processed_files += 1;
                            
                            if processed_files % 50 == 0 {
                                println!("Processed {} files...", processed_files);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 MARKOV MODEL RESULTS:");
    println!("Type        | Instances | Unique | Transitions");
    println!("------------|-----------|--------|------------");

    let mut sorted_models: Vec<_> = collector.models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.total_instances.cmp(&a.1.total_instances));

    for (type_name, model) in sorted_models.iter().take(15) {
        let transition_count: usize = model.transitions.values()
            .map(|inner| inner.len())
            .sum();
        
        println!("{:11} | {:9} | {:6} | {:11}", 
                 type_name, 
                 model.total_instances, 
                 model.unique_values.len(),
                 transition_count);
    }

    // Save detailed models
    for (type_name, model) in &collector.models {
        let filename = format!("markov_model_{}.json", type_name);
        let json = serde_json::to_string_pretty(model)?;
        fs::write(&filename, json)?;
    }

    println!("\n🎯 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Type models created: {}", collector.models.len());
    println!("Models saved to markov_model_*.json files");

    // Show most complex transitions
    if let Some((type_name, model)) = sorted_models.first() {
        println!("\n🔥 MOST ACTIVE TYPE: {}", type_name);
        println!("Sample transitions:");
        
        for (from, transitions) in model.transitions.iter().take(5) {
            for (to, count) in transitions.iter().take(3) {
                println!("  '{}' -> '{}': {} times", from, to, count);
            }
        }
    }

    Ok(())
}
