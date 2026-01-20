use std::net::TcpListener;
use std::io::{Read, Write};

#[no_mangle]
pub extern "C" fn plugin_init() -> *const u8 {
    b"zos-file-proxy\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn plugin_start() {
    std::thread::spawn(|| {
        run_file_proxy();
    });
}

fn run_file_proxy() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind file proxy");
    println!("📁 ZOS File Proxy: listening on port 8080");
    
    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            
            if let Ok(n) = stream.read(&mut buf) {
                let request = String::from_utf8_lossy(&buf[..n]);
                log_request("FILE", &request);
                
                // Serve files from /mnt/data1/meta-introspector
                let response = b"HTTP/1.1 200 OK\r\n\r\nZOS File Proxy";
                stream.write_all(response).ok();
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
