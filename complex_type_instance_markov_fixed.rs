use std::collections::HashMap;
use std::fs;
use syn::{parse_file, visit::Visit, Expr, ExprStruct, Member};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstanceModel {
    instance_count: u32,
    field_patterns: HashMap<String, u32>,
    literal_values: Vec<String>,
}

struct ComplexTypeInstanceAnalyzer {
    instance_models: HashMap<String, InstanceModel>,
    total_instances: u32,
}

impl ComplexTypeInstanceAnalyzer {
    fn new() -> Self {
        Self {
            instance_models: HashMap::new(),
            total_instances: 0,
        }
    }

    fn analyze_struct_instance(&mut self, expr: &ExprStruct) {
        if let Some(type_name) = self.extract_type_name(&expr.path) {
            self.total_instances += 1;
            
            // Collect field data first
            let mut field_data = Vec::new();
            for field in &expr.fields {
                if let Member::Named(field_name) = &field.member {
                    let literal = self.extract_literal(&field.expr);
                    field_data.push((field_name.to_string(), literal));
                }
            }
            
            // Now update the model
            let model = self.instance_models.entry(type_name).or_insert_with(|| {
                InstanceModel {
                    instance_count: 0,
                    field_patterns: HashMap::new(),
                    literal_values: Vec::new(),
                }
            });
            
            model.instance_count += 1;
            
            for (field_name, literal) in field_data {
                *model.field_patterns.entry(field_name).or_default() += 1;
                if let Some(lit) = literal {
                    model.literal_values.push(lit);
                }
            }
        }
    }

    fn extract_type_name(&self, path: &syn::Path) -> Option<String> {
        path.segments.last().map(|s| s.ident.to_string())
    }

    fn extract_literal(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Lit(lit) => {
                match &lit.lit {
                    syn::Lit::Str(s) => Some(format!("\"{}\"", s.value())),
                    syn::Lit::Int(i) => Some(i.base10_digits().to_string()),
                    syn::Lit::Float(f) => Some(f.base10_digits().to_string()),
                    syn::Lit::Bool(b) => Some(b.value.to_string()),
                    _ => Some("literal".to_string()),
                }
            },
            _ => None,
        }
    }
}

impl<'ast> Visit<'ast> for ComplexTypeInstanceAnalyzer {
    fn visit_expr_struct(&mut self, expr: &'ast ExprStruct) {
        self.analyze_struct_instance(expr);
        syn::visit::visit_expr_struct(self, expr);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏭 COMPLEX TYPE INSTANCE MARKOV ANALYSIS");
    println!("=======================================");

    let mut analyzer = ComplexTypeInstanceAnalyzer::new();
    let mut processed_files = 0;

    let base_dir = "/home/mdupont/zombie_driver2";
    
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rs") {
                    let file_path = entry.path();
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        if let Ok(syntax_tree) = parse_file(&content) {
                            analyzer.visit_file(&syntax_tree);
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 ANALYSIS RESULTS:");
    println!("Files processed: {}", processed_files);
    println!("Total instances: {}", analyzer.total_instances);
    println!("Unique types: {}", analyzer.instance_models.len());

    let mut sorted_models: Vec<_> = analyzer.instance_models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.instance_count.cmp(&a.1.instance_count));

    println!("\n🔥 TOP STRUCT INSTANCES:");
    for (type_name, model) in sorted_models.iter().take(10) {
        println!("  {}: {} instances, {} fields", 
                 type_name, model.instance_count, model.field_patterns.len());
    }

    let analysis_result = serde_json::to_string_pretty(&analyzer.instance_models)?;
    fs::write("complex_type_instance_markov.json", analysis_result)?;

    println!("\nAnalysis saved to complex_type_instance_markov.json");
    Ok(())
}
