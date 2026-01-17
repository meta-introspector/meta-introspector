use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use gix::bstr::ByteSlice; // Add this line
use std::process::Command; // For calling external commands

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Comma-separated list of Git author identifiers (names or emails) to filter by.
    #[arg(short, long, value_delimiter = ',', default_value = "Mike DuPont <jmikedupont2@gmail.com>,mike dupont <mike.dupont@introspector.local>")]
    user_authors: Vec<String>,

    /// Path to the git sources registry JSON file.
    #[arg(short, long, value_name = "FILE", default_value = "data/git-sources-registry.json")]
    registry: PathBuf,

    /// Base directory for all analysis outputs.
    #[arg(short, long, value_name = "DIR")]
    output_base_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct RepoEntry {
    name: String,
    checkout_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct GitSourcesRegistry {
    sources: HashMap<String, RepoEntry>,
}

#[derive(Debug, Serialize)]
struct AnalysisPlanEntry {
    repo_name: String,
    branch_name: String,
    rust_toolchain_version: String,
    rust_files_to_process: usize,
    plan_command: String,
    execute_command: String,
}

// Function to determine Rust toolchain version
fn get_rust_toolchain_version(repo_path: &Path) -> String {
    let toolchain_file = repo_path.join("rust-toolchain.toml");
    if toolchain_file.exists() {
        if let Ok(content) = fs::read_to_string(&toolchain_file) {
            // Very basic parsing for `channel = "..."` or `toolchain = "..."`
            if let Some(line) = content.lines().find(|l| l.trim().starts_with("channel =") || l.trim().starts_with("toolchain =")) {
                if let Some(version) = line.split('=').nth(1) {
                    return version.trim().trim_matches('"').to_string();
                }
            }
        }
    }

    let cargo_toml_file = repo_path.join("Cargo.toml");
    if cargo_toml_file.exists() {
        if let Ok(content) = fs::read_to_string(&cargo_toml_file) {
            if let Some(line) = content.lines().find(|l| l.trim().starts_with("rust-version =")) {
                if let Some(version) = line.split('=').nth(1) {
                    return version.trim().trim_matches('"').to_string();
                }
            }
        }
    }

    "default/unspecified".to_string()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("--- Repo Analysis Planner ---");
    println!("User Authors: {:?}", args.user_authors);
    println!("Registry Path: {}", args.registry.display());
    println!("Output Base Dir: {}", args.output_base_dir.display());
    println!("-----------------------------\n");

    let registry_content = fs::read_to_string(&args.registry)?;
    let registry: GitSourcesRegistry = serde_json::from_str(&registry_content)?;

    let mut overall_plan: Vec<AnalysisPlanEntry> = Vec::new();
    let mut total_rust_files_across_all_branches = 0;
    let mut unique_rust_toolchains: HashSet<String> = HashSet::new();
    let mut total_branches_to_analyze = 0;
    let mut seen_rust_blob_ids: HashSet<String> = HashSet::new(); // Initialize seen set

    // Convert Vec<String> to a single regex pattern for git log --author
    let _user_authors_git_log_pattern = args.user_authors.join("|");

    for (repo_key, repo_entry) in registry.sources {
        if !repo_entry.checkout_path.exists() || !repo_entry.checkout_path.is_dir() {
            eprintln!("Skipping repository '{}' due to invalid or missing checkout path: {}", repo_key, repo_entry.checkout_path.display());
            continue;
        }

        println!("Processing repository: {} ({})", repo_entry.name, repo_entry.checkout_path.display());

        let _repo = match gix::open(&repo_entry.checkout_path) {
            Ok(repo) => repo,
            Err(e) => {
                eprintln!("Skipping repository '{}' due to gix error: {}", repo_key, e);
                continue;
            }
        };
        
        let mut local_branch_names: Vec<String> = Vec::new();
        let git_branches_output = Command::new("git")
            .arg("-C")
            .arg(&repo_entry.checkout_path)
            .arg("for-each-ref")
            .arg("--format=%(refname:short)")
            .arg("refs/heads/")
            .output()?;

        let branches_stdout = String::from_utf8_lossy(&git_branches_output.stdout);
        for line in branches_stdout.lines() {
            local_branch_names.push(line.to_string());
        }

        for branch_name in local_branch_names {
            let mut found_user_commit_in_branch = false;

            for user_author in &args.user_authors {
                let git_log_output = Command::new("git")
                    .arg("-C")
                    .arg(&repo_entry.checkout_path)
                    .arg("log")
                    .arg("-1") // Only need one commit
                    .arg(format!("--author={}", user_author)) // Pass each author separately
                    .arg(&branch_name)
                    .output();

                if let Ok(output) = git_log_output {
                    if output.status.success() && !output.stdout.is_empty() {
                        found_user_commit_in_branch = true;
                        break; // Found a commit by this user, no need to check other authors for this branch
                    }
                }
            }
            
            // If the repository contains any commits by the user, then we need to iterate
            // through its branches and mark them for analysis.
            // This 'found_user_commit_in_branch' will now indicate if the *repo* has user commits.
            // The subsequent logic will then iterate through branches.
            
            if found_user_commit_in_branch {
                total_branches_to_analyze += 1;
                println!("  - Found user commits in branch: {}", branch_name);

                // --- Identify Rust files using crossbeam-value-lattice --list-files-only ---
                let mut new_rust_files_to_process_list: Vec<String> = Vec::new();
                let mut unique_rust_files_for_analysis = 0;

                let list_files_output = Command::new("cargo")
                    .arg("run")
                    .arg("--bin")
                    .arg("crossbeam-value-lattice")
                    .arg("--")
                    .arg("--input-path")
                    .arg(&repo_entry.checkout_path)
                    .arg("--list-files-only")
                    .output()?;
                
                let list_files_stdout = String::from_utf8_lossy(&list_files_output.stdout);
                for file_path in list_files_stdout.lines() {
                    if file_path.is_empty() { continue; }

                    // Get blob ID for the file
                    let hash_object_output = Command::new("git")
                        .arg("-C")
                        .arg(&repo_entry.checkout_path)
                        .arg("hash-object")
                        .arg("-t")
                        .arg("blob")
                        .arg(file_path)
                        .output()?;
                    let blob_id = String::from_utf8_lossy(&hash_object_output.stdout).trim().to_string();

                    if !blob_id.is_empty() && !seen_rust_blob_ids.contains(&blob_id) {
                        unique_rust_files_for_analysis += 1;
                        seen_rust_blob_ids.insert(blob_id.clone());
                        new_rust_files_to_process_list.push(file_path.to_string());
                    }
                }
                total_rust_files_across_all_branches += unique_rust_files_for_analysis;
                // --- End Identify Rust files and their blob IDs ---

                // --- Conceptual Checkout and Toolchain Detection ---
                let rust_toolchain_version = get_rust_toolchain_version(&repo_entry.checkout_path);
                unique_rust_toolchains.insert(rust_toolchain_version.clone());
                // --- End Conceptual Checkout ---

                let sanitized_branch_name = branch_name.replace("/", "_").replace("\\", "_");
                let output_repo_branch_dir = args.output_base_dir.join(&repo_entry.name).join(&sanitized_branch_name);
                fs::create_dir_all(&output_repo_branch_dir)?; // Ensure output directory exists for each branch

                // The actual `crossbeam_value_lattice` command would need to be modified
                // to accept a list of specific files if we are to pass only unique ones.
                // For now, the plan command will reflect processing the unique files identified.
                let plan_cmd = format!(
                    "cargo run --bin crossbeam-value-lattice -- --input-files {} --output-dir \"{}\" --plan-mode",
                    new_rust_files_to_process_list.join(" "), // List unique files
                    output_repo_branch_dir.display()
                );
                let execute_cmd = format!(
                    "cargo run --bin crossbeam-value-lattice -- --input-files {} --output-dir \"{}\"",
                    new_rust_files_to_process_list.join(" "),
                    output_repo_branch_dir.display()
                );

                overall_plan.push(AnalysisPlanEntry {
                    repo_name: repo_entry.name.clone(),
                    branch_name: branch_name.clone(),
                    rust_toolchain_version,
                    rust_files_to_process: unique_rust_files_for_analysis,
                    plan_command: plan_cmd,
                    execute_command: execute_cmd,
                });
            }
        }
    }

    println!("\n--- Aggregated Analysis Plan ---");
    println!("Total branches with user changes to analyze: {}", total_branches_to_analyze);
    println!("Estimated total Rust files to process: {}", total_rust_files_across_all_branches);
    println!("Unique Rust Toolchain Versions identified: {:?}", unique_rust_toolchains);
    println!("\nDetailed Plan:");
    for entry in overall_plan {
        println!("  Repo: {}, Branch: {}", entry.repo_name, entry.branch_name);
        println!("    Rust Version: {}", entry.rust_toolchain_version);
        println!("    Rust Files (plan): {}", entry.rust_files_to_process);
        println!("    Plan Command: {}", entry.plan_command);
        println!("    Execute Command: {}", entry.execute_command);
        println!();
    }

    Ok(())
}

