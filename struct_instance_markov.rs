use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Expr, ExprStruct, ExprPath, Item, ItemEnum, ItemStruct, Fields, Member};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TypeInstanceMarkov {
    type_name: String,
    type_kind: String, // "simple_struct", "complex_struct", "simple_enum", "complex_enum"
    field_count: usize,
    instance_count: u32,
    field_transitions: HashMap<String, HashMap<String, u32>>,
    value_patterns: HashMap<String, u32>,
    literal_values: Vec<String>,
    variant_usage: HashMap<String, u32>, // For enums
}

struct ComplexTypeAnalyzer {
    known_structs: HashMap<String, (usize, String)>, // name -> (field_count, complexity)
    known_enums: HashMap<String, (Vec<String>, String)>, // name -> (variants, complexity)
    instance_models: HashMap<String, TypeInstanceMarkov>,
    total_instances: u32,
}

impl ComplexTypeAnalyzer {
    fn new() -> Self {
        Self {
            known_structs: HashMap::new(),
            known_enums: HashMap::new(),
            instance_models: HashMap::new(),
            total_instances: 0,
        }
    }

    fn classify_enum_complexity(&self, enum_item: &ItemEnum) -> String {
        let has_data = enum_item.variants.iter().any(|variant| {
            !matches!(variant.fields, Fields::Unit)
        });
        
        let variant_count = enum_item.variants.len();
        
        if !has_data && variant_count <= 10 {
            "simple_enum".to_string()
        } else if has_data {
            "complex_enum".to_string()
        } else {
            "large_enum".to_string()
        }
    }

    fn classify_struct_complexity(&self, struct_item: &ItemStruct) -> String {
        if let Fields::Named(fields) = &struct_item.fields {
            let field_count = fields.named.len();
            if field_count <= 5 {
                "simple_struct".to_string()
            } else {
                "complex_struct".to_string()
            }
        } else {
            "tuple_struct".to_string()
        }
    }

    fn register_enum(&mut self, enum_item: &ItemEnum) {
        let name = enum_item.ident.to_string();
        let complexity = self.classify_enum_complexity(enum_item);
        let variants: Vec<String> = enum_item.variants.iter()
            .map(|v| v.ident.to_string())
            .collect();
        
        self.known_enums.insert(name, (variants, complexity));
    }

    fn register_struct(&mut self, struct_item: &ItemStruct) {
        let name = struct_item.ident.to_string();
        let complexity = self.classify_struct_complexity(struct_item);
        let field_count = if let Fields::Named(fields) = &struct_item.fields {
            fields.named.len()
        } else {
            0
        };
        
        self.known_structs.insert(name, (field_count, complexity));
    }

    fn analyze_struct_instance(&mut self, expr_struct: &ExprStruct) {
        let type_name = if let Some(segment) = expr_struct.path.segments.last() {
            segment.ident.to_string()
        } else {
            return;
        };

        if let Some((field_count, complexity)) = self.known_structs.get(&type_name).cloned() {
            let model = self.instance_models.entry(type_name.clone()).or_insert_with(|| {
                TypeInstanceMarkov {
                    type_name: type_name.clone(),
                    type_kind: complexity,
                    field_count,
                    instance_count: 0,
                    field_transitions: HashMap::new(),
                    value_patterns: HashMap::new(),
                    literal_values: Vec::new(),
                    variant_usage: HashMap::new(),
                }
            });

            model.instance_count += 1;
            self.total_instances += 1;

            // Analyze field patterns (same as before)
            let field_names: Vec<String> = expr_struct.fields.iter()
                .filter_map(|field| {
                    if let Member::Named(ident) = &field.member {
                        Some(ident.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            // Build field transitions
            for i in 0..field_names.len().saturating_sub(1) {
                *model.field_transitions
                    .entry(field_names[i].clone())
                    .or_default()
                    .entry(field_names[i + 1].clone())
                    .or_default() += 1;
            }

            // Analyze values
            for field in &expr_struct.fields {
                if let Member::Named(field_name) = &field.member {
                    let pattern = self.classify_value(&field.expr);
                    *model.value_patterns.entry(field_name.to_string()).or_default() += 1;
                    
                    if let Some(literal) = self.extract_literal(&field.expr) {
                        model.literal_values.push(literal);
                    }
                }
            }
        }
    }

    fn analyze_enum_usage(&mut self, expr_path: &ExprPath) {
        if let Some(path_segment) = expr_path.path.segments.last() {
            let variant_name = path_segment.ident.to_string();
            
            // Check if this is a known enum variant
            for (enum_name, (variants, complexity)) in &self.known_enums {
                if variants.contains(&variant_name) {
                    let model = self.instance_models.entry(enum_name.clone()).or_insert_with(|| {
                        TypeInstanceMarkov {
                            type_name: enum_name.clone(),
                            type_kind: complexity.clone(),
                            field_count: variants.len(),
                            instance_count: 0,
                            field_transitions: HashMap::new(),
                            value_patterns: HashMap::new(),
                            literal_values: Vec::new(),
                            variant_usage: HashMap::new(),
                        }
                    });

                    model.instance_count += 1;
                    *model.variant_usage.entry(variant_name).or_default() += 1;
                    self.total_instances += 1;
                    break;
                }
            }
        }
    }

    fn classify_value(&self, expr: &Expr) -> String {
        match expr {
            Expr::Lit(expr_lit) => {
                match &expr_lit.lit {
                    syn::Lit::Bool(_) => "bool_literal".to_string(),
                    syn::Lit::Int(_) => "int_literal".to_string(),
                    syn::Lit::Float(_) => "float_literal".to_string(),
                    syn::Lit::Str(_) => "string_literal".to_string(),
                    _ => "other_literal".to_string(),
                }
            }
            Expr::Path(_) => "identifier".to_string(),
            Expr::Call(_) => "function_call".to_string(),
            Expr::Struct(_) => "nested_struct".to_string(),
            _ => "complex_expr".to_string(),
        }
    }

    fn extract_literal(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Lit(expr_lit) => {
                match &expr_lit.lit {
                    syn::Lit::Bool(b) => Some(b.value.to_string()),
                    syn::Lit::Int(i) => Some(i.base10_digits().to_string()),
                    syn::Lit::Float(f) => Some(f.base10_digits().to_string()),
                    syn::Lit::Str(s) => Some(format!("\"{}\"", s.value())),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl<'ast> Visit<'ast> for ComplexTypeAnalyzer {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Struct(struct_item) => {
                self.register_struct(struct_item);
            }
            Item::Enum(enum_item) => {
                self.register_enum(enum_item);
            }
            _ => {}
        }
        
        syn::visit::visit_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Struct(expr_struct) => {
                self.analyze_struct_instance(expr_struct);
            }
            Expr::Path(expr_path) => {
                self.analyze_enum_usage(expr_path);
            }
            _ => {}
        }
        
        syn::visit::visit_expr(self, expr);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏭 COMPLEX TYPE INSTANCE MARKOV ANALYSIS");
    println!("=======================================");

    let mut analyzer = ComplexTypeAnalyzer::new();
    let mut processed_files = 0;

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

    println!("\n📊 TYPE INSTANCE ANALYSIS:");
    println!("Type                | Kind          | Instances | Variants/Fields | Top Pattern");
    println!("--------------------|---------------|-----------|-----------------|-------------");

    let mut sorted_models: Vec<_> = analyzer.instance_models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.instance_count.cmp(&a.1.instance_count));

    for (type_name, model) in sorted_models.iter().take(20) {
        let top_pattern = if !model.variant_usage.is_empty() {
            // For enums, show most used variant
            model.variant_usage.iter()
                .max_by_key(|(_, count)| *count)
                .map(|(variant, count)| format!("{}({})", variant, count))
                .unwrap_or_else(|| "none".to_string())
        } else {
            // For structs, show most common field transition
            model.field_transitions.iter()
                .flat_map(|(from, transitions)| {
                    transitions.iter().map(move |(to, count)| (format!("{}→{}", from, to), *count))
                })
                .max_by_key(|(_, count)| *count)
                .map(|(trans, _)| trans)
                .unwrap_or_else(|| "none".to_string())
        };

        println!("{:19} | {:13} | {:9} | {:15} | {}", 
                 if type_name.len() > 19 { &type_name[..16] } else { type_name },
                 model.type_kind,
                 model.instance_count, 
                 model.field_count,
                 if top_pattern.len() > 12 { format!("{}...", &top_pattern[..9]) } else { top_pattern });
    }

    // Analyze by type complexity
    println!("\n🎯 ANALYSIS BY TYPE COMPLEXITY:");
    let mut complexity_stats: HashMap<String, (u32, u32)> = HashMap::new(); // (types, instances)
    
    for model in analyzer.instance_models.values() {
        let (type_count, instance_count) = complexity_stats.entry(model.type_kind.clone()).or_default();
        *type_count += 1;
        *instance_count += model.instance_count;
    }

    for (complexity, (type_count, instance_count)) in &complexity_stats {
        println!("  {}: {} types, {} instances", complexity, type_count, instance_count);
    }

    // Show enum variant distributions
    println!("\n🔀 ENUM VARIANT USAGE:");
    for (type_name, model) in sorted_models.iter().take(5) {
        if !model.variant_usage.is_empty() {
            println!("{}:", type_name);
            let mut sorted_variants: Vec<_> = model.variant_usage.iter().collect();
            sorted_variants.sort_by(|a, b| b.1.cmp(a.1));
            
            for (variant, count) in sorted_variants.iter().take(5) {
                let percentage = **count as f64 / model.instance_count as f64 * 100.0;
                println!("  {}: {} times ({:.1}%)", variant, count, percentage);
            }
        }
    }

    let analysis_result = serde_json::to_string_pretty(&analyzer.instance_models)?;
    fs::write("complex_type_instance_markov.json", analysis_result)?;

    println!("\n📈 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Struct types: {}", analyzer.known_structs.len());
    println!("Enum types: {}", analyzer.known_enums.len());
    println!("Total instances: {}", analyzer.total_instances);
    println!("Analysis saved to complex_type_instance_markov.json");

    Ok(())
}
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

    println!("\n📊 STRUCT INSTANCE ANALYSIS:");
    println!("Type                | Instances | Fields | Top Transition | Literals");
    println!("--------------------|-----------|--------|----------------|----------");

    let mut sorted_models: Vec<_> = analyzer.instance_models.iter().collect();
    sorted_models.sort_by(|a, b| b.1.instance_count.cmp(&a.1.instance_count));

    for (type_name, model) in sorted_models.iter().take(20) {
        // Find most common field transition
        let top_transition = model.field_transitions.iter()
            .flat_map(|(from, transitions)| {
                transitions.iter().map(move |(to, count)| (format!("{}→{}", from, to), *count))
            })
            .max_by_key(|(_, count)| *count)
            .map(|(trans, count)| format!("{} ({})", trans, count))
            .unwrap_or_else(|| "none".to_string());

        let literal_count = model.literal_values.len();

        println!("{:19} | {:9} | {:6} | {:14} | {:8}", 
                 type_name, 
                 model.instance_count, 
                 model.field_count,
                 if top_transition.len() > 14 { 
                     format!("{}...", &top_transition[..11]) 
                 } else { 
                     top_transition 
                 },
                 literal_count);
    }

    // Analyze value patterns
    println!("\n🎯 VALUE PATTERN ANALYSIS:");
    let mut all_patterns: HashMap<String, u32> = HashMap::new();
    for model in analyzer.instance_models.values() {
        for (pattern, count) in &model.value_patterns {
            *all_patterns.entry(pattern.clone()).or_default() += count;
        }
    }

    let mut sorted_patterns: Vec<_> = all_patterns.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));

    for (pattern, count) in sorted_patterns.iter().take(10) {
        println!("  {}: {} occurrences", pattern, count);
    }

    // Show most active struct instances
    if let Some((most_active_type, most_active_model)) = sorted_models.first() {
        println!("\n🔥 MOST ACTIVE STRUCT: {}", most_active_type);
        println!("Instances: {}", most_active_model.instance_count);
        println!("Field transitions:");
        
        for (from_field, transitions) in most_active_model.field_transitions.iter().take(5) {
            for (to_field, count) in transitions.iter().take(3) {
                println!("  {} → {}: {} times", from_field, to_field, count);
            }
        }
        
        println!("Sample literals: {:?}", 
                 most_active_model.literal_values.iter().take(5).collect::<Vec<_>>());
    }

    // Save analysis
    let analysis_result = serde_json::to_string_pretty(&analyzer.instance_models)?;
    fs::write("struct_instance_markov.json", analysis_result)?;

    println!("\n📈 SUMMARY:");
    println!("Files processed: {}", processed_files);
    println!("Struct types registered: {}", analyzer.known_structs.len());
    println!("Total struct instances: {}", analyzer.total_instances);
    println!("Types with instances: {}", analyzer.instance_models.len());
    println!("Analysis saved to struct_instance_markov.json");

    Ok(())
}
