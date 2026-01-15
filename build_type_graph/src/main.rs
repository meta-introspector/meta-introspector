use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Field;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FunctionNode {
    name: String,
    short_name: String,
    strings_used: Vec<String>,
    type_patterns: Vec<String>,
    calls_to: HashSet<String>,
    called_by: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeGraph {
    functions: HashMap<String, FunctionNode>,
    total_functions: usize,
    total_strings: usize,
    total_edges: usize,
}

fn extract_short_name(mangled: &str) -> String {
    // Extract readable parts from mangled name
    if let Some(start) = mangled.rfind("Nt") {
        if let Some(end) = mangled[start..].find(|c: char| c.is_numeric() || c == 'E') {
            return mangled[start..start+end].to_string();
        }
    }
    mangled.chars().take(50).collect()
}

fn extract_called_functions(mangled: &str) -> Vec<String> {
    // Extract function references from mangled name
    let mut calls = Vec::new();
    let parts: Vec<&str> = mangled.split("Nv").collect();
    for part in parts.iter().skip(1) {
        if let Some(end) = part.find(|c: char| !c.is_alphanumeric() && c != '_') {
            calls.push(part[..end].to_string());
        }
    }
    calls
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../string_usage.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let patterns = ["Expr", "Stmt", "Item", "Pat", "Ty", "Block", "Hir", "Mir", "Thir", 
                    "ast::", "hir::", "mir::", "thir::", "Body", "Def", "Res", "Node",
                    "Generic", "Trait", "Impl"];
    
    let mut functions: HashMap<String, FunctionNode> = HashMap::new();
    let mut all_function_names: HashSet<String> = HashSet::new();
    
    println!("Phase 1: Collecting functions with type patterns...\n");
    
    for i in 0..reader.num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        let rows = row_group.get_row_iter(None)?;
        
        for row_result in rows {
            let row = row_result?;
            let mut string_val = String::new();
            let mut func_name = String::new();
            
            for (name, field) in row.get_column_iter() {
                if let Field::Str(s) = field {
                    if name == "string_value" {
                        string_val = s.to_string();
                    } else if name == "function_name" {
                        func_name = s.to_string();
                    }
                }
            }
            
            all_function_names.insert(func_name.clone());
            
            let mut matched_patterns = Vec::new();
            for pattern in &patterns {
                if func_name.contains(pattern) {
                    matched_patterns.push(pattern.to_string());
                }
            }
            
            if !matched_patterns.is_empty() {
                let entry = functions.entry(func_name.clone()).or_insert_with(|| {
                    FunctionNode {
                        name: func_name.clone(),
                        short_name: extract_short_name(&func_name),
                        strings_used: Vec::new(),
                        type_patterns: matched_patterns.clone(),
                        calls_to: HashSet::new(),
                        called_by: HashSet::new(),
                    }
                });
                
                if !string_val.is_empty() && !entry.strings_used.contains(&string_val) {
                    entry.strings_used.push(string_val);
                }
            }
        }
    }
    
    println!("Phase 2: Building call graph...\n");
    
    // Build call relationships
    let func_names: Vec<String> = functions.keys().cloned().collect();
    for func_name in &func_names {
        let called = extract_called_functions(func_name);
        for call in called {
            // Check if any function contains this call pattern
            for other_func in &func_names {
                if other_func != func_name && other_func.contains(&call) {
                    if let Some(node) = functions.get_mut(func_name) {
                        node.calls_to.insert(other_func.clone());
                    }
                }
            }
        }
    }
    
    // Build reverse edges (called_by)
    let edges: Vec<(String, String)> = functions.iter()
        .flat_map(|(name, node)| {
            node.calls_to.iter().map(|target| (name.clone(), target.clone()))
        })
        .collect();
    
    for (caller, callee) in edges {
        if let Some(node) = functions.get_mut(&callee) {
            node.called_by.insert(caller);
        }
    }
    
    let total_strings: usize = functions.values().map(|f| f.strings_used.len()).sum();
    let total_edges: usize = functions.values().map(|f| f.calls_to.len()).sum();
    
    let graph = TypeGraph {
        total_functions: functions.len(),
        total_strings,
        total_edges,
        functions,
    };
    
    println!("Graph Statistics:");
    println!("  Functions with type patterns: {}", graph.total_functions);
    println!("  Total strings used: {}", graph.total_strings);
    println!("  Total call edges: {}", graph.total_edges);
    
    let json = serde_json::to_string_pretty(&graph)?;
    std::fs::write("../type_function_graph.json", json)?;
    println!("\nSaved full graph to: type_function_graph.json");
    
    println!("\n=== Top Functions by Connectivity ===\n");
    let mut func_list: Vec<_> = graph.functions.values().collect();
    func_list.sort_by_key(|f| std::cmp::Reverse(f.calls_to.len() + f.called_by.len()));
    
    for (i, func) in func_list.iter().take(10).enumerate() {
        println!("{}. Calls: {}, Called by: {}, Strings: {}", 
                 i+1, func.calls_to.len(), func.called_by.len(), func.strings_used.len());
        println!("   Patterns: {:?}", func.type_patterns);
        println!("   Short: {}", func.short_name);
        if !func.strings_used.is_empty() {
            println!("   Strings: {:?}", &func.strings_used[..func.strings_used.len().min(2)]);
        }
        println!();
    }
    
    Ok(())
}

