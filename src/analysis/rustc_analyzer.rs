use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Expr, ExprStruct, Member};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstanceModel {
    instance_count: u32,
    field_patterns: HashMap<String, u32>,
    literal_values: Vec<String>,
}

struct RustcAnalyzer {
    instance_models: HashMap<String, InstanceModel>,
    total_instances: u32,
    processed_files: u32,
}

impl RustcAnalyzer {
    fn new() -> Self {
        Self {
            instance_models: HashMap::new(),
            total_instances: 0,
            processed_files: 0,
        }
    }

    fn analyze_struct_instance(&mut self, expr: &ExprStruct) {
        if let Some(type_name) = self.extract_type_name(&expr.path) {
            self.total_instances += 1;
            
            let mut field_data = Vec::new();
            for field in &expr.fields {
                if let Member::Named(field_name) = &field.member {
                    let literal = self.extract_literal(&field.expr);
                    field_data.push((field_name.to_string(), literal));
                }
            }
            
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

    fn process_directory(&mut self, start_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut dirs_to_process = vec![start_dir.to_path_buf()];
        
        while let Some(current_dir) = dirs_to_process.pop() {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        dirs_to_process.push(path);
                    } else if path.extension().is_some_and(|ext| ext == "rs") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(syntax_tree) = parse_file(&content) {
                                self.visit_file(&syntax_tree);
                                self.processed_files += 1;
                                
                                if self.processed_files.is_multiple_of(500) {
                                    println!("Processed {} files, {} instances", self.processed_files, self.total_instances);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'ast> Visit<'ast> for RustcAnalyzer {
    fn visit_expr_struct(&mut self, expr: &'ast ExprStruct) {
        self.analyze_struct_instance(expr);
        syn::visit::visit_expr_struct(self, expr);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let target_dir = if args.len() > 1 {
        &args[1]
    } else {
        "~/nix/vendor/rust/cargo2nix/submodules/rust-build"
    };

    println!("🦀 RUSTC COMPLEX TYPE ANALYSIS");
    println!("==============================");
    println!("Target: {}", target_dir);

    let mut analyzer = RustcAnalyzer::new();
    let path = Path::new(target_dir);
    
    analyzer.process_directory(path)?;

    println!("\n📊 RUSTC ANALYSIS RESULTS:");
    println!("Files processed: {}", analyzer.processed_files);
    println!("Total instances: {}", analyzer.total_instances);
    println!("Unique types: {}", analyzer.instance_models.len());

    let mut sorted_models: Vec<_> = analyzer.instance_models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.instance_count.cmp(&a.1.instance_count));

    println!("\n🔥 TOP RUSTC STRUCT INSTANCES:");
    for (type_name, model) in sorted_models.iter().take(15) {
        println!("  {}: {} instances, {} fields", 
                 type_name, model.instance_count, model.field_patterns.len());
    }

    let analysis_result = serde_json::to_string_pretty(&analyzer.instance_models)?;
    fs::write("rustc_complex_type_analysis.json", analysis_result)?;

    println!("\nAnalysis saved to rustc_complex_type_analysis.json");
    Ok(())
}
