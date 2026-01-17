use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, ItemStruct, ItemEnum, ItemFn};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FileContentModel {
    filename: String,
    structs: Vec<String>,
    enums: Vec<String>, 
    functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilenameContentCorrelation {
    filename_patterns: HashMap<String, u32>,
    content_patterns: HashMap<String, u32>,
    correlations: Vec<(String, String, f32)>, // (filename_part, content_item, correlation)
}

struct CorrelationAnalyzer {
    file_models: Vec<FileContentModel>,
}

impl CorrelationAnalyzer {
    fn new() -> Self {
        Self { file_models: Vec::new() }
    }

    fn analyze_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(syntax_tree) = parse_file(&content) {
                    let mut visitor = ContentVisitor::new();
                    visitor.visit_file(&syntax_tree);
                    
                    self.file_models.push(FileContentModel {
                        filename: filename.to_string(),
                        structs: visitor.structs,
                        enums: visitor.enums,
                        functions: visitor.functions,
                    });
                }
            }
        }
        Ok(())
    }

    fn compute_correlations(&self) -> FilenameContentCorrelation {
        let mut filename_patterns = HashMap::new();
        let mut content_patterns = HashMap::new();
        let mut correlations = Vec::new();

        // Extract filename patterns and content patterns
        for model in &self.file_models {
            for part in model.filename.split('_') {
                *filename_patterns.entry(part.to_string()).or_default() += 1;
            }
            
            for item in &model.structs {
                *content_patterns.entry(item.clone()).or_default() += 1;
            }
            for item in &model.enums {
                *content_patterns.entry(item.clone()).or_default() += 1;
            }
        }

        // Find correlations between filename parts and content
        for filename_part in filename_patterns.keys() {
            for content_item in content_patterns.keys() {
                let correlation = self.calculate_correlation(filename_part, content_item);
                if correlation > 0.3 { // Only significant correlations
                    correlations.push((filename_part.clone(), content_item.clone(), correlation));
                }
            }
        }

        correlations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        FilenameContentCorrelation {
            filename_patterns,
            content_patterns,
            correlations,
        }
    }

    fn calculate_correlation(&self, filename_part: &str, content_item: &str) -> f32 {
        let mut matches = 0;
        let mut total = 0;

        for model in &self.file_models {
            let has_filename_part = model.filename.contains(filename_part);
            let has_content = model.structs.iter().any(|s| s == content_item) || 
                             model.enums.iter().any(|e| e == content_item);
            
            if has_filename_part && has_content {
                matches += 1;
            }
            if has_filename_part || has_content {
                total += 1;
            }
        }

        if total > 0 { matches as f32 / total as f32 } else { 0.0 }
    }
}

struct ContentVisitor {
    structs: Vec<String>,
    enums: Vec<String>,
    functions: Vec<String>,
}

impl ContentVisitor {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
            enums: Vec::new(), 
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ContentVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.structs.push(node.ident.to_string());
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.enums.push(node.ident.to_string());
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(node.sig.ident.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = std::env::args().nth(1)
        .unwrap_or_else(|| "~/nix/vendor/rust/cargo2nix/submodules/rust-build".to_string());

    println!("🔗 FILENAME-CONTENT CORRELATION ANALYSIS");
    println!("========================================");

    let mut analyzer = CorrelationAnalyzer::new();
    let mut processed = 0;

    // Process only .rs files in target directory
    for entry in fs::read_dir(&target_dir)?.take(100) { // Limit for quick analysis
        let entry = entry?;
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            analyzer.analyze_file(&entry.path())?;
            processed += 1;
        }
    }

    let correlation_model = analyzer.compute_correlations();

    println!("\n📊 CORRELATION RESULTS:");
    println!("Files analyzed: {}", processed);
    println!("Top filename-content correlations:");
    
    for (filename_part, content_item, correlation) in correlation_model.correlations.iter().take(10) {
        println!("  '{}' ↔ '{}': {:.2}", filename_part, content_item, correlation);
    }

    let json_data = serde_json::to_string_pretty(&correlation_model)?;
    fs::write("filename_content_correlation.json", json_data)?;

    println!("\n💾 Correlation model saved to filename_content_correlation.json");
    Ok(())
}
