//! Git Trees as Temporal Category - Morphisms over Time
//! Timeline = morphisms showing how arrows replace each other (compilation witness)

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
    commit_time: u64,
    commit_sha: String,
    replaced_tree: Option<String>,
    witness_type: String,
}

struct Commit {
    sha: String,
    tree_sha: String,
    timestamp: u64,
}

fn get_commit_history(repo_path: &str) -> Result<Vec<Commit>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["-C", repo_path, "log", "--format=%H %T %ct", "--all"])
        .output()?;
    
    let mut commits = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            commits.push(Commit {
                sha: parts[0].to_string(),
                tree_sha: parts[1].to_string(),
                timestamp: parts[2].parse().unwrap_or(0),
            });
        }
    }
    Ok(commits)
}

fn get_submodule_tree_at_commit(repo_path: &str, submodule_path: &str, commit: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["-C", repo_path, "ls-tree", commit, submodule_path])
        .output()?;
    
    let line = String::from_utf8_lossy(&output.stdout);
    Ok(line.split_whitespace().nth(2).unwrap_or("").to_string())
}

fn get_previous_submodule_tree(repo_path: &str, submodule_path: &str, commit: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["-C", repo_path, "log", "--format=%H", "-n", "2", commit])
        .output()?;
    
    let commits: Vec<_> = String::from_utf8_lossy(&output.stdout).lines().collect();
    if commits.len() < 2 {
        return Ok(None);
    }
    
    let prev_commit = commits[1];
    let tree = get_submodule_tree_at_commit(repo_path, submodule_path, prev_commit)?;
    Ok(if tree.is_empty() { None } else { Some(tree) })
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

fn extract_temporal_arrows(registry_path: &Path) -> Result<Vec<GitArrow>, Box<dyn std::error::Error>> {
    let mut arrows = Vec::new();
    
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
            let commits = get_commit_history(&path)?;
            
            for commit in commits {
                let gitmodules = format!("{}/.gitmodules", path);
                if Path::new(&gitmodules).exists() {
                    let submodules = parse_gitmodules(&gitmodules)?;
                    
                    for (subpath, url) in submodules {
                        let target_tree = get_submodule_tree_at_commit(&path, &subpath, &commit.sha)?;
                        let replaced = get_previous_submodule_tree(&path, &subpath, &commit.sha)?;
                        
                        arrows.push(GitArrow {
                            source_repo: repo.clone(),
                            source_tree: commit.tree_sha.clone(),
                            target_repo: url,
                            target_tree,
                            submodule_path: subpath.clone(),
                            commit_time: commit.timestamp,
                            commit_sha: commit.sha.clone(),
                            replaced_tree: replaced,
                            witness_type: "commit".to_string(),
                        });
                    }
                }
            }
            
            current_repo = None;
            current_path = None;
        }
    }
    
    Ok(arrows)
}

fn save_temporal_arrows(arrows: &[GitArrow], output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("source_repo", DataType::Utf8, false),
        Field::new("source_tree", DataType::Utf8, false),
        Field::new("target_repo", DataType::Utf8, false),
        Field::new("target_tree", DataType::Utf8, false),
        Field::new("submodule_path", DataType::Utf8, false),
        Field::new("commit_time", DataType::UInt64, false),
        Field::new("commit_sha", DataType::Utf8, false),
        Field::new("replaced_tree", DataType::Utf8, true),
        Field::new("witness_type", DataType::Utf8, false),
    ]);
    
    let source_repos: Vec<_> = arrows.iter().map(|a| a.source_repo.clone()).collect();
    let source_trees: Vec<_> = arrows.iter().map(|a| a.source_tree.clone()).collect();
    let target_repos: Vec<_> = arrows.iter().map(|a| a.target_repo.clone()).collect();
    let target_trees: Vec<_> = arrows.iter().map(|a| a.target_tree.clone()).collect();
    let submodule_paths: Vec<_> = arrows.iter().map(|a| a.submodule_path.clone()).collect();
    let commit_times: Vec<_> = arrows.iter().map(|a| a.commit_time).collect();
    let commit_shas: Vec<_> = arrows.iter().map(|a| a.commit_sha.clone()).collect();
    let replaced_trees: Vec<_> = arrows.iter().map(|a| a.replaced_tree.clone().unwrap_or_default()).collect();
    let witness_types: Vec<_> = arrows.iter().map(|a| a.witness_type.clone()).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(StringArray::from(source_repos)),
            Arc::new(StringArray::from(source_trees)),
            Arc::new(StringArray::from(target_repos)),
            Arc::new(StringArray::from(target_trees)),
            Arc::new(StringArray::from(submodule_paths)),
            Arc::new(UInt64Array::from(commit_times)),
            Arc::new(StringArray::from(commit_shas)),
            Arc::new(StringArray::from(replaced_trees)),
            Arc::new(StringArray::from(witness_types)),
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
    println!("⏱️  Extracting temporal git morphisms...");
    
    let arrows = extract_temporal_arrows(Path::new("git-sources-registry.json"))?;
    
    println!("  Found {} temporal arrows", arrows.len());
    
    let output = Path::new("data/git_temporal_morphisms.parquet");
    std::fs::create_dir_all(output.parent().unwrap())?;
    
    save_temporal_arrows(&arrows, output)?;
    
    println!("✅ Saved to {:?}", output);
    println!("\nTemporal Category:");
    println!("  Objects: Git trees");
    println!("  Arrows: Submodule links at time T");
    println!("  Morphisms: How arrows change over time");
    println!("  Witness: Commits as proof of thought process");
    
    Ok(())
}
