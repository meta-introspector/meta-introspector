use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DirectoryMarkov {
    name: String,
    depth: usize,
    file_count: u32,
    rust_files: u32,
    subdirs: Vec<String>,
    transitions: HashMap<String, u32>, // dir_name -> count
}

#[derive(Debug, Serialize, Deserialize)]
struct RustcTreeModel {
    total_dirs: u32,
    total_files: u32,
    max_depth: usize,
    directory_models: HashMap<String, DirectoryMarkov>,
    depth_distribution: HashMap<usize, u32>,
    name_patterns: HashMap<String, u32>,
}

struct RustcTreeAnalyzer {
    model: RustcTreeModel,
}

impl RustcTreeAnalyzer {
    fn new() -> Self {
        Self {
            model: RustcTreeModel {
                total_dirs: 0,
                total_files: 0,
                max_depth: 0,
                directory_models: HashMap::new(),
                depth_distribution: HashMap::new(),
                name_patterns: HashMap::new(),
            }
        }
    }

    fn analyze_tree(&mut self, root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut dirs_to_process = vec![(root.to_path_buf(), 0)];
        
        while let Some((current_dir, depth)) = dirs_to_process.pop() {
            if depth > 20 { continue; } // Prevent excessive depth
            
            self.model.max_depth = self.model.max_depth.max(depth);
            *self.model.depth_distribution.entry(depth).or_default() += 1;
            
            let dir_name = current_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("root")
                .to_string();
            
            // Track name patterns
            *self.model.name_patterns.entry(dir_name.clone()).or_default() += 1;
            
            if let Ok(entries) = fs::read_dir(&current_dir) {
                let mut subdirs = Vec::new();
                let mut file_count = 0;
                let mut rust_files = 0;
                
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        if let Some(subdir_name) = path.file_name().and_then(|n| n.to_str()) {
                            subdirs.push(subdir_name.to_string());
                            dirs_to_process.push((path, depth + 1));
                        }
                    } else {
                        file_count += 1;
                        if path.extension().map_or(false, |ext| ext == "rs") {
                            rust_files += 1;
                        }
                    }
                }
                
                // Build transitions between subdirectories
                let mut transitions = HashMap::new();
                for i in 0..subdirs.len().saturating_sub(1) {
                    *transitions.entry(subdirs[i + 1].clone()).or_default() += 1;
                }
                
                let dir_model = DirectoryMarkov {
                    name: dir_name.clone(),
                    depth,
                    file_count,
                    rust_files,
                    subdirs: subdirs.clone(),
                    transitions,
                };
                
                self.model.directory_models.insert(dir_name, dir_model);
                self.model.total_dirs += 1;
                self.model.total_files += file_count;
                
                if self.model.total_dirs % 100 == 0 {
                    println!("Analyzed {} directories, depth {}", self.model.total_dirs, depth);
                }
            }
        }
        
        Ok(())
    }

    fn generate_report(&self) {
        println!("\n🌳 RUSTC TREE MARKOV MODEL");
        println!("=========================");
        println!("Total directories: {}", self.model.total_dirs);
        println!("Total files: {}", self.model.total_files);
        println!("Max depth: {}", self.model.max_depth);
        
        // Depth distribution
        println!("\n📊 DEPTH DISTRIBUTION:");
        let mut depths: Vec<_> = self.model.depth_distribution.iter().collect();
        depths.sort_by_key(|&(depth, _)| depth);
        for (depth, count) in depths.iter().take(10) {
            println!("  Depth {}: {} directories", depth, count);
        }
        
        // Most common directory names
        println!("\n🏷️  COMMON DIRECTORY NAMES:");
        let mut names: Vec<_> = self.model.name_patterns.iter().collect();
        names.sort_by(|a, b| b.1.cmp(a.1));
        for (name, count) in names.iter().take(15) {
            println!("  {}: {} occurrences", name, count);
        }
        
        // Directories with most subdirs
        println!("\n🌿 MOST COMPLEX DIRECTORIES:");
        let mut complex_dirs: Vec<_> = self.model.directory_models.iter().collect();
        complex_dirs.sort_by(|a, b| b.1.subdirs.len().cmp(&a.1.subdirs.len()));
        for (name, model) in complex_dirs.iter().take(10) {
            println!("  {}: {} subdirs, {} rust files", 
                     name, model.subdirs.len(), model.rust_files);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let target_dir = if args.len() > 1 {
        &args[1]
    } else {
        "~/nix/vendor/rust/cargo2nix/submodules/rust-build"
    };

    println!("🦀 RUSTC TREE MARKOV ANALYSIS");
    println!("=============================");
    println!("Target: {}", target_dir);

    let mut analyzer = RustcTreeAnalyzer::new();
    let path = Path::new(target_dir);
    
    analyzer.analyze_tree(path)?;
    analyzer.generate_report();

    // Save model
    let json_data = serde_json::to_string_pretty(&analyzer.model)?;
    fs::write("rustc_tree_markov.json", json_data)?;
    
    println!("\n💾 Tree model saved to rustc_tree_markov.json");
    Ok(())
}
