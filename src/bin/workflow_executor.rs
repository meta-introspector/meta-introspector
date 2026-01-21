// Workflow executor binary
// Runs declarative workflows and generates tool calls

use std::fs;

include!("../workflow.rs");

fn main() {
    println!("🔬 71 Language Workflow System");
    println!("================================\n");
    
    // Generate all 71 workflows
    let workflows = generate_all_workflows();
    
    println!("📋 Generated {} workflows\n", workflows.len());
    
    // Example: Show first workflow
    let rust_wf = workflow_71_complete("rust");
    println!("Example workflow: {}", rust_wf.name);
    println!("Steps: {}", rust_wf.steps.len());
    
    // Convert to tool calls
    let tool_calls = rust_wf.to_tool_calls();
    println!("\nTool calls generated: {}", tool_calls.len());
    
    // Save all workflows as JSON
    let workflows_json = serde_json::to_string_pretty(&workflows).unwrap();
    fs::write("data/71_workflows.json", workflows_json).unwrap();
    println!("\n✅ Saved to data/71_workflows.json");
    
    // Generate execution script
    generate_execution_script(&workflows);
    println!("✅ Generated execute_workflows.sh");
}

fn generate_execution_script(workflows: &[Workflow]) {
    let mut script = String::from("#!/bin/bash\n");
    script.push_str("# Auto-generated workflow execution script\n");
    script.push_str("# Generated from declarative workflow definitions\n\n");
    script.push_str("set -e\n\n");
    
    script.push_str("mkdir -p data/71_flakes_perf data/71_results\n\n");
    
    for wf in workflows {
        script.push_str(&format!("echo \"🔬 Running workflow: {}\"\n", wf.name));
        
        for tool_call in wf.to_tool_calls() {
            match tool_call {
                ToolCall::ExecuteBash { command } => {
                    script.push_str(&format!("{}\n", command));
                }
                _ => {}
            }
        }
        script.push_str("\n");
    }
    
    fs::write("execute_workflows.sh", script).unwrap();
    fs::set_permissions("execute_workflows.sh", 
        std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
}
