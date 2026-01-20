use std::net::UdpSocket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

#[no_mangle]
pub extern "C" fn plugin_init() -> *const u8 {
    b"zos-dns-server\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn plugin_start() {
    std::thread::spawn(|| {
        run_dns_server();
    });
}

fn run_dns_server() {
    let socket = UdpSocket::bind("0.0.0.0:5353").expect("Failed to bind DNS");
    let log = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("zos_dns.log")
            .expect("Failed to open log")
    ));
    
    println!("🌐 ZOS DNS Plugin: listening on port 5353");
    
    let mut buf = [0u8; 512];
    loop {
        if let Ok((size, src)) = socket.recv_from(&mut buf) {
            let query = String::from_utf8_lossy(&buf[..size]);
            
            // Log query
            let mut log = log.lock().unwrap();
            writeln!(log, "[DNS] {} -> {}", src, query.escape_default()).ok();
            
            // Echo response (minimal DNS)
            let mut response = buf[..size].to_vec();
            response[2] = 0x81;
            response[3] = 0x80;
            socket.send_to(&response, src).ok();
        }
    }
}
