// Square Wave - On/Off CPU load for clear EM proof
// 1 second ON (full load), 1 second OFF (idle), repeat

use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("⬜ SQUARE WAVE GENERATOR");
    println!("ON/OFF CPU load for clear EM signature\n");

    let active = Arc::new(AtomicBool::new(false));
    let running = Arc::new(AtomicBool::new(true));
    
    let mut handles = vec![];
    for _ in 0..24 {
        let active = Arc::clone(&active);
        let running = Arc::clone(&running);
        
        let handle = thread::spawn(move || {
            let mut acc = 0u64;
            while running.load(Ordering::Relaxed) {
                if active.load(Ordering::Relaxed) {
                    // Full load
                    for _ in 0..1000 {
                        acc = acc.wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        acc ^= acc >> 32;
                    }
                } else {
                    // Idle
                    thread::sleep(Duration::from_millis(10));
                }
            }
            acc
        });
        
        handles.push(handle);
    }
    
    println!("⬜ Square wave: 10 cycles, 1s ON / 1s OFF");
    println!();
    
    let start = Instant::now();
    
    for cycle in 0..10 {
        println!("Cycle {}: ON", cycle + 1);
        active.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_secs(1));
        
        println!("Cycle {}: OFF", cycle + 1);
        active.store(false, Ordering::Relaxed);
        thread::sleep(Duration::from_secs(1));
    }
    
    running.store(false, Ordering::Relaxed);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n✅ Complete: {:?}", start.elapsed());
    println!("\n🎯 Square wave: WiFi should alternate between two states");
}
