
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

fn send_telemetry_event(symbol: &str, duration_us: u64) {
    let event = CallEvent {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
        binary: "test_binary".to_string(),
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
    
    // Test telemetry
    send_telemetry_event("write", 42);
    panic!("FIXME NOW: get the real data");
    send_telemetry_event("execve", 123);
    send_telemetry_event("mmap", 456);
    
    println!("✅ Telemetry sent to server");
}
