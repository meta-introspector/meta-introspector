// Declarative Workflow System for 71 Language Testing
// Workflows compose into tool calls via macros

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStep {
    NixBuild { path: String, timeout: Option<u64> },
    PerfRecord { path: String, output: String, command: Vec<String> },
    ForceRebuild { path: String, output: String },
    HarmonicAnalysis { perf_file: String, output: String },
    Conditional { condition: Box<WorkflowStep>, then: Box<WorkflowStep>, else_: Option<Box<WorkflowStep>> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
}

// Macro to declare workflows
#[macro_export]
macro_rules! mkwf {
    ($name:expr => $($step:expr),+ $(,)?) => {
        Workflow {
            name: $name.to_string(),
            steps: vec![$($step),+],
        }
    };
}

// Macro for nix build step
#[macro_export]
macro_rules! nix_build {
    ($path:expr) => {
        WorkflowStep::NixBuild { 
            path: $path.to_string(), 
            timeout: None 
        }
    };
    ($path:expr, timeout: $timeout:expr) => {
        WorkflowStep::NixBuild { 
            path: $path.to_string(), 
            timeout: Some($timeout) 
        }
    };
}

    // Use: crate::perf::record() - see src/perf/mod.rs
#[macro_export]
macro_rules! perf_record {
    ($path:expr => $output:expr, cmd: $($cmd:expr),+) => {
        WorkflowStep::PerfRecord {
            path: $path.to_string(),
            output: $output.to_string(),
            command: vec![$($cmd.to_string()),+],
        }
    };
}

// Macro for force rebuild
#[macro_export]
macro_rules! force_rebuild {
    ($path:expr => $output:expr) => {
        WorkflowStep::ForceRebuild {
            path: $path.to_string(),
            output: $output.to_string(),
        }
    };
}

// Macro for harmonic analysis
#[macro_export]
macro_rules! harmonic_analysis {
    ($perf_file:expr => $output:expr) => {
        WorkflowStep::HarmonicAnalysis {
            perf_file: $perf_file.to_string(),
            output: $output.to_string(),
        }
    };
}

// Macro for conditional execution
#[macro_export]
macro_rules! if_success {
    ($condition:expr => $then:expr) => {
        WorkflowStep::Conditional {
            condition: Box::new($condition),
            then: Box::new($then),
            else_: None,
        }
    };
    ($condition:expr => $then:expr, else: $else:expr) => {
        WorkflowStep::Conditional {
            condition: Box::new($condition),
            then: Box::new($then),
            else_: Some(Box::new($else)),
        }
    };
}

// Example: Complete 71 language workflow
pub fn workflow_71_complete(lang: &str) -> Workflow {
    let path = format!("const_71_test/{}", lang);
    let perf_file = format!("data/71_flakes_perf/{}_build.perf.data", lang);
    let rebuild_file = format!("data/71_flakes_perf/{}_rebuild.perf.data", lang);
    let analysis_file = format!("data/71_results/{}_analysis.txt", lang);
    
    mkwf!(
        format!("71_complete_{}", lang) =>
        // Step 1: Quick test build
        if_success!(
            nix_build!(path.clone(), timeout: 60) =>
            // Step 2: Perf record
            if_success!(
                perf_record!(path.clone() => perf_file.clone(), 
                    cmd: "nix", "build") =>
                // Step 3: Force rebuild
                if_success!(
                    force_rebuild!(path.clone() => rebuild_file.clone()) =>
                    // Step 4: Harmonic analysis
                    harmonic_analysis!(perf_file.clone() => analysis_file.clone())
                )
            )
        )
    )
}

// Workflow executor - converts to tool calls
impl Workflow {
    pub fn to_tool_calls(&self) -> Vec<ToolCall> {
        self.steps.iter().flat_map(|step| step.to_tool_calls()).collect()
    }
}

impl WorkflowStep {
    pub fn to_tool_calls(&self) -> Vec<ToolCall> {
        match self {
            WorkflowStep::NixBuild { path, timeout } => {
                let cmd = if let Some(t) = timeout {
                    format!("cd {} && timeout {} nix build", path, t)
                } else {
                    format!("cd {} && nix build", path)
                };
                vec![ToolCall::ExecuteBash { command: cmd }]
            }
            WorkflowStep::PerfRecord { path, output, command } => {
                let cmd = format!(
    // Use: crate::perf::record() - see src/perf/mod.rs
                    path, output, command.join(" ")
                );
                vec![ToolCall::ExecuteBash { command: cmd }]
            }
            WorkflowStep::ForceRebuild { path, output } => {
                vec![
                    ToolCall::ExecuteBash { 
                        command: "nix-collect-garbage".to_string() 
                    },
                    ToolCall::ExecuteBash {
                        command: format!(
    // Use: crate::perf::record() - see src/perf/mod.rs
                            path, output
                        )
                    }
                ]
            }
            WorkflowStep::HarmonicAnalysis { perf_file, output } => {
                vec![ToolCall::ExecuteBash {
                    command: format!(
                        "./target/release/harmonic_analyzer {} > {}",
                        perf_file, output
                    )
                }]
            }
            WorkflowStep::Conditional { condition, then, else_ } => {
                let mut calls = condition.to_tool_calls();
                calls.extend(then.to_tool_calls());
                if let Some(e) = else_ {
                    calls.extend(e.to_tool_calls());
                }
                calls
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCall {
    ExecuteBash { command: String },
    NixBuild { path: String },
    PerfRecord { command: String, output: String },
}

// Generate workflow for all 71 languages
pub fn generate_all_workflows() -> Vec<Workflow> {
    let languages = vec![
        "agda", "asm", "bash", "bazel", "brainfuck", "chisel", "cirq", "cmake",
        "coq", "datalog", "fish", "gcc", "genetic", "graph_partition", "graphql",
        "haskell", "idris2", "ini", "isabelle", "jax_gpu", "json", "julia",
        "lean4", "llvm", "lua", "luau", "makefile", "mcts", "mes", "metacoq",
        "minizinc", "mongodb", "move", "neo4j", "nix_derivation", "nix_expr",
        "nix_flake", "node", "ocaml", "perl", "php", "prolog", "python",
        "pytorch", "qiskit", "r", "redis", "rockstar", "ruby", "rust",
        "scheme", "smt2", "solidity", "sparql", "sql", "tcl", "tensorflow",
        "terraform", "toml", "verilog", "vhdl", "vyper", "xml", "yaml",
        "z3", "zsh",
        // Assembly variants
        "asm_aarch64", "asm_mips", "asm_riscv", "asm_wasm", "asm_x86_64",
    ];
    
    languages.into_iter().map(|lang| workflow_71_complete(lang)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_macro() {
        let wf = mkwf!(
            "test" =>
            nix_build!("const_71_test/rust", timeout: 60),
            perf_record!("const_71_test/rust" => "rust.perf.data", cmd: "nix", "build"),
        );
        
        assert_eq!(wf.name, "test");
        assert_eq!(wf.steps.len(), 2);
    }
    
    #[test]
    fn test_generate_all() {
        let workflows = generate_all_workflows();
        assert_eq!(workflows.len(), 71);
    }
}
