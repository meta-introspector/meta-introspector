use crate::{LibraryInfo, SymbolInfo, SymbolType};
use anyhow::Result;
use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tracing::{info, warn};

#[rpc]
pub trait LibraryRpc {
    /// Call a function in the wrapped library
    #[rpc(name = "call_function")]
    fn call_function(&self, function_name: String, args: Vec<Value>) -> Result<Value, jsonrpc_core::Error>;
    
    /// Get library information
    #[rpc(name = "get_info")]
    fn get_info(&self) -> Result<Value, jsonrpc_core::Error>;
    
    /// List all available functions
    #[rpc(name = "list_functions")]
    fn list_functions(&self) -> Result<Vec<String>, jsonrpc_core::Error>;
    
    /// Get function signature
    #[rpc(name = "get_signature")]
    fn get_signature(&self, function_name: String) -> Result<Value, jsonrpc_core::Error>;
}

pub struct LibraryService {
    library_info: LibraryInfo,
    function_map: HashMap<String, SymbolInfo>,
}

impl LibraryService {
    pub fn new(library_info: LibraryInfo) -> Self {
        let mut function_map = HashMap::new();
        
        for symbol in &library_info.symbols {
            if matches!(symbol.symbol_type, SymbolType::Function) {
                function_map.insert(symbol.name.clone(), symbol.clone());
                
                // Also index by demangled name if available
                if let Some(demangled) = &symbol.demangled_name {
                    function_map.insert(demangled.clone(), symbol.clone());
                }
            }
        }
        
        Self {
            library_info,
            function_map,
        }
    }
}

impl LibraryRpc for LibraryService {
    fn call_function(&self, function_name: String, args: Vec<Value>) -> Result<Value, jsonrpc_core::Error> {
        // This is where the magic happens - dynamic function calling
        if let Some(symbol) = self.function_map.get(&function_name) {
            info!("🚀 Calling function: {} at 0x{:x}", function_name, symbol.address);
            
            // TODO: Implement actual dynamic function calling using libffi or similar
            // For now, return a mock response
            Ok(json!({
                "function": function_name,
                "address": format!("0x{:x}", symbol.address),
                "args": args,
                "result": "mock_result",
                "status": "success"
            }))
        } else {
            Err(jsonrpc_core::Error::invalid_params(format!(
                "Function '{}' not found in library '{}'", 
                function_name, 
                self.library_info.name
            )))
        }
    }
    
    fn get_info(&self) -> Result<Value, jsonrpc_core::Error> {
        Ok(json!({
            "name": self.library_info.name,
            "path": self.library_info.path,
            "size": self.library_info.size,
            "architecture": self.library_info.architecture,
            "abi_hash": self.library_info.abi_hash,
            "symbol_count": self.library_info.symbols.len(),
            "dependencies": self.library_info.dependencies,
        }))
    }
    
    fn list_functions(&self) -> Result<Vec<String>, jsonrpc_core::Error> {
        let functions: Vec<String> = self.function_map.keys().cloned().collect();
        Ok(functions)
    }
    
    fn get_signature(&self, function_name: String) -> Result<Value, jsonrpc_core::Error> {
        if let Some(symbol) = self.function_map.get(&function_name) {
            Ok(json!({
                "name": symbol.name,
                "demangled_name": symbol.demangled_name,
                "address": format!("0x{:x}", symbol.address),
                "size": symbol.size,
                "signature": symbol.signature,
            }))
        } else {
            Err(jsonrpc_core::Error::invalid_params(format!(
                "Function '{}' not found", function_name
            )))
        }
    }
}

pub async fn generate_mcp_service(library_info: &LibraryInfo, output_dir: &Path) -> Result<()> {
    let service_name = sanitize_service_name(&library_info.name);
    let service_dir = output_dir.join(&service_name);
    
    fs::create_dir_all(&service_dir).await?;
    
    // Generate MCP service manifest
    let manifest = json!({
        "name": service_name,
        "version": "1.0.0",
        "description": format!("MCP service for {}", library_info.name),
        "library": {
            "path": library_info.path,
            "abi_hash": library_info.abi_hash,
        },
        "capabilities": {
            "functions": library_info.symbols.iter()
                .filter(|s| matches!(s.symbol_type, SymbolType::Function))
                .map(|s| json!({
                    "name": s.name,
                    "demangled_name": s.demangled_name,
                    "signature": s.signature,
                }))
                .collect::<Vec<_>>(),
        },
        "endpoints": {
            "call_function": {
                "description": "Call a function in the library",
                "parameters": {
                    "function_name": "string",
                    "args": "array"
                }
            },
            "get_info": {
                "description": "Get library information"
            },
            "list_functions": {
                "description": "List all available functions"
            },
            "get_signature": {
                "description": "Get function signature",
                "parameters": {
                    "function_name": "string"
                }
            }
        }
    });
    
    let manifest_path = service_dir.join("manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?).await?;
    
    // Generate TypeScript/JavaScript client
    let client_code = generate_client_code(&service_name, library_info);
    let client_path = service_dir.join("client.ts");
    fs::write(&client_path, client_code).await?;
    
    // Generate Python client
    let python_client = generate_python_client(&service_name, library_info);
    let python_path = service_dir.join("client.py");
    fs::write(&python_path, python_client).await?;
    
    // Generate documentation
    let docs = generate_documentation(&service_name, library_info);
    let docs_path = service_dir.join("README.md");
    fs::write(&docs_path, docs).await?;
    
    info!("✅ Generated MCP service: {}", service_name);
    Ok(())
}

pub async fn start_mcp_server(port: u16, services_dir: &Path) -> Result<()> {
    let mut io = IoHandler::new();
    let mut services = HashMap::new();
    
    // Load all MCP services
    let mut entries = fs::read_dir(services_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let manifest_path = entry.path().join("manifest.json");
            if manifest_path.exists() {
                match load_service_from_manifest(&manifest_path).await {
                    Ok(service) => {
                        let service_name = entry.file_name().to_string_lossy().to_string();
                        info!("📦 Loaded MCP service: {}", service_name);
                        services.insert(service_name.clone(), Arc::new(service));
                    }
                    Err(e) => {
                        warn!("Failed to load service from {:?}: {}", manifest_path, e);
                    }
                }
            }
        }
    }
    
    // Register all services with the JSON-RPC handler
    for (service_name, service) in services {
        let service_clone = service.clone();
        
        // Register methods with service prefix
        io.add_method(&format!("{}.call_function", service_name), move |params: Params| {
            let service = service_clone.clone();
            async move {
                let params = params.parse::<(String, Vec<Value>)>()?;
                service.call_function(params.0, params.1)
            }
        });
        
        let service_clone = service.clone();
        io.add_method(&format!("{}.get_info", service_name), move |_: Params| {
            let service = service_clone.clone();
            async move { service.get_info() }
        });
        
        let service_clone = service.clone();
        io.add_method(&format!("{}.list_functions", service_name), move |_: Params| {
            let service = service_clone.clone();
            async move { service.list_functions() }
        });
        
        let service_clone = service.clone();
        io.add_method(&format!("{}.get_signature", service_name), move |params: Params| {
            let service = service_clone.clone();
            async move {
                let function_name = params.parse::<String>()?;
                service.get_signature(function_name)
            }
        });
    }
    
    // Add meta endpoints
    io.add_method("list_services", move |_: Params| {
        let service_names: Vec<String> = services.keys().cloned().collect();
        async move { Ok(Value::Array(service_names.into_iter().map(Value::String).collect())) }
    });
    
    info!("🌐 Starting MCP server on http://localhost:{}", port);
    
    let server = ServerBuilder::new(io)
        .cors(jsonrpc_http_server::cors::DomainsValidation::AllowOnly(vec![
            jsonrpc_http_server::cors::AccessControlAllowOrigin::Any,
        ]))
        .start_http(&format!("127.0.0.1:{}", port).parse()?)?;
    
    info!("✅ MCP server running! Try:");
    info!("   curl -X POST http://localhost:{} -H 'Content-Type: application/json' -d '{{\"jsonrpc\":\"2.0\",\"method\":\"list_services\",\"id\":1}}'", port);
    
    server.wait();
    Ok(())
}

async fn load_service_from_manifest(manifest_path: &Path) -> Result<LibraryService> {
    let manifest_content = fs::read_to_string(manifest_path).await?;
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content)?;
    
    let library_path = manifest["library"]["path"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing library path in manifest"))?;
    
    // Re-extract library info (in production, this could be cached)
    let library_info = crate::abi_extractor::extract_abi(Path::new(library_path)).await?;
    
    Ok(LibraryService::new(library_info))
}

fn sanitize_service_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_lowercase()
}

fn generate_client_code(service_name: &str, library_info: &LibraryInfo) -> String {
    format!(r#"
// Generated MCP client for {}
export class {}Client {{
    private baseUrl: string;
    
    constructor(baseUrl: string = 'http://localhost:8080') {{
        this.baseUrl = baseUrl;
    }}
    
    async callFunction(functionName: string, args: any[] = []): Promise<any> {{
        return this.rpcCall('{}.call_function', [functionName, args]);
    }}
    
    async getInfo(): Promise<any> {{
        return this.rpcCall('{}.get_info', []);
    }}
    
    async listFunctions(): Promise<string[]> {{
        return this.rpcCall('{}.list_functions', []);
    }}
    
    async getSignature(functionName: string): Promise<any> {{
        return this.rpcCall('{}.get_signature', [functionName]);
    }}
    
    private async rpcCall(method: string, params: any[]): Promise<any> {{
        const response = await fetch(this.baseUrl, {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                jsonrpc: '2.0',
                method,
                params,
                id: Date.now()
            }})
        }});
        
        const result = await response.json();
        if (result.error) {{
            throw new Error(result.error.message);
        }}
        return result.result;
    }}
}}
"#, 
        library_info.name,
        service_name.to_uppercase(),
        service_name,
        service_name,
        service_name,
        service_name
    )
}

fn generate_python_client(service_name: &str, library_info: &LibraryInfo) -> String {
    format!(r#"
"""Generated MCP client for {}"""
import requests
import json
from typing import List, Any, Dict

class {}Client:
    def __init__(self, base_url: str = 'http://localhost:8080'):
        self.base_url = base_url
    
    def call_function(self, function_name: str, args: List[Any] = None) -> Any:
        """Call a function in the library"""
        if args is None:
            args = []
        return self._rpc_call('{}.call_function', [function_name, args])
    
    def get_info(self) -> Dict[str, Any]:
        """Get library information"""
        return self._rpc_call('{}.get_info', [])
    
    def list_functions(self) -> List[str]:
        """List all available functions"""
        return self._rpc_call('{}.list_functions', [])
    
    def get_signature(self, function_name: str) -> Dict[str, Any]:
        """Get function signature"""
        return self._rpc_call('{}.get_signature', [function_name])
    
    def _rpc_call(self, method: str, params: List[Any]) -> Any:
        payload = {{
            'jsonrpc': '2.0',
            'method': method,
            'params': params,
            'id': 1
        }}
        
        response = requests.post(
            self.base_url,
            headers={{'Content-Type': 'application/json'}},
            data=json.dumps(payload)
        )
        
        result = response.json()
        if 'error' in result:
            raise Exception(result['error']['message'])
        
        return result['result']
"#,
        library_info.name,
        service_name.upper(),
        service_name,
        service_name,
        service_name,
        service_name
    )
}

fn generate_documentation(service_name: &str, library_info: &LibraryInfo) -> String {
    let functions = library_info.symbols.iter()
        .filter(|s| matches!(s.symbol_type, SymbolType::Function))
        .take(20) // Limit for readability
        .map(|s| format!("- `{}` ({})", s.name, s.demangled_name.as_deref().unwrap_or("no demangling")))
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(r#"
# {} MCP Service

Auto-generated MCP (Model Context Protocol) service for the `{}` library.

## Library Information

- **Path**: `{}`
- **Size**: {} bytes
- **Architecture**: {}
- **ABI Hash**: `{}`
- **Dependencies**: {}

## Available Functions

{}

## Usage

### TypeScript/JavaScript
```typescript
import {{ {}Client }} from './client';

const client = new {}Client();
const info = await client.getInfo();
const functions = await client.listFunctions();
const result = await client.callFunction('function_name', [arg1, arg2]);
```

### Python
```python
from client import {}Client

client = {}Client()
info = client.get_info()
functions = client.list_functions()
result = client.call_function('function_name', [arg1, arg2])
```

### Direct JSON-RPC
```bash
curl -X POST http://localhost:8080 \\
  -H 'Content-Type: application/json' \\
  -d '{{
    "jsonrpc": "2.0",
    "method": "{}.call_function",
    "params": ["function_name", [arg1, arg2]],
    "id": 1
  }}'
```

## API Methods

- `{}.call_function(function_name, args)` - Call a library function
- `{}.get_info()` - Get library metadata
- `{}.list_functions()` - List all available functions  
- `{}.get_signature(function_name)` - Get function signature details

## Generated by nixso2wrap

This service was automatically generated by `nixso2wrap` - the Nix store shared library wrapper and MCP service generator.
"#,
        service_name.to_uppercase(),
        library_info.name,
        library_info.path.display(),
        library_info.size,
        library_info.architecture,
        library_info.abi_hash,
        library_info.dependencies.join(", "),
        functions,
        service_name.to_uppercase(),
        service_name.to_uppercase(),
        service_name.to_uppercase(),
        service_name.to_uppercase(),
        service_name,
        service_name,
        service_name,
        service_name,
        service_name
    )
}
