use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
pub extern "C" fn plugin_init() -> *const u8 {
    b"zos-llm-proxy\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn plugin_start() {
    std::thread::spawn(|| {
        run_llm_proxy();
    });
}

fn run_llm_proxy() {
    let listener = TcpListener::bind("0.0.0.0:11435").expect("Failed to bind LLM proxy");
    println!("🤖 ZOS LLM Proxy: listening on port 11435 (proxying to Ollama:11434)");
    
    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            let mut client = stream;
            let mut buf = [0u8; 8192];
            
            if let Ok(n) = client.read(&mut buf) {
                let request = String::from_utf8_lossy(&buf[..n]);
                log_llm_request(&request);
                
                // Proxy to Ollama
                if let Ok(mut ollama) = std::net::TcpStream::connect("127.0.0.1:11434") {
                    ollama.write_all(&buf[..n]).ok();
                    
                    let mut response = Vec::new();
                    ollama.read_to_end(&mut response).ok();
                    
                    log_llm_response(&String::from_utf8_lossy(&response));
                    client.write_all(&response).ok();
                }
            }
        });
    }
}

fn log_llm_request(request: &str) {
    use std::fs::OpenOptions;
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("zos_llm.log")
        .unwrap();
    
    writeln!(log, "[REQ] {}", request.lines().next().unwrap_or("")).ok();
}

fn log_llm_response(response: &str) {
    use std::fs::OpenOptions;
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("zos_llm.log")
        .unwrap();
    
    writeln!(log, "[RES] {}", response.chars().take(100).collect::<String>()).ok();
}
