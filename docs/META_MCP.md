# Meta-MCP: Register, Download, Build, Eval MCP Tools

## Overview

Meta-MCP is a system that **registers, downloads, builds, and evaluates** MCP (Model Context Protocol) tools from git repositories. Each tool is packaged as a Nix flake, built in isolation, and evaluated in secure containers.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Meta-MCP                              │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Register │→ │ Download │→ │  Build   │→ │   Eval   │   │
│  │  (Git)   │  │  (Clone) │  │  (Nix)   │  │(Container)│   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│       ↓             ↓             ↓             ↓           │
│   Registry      /tmp/mcp     Nix Store    Docker/Podman    │
└─────────────────────────────────────────────────────────────┘
```

## Features

- **Register**: Add MCP tools from any git repository
- **Download**: Clone tool source code
- **Build**: Build with Nix for reproducibility
- **Eval**: Run in secure containers with minimal permissions
- **Auto-generate**: Create Nix flakes automatically

## Usage

### Register MCP Tools

```rust
use mcp::{MetaMCP, MCPToolSpec};
use std::path::PathBuf;

let mut meta_mcp = MetaMCP::new(PathBuf::from("/tmp/mcp-tools"));

meta_mcp.register(MCPToolSpec {
    name: "filesystem-mcp".to_string(),
    git_url: "https://github.com/modelcontextprotocol/servers".to_string(),
    flake_ref: Some("filesystem".to_string()),
    description: "MCP server for filesystem operations".to_string(),
    version: "main".to_string(),
})?;
```

### Download Tool

```rust
let tool_dir = meta_mcp.download("filesystem-mcp")?;
println!("Downloaded to: {}", tool_dir.display());
```

### Build with Nix

```rust
let store_path = meta_mcp.build("filesystem-mcp")?;
println!("Built: {}", store_path.display());
```

### Evaluate in Secure Container

```rust
use mcp::MCPEvalRequest;

let request = MCPEvalRequest {
    tool_name: "filesystem-mcp".to_string(),
    method: "list_files".to_string(),
    args: serde_json::json!({"path": "/tmp"}),
};

let response = meta_mcp.eval(request).await?;
println!("Result: {:?}", response.result);
```

## Security Model

All MCP tools run in **secure containers** with:

- ✅ **No network access** (`--network=none`)
- ✅ **Read-only filesystem** (`--read-only`)
- ✅ **No new privileges** (`--security-opt=no-new-privileges`)
- ✅ **All capabilities dropped** (`--cap-drop=ALL`)
- ✅ **Nix store mounted read-only** (`-v /nix/store/...:ro`)

## Auto-Generated Nix Flakes

Meta-MCP automatically generates Nix flakes for each tool:

```nix
{
  description = "MCP Tool: MCP server for filesystem operations";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.stdenv.mkDerivation {
      name = "filesystem-mcp";
      src = builtins.fetchGit { 
        url = "https://github.com/modelcontextprotocol/servers"; 
        rev = "main"; 
      };
      installPhase = "mkdir -p $out/bin && cp -r * $out/";
    };
  };
}
```

## Example MCP Tools

### Official MCP Servers

```rust
// Filesystem operations
meta_mcp.register(MCPToolSpec {
    name: "filesystem-mcp".to_string(),
    git_url: "https://github.com/modelcontextprotocol/servers".to_string(),
    flake_ref: Some("filesystem".to_string()),
    description: "MCP server for filesystem operations".to_string(),
    version: "main".to_string(),
})?;

// Git operations
meta_mcp.register(MCPToolSpec {
    name: "git-mcp".to_string(),
    git_url: "https://github.com/modelcontextprotocol/servers".to_string(),
    flake_ref: Some("git".to_string()),
    description: "MCP server for git operations".to_string(),
    version: "main".to_string(),
})?;

// PostgreSQL
meta_mcp.register(MCPToolSpec {
    name: "postgres-mcp".to_string(),
    git_url: "https://github.com/modelcontextprotocol/servers".to_string(),
    flake_ref: Some("postgres".to_string()),
    description: "MCP server for PostgreSQL".to_string(),
    version: "main".to_string(),
})?;
```

### Custom MCP Tools

```rust
// Your custom MCP tool
meta_mcp.register(MCPToolSpec {
    name: "my-custom-mcp".to_string(),
    git_url: "https://github.com/myorg/my-mcp-tool".to_string(),
    flake_ref: None, // Use default flake
    description: "My custom MCP tool".to_string(),
    version: "v1.0.0".to_string(),
})?;
```

## Demo

```bash
cargo build --bin demo_meta_mcp --release
./target/release/demo_meta_mcp
```

Output:
```
🚀 Meta-MCP Demo: Register, Download, Build, Eval

📝 Registering MCP tools...

📋 Registered MCP Tools:
  • filesystem-mcp - MCP server for filesystem operations
  • git-mcp - MCP server for git operations
  • postgres-mcp - MCP server for PostgreSQL

📦 Generated Nix Flake for filesystem-mcp:
{
  description = "MCP Tool: MCP server for filesystem operations";
  ...
}

🎉 Meta-MCP Demo Complete!

Next steps:
  1. Download: meta_mcp.download("tool-name")
  2. Build: meta_mcp.build("tool-name")
  3. Eval: meta_mcp.eval(request).await

🔒 All evaluations run in secure containers with:
  • No network access
  • Read-only filesystem
  • No new privileges
  • All capabilities dropped
```

## Integration with Minimal Build Server

```rust
// In minimal_build_server.rs
use mcp::{MetaMCP, MCPToolSpec, MCPEvalRequest};

async fn mcp_register(Json(spec): Json<MCPToolSpec>) -> Json<serde_json::Value> {
    let mut meta_mcp = get_meta_mcp();
    match meta_mcp.register(spec) {
        Ok(_) => json!({"success": true}),
        Err(e) => json!({"success": false, "error": e.to_string()}),
    }
}

async fn mcp_eval(Json(req): Json<MCPEvalRequest>) -> Json<serde_json::Value> {
    let meta_mcp = get_meta_mcp();
    match meta_mcp.eval(req).await {
        Ok(response) => json!(response),
        Err(e) => json!({"success": false, "error": e.to_string()}),
    }
}

// Add routes
.route("/mcp/register", post(mcp_register))
.route("/mcp/eval", post(mcp_eval))
```

## API Endpoints

### Register Tool

```bash
curl -X POST http://localhost:3000/mcp/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "filesystem-mcp",
    "git_url": "https://github.com/modelcontextprotocol/servers",
    "flake_ref": "filesystem",
    "description": "MCP server for filesystem operations",
    "version": "main"
  }'
```

### Evaluate Tool

```bash
curl -X POST http://localhost:3000/mcp/eval \
  -H "Content-Type: application/json" \
  -d '{
    "tool_name": "filesystem-mcp",
    "method": "list_files",
    "args": {"path": "/tmp"}
  }'
```

## Benefits

1. **Reproducible**: Nix ensures identical builds
2. **Secure**: Containers isolate execution
3. **Discoverable**: Registry of all available tools
4. **Composable**: Chain MCP tools together
5. **Versioned**: Pin specific git revisions
6. **Auditable**: All builds tracked in Nix store

## Future Enhancements

- [ ] MCP tool marketplace
- [ ] Automatic dependency resolution
- [ ] Tool composition (pipe outputs)
- [ ] Caching layer for builds
- [ ] WebAssembly support
- [ ] Distributed evaluation
- [ ] Tool verification/signing

---

**Status:** Phase 1 Complete  
**Last Updated:** 2026-01-17
