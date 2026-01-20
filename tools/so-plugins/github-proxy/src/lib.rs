use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
pub extern "C" fn plugin_init() -> *const u8 {
    b"zos-github-proxy\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn plugin_start() {
    std::thread::spawn(|| {
        run_github_proxy();
    });
}

fn run_github_proxy() {
    let listener = TcpListener::bind("0.0.0.0:9418").expect("Failed to bind github proxy");
    println!("🐙 ZOS GitHub Proxy: listening on port 9418");
    
    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            
            if let Ok(n) = stream.read(&mut buf) {
                let request = String::from_utf8_lossy(&buf[..n]);
                log_request("GITHUB", &request);
                
                // Proxy to local git mirror
                stream.write_all(b"OK\n").ok();
            }
        });
    }
}

fn log_request(service: &str, request: &str) {
    use std::fs::OpenOptions;
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("zos_proxy.log")
        .unwrap();
    writeln!(log, "[{}] {}", service, request.lines().next().unwrap_or("")).ok();
}
