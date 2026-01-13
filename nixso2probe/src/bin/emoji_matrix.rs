use anyhow::Result;
use nixso2probe::emoji_compiler_matrix::{EmojiMatrixBuilder, EmojiCompilerMatrix};

fn main() -> Result<()> {
    println!("🎯 Creating Emoji Compiler Matrix...");
    
    // Create diagonal matrix
    let diagonal_matrix = EmojiMatrixBuilder::create_diagonal_matrix()?;
    println!("✅ Diagonal matrix created");
    
    // Fill in cross-compilation paths
    let full_matrix = EmojiMatrixBuilder::fill_cross_compilation_matrix(diagonal_matrix)?;
    println!("✅ Cross-compilation matrix filled");
    
    // Print the matrix
    EmojiMatrixBuilder::print_matrix(&full_matrix);
    
    // Save to file
    EmojiMatrixBuilder::save_matrix_to_file(&full_matrix)?;
    println!("✅ Matrix saved to emoji_compiler_matrix.json");
    
    // Generate Nix derivations
    let nix_derivations = EmojiMatrixBuilder::generate_nix_matrix_derivations(&full_matrix);
    std::fs::write("/mnt/data1/meta-introspector/emoji_matrix_derivations.nix", nix_derivations)?;
    println!("✅ Nix derivations generated");
    
    println!();
    println!("🚀 COMPLETE EMOJI COMPILER MATRIX:");
    println!("   🔥 Diagonal: Self-hosting paths");
    println!("   🌈 Upper: Cross-compilation paths");
    println!("   ⭐ Lower: Bootstrap paths");
    println!("   📦 All paths have Nix derivations!");
    
    Ok(())
}
