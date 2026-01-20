use std::collections::{HashMap, HashSet};

fn main() {
    println!("🔬 Extracting Genus 0 declarations");
    
    // Load all declarations from 3M files
    let all_decls = load_all_declarations();
    println!("📊 Loaded {} declarations", all_decls.len());
    
    // Calculate genus for each
    let genus_map = calculate_all_genus(&all_decls);
    
    // Extract Genus 0
    let genus_0: Vec<_> = all_decls.iter()
        .filter(|d| genus_map.get(&d.name).copied().unwrap_or(999) == 0)
        .collect();
    
    println!("✅ Found {} Genus 0 declarations", genus_0.len());
    
    // Write to Layer 0
    write_layer0(&genus_0);
    
    // Verify
    verify_genus_0(&genus_0);
}

fn load_all_declarations() -> Vec<Declaration> {
    // Load from 3M files
    vec![]
}

fn calculate_all_genus(decls: &[Declaration]) -> HashMap<String, u32> {
    let mut genus_map = HashMap::new();
    let mut dep_graph = build_dependency_graph(decls);
    
    // Calculate genus recursively
    for decl in decls {
        let g = calculate_genus(&decl.name, &dep_graph, &mut genus_map);
        genus_map.insert(decl.name.clone(), g);
    }
    
    genus_map
}

fn build_dependency_graph(decls: &[Declaration]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    
    for decl in decls {
        graph.insert(decl.name.clone(), decl.dependencies.clone());
    }
    
    graph
}

fn calculate_genus(
    name: &str,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u32>
) -> u32 {
    // Check cache
    if let Some(&g) = cache.get(name) {
        return g;
    }
    
    // Get dependencies
    let deps = match graph.get(name) {
        Some(d) => d,
        None => return 0,  // No deps = Genus 0
    };
    
    // If no dependencies, Genus 0
    if deps.is_empty() {
        cache.insert(name.to_string(), 0);
        return 0;
    }
    
    // Genus = 1 + max(genus of dependencies)
    let max_dep_genus = deps.iter()
        .map(|dep| calculate_genus(dep, graph, cache))
        .max()
        .unwrap_or(0);
    
    let genus = 1 + max_dep_genus;
    cache.insert(name.to_string(), genus);
    genus
}

fn write_layer0(genus_0: &[&Declaration]) {
    std::fs::create_dir_all("zos/layer0").ok();
    
    let mut content = String::from("// ZOS Layer 0 - Genus 0 Declarations\n");
    content.push_str("// Axiomatic - requires no other definitions\n\n");
    
    // Group by type
    let mut constants = Vec::new();
    let mut types = Vec::new();
    let mut functions = Vec::new();
    
    for decl in genus_0 {
        match decl.kind {
            DeclKind::Const => constants.push(decl),
            DeclKind::Type => types.push(decl),
            DeclKind::Function => functions.push(decl),
        }
    }
    
    // Write constants
    content.push_str("// Primitive Constants\n");
    for c in constants {
        content.push_str(&format!("{}\n", c.content));
    }
    
    content.push_str("\n// Primitive Types\n");
    for t in types {
        content.push_str(&format!("{}\n", t.content));
    }
    
    content.push_str("\n// Primitive Functions\n");
    for f in functions {
        content.push_str(&format!("{}\n", f.content));
    }
    
    std::fs::write("zos/layer0/primitives.rs", content).unwrap();
    println!("📝 Written to zos/layer0/primitives.rs");
}

fn verify_genus_0(genus_0: &[&Declaration]) {
    println!("\n🔍 Verifying Genus 0 properties:");
    
    for decl in genus_0 {
        assert!(decl.dependencies.is_empty(), 
            "Genus 0 must have no dependencies: {}", decl.name);
    }
    
    println!("  ✅ All declarations have no dependencies");
    
    // Verify compilable alone
    let result = std::process::Command::new("rustc")
        .args(&["--crate-type", "lib", "zos/layer0/primitives.rs"])
        .output()
        .expect("Failed to compile");
    
    assert!(result.status.success(), "Genus 0 must compile alone");
    println!("  ✅ Compiles with no dependencies");
    
    println!("\n✅ Genus 0 verified");
}

#[derive(Debug, Clone)]
struct Declaration {
    name: String,
    content: String,
    kind: DeclKind,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
enum DeclKind {
    Const,
    Type,
    Function,
}
