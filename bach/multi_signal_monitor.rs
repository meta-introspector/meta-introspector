// Multi-Signal Monitor - Complete sensor battery for computational work measurement
// Combines: Temperature, WiFi EM, CPU frequency, disk I/O, memory, power

use std::fs;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn read_temp() -> f32 {
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|t| t / 1000.0)
        .unwrap_or(0.0)
}

fn read_wifi_signal() -> i32 {
    fs::read_to_string("/proc/net/wireless")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.contains("wlo1"))
                .and_then(|l| l.split_whitespace().nth(3))
                .and_then(|v| v.trim_end_matches('.').parse().ok())
        })
        .unwrap_or(-99)
}

fn read_cpu_freq() -> f32 {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.contains("cpu MHz"))
                .and_then(|l| l.split(':').nth(1))
                .and_then(|v| v.trim().parse().ok())
        })
        .unwrap_or(0.0)
}

fn read_disk_io() -> (u64, u64) {
    fs::read_to_string("/proc/diskstats")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.contains("nvme0n1 "))
                .map(|l| {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    let read = parts.get(5).and_then(|v| v.parse().ok()).unwrap_or(0);
                    let write = parts.get(9).and_then(|v| v.parse().ok()).unwrap_or(0);
                    (read, write)
                })
        })
        .unwrap_or((0, 0))
}

fn read_mem_usage() -> u64 {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            let total: u64 = s.lines()
                .find(|l| l.starts_with("MemTotal"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            let avail: u64 = s.lines()
                .find(|l| l.starts_with("MemAvailable"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            Some(total - avail)
        })
        .unwrap_or(0)
}

fn main() {
    let output_file = format!("/mnt/data1/bach/multi_signal_{}.csv", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    
    println!("📊 MULTI-SIGNAL MONITOR");
    println!("Battery: Temp, WiFi EM, CPU Freq, Disk I/O, Memory");
    println!("Output: {}", output_file);
    println!();
    
    let mut file = fs::File::create(&output_file).unwrap();
    writeln!(file, "timestamp_ms,temp_c,wifi_dbm,cpu_freq_mhz,disk_read_kb,disk_write_kb,mem_used_kb").unwrap();
    
    let start = Instant::now();
    
    for i in 0..600 {  // 60 seconds at 10Hz
        let elapsed = start.elapsed().as_millis();
        
        let temp = read_temp();
        let wifi = read_wifi_signal();
        let cpu_freq = read_cpu_freq();
        let (disk_read, disk_write) = read_disk_io();
        let mem_used = read_mem_usage();
        
        writeln!(file, "{},{},{},{},{},{},{}", 
            elapsed, temp, wifi, cpu_freq, disk_read, disk_write, mem_used).unwrap();
        
        if i % 50 == 0 {
            println!("T+{:5}ms: Temp={:4.1}°C WiFi={:3}dBm Freq={:7.1}MHz Mem={:5}MB", 
                elapsed, temp, wifi, cpu_freq, mem_used / 1024);
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("\n✅ Complete: {}", output_file);
    println!("\n🎯 Full sensor battery captured - ready for correlation analysis");
}
