use std::fs;
use std::path::Path;
use std::process::Command;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 CREATING COMPRESSED DECLARATIONS ARCHIVE");
    
    let declarations_dir = "compressed_declarations";
    let archive_name = "rust_declarations_compressed.tar.gz";
    
    // Read summary to get declaration info
    let summary_path = format!("{}/summary.json", declarations_dir);
    let summary_content = fs::read_to_string(&summary_path)?;
    let summary: serde_json::Value = serde_json::from_str(&summary_content)?;
    
    let declarations = summary["declarations"].as_array().unwrap();
    
    println!("🔄 Renaming {} declarations with nice names...", declarations.len());
    
    // Create temp directory for renamed files
    let temp_dir = "temp_renamed_declarations";
    fs::create_dir_all(temp_dir)?;
    
    // Copy and rename files with nice names
    for (i, decl) in declarations.iter().enumerate() {
        let decl_type = decl["type"].as_str().unwrap();
        let name = decl["name"].as_str().unwrap();
        let lines = decl["lines"].as_str().unwrap();
        let original_size = decl["original_size"].as_u64().unwrap();
        let compressed_size = decl["compressed_size"].as_u64().unwrap();
        
        // Create nice filename with real string names
        let safe_name = name.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_");
        let real_name = if safe_name == "mod" || safe_name == "fn" || safe_name == "struct" {
            // Extract real name from the JSON file
            let old_files: Vec<_> = glob::glob(&format!("{}/lib_{:03}_*.json", declarations_dir, i))?.collect();
            if let Some(Ok(actual_old_path)) = old_files.first() {
                if let Ok(content) = fs::read_to_string(actual_old_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        // Try to extract a better name from the original path or content
                        let original_path = json["original_path"].as_str().unwrap_or("");
                        let file_stem = Path::new(original_path).file_stem()
                            .and_then(|s| s.to_str()).unwrap_or("unknown");
                        format!("{}_{}", file_stem, i)
                    } else {
                        format!("{}_{}", safe_name, i)
                    }
                } else {
                    format!("{}_{}", safe_name, i)
                }
            } else {
                format!("{}_{}", safe_name, i)
            }
        } else {
            safe_name.clone()
        };
        
        let nice_filename = format!("{:03}_{}_{}_{}_{}b_to_{}b.json", 
            i, 
            decl_type,
            real_name,
            lines.replace("-", "_"),
            original_size,
            compressed_size
        );
        
        // Copy original file to new name
        let old_path = format!("{}/lib_{:03}_{}.json", declarations_dir, i, safe_name);
        let new_path = format!("{}/{}", temp_dir, nice_filename);
        
        // Find the actual old file (name might be different)
        let pattern = format!("{}/lib_{:03}_*.json", declarations_dir, i);
        let old_files: Vec<_> = glob::glob(&pattern)?.collect();
        
        if let Some(Ok(actual_old_path)) = old_files.first() {
            fs::copy(actual_old_path, &new_path)?;
            println!("  {} -> {}", actual_old_path.display(), nice_filename);
        }
    }
    
    // Copy summary with nice name
    let nice_summary = format!("{}/000_SUMMARY_{}decls_{}kb.json", 
        temp_dir,
        declarations.len(),
        summary["total_original_size"].as_u64().unwrap() / 1024
    );
    fs::copy(&summary_path, &nice_summary)?;
    
    println!("\n📦 Creating tar.gz archive...");
    
    // Create tar.gz archive
    let output = Command::new("tar")
        .args(&["-czf", archive_name, "-C", temp_dir, "."])
        .output()?;
    
    if output.status.success() {
        println!("✅ Archive created: {}", archive_name);
        
        // Show archive info
        let archive_size = fs::metadata(archive_name)?.len();
        println!("📊 Archive size: {} KB", archive_size / 1024);
        
        // Show contents
        println!("\n📋 Archive contents:");
        let list_output = Command::new("tar")
            .args(&["-tzf", archive_name])
            .output()?;
        
        let contents = String::from_utf8_lossy(&list_output.stdout);
        for (i, line) in contents.lines().take(10).enumerate() {
            println!("  {}", line);
        }
        
        if contents.lines().count() > 10 {
            println!("  ... and {} more files", contents.lines().count() - 10);
        }
        
        // Cleanup temp directory
        fs::remove_dir_all(temp_dir)?;
        
        println!("\n🎯 ARCHIVE COMPLETE!");
        println!("📁 File: {}", archive_name);
        println!("💾 Size: {} KB", archive_size / 1024);
        println!("📦 Contains: {} compressed declarations", declarations.len());
        println!("🔧 Extract with: tar -xzf {}", archive_name);
        
    } else {
        eprintln!("❌ Failed to create archive: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
