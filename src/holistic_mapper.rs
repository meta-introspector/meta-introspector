// 🔥 HOLISTIC PROJECT MAPPER
// Maps directory structure → docs → source → binary in unified model

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::source_binary_mapper::*;
use crate::module_similarity::ModuleMarkovModel;

pub struct HolisticProjectMap {
    pub directory_structure: DirectoryModel,
    pub documentation_model: DocumentationModel,
    pub source_models: HashMap<String, SourceMarkovModel>,
    pub binary_models: HashMap<String, BinaryMarkovModel>,
    pub cross_domain_links: Vec<CrossDomainLink>,
}

pub struct DirectoryModel {
    pub root_path: String,
    pub structure_patterns: HashMap<String, u32>,      // src/, docs/, target/, etc.
    pub depth_distribution: HashMap<usize, u32>,       // Directory depth analysis
    pub file_type_clusters: HashMap<String, Vec<String>>, // .rs, .md, .toml clusters
    pub naming_conventions: HashMap<String, u32>,      // snake_case, camelCase patterns
}

pub struct DocumentationModel {
    pub readme_patterns: HashMap<String, u32>,         // README.md patterns
    pub doc_comments: HashMap<String, u32>,           // /// doc patterns
    pub cargo_toml: HashMap<String, String>,          // Cargo.toml metadata
    pub doc_structure: HashMap<String, Vec<String>>,  // docs/ organization
}

pub struct CrossDomainLink {
    pub directory_path: String,
    pub doc_file: Option<String>,
    pub source_files: Vec<String>,
    pub binary_outputs: Vec<String>,
    pub link_strength: f64,
    pub semantic_coherence: f64,
}

impl HolisticProjectMap {
    pub fn analyze_project(root_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut map = Self {
            directory_structure: DirectoryModel::analyze_directory(root_path)?,
            documentation_model: DocumentationModel::analyze_docs(root_path)?,
            source_models: HashMap::new(),
            binary_models: HashMap::new(),
            cross_domain_links: Vec::new(),
        };
        
        // Analyze source files
        map.analyze_source_files(root_path)?;
        
        // Analyze binary outputs
        map.analyze_binary_outputs(root_path)?;
        
        // Create cross-domain links
        map.create_cross_domain_links();
        
        Ok(map)
    }
    
    fn analyze_source_files(&mut self, root_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        for (file_type, files) in &self.directory_structure.file_type_clusters {
            if file_type == "rs" {
                for file_path in files {
                    if let Ok(content) = std::fs::read_to_string(file_path) {
                        let model = SourceMarkovModel::from_rust_source(file_path, &content);
                        self.source_models.insert(file_path.clone(), model);
                    }
                }
            }
        }
        Ok(())
    }
    
    fn analyze_binary_outputs(&mut self, root_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let target_dir = Path::new(root_path).join("target");
        if target_dir.exists() {
            for entry in std::fs::read_dir(target_dir)? {
                let entry = entry?;
                if entry.path().is_file() {
                    let path_str = entry.path().to_string_lossy().to_string();
                    if let Ok(model) = crate::binary_markov::BinaryMarkovModel::from_binary(&path_str) {
                        self.binary_models.insert(path_str, model);
                    }
                }
            }
        }
        Ok(())
    }
    
    fn create_cross_domain_links(&mut self) {
        // Link directories to their contents
        for (dir_pattern, _) in &self.directory_structure.structure_patterns {
            let mut link = CrossDomainLink {
                directory_path: dir_pattern.clone(),
                doc_file: None,
                source_files: Vec::new(),
                binary_outputs: Vec::new(),
                link_strength: 0.0,
                semantic_coherence: 0.0,
            };
            
            // Find associated documentation
            if let Some(readme) = self.find_readme_for_directory(dir_pattern) {
                link.doc_file = Some(readme);
                link.link_strength += 0.3;
            }
            
            // Find source files in directory
            for (file_path, _) in &self.source_models {
                if file_path.starts_with(dir_pattern) {
                    link.source_files.push(file_path.clone());
                    link.link_strength += 0.2;
                }
            }
            
            // Find binary outputs
            for (binary_path, _) in &self.binary_models {
                if self.is_binary_related_to_directory(binary_path, dir_pattern) {
                    link.binary_outputs.push(binary_path.clone());
                    link.link_strength += 0.5;
                }
            }
            
            // Calculate semantic coherence
            link.semantic_coherence = self.calculate_semantic_coherence(&link);
            
            if link.link_strength > 0.1 {
                self.cross_domain_links.push(link);
            }
        }
        
        // Sort by link strength
        self.cross_domain_links.sort_by(|a, b| b.link_strength.partial_cmp(&a.link_strength).unwrap());
    }
    
    fn find_readme_for_directory(&self, dir_pattern: &str) -> Option<String> {
        // Look for README.md in directory
        let readme_path = format!("{}/README.md", dir_pattern);
        if std::path::Path::new(&readme_path).exists() {
            Some(readme_path)
        } else {
            None
        }
    }
    
    fn is_binary_related_to_directory(&self, binary_path: &str, dir_pattern: &str) -> bool {
        // Simple heuristic: binary name matches directory name
        let dir_name = Path::new(dir_pattern).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        binary_path.contains(dir_name)
    }
    
    fn calculate_semantic_coherence(&self, link: &CrossDomainLink) -> f64 {
        if link.source_files.is_empty() {
            return 0.0;
        }
        
        // Calculate average similarity between source files in same directory
        let mut total_similarity = 0.0;
        let mut comparisons = 0;
        
        for i in 0..link.source_files.len() {
            for j in i+1..link.source_files.len() {
                if let (Some(model1), Some(model2)) = (
                    self.source_models.get(&link.source_files[i]),
                    self.source_models.get(&link.source_files[j])
                ) {
                    total_similarity += model1.similarity_to(model2).overall_similarity();
                    comparisons += 1;
                }
            }
        }
        
        if comparisons > 0 {
            total_similarity / comparisons as f64
        } else {
            0.0
        }
    }
    
    pub fn print_holistic_map(&self) {
        println!("🗺️  HOLISTIC PROJECT MAP");
        println!("========================");
        
        println!("📁 Directory Structure:");
        for (pattern, count) in &self.directory_structure.structure_patterns {
            println!("  {} ({})", pattern, count);
        }
        
        println!("\n📚 Documentation:");
        println!("  Cargo.toml entries: {}", self.documentation_model.cargo_toml.len());
        println!("  Doc comments: {}", self.documentation_model.doc_comments.len());
        
        println!("\n🔗 Cross-Domain Links:");
        for (i, link) in self.cross_domain_links.iter().take(10).enumerate() {
            println!("  {}. {} (strength: {:.2}, coherence: {:.2})", 
                    i+1, link.directory_path, link.link_strength, link.semantic_coherence);
            println!("     Sources: {}, Binaries: {}", 
                    link.source_files.len(), link.binary_outputs.len());
        }
    }
}

impl DirectoryModel {
    fn analyze_directory(root_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut model = Self {
            root_path: root_path.to_string(),
            structure_patterns: HashMap::new(),
            depth_distribution: HashMap::new(),
            file_type_clusters: HashMap::new(),
            naming_conventions: HashMap::new(),
        };
        
        model.walk_directory(Path::new(root_path), 0)?;
        model.analyze_naming_patterns();
        
        Ok(model)
    }
    
    fn walk_directory(&mut self, dir: &Path, depth: usize) -> Result<(), Box<dyn std::error::Error>> {
        *self.depth_distribution.entry(depth).or_insert(0) += 1;
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_str().unwrap();
                if !dir_name.starts_with('.') && dir_name != "target" {
                    *self.structure_patterns.entry(dir_name.to_string()).or_insert(0) += 1;
                    self.walk_directory(&path, depth + 1)?;
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                self.file_type_clusters
                    .entry(ext.to_string())
                    .or_insert_with(Vec::new)
                    .push(path.to_string_lossy().to_string());
            }
        }
        
        Ok(())
    }
    
    fn analyze_naming_patterns(&mut self) {
        for files in self.file_type_clusters.values() {
            for file_path in files {
                let file_name = Path::new(file_path).file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                
                if file_name.contains('_') {
                    *self.naming_conventions.entry("snake_case".to_string()).or_insert(0) += 1;
                } else if file_name.chars().any(|c| c.is_uppercase()) {
                    *self.naming_conventions.entry("camelCase".to_string()).or_insert(0) += 1;
                }
            }
        }
    }
}

impl DocumentationModel {
    fn analyze_docs(root_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut model = Self {
            readme_patterns: HashMap::new(),
            doc_comments: HashMap::new(),
            cargo_toml: HashMap::new(),
            doc_structure: HashMap::new(),
        };
        
        // Analyze Cargo.toml
        let cargo_path = Path::new(root_path).join("Cargo.toml");
        if cargo_path.exists() {
            model.analyze_cargo_toml(&cargo_path)?;
        }
        
        // Analyze README
        let readme_path = Path::new(root_path).join("README.md");
        if readme_path.exists() {
            model.analyze_readme(&readme_path)?;
        }
        
        Ok(model)
    }
    
    fn analyze_cargo_toml(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        
        for line in content.lines() {
            if line.contains('=') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    self.cargo_toml.insert(key, value);
                }
            }
        }
        
        Ok(())
    }
    
    fn analyze_readme(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        
        for line in content.lines() {
            if line.starts_with('#') {
                *self.readme_patterns.entry("header".to_string()).or_insert(0) += 1;
            } else if line.starts_with('-') || line.starts_with('*') {
                *self.readme_patterns.entry("list_item".to_string()).or_insert(0) += 1;
            } else if line.starts_with("```") {
                *self.readme_patterns.entry("code_block".to_string()).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }
}
