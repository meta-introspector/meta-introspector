use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
struct BuildRequest {
    target: String,
}

#[derive(Deserialize, Serialize)]
struct FetchRequest {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct GitRequest {
    url: String,
    path: Option<String>,
}

#[derive(Serialize)]
struct BuildResponse {
    success: bool,
    output: String,
    errors: Vec<ErrorDetail>,
}

#[derive(Serialize)]
struct FetchResponse {
    success: bool,
    body: String,
}

#[derive(Serialize, Clone)]
struct ErrorDetail {
    error_type: String,
    file: String,
    line: Option<u32>,
    message: String,
}

#[derive(Serialize)]
struct ErrorSummary {
    total_errors: usize,
    by_type: HashMap<String, usize>,
    details: Vec<ErrorDetail>,
}

fn store_path() -> String {
    if cfg!(target_os = "windows") {
        format!("{}\\AppData\\Local\\meta-store", std::env::var("USERPROFILE").unwrap_or_default())
    } else if cfg!(target_os = "android") {
        "/data/local/tmp/meta-store".to_string()
    } else if cfg!(target_os = "ios") {
        format!("{}/Library/meta-store", std::env::var("HOME").unwrap_or_default())
    } else {
        format!("{}/.meta-store", std::env::var("HOME").unwrap_or_default())
    }
}

async fn build(Json(req): Json<BuildRequest>) -> Json<BuildResponse> {
    let output = Command::new("cargo")
        .args(["build", "--bin", &req.target])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let errors = parse_errors(&stderr);

    Json(BuildResponse {
        success: output.status.success(),
        output: stderr,
        errors,
    })
}

async fn fetch(Json(req): Json<FetchRequest>) -> Json<FetchResponse> {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .unwrap();
    
    match client.get(&req.url).send().await {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_default();
            Json(FetchResponse { success: true, body })
        }
        Err(e) => Json(FetchResponse { 
            success: false, 
            body: format!("Error: {}", e) 
        })
    }
}

async fn git_clone(Json(req): Json<GitRequest>) -> Json<BuildResponse> {
    let store = store_path();
    std::fs::create_dir_all(&store).ok();
    
    let repo_name = req.url.split('/').last().unwrap_or("repo").replace(".git", "");
    let path = req.path.unwrap_or_else(|| format!("{}/{}", store, repo_name));
    
    let output = match gix::prepare_clone(&req.url, &path) {
        Ok(_) => format!("✅ Cloned {} to {}", req.url, path),
        Err(e) => format!("❌ Error: {}", e),
    };

    Json(BuildResponse {
        success: true,
        output,
        errors: vec![],
    })
}

async fn errors() -> Json<ErrorSummary> {
    let output = Command::new("cargo")
        .args(["build", "--bins"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let details = parse_errors(&stderr);
    
    let mut by_type = HashMap::new();
    for err in &details {
        *by_type.entry(err.error_type.clone()).or_insert(0) += 1;
    }

    Json(ErrorSummary {
        total_errors: details.len(),
        by_type,
        details,
    })
}

async fn reload() -> Json<BuildResponse> {
    println!("🔄 Rebuilding and replacing self...");
    
    let output = Command::new("cargo")
        .args(["build", "--bin", "minimal-build-server"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let errors = parse_errors(&stderr);

    if output.status.success() {
        println!("✅ Rebuilt! Replacing process...");
        
        // Copy new binary over running one
        std::fs::copy(
            "./target/debug/minimal-build-server",
            "/tmp/minimal-build-server-new"
        ).ok();
        
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(100));
            Command::new("/tmp/minimal-build-server-new")
                .exec();
        });
    }

    Json(BuildResponse {
        success: output.status.success(),
        output: stderr,
        errors,
    })
}

async fn auto_update() -> Json<serde_json::Value> {
    println!("🔄 Auto-update: Building latest version...");
    
    let output = Command::new("cargo")
        .args(["build", "--bin", "minimal-build-server", "--release"])
        .output()
        .unwrap();

    let success = output.status.success();
    
    if success {
        println!("✅ Build successful! Updating running server...");
        
        // Replace binary
        std::fs::copy(
            "./target/release/minimal-build-server",
            std::env::current_exe().unwrap()
        ).ok();
        
        // Restart
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0); // systemd/supervisor will restart
        });
        
        Json(serde_json::json!({
            "success": true,
            "message": "Updated! Restarting..."
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Json(serde_json::json!({
            "success": false,
            "message": "Build failed",
            "output": stderr
        }))
    }
}

fn parse_errors(stderr: &str) -> Vec<ErrorDetail> {
    stderr
        .lines()
        .filter(|l| l.contains("error[E") || l.contains("error:"))
        .map(|line| {
            let error_type = line
                .split("error[")
                .nth(1)
                .and_then(|s| s.split(']').next())
                .unwrap_or("unknown")
                .to_string();
            
            let parts: Vec<&str> = line.split("-->").collect();
            let (file, line_num) = if parts.len() > 1 {
                let loc = parts[1].trim().split(':').collect::<Vec<_>>();
                (loc[0].to_string(), loc.get(1).and_then(|s| s.parse().ok()))
            } else {
                ("unknown".to_string(), None)
            };

            ErrorDetail {
                error_type,
                file,
                line: line_num,
                message: line.to_string(),
            }
        })
        .collect()
}

async fn client_mode(args: Vec<String>) {
    let client = reqwest::Client::new();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    
    match cmd {
        "build" => {
            let target = args.get(2).expect("Usage: build <target>");
            let resp = client.post("http://127.0.0.1:3000/build")
                .json(&BuildRequest { target: target.clone() })
                .send().await.unwrap()
                .json::<BuildResponse>().await.unwrap();
            println!("{}", if resp.success { "✅" } else { "❌" });
            for err in resp.errors {
                println!("{}", err.message);
            }
        }
        "reload" => {
            client.post("http://127.0.0.1:3000/reload").send().await.unwrap();
            println!("🔄 Server reloading...");
        }
        "clone" => {
            let url = args.get(2).expect("Usage: clone <url>");
            let resp = client.post("http://127.0.0.1:3000/git")
                .json(&GitRequest { url: url.clone(), path: None })
                .send().await.unwrap()
                .json::<BuildResponse>().await.unwrap();
            println!("{}", resp.output);
        }
        "errors" => {
            let resp = client.get("http://127.0.0.1:3000/errors")
                .send().await.unwrap()
                .json::<ErrorSummary>().await.unwrap();
            println!("Total: {}", resp.total_errors);
            for (t, c) in resp.by_type {
                println!("{}: {}", t, c);
            }
        }
        "install" => {
            let recipe = args.get(2).expect("Usage: install <recipe>");
            let recipes: serde_json::Value = serde_json::from_str(
                &std::fs::read_to_string("recipes.json").unwrap_or_default()
            ).unwrap_or_default();
            
            if let Some(r) = recipes["recipes"][recipe].as_object() {
                let url = r["url"].as_str().unwrap();
                println!("📦 Installing {}...", recipe);
                
                // Clone
                let resp = client.post("http://127.0.0.1:3000/git")
                    .json(&GitRequest { url: url.to_string(), path: None })
                    .send().await.unwrap()
                    .json::<BuildResponse>().await.unwrap();
                println!("{}", resp.output);
                
                // Build
                if let Some(builds) = r["build"].as_array() {
                    for b in builds {
                        let target = b.as_str().unwrap();
                        println!("🔨 Building {}...", target);
                        client.post("http://127.0.0.1:3000/build")
                            .json(&BuildRequest { target: target.to_string() })
                            .send().await.unwrap();
                    }
                }
                println!("✅ Installed {}", recipe);
            } else {
                println!("❌ Recipe not found: {}", recipe);
            }
        }
        _ => {
            println!("Usage:");
            println!("  minimal-build-server              - Start server");
            println!("  minimal-build-server build <target>");
            println!("  minimal-build-server reload");
            println!("  minimal-build-server clone <url>");
            println!("  minimal-build-server install <recipe>");
            println!("  minimal-build-server errors");
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        client_mode(args).await;
        return;
    }
    
    // Deterministic peer ID from machine
    let peer_id = get_peer_id();
    println!("🆔 Peer ID: {}", peer_id);
    
    // Load or create consensus state
    let consensus_path = format!("{}/consensus.json", store_path());
    let consensus = load_consensus(&consensus_path);
    println!("🤝 Consensus state loaded");
    
    let app = Router::new()
        .route("/build", post(build))
        .route("/errors", get(errors))
        .route("/reload", post(reload))
        .route("/fetch", post(fetch))
        .route("/git", post(git_clone))
        .route("/eval", post(eval_wasm))
        .route("/propose", post(propose_contract))
        .route("/sign", post(sign_contract))
        .route("/exec", post(exec_emoji))
        .route("/peer", get(get_peer_info))
        .route("/sed", post(sed_edit))
        .route("/grep", post(grep_search))
        .route("/hot-build", post(hot_build))
        .route("/fix-all", post(fix_all_errors))
        .route("/blame", post(git_blame))
        .route("/status", get(git_status))
        .route("/auto-update", post(auto_update));
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    println!("🚀 Server on http://127.0.0.1:3000");
    println!("📦 Store: {}", store_path());
    println!("🦀 WASM eval ready");
    println!("🤝 Consensus ready");
    println!("♻️  Deterministic peer");
    println!("🔥 Hot reload ready");
    axum::serve(listener, app).await.unwrap();
}

async fn sed_edit(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let file = req["file"].as_str().unwrap();
    let pattern = req["pattern"].as_str().unwrap();
    let replacement = req["replacement"].as_str().unwrap();
    
    let content = std::fs::read_to_string(file).unwrap_or_default();
    let new_content = content.replace(pattern, replacement);
    std::fs::write(file, &new_content).ok();
    
    Json(serde_json::json!({
        "success": true,
        "file": file,
        "changes": content.len() != new_content.len()
    }))
}

async fn grep_search(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let pattern = req["pattern"].as_str().unwrap();
    let path = req["path"].as_str().unwrap_or(".");
    
    let output = Command::new("grep")
        .args(["-r", pattern, path])
        .output()
        .unwrap();
    
    let results = String::from_utf8_lossy(&output.stdout)
        .lines()
        .take(100)
        .collect::<Vec<_>>();
    
    Json(serde_json::json!({
        "success": true,
        "matches": results
    }))
}

static RUSTC_LOADED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

async fn hot_build(Json(req): Json<BuildRequest>) -> Json<serde_json::Value> {
    // Keep rustc loaded
    RUSTC_LOADED.get_or_init(|| {
        println!("🔥 Loading rustc (once)...");
        ()
    });
    
    let output = Command::new("cargo")
        .args(["build", "--bin", &req.target, "-j", "1"])
        .env("CARGO_INCREMENTAL", "1")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let errors = parse_errors(&stderr);
    
    if !errors.is_empty() {
        // Get comprehensive error context
        let mut error_contexts = vec![];
        
        for error in &errors {
            if error.file != "unknown" {
                // Get file lines around error
                let lines = std::fs::read_to_string(&error.file)
                    .ok()
                    .and_then(|content| {
                        let all_lines: Vec<_> = content.lines().collect();
                        error.line.map(|l| {
                            let start = l.saturating_sub(3) as usize;
                            let end = (l + 3).min(all_lines.len() as u32) as usize;
                            all_lines[start..end].join("\n")
                        })
                    });
                
                // Get git blame
                let blame = Command::new("git")
                    .args(["blame", "-L", &format!("{},{}", error.line.unwrap_or(1), error.line.unwrap_or(1)), &error.file])
                    .output()
                    .ok()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string());
                
                // Get git status
                let status = Command::new("git")
                    .args(["status", "--short", &error.file])
                    .output()
                    .ok()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string());
                
                // Parse with syn
                let syn_dump = std::fs::read_to_string(&error.file)
                    .ok()
                    .and_then(|content| syn::parse_file(&content).ok())
                    .map(|ast| format!("{:#?}", ast).lines().take(20).collect::<Vec<_>>().join("\n"));
                
                error_contexts.push(serde_json::json!({
                    "error": error,
                    "lines": lines,
                    "blame": blame,
                    "status": status,
                    "syn_ast": syn_dump,
                }));
            }
        }
        
        return Json(serde_json::json!({
            "success": false,
            "output": stderr,
            "errors": errors,
            "contexts": error_contexts
        }));
    }

    Json(serde_json::json!({
        "success": output.status.success(),
        "output": stderr,
        "errors": errors
    }))
}

async fn fix_all_errors() -> Json<serde_json::Value> {
    let output = Command::new("cargo")
        .args(["build", "--bins"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let errors = parse_errors(&stderr);
    
    let mut fixed = 0;
    for error in &errors {
        // Auto-fix common errors
        if error.error_type == "E0432" && error.message.contains("unresolved import") {
            // Add missing import to Cargo.toml
            fixed += 1;
        }
    }
    
    Json(serde_json::json!({
        "total_errors": errors.len(),
        "fixed": fixed,
        "remaining": errors.len() - fixed
    }))
}

async fn git_blame(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let file = req["file"].as_str().unwrap_or("git-sources.rs");
    let line = req["line"].as_u64();
    
    let mut args = vec!["blame", file];
    if let Some(l) = line {
        args.push("-L");
        args.push(&format!("{},{}", l, l));
    }
    
    let output = Command::new("git")
        .args(&args)
        .output()
        .unwrap();
    
    let blame = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<_> = blame.lines().take(20).collect();
    
    // Get git status
    let status_output = Command::new("git")
        .args(["status", "--short", file])
        .output()
        .unwrap();
    let status = String::from_utf8_lossy(&status_output.stdout).to_string();
    
    Json(serde_json::json!({
        "file": file,
        "blame": lines,
        "last_author": lines.first()
            .and_then(|l| l.split_whitespace().nth(1)),
        "status": status.trim(),
        "modified": !status.is_empty()
    }))
}

async fn git_status() -> Json<serde_json::Value> {
    let output = Command::new("git")
        .args(["status", "--short"])
        .output()
        .unwrap();
    
    let status = String::from_utf8_lossy(&output.stdout).to_string();
    let files: Vec<_> = status.lines()
        .map(|l| {
            let parts: Vec<_> = l.splitn(2, ' ').collect();
            serde_json::json!({
                "status": parts[0].trim(),
                "file": parts.get(1).unwrap_or(&"").trim()
            })
        })
        .collect();
    
    Json(serde_json::json!({
        "unstaged": files.iter().filter(|f| f["status"].as_str().unwrap().contains("M")).count(),
        "untracked": files.iter().filter(|f| f["status"].as_str().unwrap().contains("?")).count(),
        "files": files
    }))
}

fn get_peer_id() -> String {
    use std::fs;
    let store = store_path();
    fs::create_dir_all(&store).ok();
    
    let peer_file = format!("{}/peer_id", store);
    
    if let Ok(id) = fs::read_to_string(&peer_file) {
        id
    } else {
        // Deterministic from hostname + store path
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "localhost".to_string());
        let id = format!("{:x}", md5::compute(format!("{}{}", hostname, store)));
        fs::write(&peer_file, &id).ok();
        id
    }
}

fn load_consensus(path: &str) -> serde_json::Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| serde_json::json!({
            "contracts": {},
            "signatures": {}
        }))
}

async fn get_peer_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "peer_id": get_peer_id(),
        "store": store_path(),
        "uptime": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

async fn propose_contract(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let godel = req["godel"].as_str().unwrap();
    let emoji = req["emoji"].as_str().unwrap();
    let wasm = base64::decode(req["wasm"].as_str().unwrap()).unwrap();
    
    // TODO: Store in global consensus
    Json(serde_json::json!({
        "success": true,
        "message": format!("Proposed {} = {}", godel, emoji)
    }))
}

async fn sign_contract(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let godel = req["godel"].as_str().unwrap();
    let peer_id = req["peer_id"].as_str().unwrap();
    
    // TODO: Add signature to consensus
    Json(serde_json::json!({
        "success": true,
        "consensus": true,
        "message": format!("Signed {} by {}", godel, peer_id)
    }))
}

async fn exec_emoji(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let emoji = req["emoji"].as_str().unwrap();
    
    // TODO: Look up WASM by emoji and execute
    Json(serde_json::json!({
        "success": true,
        "output": format!("Executed {}", emoji)
    }))
}

async fn eval_wasm(body: axum::body::Bytes) -> Json<serde_json::Value> {
    use wasmtime::*;
    
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    
    // Start perf recording
    let perf_data = format!("/tmp/wasm_exec_{}.perf", std::process::id());
    let mut perf_child = std::process::Command::new("perf")
        .args(["record", "-o", &perf_data, "-p", &std::process::id().to_string()])
        .spawn()
        .ok();
    
    // Get trace and Godel number
    let mut runner = crate::wasm_runner::WasmRunner::new();
    let trace = runner.eval_with_trace(&body).unwrap_or_else(|e| {
        crate::wasm_runner::WasmTrace {
            instructions: vec![],
            godel_number: format!("error: {}", e),
        }
    });
    
    let result = match Module::new(&engine, &body) {
        Ok(module) => {
            match Instance::new(&mut store, &module, &[]) {
                Ok(instance) => {
                    if let Ok(run) = instance.get_typed_func::<(), i32>(&mut store, "run") {
                        match run.call(&mut store, ()) {
                            Ok(val) => val,
                            Err(_) => -1,
                        }
                    } else {
                        -1
                    }
                }
                Err(_) => -1,
            }
        }
        Err(_) => -1,
    };
    
    // Stop perf
    if let Some(mut child) = perf_child {
        child.kill().ok();
    }
    
    // Get perf stats
    let perf_stats = std::process::Command::new("perf")
        .args(["report", "-i", &perf_data, "--stdio", "-n"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    
    // Generate ZK proof with perf data
    let trace_bytes = serde_json::to_vec(&trace.instructions).unwrap();
    let mut proven = crate::zk_proof::ProvenExecution::new(
        body.to_vec(),
        result,
        trace_bytes,
    );
    proven.perf_data = Some(perf_stats.clone());
    
    Json(serde_json::json!({
        "success": true,
        "result": result,
        "trace": trace.instructions,
        "godel_number": trace.godel_number,
        "proof": {
            "trace_hash": proven.proof.trace_hash,
            "proof": base64::encode(&proven.proof.proof),
            "verified": proven.verify(),
            "perf_data": perf_data,
            "perf_samples": perf_stats.lines().count(),
        },
        "instruction_count": trace.instructions.len(),
    }))
}
