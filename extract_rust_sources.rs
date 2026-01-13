use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use syn::{parse_file, visit::Visit};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RustSourceAnalysis {
    total_files: u32,
    models: HashMap<String, TypeModel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeModel {
    instances: u32,
    patterns: HashMap<String, u32>,
}

struct InMemoryAnalyzer {
    models: HashMap<String, TypeModel>,
    file_count: u32,
}

impl InMemoryAnalyzer {
    fn new() -> Self {
        Self {
            models: HashMap::new(),
            file_count: 0,
        }
    }

    fn analyze_content(&mut self, content: &str) {
        if let Ok(file) = parse_file(content) {
            self.visit_file(&file);
            self.file_count += 1;
        }
    }
}

impl<'ast> Visit<'ast> for InMemoryAnalyzer {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if let syn::Expr::Lit(lit) = expr {
            let type_name = match &lit.lit {
                syn::Lit::Str(_) => "String",
                syn::Lit::Int(_) => "integer", 
                syn::Lit::Bool(_) => "bool",
                _ => "other",
            };
            
            let model = self.models.entry(type_name.to_string()).or_insert_with(|| {
                TypeModel { instances: 0, patterns: HashMap::new() }
            });
            model.instances += 1;
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let archives = [
        "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz",
        "/nix/store/xp98ag7yvxjk13a3yan8qilb97wsavgy-rust-src-nightly.tar.xz"
    ];

    let mut analyzer = InMemoryAnalyzer::new();

    for archive_path in &archives {
        println!("Processing {}", archive_path);
        
        let file = File::open(archive_path)?;
        let reader = BufReader::new(file);
        let xz_decoder = xz2::read::XzDecoder::new(reader);
        let mut tar = tar::Archive::new(xz_decoder);

        for entry in tar.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            
            if path.extension().map_or(false, |ext| ext == "rs") {
                let mut content = String::new();
                entry.read_to_string(&mut content)?;
                analyzer.analyze_content(&content);
            }
        }
    }

    let result = RustSourceAnalysis {
        total_files: analyzer.file_count,
        models: analyzer.models,
    };

    println!("Analyzed {} files", result.total_files);
    for (type_name, model) in &result.models {
        println!("{}: {} instances", type_name, model.instances);
    }

    std::fs::write("rust_source_analysis.json", serde_json::to_string_pretty(&result)?)?;
    Ok(())
}
