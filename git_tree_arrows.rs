//! Git Trees as Category Arrows in Parquet
//! Trees = objects, Submodules = arrows between them

use arrow::array::{StringArray, UInt64Array};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::sync::Arc;
use std::process::Command;
use std::path::Path;

#[derive(Debug)]
struct GitArrow {
    source_repo: String,
    source_tree: String,
    target_repo: String,
    target_tree: String,
    submodule_path: String,
}

fn extract_git_arrows(registry_path: &Path) -> Result<Vec<GitArrow>, Box<dyn std::error::Error>> {
    let mut arrows = Vec::new();
    
    // Get all repos from git-sources
    let output = Command::new("./target/release/git-sources")
        .arg("list")
        .output()?;
    
    let registry = String::from_utf8_lossy(&output.stdout);
    let mut current_repo = None;
    let mut current_path = None;
    
    for line in registry.lines() {
        if line.contains("URL:") {
            current_repo = Some(line.split("URL:").nth(1).unwrap().trim().to_string());
        }
        if line.contains("Path:") {
            current_path = Some(line.split("Path:").nth(1).unwrap().trim().to_string());
        }
        
        if let (Some(repo), Some(path)) = (&current_repo, &current_path) {
            // Get tree SHA
            let tree = get_tree_sha(&path)?;
            
            // Check for submodules
            let gitmodules = format!("{}/.gitmodules", path);
            if Path::new(&gitmodules).exists() {
                let submodules = parse_gitmodules(&gitmodules)?;
                
                for (subpath, url) in submodules {
                    let target_tree = get_submodule_tree(&path, &subpath)?;
                    
                    arrows.push(GitArrow {
                        source_repo: repo.clone(),
                        source_tree: tree.clone(),
                        target_repo: url,
                        target_tree,
                        submodule_path: subpath,
                    });
                }
            }
            
            current_repo = None;
            current_path = None;
        }
    }
    
    Ok(arrows)
}

fn get_tree_sha(repo_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["-C", repo_path, "rev-parse", "HEAD^{tree}"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_submodule_tree(repo_path: &str, submodule_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["-C", repo_path, "ls-tree", "HEAD", submodule_path])
        .output()?;
    
    let line = String::from_utf8_lossy(&output.stdout);
    // Format: "160000 commit <sha> <path>"
    Ok(line.split_whitespace().nth(2).unwrap_or("").to_string())
}

fn parse_gitmodules(path: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut submodules = Vec::new();
    let mut current_path = None;
    let mut current_url = None;
    
    for line in content.lines() {
        if line.trim().starts_with("path =") {
            current_path = Some(line.split('=').nth(1).unwrap().trim().to_string());
        }
        if line.trim().starts_with("url =") {
            current_url = Some(line.split('=').nth(1).unwrap().trim().to_string());
        }
        
        if let (Some(p), Some(u)) = (&current_path, &current_url) {
            submodules.push((p.clone(), u.clone()));
            current_path = None;
            current_url = None;
        }
    }
    
    Ok(submodules)
}

fn save_arrows_parquet(arrows: &[GitArrow], output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("source_repo", DataType::Utf8, false),
        Field::new("source_tree", DataType::Utf8, false),
        Field::new("target_repo", DataType::Utf8, false),
        Field::new("target_tree", DataType::Utf8, false),
        Field::new("submodule_path", DataType::Utf8, false),
    ]);
    
    let source_repos: Vec<_> = arrows.iter().map(|a| a.source_repo.clone()).collect();
    let source_trees: Vec<_> = arrows.iter().map(|a| a.source_tree.clone()).collect();
    let target_repos: Vec<_> = arrows.iter().map(|a| a.target_repo.clone()).collect();
    let target_trees: Vec<_> = arrows.iter().map(|a| a.target_tree.clone()).collect();
    let submodule_paths: Vec<_> = arrows.iter().map(|a| a.submodule_path.clone()).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(StringArray::from(source_repos)),
            Arc::new(StringArray::from(source_trees)),
            Arc::new(StringArray::from(target_repos)),
            Arc::new(StringArray::from(target_trees)),
            Arc::new(StringArray::from(submodule_paths)),
        ],
    )?;
    
    let file = std::fs::File::create(output)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
    writer.write(&batch)?;
    writer.close()?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Extracting git tree arrows...");
    
    let arrows = extract_git_arrows(Path::new("git-sources-registry.json"))?;
    
    println!("  Found {} arrows (submodule links)", arrows.len());
    
    let output = Path::new("data/git_tree_arrows.parquet");
    std::fs::create_dir_all(output.parent().unwrap())?;
    
    save_arrows_parquet(&arrows, output)?;
    
    println!("✅ Saved to {:?}", output);
    println!("\nCategory structure:");
    println!("  Objects: Git trees (tree SHAs)");
    println!("  Arrows: Submodule links");
    println!("  Composition: Transitive submodule dependencies");
    
    Ok(())
}
