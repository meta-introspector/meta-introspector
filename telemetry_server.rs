// 🔥 TELEMETRY SERVER: Structured capture of all wrapped calls
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CallEvent {
    timestamp: u64,
    binary: String,
    symbol: String,
    duration_us: u64,
    args_count: usize,
    pid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TelemetrySession {
    session_id: String,
    start_time: u64,
    events: Vec<CallEvent>,
    total_calls: usize,
}

fn main() {
    println!("🔥 TELEMETRY SERVER");
    println!("==================");
    
    let session = Arc::new(Mutex::new(TelemetrySession {
        session_id: format!("telemetry_{}", 
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
        start_time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
        events: Vec::new(),
        total_calls: 0,
    }));
    
    // Start telemetry server
    let server_session = session.clone();
    thread::spawn(move || {
        start_telemetry_server(server_session);
    });
    
    // Generate preload client
    generate_preload_client();
    
    // Wait and show results
    thread::sleep(std::time::Duration::from_secs(2));
    show_telemetry_results(&session);
}

fn start_telemetry_server(session: Arc<Mutex<TelemetrySession>>) {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("🌐 Telemetry server listening on 127.0.0.1:8888");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let session_clone = session.clone();
                thread::spawn(move || {
                    handle_telemetry_client(stream, session_clone);
                });
            }
            Err(_) => break,
        }
    }
}

fn handle_telemetry_client(mut stream: TcpStream, session: Arc<Mutex<TelemetrySession>>) {
    let mut buffer = [0; 1024];
    
    if let Ok(size) = stream.read(&mut buffer) {
        let data = String::from_utf8_lossy(&buffer[..size]);
        
        if let Ok(event) = serde_json::from_str::<CallEvent>(&data) {
            println!("📊 Received: {} -> {} ({}μs)", event.binary, event.symbol, event.duration_us);
            
            if let Ok(mut session) = session.lock() {
                session.events.push(event);
                session.total_calls += 1;
            }
            
            let _ = stream.write_all(b"OK");
        }
    }
}

fn generate_preload_client() {
    println!("\n🔧 GENERATING PRELOAD CLIENT");
    println!("============================");
    
    let client_code = r#"
// 🔥 PRELOAD TELEMETRY CLIENT
use std::net::TcpStream;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CallEvent {
    timestamp: u64,
    binary: String,
    symbol: String,
    duration_us: u64,
    args_count: usize,
    pid: u32,
}

// Include all our generated wrappers
include!("master_all_calls_allcalls_1768321896.rs");

// Override telemetry macro to send to server
macro_rules! call_telemetry {
    ($name:literal, $call:expr) => {{
        let start = std::time::Instant::now();
        let result = $call;
        let duration = start.elapsed().as_micros() as u64;
        
        // Send to telemetry server
        send_telemetry_event($name, duration);
        
        result
    }};
}

fn send_telemetry_event(symbol: &str, duration_us: u64) {
    let event = CallEvent {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
        binary: std::env::current_exe().unwrap_or_default().to_string_lossy().to_string(),
        symbol: symbol.to_string(),
        duration_us,
        args_count: 0,
        pid: std::process::id(),
    };
    
    if let Ok(json) = serde_json::to_string(&event) {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
            let _ = stream.write_all(json.as_bytes());
        }
    }
}

fn main() {
    println!("🔥 PRELOAD CLIENT ACTIVE");
    init_all_call_wrappers!();
    
    // Test some wrapped calls
    call__obstack_allocated_p_wrapped!();
    
    println!("✅ Telemetry sent to server");
}
"#;
    
    std::fs::write("telemetry_client.rs", client_code).unwrap();
    println!("✅ Generated telemetry_client.rs");
}

fn show_telemetry_results(session: &Arc<Mutex<TelemetrySession>>) {
    println!("\n📊 TELEMETRY RESULTS");
    println!("===================");
    
    if let Ok(session) = session.lock() {
        println!("📈 Session: {}", session.session_id);
        println!("🕐 Start time: {}", session.start_time);
        println!("📞 Total calls: {}", session.total_calls);
        
        if !session.events.is_empty() {
            println!("\n🔥 CAPTURED EVENTS:");
            for (i, event) in session.events.iter().enumerate() {
                println!("  {}. {} -> {} ({}μs) PID:{}", 
                    i+1, event.binary, event.symbol, event.duration_us, event.pid);
            }
        }
        
        // Save results
        let results_file = format!("telemetry_results_{}.json", session.session_id);
        if let Ok(json) = serde_json::to_string_pretty(&*session) {
            if std::fs::write(&results_file, json).is_ok() {
                println!("\n✅ Saved results: {}", results_file);
            }
        }
    }
    
    println!("\n🎯 STRUCTURED TELEMETRY CAPTURE READY!");
    println!("🌐 Server running on 127.0.0.1:8888");
    println!("📡 All wrapped calls will be captured in real-time");
}
