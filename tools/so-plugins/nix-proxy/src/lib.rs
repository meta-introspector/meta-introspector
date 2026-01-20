use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
pub extern "C" fn plugin_init() -> *const u8 {
    b"zos-nix-proxy\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn plugin_start() {
    std::thread::spawn(|| {
        run_nix_proxy();
    });
}

fn run_nix_proxy() {
    let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind nix proxy");
    println!("❄️  ZOS Nix Proxy: listening on port 5000");
    
    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            
            if let Ok(n) = stream.read(&mut buf) {
                let request = String::from_utf8_lossy(&buf[..n]);
                log_request("NIX", &request);
                
                // Proxy to nix-serve
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").ok();
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
