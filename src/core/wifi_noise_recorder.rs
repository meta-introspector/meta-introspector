// WiFi Noise Recorder - Capture noise floor during Bach computation
// Uses /proc/net/wireless for simple noise monitoring

use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use std::io::Write;

fn main() {
    let interface = "wlo1";
    let output_file = format!("/mnt/data1/bach/wifi_noise_{}.csv", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    
    println!("📡 WiFi Noise Recorder");
    println!("Interface: {}", interface);
    println!("Output: {}", output_file);
    println!();
    
    let mut file = fs::File::create(&output_file).unwrap();
    writeln!(file, "timestamp_ms,signal_dbm,noise_dbm,link_quality").unwrap();
    
    let start = Instant::now();
    
    for _ in 0..600 {  // 60 seconds at 10Hz
        let elapsed = start.elapsed().as_millis();
        
        // Read /proc/net/wireless
        if let Ok(wireless) = fs::read_to_string("/proc/net/wireless") {
            for line in wireless.lines() {
                if line.contains(interface) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let quality = parts[2].trim_end_matches('.');
                        let signal = parts[3].trim_end_matches('.');
                        let noise = parts[4].trim_end_matches('.');
                        
                        writeln!(file, "{},{},{},{}", elapsed, signal, noise, quality).unwrap();
                        
                        if elapsed % 5000 < 100 {  // Print every 5 seconds
                            println!("T+{}ms: signal={} noise={} quality={}", 
                                elapsed, signal, noise, quality);
                        }
                    }
                }
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("\n✅ Recording complete: {}", output_file);
}
