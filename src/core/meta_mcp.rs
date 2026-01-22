use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::process::Command;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MCPToolSpec {
    pub name: String,
    pub git_url: String,
    pub flake_ref: Option<String>,
    pub description: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCPEvalRequest {
    pub tool_name: String,
    pub method: String,
    pub args: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCPEvalResponse {
    pub success: bool,
    pub result: serde_json::Value,
    pub container_id: Option<String>,
}

pub struct MetaMCP {
    registry: HashMap<String, MCPToolSpec>,
    cache_dir: PathBuf,
    container_runtime: String, // "docker" or "podman"
}

impl MetaMCP {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            registry: HashMap::new(),
            cache_dir,
            container_runtime: "docker".to_string(),
        }
    }

    /// Register an MCP tool from git
    pub fn register(&mut self, spec: MCPToolSpec) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 Registering MCP tool: {}", spec.name);
        self.registry.insert(spec.name.clone(), spec);
        Ok(())
    }

    /// Download MCP tool from git
    pub fn download(&self, tool_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let spec = self.registry.get(tool_name)
            .ok_or(format!("Tool {} not found", tool_name))?;

        let tool_dir = self.cache_dir.join(&spec.name);
        
        if tool_dir.exists() {
            println!("✅ Tool already downloaded: {}", tool_name);
            return Ok(tool_dir);
        }

        println!("⬇️  Downloading {} from {}", tool_name, spec.git_url);
        
        Command::new("git")
            .arg("clone")
            .arg(&spec.git_url)
            .arg(&tool_dir)
            .status()?;

        Ok(tool_dir)
    }

    /// Build MCP tool with Nix
    pub fn build(&self, tool_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let tool_dir = self.download(tool_name)?;
        let spec = self.registry.get(tool_name).unwrap();

        println!("🔨 Building {} with Nix", tool_name);

        let flake_ref = spec.flake_ref.as_ref()
            .map(|f| format!("{}#{}", tool_dir.display(), f))
            .unwrap_or_else(|| tool_dir.display().to_string());

        let output = Command::new("nix")
            .arg("build")
            .arg(&flake_ref)
            .arg("--json")
            .arg("--no-link")
            .output()?;

        if !output.status.success() {
            return Err(format!("Nix build failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        // Parse nix build output to get store path
        let build_result: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        let store_path = build_result[0]["outputs"]["out"]
            .as_str()
            .ok_or("No output path")?;

        println!("✅ Built: {}", store_path);
        Ok(PathBuf::from(store_path))
    }

    /// Evaluate MCP tool in secure container
    pub async fn eval(&self, request: MCPEvalRequest) -> Result<MCPEvalResponse, Box<dyn std::error::Error>> {
        let store_path = self.build(&request.tool_name)?;
        
        println!("🐳 Running {} in secure container", request.tool_name);

        // Create container with minimal permissions
        let container_id = format!("mcp-{}-{}", request.tool_name, uuid::Uuid::new_v4());
        
        let args_json = serde_json::to_string(&request.args)?;
        
        let output = Command::new(&self.container_runtime)
            .arg("run")
            .arg("--rm")
            .arg("--name").arg(&container_id)
            .arg("--network=none") // No network access
            .arg("--read-only") // Read-only filesystem
            .arg("--security-opt=no-new-privileges")
            .arg("--cap-drop=ALL") // Drop all capabilities
            .arg("-v").arg(format!("{}:/mcp:ro", store_path.display()))
            .arg("nixos/nix")
            .arg("/mcp/bin/mcp-server")
            .arg("--method").arg(&request.method)
            .arg("--args").arg(&args_json)
            .output()?;

        let result = if output.status.success() {
            serde_json::from_slice(&output.stdout)?
        } else {
            serde_json::json!({
                "error": String::from_utf8_lossy(&output.stderr)
            })
        };

        Ok(MCPEvalResponse {
            success: output.status.success(),
            result,
            container_id: Some(container_id),
        })
    }

    /// List registered tools
    pub fn list(&self) -> Vec<&MCPToolSpec> {
        self.registry.values().collect()
    }

    /// Generate Nix flake for MCP tool
    pub fn generate_flake(&self, tool_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let spec = self.registry.get(tool_name)
            .ok_or(format!("Tool {} not found", tool_name))?;

        Ok(format!(r#"{{
  description = "MCP Tool: {}";

  inputs = {{
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  }};

  outputs = {{ self, nixpkgs }}: {{
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.stdenv.mkDerivation {{
      name = "{}";
      src = builtins.fetchGit {{
        url = "{}";
        rev = "{}";
      }};
      
      buildInputs = with nixpkgs.legacyPackages.x86_64-linux; [
        # Add dependencies here
      ];
      
      installPhase = ''
        mkdir -p $out/bin
        cp -r * $out/
      '';
    }};
  }};
}}
"#, spec.description, spec.name, spec.git_url, spec.version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut meta = MetaMCP::new(PathBuf::from("/tmp/mcp-cache"));
        let spec = MCPToolSpec {
            name: "test-tool".to_string(),
            git_url: "https://github.com/example/mcp-tool".to_string(),
            flake_ref: None,
            description: "Test MCP tool".to_string(),
            version: "v1.0.0".to_string(),
        };
        
        assert!(meta.register(spec).is_ok());
        assert_eq!(meta.list().len(), 1);
    }
}
