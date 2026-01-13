use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct CompressedDeclaration {
    original_path: String,
    decl_type: String, // "struct", "enum", "fn", "impl", "use", etc.
    name: String,
    tokens: Vec<u16>,
    original_size: usize,
    line_start: usize,
    line_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompressedFile {
    path: String,
    declarations: Vec<CompressedDeclaration>,
    total_original_size: usize,
    total_compressed_size: usize,
}

struct DeclarationExtractor {
    patterns: HashMap<String, u16>,
    next_token: u16,
}

impl DeclarationExtractor {
    fn new() -> Self {
        let mut extractor = Self {
            patterns: HashMap::new(),
            next_token: 1,
        };
        
        // Pre-load patterns
        extractor.add_pattern("use ");
        extractor.add_pattern("fn ");
        extractor.add_pattern("impl ");
        extractor.add_pattern("struct ");
        extractor.add_pattern("enum ");
        extractor.add_pattern("pub ");
        extractor.add_pattern("rustc_");
        
        extractor
    }
    
    fn add_pattern(&mut self, pattern: &str) -> u16 {
        if let Some(&token) = self.patterns.get(pattern) {
            return token;
        }
        let token = self.next_token;
        self.next_token += 1;
        self.patterns.insert(pattern.to_string(), token);
        token
    }
    
    fn extract_declarations(&mut self, file_path: &str) -> Result<CompressedFile, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut declarations = Vec::new();
        let mut current_decl: Option<(String, String, usize)> = None; // (type, name, start_line)
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Start of new declaration
            if let Some((decl_type, name)) = self.parse_declaration_start(trimmed) {
                // Save previous declaration if exists
                if let Some((prev_type, prev_name, start_line)) = current_decl.take() {
                    let decl_lines = &lines[start_line..line_num];
                    let decl_content = decl_lines.join("\n");
                    let tokens = self.compress_content(&decl_content);
                    
                    declarations.push(CompressedDeclaration {
                        original_path: file_path.to_string(),
                        decl_type: prev_type,
                        name: prev_name,
                        tokens,
                        original_size: decl_content.len(),
                        line_start: start_line,
                        line_end: line_num - 1,
                    });
                }
                
                current_decl = Some((decl_type, name, line_num));
            }
        }
        
        // Handle last declaration
        if let Some((decl_type, name, start_line)) = current_decl {
            let decl_lines = &lines[start_line..];
            let decl_content = decl_lines.join("\n");
            let tokens = self.compress_content(&decl_content);
            
            declarations.push(CompressedDeclaration {
                original_path: file_path.to_string(),
                decl_type,
                name,
                tokens,
                original_size: decl_content.len(),
                line_start: start_line,
                line_end: lines.len() - 1,
            });
        }
        
        let total_original_size = content.len();
        let total_compressed_size = declarations.iter().map(|d| d.tokens.len() * 2).sum();
        
        Ok(CompressedFile {
            path: file_path.to_string(),
            declarations,
            total_original_size,
            total_compressed_size,
        })
    }
    
    fn parse_declaration_start(&self, line: &str) -> Option<(String, String)> {
        let line = line.trim();
        
        if line.starts_with("pub fn ") || line.starts_with("fn ") {
            let name = line.split_whitespace().nth(1)?.split('(').next()?.to_string();
            Some(("fn".to_string(), name))
        } else if line.starts_with("pub struct ") || line.starts_with("struct ") {
            let name = line.split_whitespace().nth(1)?.split('<').next()?.split('{').next()?.to_string();
            Some(("struct".to_string(), name))
        } else if line.starts_with("pub enum ") || line.starts_with("enum ") {
            let name = line.split_whitespace().nth(1)?.split('<').next()?.split('{').next()?.to_string();
            Some(("enum".to_string(), name))
        } else if line.starts_with("impl ") {
            // Extract impl target name
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = if parts.len() > 1 {
                parts[1].split('<').next().unwrap_or("impl").to_string()
            } else {
                "impl".to_string()
            };
            Some(("impl".to_string(), name))
        } else if line.starts_with("use ") {
            // Extract the main module being used
            let use_part = line.strip_prefix("use ").unwrap_or("");
            let name = use_part.split(';').next().unwrap_or("")
                .split("::").last().unwrap_or("use")
                .split('{').next().unwrap_or("use")
                .trim().to_string();
            Some(("use".to_string(), name))
        } else if line.starts_with("pub mod ") || line.starts_with("mod ") {
            let name = line.split_whitespace().nth(1)?.split(';').next()?.to_string();
            Some(("mod".to_string(), name))
        } else if line.starts_with("pub trait ") || line.starts_with("trait ") {
            let name = line.split_whitespace().nth(1)?.split('<').next()?.split('{').next()?.to_string();
            Some(("trait".to_string(), name))
        } else {
            None
        }
    }
    
    fn compress_content(&mut self, content: &str) -> Vec<u16> {
        let mut tokens = Vec::new();
        for line in content.lines() {
            let mut matched = false;
            for (pattern, &token) in &self.patterns {
                if line.contains(pattern) {
                    tokens.push(token);
                    matched = true;
                    break;
                }
            }
            if !matched {
                tokens.push((line.len() % 65535) as u16);
            }
        }
        tokens
    }
    
    fn save_declarations(&self, compressed_file: &CompressedFile, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dir_path = Path::new(output_dir);
        fs::create_dir_all(dir_path)?;
        
        // Save each declaration as separate file
        for (i, decl) in compressed_file.declarations.iter().enumerate() {
            let filename = format!("{}_{:03}_{}.json", 
                Path::new(&compressed_file.path).file_stem().unwrap().to_string_lossy(),
                i,
                decl.name.replace("::", "_")
            );
            
            let file_path = dir_path.join(filename);
            let json = serde_json::to_string_pretty(decl)?;
            fs::write(file_path, json)?;
        }
        
        // Save summary
        let summary_path = dir_path.join("summary.json");
        let summary = serde_json::json!({
            "file_path": compressed_file.path,
            "total_declarations": compressed_file.declarations.len(),
            "total_original_size": compressed_file.total_original_size,
            "total_compressed_size": compressed_file.total_compressed_size,
            "compression_ratio": compressed_file.total_compressed_size as f64 / compressed_file.total_original_size as f64,
            "declarations": compressed_file.declarations.iter().map(|d| {
                serde_json::json!({
                    "type": d.decl_type,
                    "name": d.name,
                    "lines": format!("{}-{}", d.line_start, d.line_end),
                    "original_size": d.original_size,
                    "compressed_size": d.tokens.len() * 2
                })
            }).collect::<Vec<_>>()
        });
        
        fs::write(summary_path, serde_json::to_string_pretty(&summary)?)?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 DECLARATION-LEVEL COMPRESSION EXTRACTOR");
    
    let mut extractor = DeclarationExtractor::new();
    
    // Test with a sample file first
    let test_file = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/rustc_data_structures/src/lib.rs";
    
    if Path::new(test_file).exists() {
        println!("📁 Processing: {}", test_file);
        
        let compressed = extractor.extract_declarations(test_file)?;
        
        println!("📊 Results:");
        println!("  Declarations found: {}", compressed.declarations.len());
        println!("  Original size: {} bytes", compressed.total_original_size);
        println!("  Compressed size: {} bytes", compressed.total_compressed_size);
        println!("  Compression ratio: {:.1}%", (compressed.total_compressed_size as f64 / compressed.total_original_size as f64) * 100.0);
        
        // Save declarations
        extractor.save_declarations(&compressed, "compressed_declarations")?;
        
        println!("\n💾 Declarations saved to: compressed_declarations/");
        println!("📋 Each declaration is a separate JSON file");
        println!("📄 Summary available in: compressed_declarations/summary.json");
        
        // Show declaration breakdown
        println!("\n🔍 Declaration breakdown:");
        for decl in &compressed.declarations {
            println!("  {} {}: {} bytes -> {} tokens (lines {}-{})", 
                decl.decl_type, decl.name, decl.original_size, decl.tokens.len(),
                decl.line_start, decl.line_end);
        }
    } else {
        println!("❌ Test file not found: {}", test_file);
    }
    
    Ok(())
}
