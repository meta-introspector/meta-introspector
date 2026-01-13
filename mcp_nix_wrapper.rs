// 🌟 MCP NIX WRAPPER: Model Context Protocol integration for loaded nix flake libraries
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use libloading::{Library, Symbol};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub content_address: String,
    pub library_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
    pub content_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPCallRequest {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPCallResponse {
    pub content: Vec<MCPContent>,
    pub is_error: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPContent {
    pub r#type: String,
    pub text: Option<String>,
    pub data: Option<String>,
    pub mime_type: Option<String>,
}

pub struct MCPNixWrapper {
    pub content_address: String,
    pub libraries: HashMap<String, Arc<Library>>,
    pub available_tools: Vec<MCPTool>,
    pub available_resources: Vec<MCPResource>,
}

impl MCPNixWrapper {
    pub fn new(content_address: String, libraries: HashMap<String, Arc<Library>>) -> Self {
        let mut wrapper = Self {
            content_address: content_address.clone(),
            libraries,
            available_tools: Vec::new(),
            available_resources: Vec::new(),
        };
        
        wrapper.discover_mcp_capabilities();
        wrapper
    }

    fn discover_mcp_capabilities(&mut self) {
        for (lib_name, library) in &self.libraries {
            // Discover tools
            self.discover_tools(lib_name, library);
            
            // Discover resources
            self.discover_resources(lib_name, library);
        }
    }

    fn discover_tools(&mut self, lib_name: &str, library: &Library) {
        // Common tool discovery patterns
        let tool_patterns = [
            ("execute", "Execute code or commands"),
            ("analyze", "Analyze data or code"),
            ("transform", "Transform data between formats"),
            ("validate", "Validate input data"),
            ("generate", "Generate new content"),
            ("search", "Search through data"),
            ("compile", "Compile source code"),
            ("parse", "Parse structured data"),
        ];

        for (tool_name, description) in &tool_patterns {
            let symbol_name = format!("mcp_tool_{}", tool_name);
            
            // Try to find the symbol in the library
            if let Ok(_symbol) = unsafe { library.get::<fn()>(symbol_name.as_bytes()) } {
                let tool = MCPTool {
                    name: format!("{}_{}", lib_name, tool_name),
                    description: format!("{} (from {})", description, lib_name),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "input": {
                                "type": "string",
                                "description": "Input data for the tool"
                            }
                        },
                        "required": ["input"]
                    }),
                    content_address: self.content_address.clone(),
                    library_name: lib_name.to_string(),
                };
                
                self.available_tools.push(tool);
            }
        }
    }

    fn discover_resources(&mut self, lib_name: &str, library: &Library) {
        // Common resource patterns
        let resource_patterns = [
            ("config", "application/json", "Configuration data"),
            ("schema", "application/json", "Data schema definitions"),
            ("docs", "text/markdown", "Documentation"),
            ("examples", "text/plain", "Usage examples"),
        ];

        for (resource_type, mime_type, description) in &resource_patterns {
            let symbol_name = format!("mcp_resource_{}", resource_type);
            
            if let Ok(_symbol) = unsafe { library.get::<fn()>(symbol_name.as_bytes()) } {
                let resource = MCPResource {
                    uri: format!("nix://{}/{}/{}", self.content_address, lib_name, resource_type),
                    name: format!("{} {}", lib_name, resource_type),
                    description: format!("{} from {}", description, lib_name),
                    mime_type: mime_type.to_string(),
                    content_address: self.content_address.clone(),
                };
                
                self.available_resources.push(resource);
            }
        }
    }

    pub fn list_tools(&self) -> Vec<MCPTool> {
        self.available_tools.clone()
    }

    pub fn list_resources(&self) -> Vec<MCPResource> {
        self.available_resources.clone()
    }

    pub fn call_tool(&self, request: MCPCallRequest) -> Result<MCPCallResponse, Box<dyn std::error::Error>> {
        // Find the tool
        let tool = self.available_tools.iter()
            .find(|t| t.name == request.name)
            .ok_or("Tool not found")?;

        // Get the library
        let library = self.libraries.get(&tool.library_name)
            .ok_or("Library not loaded")?;

        // Extract tool function name from tool name
        let tool_func = tool.name.split('_').last().unwrap_or("execute");
        let symbol_name = format!("mcp_tool_{}", tool_func);

        // Call the tool function via FFI
        match self.call_library_function(library, &symbol_name, &request.arguments) {
            Ok(result) => Ok(MCPCallResponse {
                content: vec![MCPContent {
                    r#type: "text".to_string(),
                    text: Some(result),
                    data: None,
                    mime_type: Some("text/plain".to_string()),
                }],
                is_error: false,
            }),
            Err(e) => Ok(MCPCallResponse {
                content: vec![MCPContent {
                    r#type: "text".to_string(),
                    text: Some(format!("Error: {}", e)),
                    data: None,
                    mime_type: Some("text/plain".to_string()),
                }],
                is_error: true,
            }),
        }
    }

    pub fn read_resource(&self, uri: &str) -> Result<MCPContent, Box<dyn std::error::Error>> {
        // Find the resource
        let resource = self.available_resources.iter()
            .find(|r| r.uri == uri)
            .ok_or("Resource not found")?;

        // Parse URI to extract library and resource type
        let uri_parts: Vec<&str> = uri.split('/').collect();
        if uri_parts.len() < 4 {
            return Err("Invalid resource URI".into());
        }

        let lib_name = uri_parts[2];
        let resource_type = uri_parts[3];

        // Get the library
        let library = self.libraries.get(lib_name)
            .ok_or("Library not loaded")?;

        let symbol_name = format!("mcp_resource_{}", resource_type);

        // Call the resource function
        match self.call_library_function(library, &symbol_name, &HashMap::new()) {
            Ok(content) => Ok(MCPContent {
                r#type: "text".to_string(),
                text: Some(content),
                data: None,
                mime_type: Some(resource.mime_type.clone()),
            }),
            Err(e) => Err(format!("Failed to read resource: {}", e).into()),
        }
    }

    fn call_library_function(
        &self,
        library: &Library,
        symbol_name: &str,
        args: &HashMap<String, serde_json::Value>
    ) -> Result<String, Box<dyn std::error::Error>> {
        // This is a simplified FFI call - real implementation would need proper type handling
        
        // Try different function signatures
        
        // 1. Try simple string function: char* func(char* input)
        if let Ok(func) = unsafe { library.get::<unsafe extern "C" fn(*const c_char) -> *const c_char>(symbol_name.as_bytes()) } {
            let input = args.get("input")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let input_cstr = CString::new(input)?;
            let result_ptr = unsafe { func(input_cstr.as_ptr()) };
            
            if !result_ptr.is_null() {
                let result_cstr = unsafe { CStr::from_ptr(result_ptr) };
                return Ok(result_cstr.to_string_lossy().to_string());
            }
        }

        // 2. Try JSON function: char* func(char* json_input)
        if let Ok(func) = unsafe { library.get::<unsafe extern "C" fn(*const c_char) -> *const c_char>(symbol_name.as_bytes()) } {
            let json_input = serde_json::to_string(args)?;
            let input_cstr = CString::new(json_input)?;
            let result_ptr = unsafe { func(input_cstr.as_ptr()) };
            
            if !result_ptr.is_null() {
                let result_cstr = unsafe { CStr::from_ptr(result_ptr) };
                return Ok(result_cstr.to_string_lossy().to_string());
            }
        }

        // 3. Try no-args function: char* func()
        if let Ok(func) = unsafe { library.get::<unsafe extern "C" fn() -> *const c_char>(symbol_name.as_bytes()) } {
            let result_ptr = unsafe { func() };
            
            if !result_ptr.is_null() {
                let result_cstr = unsafe { CStr::from_ptr(result_ptr) };
                return Ok(result_cstr.to_string_lossy().to_string());
            }
        }

        Err(format!("Function {} not found or incompatible signature", symbol_name).into())
    }

    pub fn get_mcp_server_info(&self) -> serde_json::Value {
        serde_json::json!({
            "name": format!("nix-flake-{}", self.content_address),
            "version": "1.0.0",
            "description": format!("MCP server for nix flake with content address {}", self.content_address),
            "capabilities": {
                "tools": {
                    "listChanged": false
                },
                "resources": {
                    "subscribe": false,
                    "listChanged": false
                }
            },
            "content_address": self.content_address,
            "loaded_libraries": self.libraries.keys().collect::<Vec<_>>()
        })
    }
}

// MCP Protocol handlers
pub fn handle_mcp_initialize(wrapper: &MCPNixWrapper) -> serde_json::Value {
    wrapper.get_mcp_server_info()
}

pub fn handle_mcp_list_tools(wrapper: &MCPNixWrapper) -> serde_json::Value {
    let tools: Vec<serde_json::Value> = wrapper.list_tools()
        .into_iter()
        .map(|tool| serde_json::json!({
            "name": tool.name,
            "description": tool.description,
            "inputSchema": tool.input_schema
        }))
        .collect();

    serde_json::json!({
        "tools": tools
    })
}

pub fn handle_mcp_list_resources(wrapper: &MCPNixWrapper) -> serde_json::Value {
    let resources: Vec<serde_json::Value> = wrapper.list_resources()
        .into_iter()
        .map(|resource| serde_json::json!({
            "uri": resource.uri,
            "name": resource.name,
            "description": resource.description,
            "mimeType": resource.mime_type
        }))
        .collect();

    serde_json::json!({
        "resources": resources
    })
}

pub fn handle_mcp_call_tool(wrapper: &MCPNixWrapper, request: MCPCallRequest) -> serde_json::Value {
    match wrapper.call_tool(request) {
        Ok(response) => serde_json::json!({
            "content": response.content,
            "isError": response.is_error
        }),
        Err(e) => serde_json::json!({
            "content": [{
                "type": "text",
                "text": format!("Error calling tool: {}", e)
            }],
            "isError": true
        })
    }
}

pub fn handle_mcp_read_resource(wrapper: &MCPNixWrapper, uri: &str) -> serde_json::Value {
    match wrapper.read_resource(uri) {
        Ok(content) => serde_json::json!({
            "contents": [content]
        }),
        Err(e) => serde_json::json!({
            "error": format!("Error reading resource: {}", e)
        })
    }
}
