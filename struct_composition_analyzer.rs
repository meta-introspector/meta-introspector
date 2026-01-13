use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Item, ItemStruct, ItemEnum, Fields, Type, PathSegment};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StructComposition {
    field_count: usize,
    field_types: Vec<String>,
    struct_name: String,
    simple_type_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompositionAnalysis {
    field_count: usize,
    struct_count: u32,
    compositions: Vec<StructComposition>,
    type_frequency: HashMap<String, u32>,
}

struct StructAnalyzer {
    simple_types: std::collections::HashSet<String>,
    simple_enums: std::collections::HashSet<String>,
    compositions: HashMap<usize, CompositionAnalysis>,
    total_structs: u32,
}

impl StructAnalyzer {
    fn new() -> Self {
        let mut simple_types = std::collections::HashSet::new();
        
        // Add primitive types
        simple_types.insert("bool".to_string());
        simple_types.insert("u8".to_string());
        simple_types.insert("u16".to_string());
        simple_types.insert("u32".to_string());
        simple_types.insert("u64".to_string());
        simple_types.insert("u128".to_string());
        simple_types.insert("usize".to_string());
        simple_types.insert("i8".to_string());
        simple_types.insert("i16".to_string());
        simple_types.insert("i32".to_string());
        simple_types.insert("i64".to_string());
        simple_types.insert("i128".to_string());
        simple_types.insert("isize".to_string());
        simple_types.insert("f32".to_string());
        simple_types.insert("f64".to_string());
        simple_types.insert("char".to_string());
        
        Self {
            simple_types,
            simple_enums: std::collections::HashSet::new(),
            compositions: HashMap::new(),
            total_structs: 0,
        }
    }

    fn extract_type_name(&self, ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    segment.ident.to_string()
                } else {
                    "unknown".to_string()
                }
            }
            Type::Reference(type_ref) => {
                format!("&{}", self.extract_type_name(&type_ref.elem))
            }
            Type::Array(type_array) => {
                format!("[{}]", self.extract_type_name(&type_array.elem))
            }
            Type::Slice(type_slice) => {
                format!("[{}]", self.extract_type_name(&type_slice.elem))
            }
            _ => "complex".to_string(),
        }
    }

    fn is_simple_type(&self, type_name: &str) -> bool {
        // Remove reference markers and array brackets for checking
        let clean_type = type_name.trim_start_matches('&')
            .trim_start_matches('[')
            .trim_end_matches(']');
            
        self.simple_types.contains(clean_type) || self.simple_enums.contains(clean_type)
    }

    fn add_simple_enum(&mut self, enum_name: &str) {
        self.simple_enums.insert(enum_name.to_string());
    }

    fn analyze_struct(&mut self, struct_item: &ItemStruct) {
        if let Fields::Named(fields_named) = &struct_item.fields {
            let field_count = fields_named.named.len();
            let struct_name = struct_item.ident.to_string();
            
            let mut field_types = Vec::new();
            let mut simple_count = 0;
            
            for field in &fields_named.named {
                let type_name = self.extract_type_name(&field.ty);
                field_types.push(type_name.clone());
                
                if self.is_simple_type(&type_name) {
                    simple_count += 1;
                }
            }
            
            let simple_type_ratio = if field_count > 0 {
                simple_count as f64 / field_count as f64
            } else {
                0.0
            };
            
            // Only include structs with at least some simple types
            if simple_type_ratio > 0.0 {
                let composition = StructComposition {
                    field_count,
                    field_types: field_types.clone(),
                    struct_name,
                    simple_type_ratio,
                };
                
                let analysis = self.compositions.entry(field_count).or_insert_with(|| {
                    CompositionAnalysis {
                        field_count,
                        struct_count: 0,
                        compositions: Vec::new(),
                        type_frequency: HashMap::new(),
                    }
                });
                
                analysis.struct_count += 1;
                analysis.compositions.push(composition);
                
                // Count type frequency
                for type_name in field_types {
                    *analysis.type_frequency.entry(type_name).or_default() += 1;
                }
                
                self.total_structs += 1;
            }
        }
    }
}

impl<'ast> Visit<'ast> for StructAnalyzer {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Enum(enum_item) => {
                // Check if it's a simple enum (all unit variants)
                let all_unit = enum_item.variants.iter()
                    .all(|variant| matches!(variant.fields, Fields::Unit));
                
                if all_unit && enum_item.variants.len() <= 10 {
                    self.add_simple_enum(&enum_item.ident.to_string());
                }
            }
            Item::Struct(struct_item) => {
                self.analyze_struct(struct_item);
            }
            _ => {}
        }
        
        syn::visit::visit_item(self, item);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  STRUCT COMPOSITION ANALYSIS");
    println!("==============================");

    let mut analyzer = StructAnalyzer::new();
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

    println!("\n📊 STRUCT COMPOSITION BY FIELD COUNT:");
    println!("Fields | Count | Avg Simple% | Examples");
    println!("-------|-------|-------------|----------");

    let mut sorted_compositions: Vec<_> = analyzer.compositions.iter().collect();
    sorted_compositions.sort_by_key(|(field_count, _)| *field_count);

    for (field_count, analysis) in &sorted_compositions {
        let avg_simple_ratio = analysis.compositions.iter()
            .map(|c| c.simple_type_ratio)
            .sum::<f64>() / analysis.compositions.len() as f64 * 100.0;
            
        let examples: Vec<String> = analysis.compositions.iter()
            .take(3)
            .map(|c| c.struct_name.clone())
            .collect();
        let examples_str = examples.join(", ");

        println!("{:6} | {:5} | {:10.1}% | {}", 
                 field_count, 
                 analysis.struct_count, 
                 avg_simple_ratio,
                 examples_str);
    }

    // Analyze most common field types
    println!("\n🔧 MOST COMMON FIELD TYPES:");
    let mut all_type_freq: HashMap<String, u32> = HashMap::new();
    for analysis in analyzer.compositions.values() {
        for (type_name, count) in &analysis.type_frequency {
            *all_type_freq.entry(type_name.clone()).or_default() += count;
        }
    }

    let mut sorted_types: Vec<_> = all_type_freq.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    for (type_name, count) in sorted_types.iter().take(15) {
        let is_simple = analyzer.is_simple_type(type_name);
        let marker = if is_simple { "✓" } else { " " };
        println!("  {} {}: {} times", marker, type_name, count);
    }

    // Pure simple type structs
    let pure_simple_structs: Vec<_> = analyzer.compositions.values()
        .flat_map(|analysis| &analysis.compositions)
        .filter(|comp| comp.simple_type_ratio == 1.0)
        .collect();

    println!("\n🎯 PURE SIMPLE TYPE STRUCTS:");
    println!("Found {} structs with only simple types", pure_simple_structs.len());
    
    for comp in pure_simple_structs.iter().take(10) {
        println!("  {}: {} fields ({})", 
                 comp.struct_name, 
                 comp.field_count,
                 comp.field_types.join(", "));
    }

    // Save analysis
    let analysis_result = serde_json::to_string_pretty(&analyzer.compositions)?;
    fs::write("struct_composition_analysis.json", analysis_result)?;

    println!("\n📈 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Simple enums found: {}", analyzer.simple_enums.len());
    println!("Structs with simple types: {}", analyzer.total_structs);
    println!("Pure simple structs: {}", pure_simple_structs.len());
    println!("Analysis saved to struct_composition_analysis.json");

    Ok(())
}
