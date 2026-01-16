// Demo: Nix Workflow Scheduler

mod nix_workflow_scheduler;
use nix_workflow_scheduler::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 NIX WORKFLOW SCHEDULER");
    println!("=========================\n");
    println!("Composable tasks with canonical I/O via nix daemon\n");
    
    let mut scheduler = NixWorkflowScheduler::new();
    
    // Task 1: Branch prediction mining
    println!("📋 Task 1: Branch Prediction Mining");
    let branch_task = create_mining_task("branch_mining", vec![]);
    scheduler.add_task(branch_task);
    
    // Task 2: Markov chain mining
    println!("📋 Task 2: Markov Chain Mining");
    let markov_task = create_mining_task("markov_mining", vec![]);
    scheduler.add_task(markov_task);
    
    // Task 3: Block market
    println!("📋 Task 3: Block Market");
    let block_task = create_mining_task("block_market", vec![]);
    scheduler.add_task(block_task);
    
    println!("\n🚀 Running all tasks...\n");
    
    // Run all mining tasks
    scheduler.run_all().await?;
    
    // Show results
    println!("\n📊 Task Results:");
    for result in &scheduler.results {
        println!("  ✓ {} - {} ({} ms)", 
            result.task_name,
            result.nix_store_path,
            result.duration_ms
        );
        for (name, path) in &result.outputs {
            println!("    - {}: {}", name, path);
        }
    }
    
    // Compose tasks
    println!("\n🔗 Composing tasks...");
    let composed = scheduler.compose_tasks(&vec![
        "branch_mining".to_string(),
        "markov_mining".to_string(),
    ])?;
    println!("  Composed task: {}", composed.name);
    println!("  Inputs: {}", composed.inputs.len());
    
    // Munge results
    println!("\n🔀 Munging results...");
    let munged = scheduler.munge_results(&vec![
        "branch_mining".to_string(),
        "markov_mining".to_string(),
        "block_market".to_string(),
    ])?;
    println!("  Munged store path: {}", munged);
    
    // Task 4: Analyze all mining results with LLM
    println!("\n📋 Task 4: LLM Analysis");
    let analysis_task = create_analysis_task(
        "llm_analysis",
        vec![munged.clone()],
    );
    scheduler.add_task(analysis_task);
    
    // Task 5: Export to HuggingFace
    println!("📋 Task 5: Export to HuggingFace");
    let export_task = create_export_task(
        "hf_export",
        scheduler.results.iter()
            .map(|r| r.nix_store_path.clone())
            .collect(),
    );
    scheduler.add_task(export_task);
    
    // Export workflow as flake
    println!("\n💾 Exporting workflow as flake...");
    scheduler.export_as_flake("/tmp/workflow.nix")?;
    
    println!("\n✅ Workflow Features:");
    println!("  • All tasks run via nix daemon");
    println!("  • Canonical inputs/outputs in nix store");
    println!("  • Composable: combine multiple tasks");
    println!("  • Mungeable: merge results");
    println!("  • Importable: load as flake");
    println!("  • Pure & impure builds supported");
    
    println!("\n💡 Use Cases:");
    println!("  • Run all mining demos in sequence");
    println!("  • Compose results for analysis");
    println!("  • Munge multiple runs together");
    println!("  • Import workflows from other projects");
    println!("  • Export to HuggingFace datasets");
    
    println!("\n🔄 Workflow Pattern:");
    println!("  1. Mine → Nix store");
    println!("  2. Analyze → Nix store");
    println!("  3. Munge → Combined nix store");
    println!("  4. Export → HuggingFace");
    println!("  5. Import → Reuse in other workflows");
    
    Ok(())
}
