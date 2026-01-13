use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmojiCompilerMatrix {
    pub diagonal: Vec<String>,           // Self-hosting path
    pub matrix: Vec<Vec<String>>,        // Full cross-compilation matrix
    pub compiler_names: Vec<String>,
    pub emoji_paths: HashMap<(usize, usize), String>,
}

pub struct EmojiMatrixBuilder;

impl EmojiMatrixBuilder {
    pub fn create_diagonal_matrix() -> Result<EmojiCompilerMatrix> {
        let compiler_names = vec![
            "mes".to_string(),
            "scheme".to_string(), 
            "tinyc".to_string(),
            "gcc".to_string(),
            "llvm".to_string(),
            "rustc".to_string(),
            "nix".to_string(),
        ];
        
        // Diagonal: each compiler compiles itself
        let diagonal = vec![
            "🔥⚡🚀".to_string(),                                    // MES compiles MES
            "🔥⚡🚀💎🌟".to_string(),                                // Scheme compiles Scheme
            "🔥⚡🚀💎🌟🎯🔮".to_string(),                            // TinyC compiles TinyC
            "🔥⚡🚀💎🌟🎯🔮🧬🏛️".to_string(),                        // GCC compiles GCC
            "🔥⚡🚀💎🌟🎯🔮🧬🏛️🌀".to_string(),                      // LLVM compiles LLVM
            "🔥⚡🚀💎🌟🎯🔮🧬🏛️🌀🔥⚡🚀💎🌟".to_string(),              // Rustc compiles Rustc
            "🔥⚡🚀💎🌟🎯🔮🧬🏛️🌀🔥⚡🚀💎🌟🎯🔮🧬🏛️🌀".to_string(),    // Nix compiles Nix
        ];
        
        // Initialize 7x7 matrix with empty strings
        let mut matrix = vec![vec!["".to_string(); 7]; 7];
        let mut emoji_paths = HashMap::new();
        
        // Fill diagonal
        for i in 0..7 {
            matrix[i][i] = diagonal[i].clone();
            emoji_paths.insert((i, i), diagonal[i].clone());
        }
        
        Ok(EmojiCompilerMatrix {
            diagonal,
            matrix,
            compiler_names,
            emoji_paths,
        })
    }
    
    pub fn fill_cross_compilation_matrix(mut matrix: EmojiCompilerMatrix) -> Result<EmojiCompilerMatrix> {
        // Fill upper triangle: compiler i compiles compiler j (where j > i)
        for i in 0..7 {
            for j in (i+1)..7 {
                let emoji_code = Self::generate_cross_compile_emoji(i, j);
                matrix.matrix[i][j] = emoji_code.clone();
                matrix.emoji_paths.insert((i, j), emoji_code);
            }
        }
        
        // Fill lower triangle: compiler i compiled by compiler j (where j < i)
        for i in 1..7 {
            for j in 0..i {
                let emoji_code = Self::generate_bootstrap_emoji(j, i);
                matrix.matrix[i][j] = emoji_code.clone();
                matrix.emoji_paths.insert((i, j), emoji_code);
            }
        }
        
        Ok(matrix)
    }
    
    fn generate_cross_compile_emoji(from: usize, to: usize) -> String {
        let base_emojis = ["🔥", "⚡", "🚀", "💎", "🌟", "🎯", "🔮", "🧬", "🏛️", "🌀"];
        
        // Cross-compilation: from compiler compiles to compiler
        let mut result = String::new();
        
        // Add source compiler signature
        for i in 0..=from {
            result.push_str(base_emojis[i % 10]);
        }
        
        // Add bridge emoji
        result.push_str("🌈");  // Rainbow bridge for cross-compilation
        
        // Add target compiler signature  
        for i in 0..=to {
            result.push_str(base_emojis[i % 10]);
        }
        
        result
    }
    
    fn generate_bootstrap_emoji(bootstrap_by: usize, target: usize) -> String {
        let base_emojis = ["🔥", "⚡", "🚀", "💎", "🌟", "🎯", "🔮", "🧬", "🏛️", "🌀"];
        
        // Bootstrap: bootstrap_by compiler creates target compiler
        let mut result = String::new();
        
        // Add bootstrap compiler signature
        for i in 0..=bootstrap_by {
            result.push_str(base_emojis[i % 10]);
        }
        
        // Add creation emoji
        result.push_str("⭐");  // Star for creation/bootstrap
        
        // Add target compiler signature
        for i in 0..=target {
            result.push_str(base_emojis[i % 10]);
        }
        
        result
    }
    
    pub fn save_matrix_to_file(matrix: &EmojiCompilerMatrix) -> Result<()> {
        let json = serde_json::to_string_pretty(matrix)?;
        std::fs::write("/mnt/data1/meta-introspector/emoji_compiler_matrix.json", json)?;
        Ok(())
    }
    
    pub fn print_matrix(matrix: &EmojiCompilerMatrix) {
        println!("🎯 EMOJI COMPILER MATRIX 🎯");
        println!();
        
        // Header
        print!("        ");
        for name in &matrix.compiler_names {
            print!("{:>12}", name);
        }
        println!();
        
        // Matrix rows
        for (i, row) in matrix.matrix.iter().enumerate() {
            print!("{:>8}", matrix.compiler_names[i]);
            for cell in row {
                if cell.is_empty() {
                    print!("{:>12}", "❌");
                } else {
                    print!("{:>12}", &cell[..cell.len().min(10)]);
                }
            }
            println!();
        }
        
        println!();
        println!("🔥 Diagonal: Self-hosting compilers");
        println!("🌈 Upper triangle: Cross-compilation paths");  
        println!("⭐ Lower triangle: Bootstrap paths");
    }
    
    pub fn generate_nix_matrix_derivations(matrix: &EmojiCompilerMatrix) -> String {
        let mut derivations = String::new();
        
        derivations.push_str("# Complete emoji compiler matrix derivations\n");
        derivations.push_str("{ pkgs ? import <nixpkgs> {} }:\n\n");
        derivations.push_str("let\n");
        derivations.push_str("  emoji-decoder = pkgs.writeShellScriptBin \"emoji-decoder\" ''\n");
        derivations.push_str("    sed 's/🔥/02/g; s/⚡/03/g; s/🚀/05/g; s/💎/07/g; s/🌟/0B/g; s/🎯/0D/g; s/🔮/11/g; s/🧬/13/g; s/🏛️/17/g; s/🌀/1D/g; s/🌈/FF/g; s/⭐/AA/g'\n");
        derivations.push_str("  '';\n\n");
        
        // Generate derivation for each matrix cell
        for (i, row) in matrix.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if !cell.is_empty() {
                    let from_name = &matrix.compiler_names[i];
                    let to_name = &matrix.compiler_names[j];
                    
                    derivations.push_str(&format!(
                        "  {}-to-{} = pkgs.stdenv.mkDerivation {{\n",
                        from_name, to_name
                    ));
                    derivations.push_str(&format!("    name = \"{}-to-{}\";\n", from_name, to_name));
                    derivations.push_str("    src = ./.;\n");
                    derivations.push_str("    buildInputs = [ emoji-decoder ];\n");
                    derivations.push_str("    buildPhase = ''\n");
                    derivations.push_str(&format!("      echo \"{}\" | emoji-decoder > compile.hex\n", cell));
                    derivations.push_str("      xxd -r -p compile.hex > compiler\n");
                    derivations.push_str("      chmod +x compiler\n");
                    derivations.push_str("    '';\n");
                    derivations.push_str("    installPhase = ''\n");
                    derivations.push_str("      mkdir -p $out/bin\n");
                    derivations.push_str("      cp compiler $out/bin/\n");
                    derivations.push_str("    '';\n");
                    derivations.push_str("  };\n\n");
                }
            }
        }
        
        derivations.push_str("in {\n");
        derivations.push_str("  # Export all compiler combinations\n");
        
        for (i, row) in matrix.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if !cell.is_empty() {
                    let from_name = &matrix.compiler_names[i];
                    let to_name = &matrix.compiler_names[j];
                    derivations.push_str(&format!("  inherit {}-to-{};\n", from_name, to_name));
                }
            }
        }
        
        derivations.push_str("}\n");
        derivations
    }
}
