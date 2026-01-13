use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use syn::{parse_file, Item, ImplItem};

#[derive(Debug, Serialize, Deserialize)]
struct SynCompressedDeclaration {
    original_path: String,
    decl_type: String,
    name: String,
    tokens: Vec<u16>,
    original_size: usize,
    syn_data: String, // Serialized syn AST data
}

#[derive(Debug, Serialize, Deserialize)]
struct SynCompressedFile {
    path: String,
    declarations: Vec<SynCompressedDeclaration>,
    total_original_size: usize,
    total_compressed_size: usize,
}

struct SynCompressor {
    patterns: HashMap<String, u16>,
    next_token: u16,
}

impl SynCompressor {
    fn new() -> Self {
        let mut compressor = Self {
            patterns: HashMap::new(),
            next_token: 1,
        };
        
        compressor.add_pattern("fn");
        compressor.add_pattern("struct");
        compressor.add_pattern("enum");
        compressor.add_pattern("impl");
        compressor.add_pattern("use");
        compressor.add_pattern("mod");
        compressor.add_pattern("trait");
        compressor.add_pattern("pub");
        
        compressor
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
    
    fn compress_file(&mut self, file_path: &str) -> Result<SynCompressedFile, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let syntax_tree = parse_file(&content)?;
        
        let mut declarations = Vec::new();
        
        for item in &syntax_tree.items {
            let (decl_type, name, syn_data) = match &item {
                Item::Fn(func) => {
                    ("fn".to_string(), func.sig.ident.to_string(), quote::quote!(#func).to_string())
                }
                Item::Struct(s) => {
                    ("struct".to_string(), s.ident.to_string(), quote::quote!(#s).to_string())
                }
                Item::Enum(e) => {
                    ("enum".to_string(), e.ident.to_string(), quote::quote!(#e).to_string())
                }
                Item::Mod(m) => {
                    ("mod".to_string(), m.ident.to_string(), quote::quote!(#m).to_string())
                }
                Item::Use(u) => {
                    ("use".to_string(), "use_stmt".to_string(), quote::quote!(#u).to_string())
                }
                Item::Impl(impl_block) => {
                    let impl_name = if let Some((_, path, _)) = &impl_block.trait_ {
                        format!("impl_{}", quote::quote!(#path).to_string().replace(" ", ""))
                    } else {
                        format!("impl_{}", quote::quote!(#impl_block.self_ty).to_string().replace(" ", ""))
                    };
                    ("impl".to_string(), impl_name, quote::quote!(#impl_block).to_string())
                }
                Item::Trait(t) => {
                    ("trait".to_string(), t.ident.to_string(), quote::quote!(#t).to_string())
                }
                _ => continue,
            };
            
            // Compress the syn data
            let tokens = self.compress_syn_data(&syn_data);
            
            declarations.push(SynCompressedDeclaration {
                original_path: file_path.to_string(),
                decl_type,
                name,
                tokens,
                original_size: syn_data.len(),
                syn_data: syn_data.chars().take(200).collect(), // Keep first 200 chars for reference
            });
        }
        
        // Also compress impl methods
        for item in &syntax_tree.items {
            if let Item::Impl(impl_block) = item {
                for impl_item in &impl_block.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let syn_data = quote::quote!(#method).to_string();
                        let tokens = self.compress_syn_data(&syn_data);
                        
                        declarations.push(SynCompressedDeclaration {
                            original_path: file_path.to_string(),
                            decl_type: "method".to_string(),
                            name: method.sig.ident.to_string(),
                            tokens,
                            original_size: syn_data.len(),
                            syn_data: syn_data.chars().take(200).collect(),
                        });
                    }
                }
            }
        }
        
        let total_original_size = content.len();
        let total_compressed_size = declarations.iter().map(|d| d.tokens.len() * 2).sum();
        
        Ok(SynCompressedFile {
            path: file_path.to_string(),
            declarations,
            total_original_size,
            total_compressed_size,
        })
    }
    
    fn compress_syn_data(&mut self, syn_data: &str) -> Vec<u16> {
        let mut tokens = Vec::new();
        
        // Simple compression based on common syn patterns
        for line in syn_data.lines() {
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
    
    fn save_declarations(&self, compressed_file: &SynCompressedFile, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dir_path = Path::new(output_dir);
        fs::create_dir_all(dir_path)?;
        
        for (i, decl) in compressed_file.declarations.iter().enumerate() {
            let filename = format!("{:03}_{}_{}_{}_{}b_to_{}b.json", 
                i,
                decl.decl_type,
                decl.name.replace("::", "_").replace("<", "_").replace(">", "_"),
                decl.original_size,
                decl.tokens.len() * 2,
                decl.tokens.len() * 2
            );
            
            let file_path = dir_path.join(filename);
            let json = serde_json::to_string_pretty(decl)?;
            fs::write(file_path, json)?;
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 SYN-BASED DECLARATION COMPRESSOR");
    
    let mut compressor = SynCompressor::new();
    let test_file = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/rustc_data_structures/src/lib.rs";
    
    if Path::new(test_file).exists() {
        println!("📁 Processing with syn: {}", test_file);
        
        let compressed = compressor.compress_file(test_file)?;
        
        println!("📊 Syn-based Results:");
        println!("  Declarations found: {}", compressed.declarations.len());
        println!("  Original size: {} bytes", compressed.total_original_size);
        println!("  Compressed size: {} bytes", compressed.total_compressed_size);
        println!("  Compression ratio: {:.1}%", (compressed.total_compressed_size as f64 / compressed.total_original_size as f64) * 100.0);
        
        compressor.save_declarations(&compressed, "syn_compressed_declarations")?;
        
        println!("\n🔍 Syn Declaration breakdown:");
        for decl in &compressed.declarations {
            println!("  {} {}: {} bytes -> {} tokens", 
                decl.decl_type, decl.name, decl.original_size, decl.tokens.len());
        }
        
        println!("\n💾 Syn-compressed declarations saved to: syn_compressed_declarations/");
    }
    
    Ok(())
}
