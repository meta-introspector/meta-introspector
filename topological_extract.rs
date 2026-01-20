use std::collections::{HashMap, HashSet, VecDeque};
use parquet::arrow::ParquetRecordBatchReaderBuilder;
use std::fs::File;

fn main() {
    println!("🔀 Topological ordering of 3M files");
    
    // Load 3M file index
    let files = load_file_index("indexes/files.parquet");
    println!("📊 Loaded {} files", files.len());
    
    // Build dependency graph
    let graph = build_dependency_graph(&files);
    println!("🔗 Built dependency graph");
    
    // Topological sort
    let ordered = topological_sort(&graph);
    println!("📋 Topologically ordered {} files", ordered.len());
    
    // Split by AST harmonics
    let harmonic_groups = split_by_harmonics(&ordered);
    println!("🎵 Split into {} harmonic groups", harmonic_groups.len());
    
    // Extract complexity 1 declarations
    let layer1_decls = extract_simple_decls(&harmonic_groups);
    println!("📝 Extracted {} simple declarations", layer1_decls.len());
    
    // Write to Layer 1
    write_layer1(&layer1_decls, "zos/layer1/");
    println!("✅ Written to zos/layer1/");
}

fn load_file_index(path: &str) -> Vec<FileInfo> {
    let mut files = Vec::new();
    
    let file = File::open(path).expect("Failed to open parquet");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let reader = builder.build().unwrap();
    
    for batch in reader.flatten() {
        // Extract file_path, git_repo, dependencies
        // files.push(FileInfo { ... });
    }
    
    files
}

fn build_dependency_graph(files: &[FileInfo]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    
    for file in files {
        let deps = extract_dependencies(&file.path);
        graph.insert(file.path.clone(), deps);
    }
    
    graph
}

fn extract_dependencies(file_path: &str) -> Vec<String> {
    let mut deps = Vec::new();
    
    if let Ok(content) = std::fs::read_to_string(file_path) {
        // Rust: use statements
        for line in content.lines() {
            if line.trim().starts_with("use ") {
                if let Some(dep) = line.split("::").nth(1) {
                    deps.push(dep.trim().to_string());
                }
            }
        }
    }
    
    deps
}

fn topological_sort(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build in-degree map
    for (node, deps) in graph {
        in_degree.entry(node.clone()).or_insert(0);
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
            adj_list.entry(node.clone()).or_insert(Vec::new()).push(dep.clone());
        }
    }
    
    // Kahn's algorithm
    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(node, _)| node.clone())
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        
        if let Some(neighbors) = adj_list.get(&node) {
            for neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }
    
    result
}

fn split_by_harmonics(files: &[String]) -> HashMap<u32, Vec<String>> {
    let mut groups = HashMap::new();
    
    for file in files {
        let harmonic = calculate_harmonic(file);
        groups.entry(harmonic).or_insert(Vec::new()).push(file.clone());
    }
    
    groups
}

fn calculate_harmonic(file_path: &str) -> u32 {
    // Parse AST and calculate harmonic frequency
    if let Ok(content) = std::fs::read_to_string(file_path) {
        let ast_depth = content.matches('{').count();
        let ast_width = content.lines().count();
        
        // Harmonic = depth * width mod 256
        ((ast_depth * ast_width) % 256) as u32
    } else {
        0
    }
}

fn extract_simple_decls(groups: &HashMap<u32, Vec<String>>) -> Vec<Declaration> {
    let mut decls = Vec::new();
    
    for files in groups.values() {
        for file in files {
            if let Ok(content) = std::fs::read_to_string(file) {
                for line in content.lines() {
                    // Extract declarations of complexity 1
                    if is_simple_decl(line) {
                        decls.push(Declaration {
                            name: extract_name(line),
                            content: line.to_string(),
                            complexity: 1,
                            length: line.len(),
                        });
                    }
                }
            }
        }
    }
    
    decls
}

fn is_simple_decl(line: &str) -> bool {
    let trimmed = line.trim();
    
    // Complexity 1: single line, no nesting
    (trimmed.starts_with("fn ") || 
     trimmed.starts_with("struct ") ||
     trimmed.starts_with("const ") ||
     trimmed.starts_with("type ")) &&
    !trimmed.contains('{') &&
    trimmed.ends_with(';')
}

fn extract_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .trim_end_matches(|c| c == ':' || c == ';')
        .to_string()
}

fn write_layer1(decls: &[Declaration], output_dir: &str) {
    std::fs::create_dir_all(output_dir).ok();
    
    let mut content = String::from("// ZOS Layer 1 - Simple Declarations\n\n");
    
    for decl in decls {
        content.push_str(&format!("{}\n", decl.content));
    }
    
    std::fs::write(format!("{}/layer1.rs", output_dir), content).unwrap();
    println!("📝 Written {} declarations to layer1.rs", decls.len());
}

#[derive(Debug)]
struct FileInfo {
    path: String,
    git_repo: String,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    content: String,
    complexity: u32,
    length: usize,
}
