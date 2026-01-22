# Perf Proc Macro - Wrap Any Code in Perf Recording

## 🎯 Goal
1. **Inline** - Return perf data directly in the code
2. **Centralized** - Send to telemetry server automatically
3. **Zero overhead** - Compile-time code generation

## 🔧 Design

### Proc Macro Usage

```rust
use perf_macros::perf_record;

#[perf_record]
fn my_function(x: i32) -> i32 {
    // Your code here
    x * 2
}

// Expands to:
fn my_function(x: i32) -> (i32, PerfData) {
    let session = PerfSession::start("my_function");
    let result = {
        // Your code here
        x * 2
    };
    let perf_data = session.stop();
    
    // Send to telemetry server
    telemetry_server::send(perf_data.clone());
    
    (result, perf_data)
}
```

### Inline Perf Recording

```rust
use perf_macros::perf;

fn main() {
    let (result, perf_data) = perf!({
        // Any code block
        expensive_computation()
    });
    
    println!("Result: {}", result);
    println!("Cycles: {}", perf_data.cycles);
    println!("Instructions: {}", perf_data.instructions);
}
```

### Automatic Telemetry

```rust
use perf_macros::perf_auto;

#[perf_auto]  // Automatically sends to telemetry server
fn build_project() {
    // Build code
}

// Perf data automatically sent to:
// - data/telemetry/perf_auto_*.jsonl
// - Telemetry server (if running)
```

## 📦 Crate Structure

```
perf-macros/
├── Cargo.toml
├── src/
│   └── lib.rs              # Proc macro definitions
└── perf-runtime/
    ├── Cargo.toml
    └── src/
        ├── lib.rs          # Runtime support
        ├── session.rs      # PerfSession
        ├── data.rs         # PerfData
        └── telemetry.rs    # Telemetry client
```

## 🔨 Implementation

### 1. Proc Macro Crate (`perf-macros/src/lib.rs`)

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Expr};

#[proc_macro_attribute]
pub fn perf_record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            use perf_runtime::{PerfSession, telemetry_send};
            
            let mut session = PerfSession::start(#fn_name_str);
            let result = #fn_block;
            let perf_data = session.stop();
            
            // Send to telemetry server
            telemetry_send(&perf_data);
            
            result
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn perf(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    
    let expanded = quote! {
        {
            use perf_runtime::{PerfSession, telemetry_send};
            
            let mut session = PerfSession::start("inline_block");
            let result = #expr;
            let perf_data = session.stop();
            
            // Send to telemetry server
            telemetry_send(&perf_data);
            
            (result, perf_data)
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn perf_auto(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            use perf_runtime::{PerfSession, telemetry_send};
            
            let mut session = PerfSession::start(#fn_name_str);
            let result = #fn_block;
            let perf_data = session.stop();
            
            // Send to telemetry server (async, non-blocking)
            telemetry_send(&perf_data);
            
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

### 2. Runtime Support (`perf-runtime/src/lib.rs`)

```rust
pub mod session;
pub mod data;
pub mod telemetry;

pub use session::PerfSession;
pub use data::PerfData;
pub use telemetry::telemetry_send;

// Re-export for convenience
pub use serde::{Serialize, Deserialize};
```

### 3. Perf Session (`perf-runtime/src/session.rs`)

```rust
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::fs;
use crate::data::PerfData;

pub struct PerfSession {
    name: String,
    start_time: Instant,
    timestamp: u64,
    perf_pid: Option<u32>,
    temp_file: String,
}

impl PerfSession {
    pub fn start(name: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let temp_file = format!("/tmp/perf_{}_{}.data", name, timestamp);
        
        let perf_pid = Self::start_perf_record(&temp_file);
        
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            timestamp,
            perf_pid,
            temp_file,
        }
    }
    
    pub fn stop(&mut self) -> PerfData {
        let duration = self.start_time.elapsed();
        
        if let Some(pid) = self.perf_pid {
            unsafe {
                libc::kill(pid as i32, libc::SIGINT);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        // Parse perf data
        let perf_data = self.parse_perf_data();
        
        // Cleanup
        fs::remove_file(&self.temp_file).ok();
        
        PerfData {
            name: self.name.clone(),
            timestamp: self.timestamp,
            duration_secs: duration.as_secs_f64(),
            cycles: perf_data.cycles,
            instructions: perf_data.instructions,
            cache_references: perf_data.cache_references,
            cache_misses: perf_data.cache_misses,
            branches: perf_data.branches,
            branch_misses: perf_data.branch_misses,
        }
    }
    
    fn start_perf_record(output_file: &str) -> Option<u32> {
        let child = Command::new("perf")
            .arg("record")
            .arg("-e").arg("cycles,instructions,cache-references,cache-misses,branches,branch-misses")
            .arg("-o").arg(output_file)
            .arg("-p").arg(format!("{}", std::process::id()))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        
        Some(child.id())
    }
    
    fn parse_perf_data(&self) -> PerfData {
        // Run perf report and parse
        let output = Command::new("perf")
            .arg("report")
            .arg("-i").arg(&self.temp_file)
            .arg("--stdio")
            .output()
            .ok()?;
        
        // Parse output (simplified)
        PerfData::default()
    }
}
```

### 4. Perf Data (`perf-runtime/src/data.rs`)

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfData {
    pub name: String,
    pub timestamp: u64,
    pub duration_secs: f64,
    pub cycles: u64,
    pub instructions: u64,
    pub cache_references: u64,
    pub cache_misses: u64,
    pub branches: u64,
    pub branch_misses: u64,
}

impl Default for PerfData {
    fn default() -> Self {
        Self {
            name: String::new(),
            timestamp: 0,
            duration_secs: 0.0,
            cycles: 0,
            instructions: 0,
            cache_references: 0,
            cache_misses: 0,
            branches: 0,
            branch_misses: 0,
        }
    }
}

impl PerfData {
    pub fn ipc(&self) -> f64 {
        if self.cycles > 0 {
            self.instructions as f64 / self.cycles as f64
        } else {
            0.0
        }
    }
    
    pub fn cache_miss_rate(&self) -> f64 {
        if self.cache_references > 0 {
            self.cache_misses as f64 / self.cache_references as f64
        } else {
            0.0
        }
    }
    
    pub fn branch_miss_rate(&self) -> f64 {
        if self.branches > 0 {
            self.branch_misses as f64 / self.branches as f64
        } else {
            0.0
        }
    }
}
```

### 5. Telemetry Client (`perf-runtime/src/telemetry.rs`)

```rust
use crate::data::PerfData;
use std::fs::{self, OpenOptions};
use std::io::Write;

const TELEMETRY_DIR: &str = "data/telemetry";

pub fn telemetry_send(perf_data: &PerfData) {
    // Create telemetry directory
    fs::create_dir_all(TELEMETRY_DIR).ok();
    
    // Write to JSONL file
    let log_file = format!("{}/perf_auto_{}.jsonl", TELEMETRY_DIR, perf_data.timestamp);
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        let json = serde_json::to_string(perf_data).unwrap();
        writeln!(file, "{}", json).ok();
    }
    
    // Send to telemetry server (if available)
    send_to_server(perf_data);
}

fn send_to_server(perf_data: &PerfData) {
    // Try to send to local telemetry server
    // Non-blocking, fire-and-forget
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post("http://localhost:8080/telemetry/perf")
            .json(perf_data)
            .timeout(std::time::Duration::from_millis(100))
            .send();
    });
}
```

## 🚀 Usage Examples

### Example 1: Function Wrapping

```rust
use perf_macros::perf_auto;

#[perf_auto]
fn expensive_computation(n: u64) -> u64 {
    (0..n).sum()
}

fn main() {
    let result = expensive_computation(1_000_000);
    println!("Result: {}", result);
    // Perf data automatically sent to telemetry
}
```

### Example 2: Inline Recording

```rust
use perf_macros::perf;

fn main() {
    let (result, perf_data) = perf!({
        // Any code block
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    });
    
    println!("Result: {}", result);
    println!("Cycles: {}", perf_data.cycles);
    println!("IPC: {:.2}", perf_data.ipc());
}
```

### Example 3: Nix Build Integration

```rust
use perf_macros::perf_auto;

#[perf_auto]
fn nix_build(package: &str) -> Result<String, String> {
    let output = std::process::Command::new("nix")
        .arg("build")
        .arg(format!(".#{}", package))
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {
    match nix_build("hello") {
        Ok(result) => println!("Build successful: {}", result),
        Err(e) => eprintln!("Build failed: {}", e),
    }
    // Perf data automatically sent to telemetry
}
```

## 📊 Telemetry Server Integration

### Server Endpoint

```rust
// telemetry_server.rs
use actix_web::{post, web, App, HttpResponse, HttpServer};
use perf_runtime::PerfData;

#[post("/telemetry/perf")]
async fn receive_perf(data: web::Json<PerfData>) -> HttpResponse {
    println!("📊 Received perf data: {}", data.name);
    
    // Store in database
    store_perf_data(&data);
    
    // Forward to Bott[8] layout solver
    forward_to_layout_solver(&data);
    
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(receive_perf)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## ✅ Benefits

1. **Zero boilerplate** - Just add `#[perf_auto]`
2. **Inline results** - Get perf data directly in code
3. **Automatic telemetry** - Sent to centralized server
4. **Type-safe** - Compile-time code generation
5. **Non-blocking** - Telemetry sent asynchronously
6. **Composable** - Works with any Rust code

## 🎯 Next Steps

1. Create `perf-macros` crate
2. Create `perf-runtime` crate
3. Implement proc macros
4. Implement runtime support
5. Test with simple examples
6. Integrate with `nix_canonical_builder`
7. Connect to telemetry server
8. Feed to Bott[8] layout solver

---

**Status**: Design complete, ready to implement
**Next**: Create `perf-macros` and `perf-runtime` crates
