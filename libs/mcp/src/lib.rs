use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub mod meta_mcp;
pub use meta_mcp::{MetaMCP, MCPToolSpec, MCPEvalRequest, MCPEvalResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub library: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCPRequest {
    pub tool: String,
    pub args: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCPResponse {
    pub success: bool,
    pub result: serde_json::Value,
}

pub trait MCPProvider {
    fn discover_tools(&self) -> Result<Vec<MCPTool>, Box<dyn std::error::Error>>;
    fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}

pub struct MCPRegistry {
    tools: HashMap<String, MCPTool>,
}

impl MCPRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: MCPTool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&MCPTool> {
        self.tools.get(name)
    }

    pub fn list(&self) -> Vec<&MCPTool> {
        self.tools.values().collect()
    }
}

#[no_mangle]
pub extern "C" fn mcp_discover_tools() -> *mut std::os::raw::c_char {
    let tools = vec![
        MCPTool {
            name: "nix_build".to_string(),
            description: "Build Nix flake".to_string(),
            library: "libnix.so".to_string(),
            symbol: "nix_build".to_string(),
        },
    ];
    let json = serde_json::to_string(&tools).unwrap();
    std::ffi::CString::new(json).unwrap().into_raw()
}
