// Nix Workflow Scheduler: Composable tasks with canonical I/O
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixTask {
    pub name: String,
    pub executable: String,
    pub inputs: Vec<String>,      // Nix store paths
    pub outputs: Vec<String>,     // Output names
    pub env: HashMap<String, String>,
    pub pure: bool,               // Pure or impure build
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_name: String,
    pub nix_store_path: String,
    pub outputs: HashMap<String, String>,
    pub duration_ms: u64,
    pub success: bool,
}

pub struct NixWorkflowScheduler {
    pub tasks: Vec<NixTask>,
    pub results: Vec<TaskResult>,
    pub nix_daemon: bool,
}

impl NixWorkflowScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            results: Vec::new(),
            nix_daemon: true,
        }
    }
    
    pub fn add_task(&mut self, task: NixTask) {
        self.tasks.push(task);
    }
    
    pub async fn run_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Running {} tasks with nix daemon", self.tasks.len());
        
        for task in &self.tasks {
            let result = self.run_task(task).await?;
            self.results.push(result);
        }
        
        Ok(())
    }
    
    async fn run_task(&self, task: &NixTask) -> Result<TaskResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        // Build nix derivation for this task
        let drv = self.build_derivation(task)?;
        
        // Run with nix-build
        let output = if task.pure {
            self.nix_build_pure(&drv)?
        } else {
            self.nix_build_impure(&drv)?
        };
        
        let duration = start.elapsed().as_millis() as u64;
        
        // Parse outputs from nix store
        let outputs = self.parse_outputs(&output)?;
        
        Ok(TaskResult {
            task_name: task.name.clone(),
            nix_store_path: output,
            outputs,
            duration_ms: duration,
            success: true,
        })
    }
    
    fn build_derivation(&self, task: &NixTask) -> Result<String, Box<dyn std::error::Error>> {
        // Generate nix derivation
        let drv = format!(r#"
{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation {{
  name = "{name}";
  
  {impure}
  
  # Canonical inputs from nix store
  inputs = [ {inputs} ];
  
  buildInputs = [ pkgs.jq pkgs.parquet-tools ];
  
  # Environment variables
  {env_vars}
  
  buildPhase = ''
    echo "Running task: {name}"
    
    # Read inputs from nix store
    for input in $inputs; do
      echo "Input: $input"
    done
    
    # Run executable with canonical I/O
    {executable} \
      --inputs "$inputs" \
      --output $out \
      {args}
  '';
  
  installPhase = ''
    # Outputs are already in $out
    echo "Task complete: {name}"
    echo "Output: $out"
  '';
}}
"#,
            name = task.name,
            impure = if task.pure { "" } else { "__impure = true;" },
            inputs = task.inputs.join(" "),
            env_vars = task.env.iter()
                .map(|(k, v)| format!("{} = \"{}\";", k, v))
                .collect::<Vec<_>>()
                .join("\n  "),
            executable = task.executable,
            args = task.outputs.iter()
                .map(|o| format!("--output-{} $out/{}", o, o))
                .collect::<Vec<_>>()
                .join(" \\\n      "),
        );
        
        // Write derivation to temp file
        let drv_path = format!("/tmp/nix-task-{}.nix", task.name);
        std::fs::write(&drv_path, drv)?;
        
        Ok(drv_path)
    }
    
    fn nix_build_pure(&self, drv_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("nix-build")
            .arg(drv_path)
            .arg("--no-out-link")
            .output()?;
        
        if !output.status.success() {
            return Err(format!("nix-build failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn nix_build_impure(&self, drv_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("nix-build")
            .arg(drv_path)
            .arg("--impure")
            .arg("--no-out-link")
            .output()?;
        
        if !output.status.success() {
            return Err(format!("nix-build failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn parse_outputs(&self, store_path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut outputs = HashMap::new();
        
        // List all files in nix store output
        let output = Command::new("ls")
            .arg("-1")
            .arg(store_path)
            .output()?;
        
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let file_path = format!("{}/{}", store_path, line);
            outputs.insert(line.to_string(), file_path);
        }
        
        Ok(outputs)
    }
    
    pub fn compose_tasks(&self, task_names: &[String]) -> Result<NixTask, Box<dyn std::error::Error>> {
        // Compose multiple tasks into one
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        
        for name in task_names {
            if let Some(result) = self.results.iter().find(|r| &r.task_name == name) {
                inputs.push(result.nix_store_path.clone());
                outputs.extend(result.outputs.keys().cloned());
            }
        }
        
        Ok(NixTask {
            name: format!("composed-{}", task_names.join("-")),
            executable: "compose-outputs".to_string(),
            inputs,
            outputs,
            env: HashMap::new(),
            pure: true,
        })
    }
    
    pub fn export_as_flake(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Export workflow as importable flake
        let flake = format!(r#"
{{
  description = "Workflow with {} tasks";
  
  outputs = {{ self }}: {{
    tasks = [
      {}
    ];
    
    results = [
      {}
    ];
  }};
}}
"#,
            self.tasks.len(),
            self.tasks.iter()
                .map(|t| format!("{{ name = \"{}\"; path = \"{}\"; }}", t.name, t.executable))
                .collect::<Vec<_>>()
                .join("\n      "),
            self.results.iter()
                .map(|r| format!("{{ task = \"{}\"; store = \"{}\"; }}", r.task_name, r.nix_store_path))
                .collect::<Vec<_>>()
                .join("\n      "),
        );
        
        std::fs::write(output_path, flake)?;
        println!("✅ Exported workflow to {}", output_path);
        
        Ok(())
    }
    
    pub fn import_workflow(&mut self, flake_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Import workflow from flake
        let output = Command::new("nix")
            .arg("eval")
            .arg("--json")
            .arg(format!("{}#tasks", flake_path))
            .output()?;
        
        let tasks: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        
        for task_json in tasks {
            // Parse and add task
            println!("Imported task: {}", task_json["name"]);
        }
        
        Ok(())
    }
    
    pub fn munge_results(&self, result_names: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        // Combine multiple results into one nix store path
        let mut combined_inputs = Vec::new();
        
        for name in result_names {
            if let Some(result) = self.results.iter().find(|r| &r.task_name == name) {
                combined_inputs.push(result.nix_store_path.clone());
            }
        }
        
        // Build derivation that combines all inputs
        let drv = format!(r#"
{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation {{
  name = "munged-results";
  
  inputs = [ {} ];
  
  buildPhase = ''
    mkdir -p $out
    
    # Combine all inputs
    for input in $inputs; do
      cp -r $input/* $out/
    done
    
    # Create manifest
    cat > $out/manifest.json << EOF
    {{
      "sources": [{}],
      "timestamp": "$(date -Iseconds)"
    }}
    EOF
  '';
}}
"#,
            combined_inputs.join(" "),
            combined_inputs.iter()
                .map(|p| format!("\"{}\"", p))
                .collect::<Vec<_>>()
                .join(", "),
        );
        
        let drv_path = "/tmp/munge-results.nix";
        std::fs::write(drv_path, drv)?;
        
        let output = Command::new("nix-build")
            .arg(drv_path)
            .arg("--no-out-link")
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

// Example task executables with canonical I/O
pub fn create_mining_task(name: &str, inputs: Vec<String>) -> NixTask {
    NixTask {
        name: name.to_string(),
        executable: format!("demo_{}", name),
        inputs,
        outputs: vec!["results.parquet".to_string(), "telemetry.json".to_string()],
        env: HashMap::new(),
        pure: false, // Mining tasks are impure (network, LLM)
    }
}

pub fn create_analysis_task(name: &str, mining_results: Vec<String>) -> NixTask {
    NixTask {
        name: name.to_string(),
        executable: "demo_universal_llm_proxy".to_string(),
        inputs: mining_results,
        outputs: vec!["analysis.parquet".to_string(), "llm-telemetry.json".to_string()],
        env: [("GEMINI_API_KEY".to_string(), std::env::var("GEMINI_API_KEY").unwrap_or_default())]
            .iter().cloned().collect(),
        pure: false, // LLM queries are impure
    }
}

pub fn create_export_task(name: &str, analysis_results: Vec<String>) -> NixTask {
    NixTask {
        name: name.to_string(),
        executable: "export-to-huggingface".to_string(),
        inputs: analysis_results,
        outputs: vec!["dataset-card.md".to_string(), "upload-log.txt".to_string()],
        env: HashMap::new(),
        pure: true, // Export is deterministic
    }
}
