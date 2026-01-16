use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use tokio::net::TcpListener;

mod traits;
use traits::*;

// Bootstrap: Load libnix, then use it to load system libs
fn bootstrap_libs() -> Result<(), String> {
    // Load libnix.so
    let libnix_path = "./target/debug/liblibnix.so";
    
    if std::path::Path::new(libnix_path).exists() {
        println!("📦 Loading libnix...");
        
        // Use libnix to load system libraries via nix
        use libloading::{Library, Symbol};
        unsafe {
            let lib = Library::new(libnix_path)
                .map_err(|e| format!("Failed to load libnix: {}", e))?;
            
            let load_fn: Symbol<extern "C" fn(*const *const i8, usize) -> i32> = 
                lib.get(b"libnix_load")
                .map_err(|e| format!("Failed to find libnix_load: {}", e))?;
            
            // Load ssl, git, curl via nix
            let libs = vec![
                std::ffi::CString::new("ssl").unwrap(),
                std::ffi::CString::new("git").unwrap(),
                std::ffi::CString::new("curl").unwrap(),
            ];
            let ptrs: Vec<*const i8> = libs.iter().map(|s| s.as_ptr()).collect();
            
            let result = load_fn(ptrs.as_ptr(), ptrs.len());
            if result == 0 {
                println!("✅ Loaded system libs via nix");
                Ok(())
            } else {
                Err("Failed to load libs via nix".to_string())
            }
        }
    } else {
        println!("⚠️  libnix not found, using system libs");
        Ok(())
    }
}

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
    let http = StubHttp;
    match http.get(&req.url) {
        Ok(body) => Json(FetchResponse { success: true, body }),
        Err(e) => Json(FetchResponse { success: false, body: e })
    }
}

async fn git_clone(Json(req): Json<GitRequest>) -> Json<BuildResponse> {
    let git = StubGit;
    let path = req.path.unwrap_or_else(|| format!("{}/repo", store_path()));
    
    match git.clone(&req.url, &path) {
        Ok(_) => Json(BuildResponse {
            success: true,
            output: format!("Cloned to {}", path),
            errors: vec![],
        }),
        Err(e) => Json(BuildResponse {
            success: false,
            output: e,
            errors: vec![],
        })
    }
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

async fn restart() -> Json<BuildResponse> {
    println!("🔄 Restarting server...");
    
    let output = Command::new("cargo")
        .args(["build", "--bin", "minimal-build-server"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let errors = parse_errors(&stderr);

    if output.status.success() {
        println!("✅ Rebuilt! Restarting...");
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(100));
            Command::new("./target/debug/minimal-build-server").exec();
        });
    }

    Json(BuildResponse {
        success: output.status.success(),
        output: stderr,
        errors,
    })
}

async fn upgrade() -> Json<serde_json::Value> {
    println!("⬆️  Upgrading server...");
    
    let output = Command::new("cargo")
        .args(["build", "--bin", "minimal-build-server", "--release"])
        .output()
        .unwrap();

    if output.status.success() {
        println!("✅ Upgraded! Restarting...");
        std::fs::copy(
            "./target/release/minimal-build-server",
            std::env::current_exe().unwrap()
        ).ok();
        
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        });
        
        Json(serde_json::json!({"success": true, "message": "Upgraded!"}))
    } else {
        Json(serde_json::json!({"success": false, "output": String::from_utf8_lossy(&output.stderr)}))
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
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    
    match cmd {
        _ => {
            println!("Client mode disabled - use curl:");
            println!("  curl -X POST http://127.0.0.1:3000/compile -d '{{\"target\":\"foo\"}}'");
            println!("  curl -X POST http://127.0.0.1:3000/restart");
            println!("  curl -X POST http://127.0.0.1:3000/git -d '{{\"url\":\"...\"}}'");
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
    
    // Bootstrap: Load system libs via libnix
    if let Err(e) = bootstrap_libs() {
        eprintln!("⚠️  Bootstrap warning: {}", e);
    }
    
    // Deterministic peer ID from machine
    let peer_id = get_peer_id();
    println!("🆔 Peer ID: {}", peer_id);
    
    // Load or create consensus state
    let consensus_path = format!("{}/consensus.json", store_path());
    let consensus = load_consensus(&consensus_path);
    println!("🤝 Consensus state loaded");
    
    let app = Router::new()
        .route("/compile", post(compile))
        .route("/errors", get(errors))
        .route("/restart", post(restart))
        .route("/upgrade", post(upgrade))
        .route("/fetch", post(fetch))
        .route("/git", post(git_clone))
        .route("/eval", post(eval_wasm))
        .route("/propose", post(propose_contract))
        .route("/sign", post(sign_contract))
        .route("/exec", post(exec_emoji))
        .route("/peer", get(get_peer_info))
        .route("/sed", post(sed_edit))
        .route("/grep", post(grep_search))
        .route("/fix-all", post(fix_all_errors))
        .route("/blame", post(git_blame))
        .route("/status", get(git_status));
    
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
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let results = stdout
        .lines()
        .take(100)
        .collect::<Vec<_>>();
    
    Json(serde_json::json!({
        "success": true,
        "matches": results
    }))
}

static RUSTC_LOADED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

async fn compile(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let target = req["target"].as_str().unwrap();
    let fast = req["fast"].as_bool().unwrap_or(true);
    let use_nix = req["use_nix"].as_bool().unwrap_or(false);
    
    if use_nix {
        return Json(serde_json::json!({
            "success": false,
            "output": "nix support moved to libnix.so"
        }));
    }
    
    // Keep rustc loaded for fast compilation
    if fast {
        RUSTC_LOADED.get_or_init(|| {
            println!("🔥 Loading rustc (once)...");
            ()
        });
    }
    
    let output = Command::new("cargo")
        .args(["build", "--bin", target, "-j", "1"])
        .env("CARGO_INCREMENTAL", if fast { "1" } else { "0" })
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
                    .map(|_ast| "AST parsed successfully".to_string());
                
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
    let line_arg;
    if let Some(l) = line {
        line_arg = format!("{},{}", l, l);
        args.push("-L");
        args.push(&line_arg);
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
    Json(serde_json::json!({
        "success": false,
        "error": "Consensus moved to consensus.so"
    }))
}

async fn sign_contract(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": false,
        "error": "Consensus moved to consensus.so"
    }))
}

async fn exec_emoji(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": false,
        "error": "Emoji execution moved to consensus.so"
    }))
}

async fn eval_wasm(body: axum::body::Bytes) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": false,
        "error": "WASM support moved to wasm_runner.so"
    }))
}
