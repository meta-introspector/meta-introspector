// Triangle Wave Generator - Prove EM modulation with predictable waveform
// Ramp CPU load up and down to create triangle wave in WiFi signal

use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("📐 TRIANGLE WAVE GENERATOR");
    println!("Creating predictable EM waveform\n");

    let intensity = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicUsize::new(1));
    
    // Spawn worker threads
    let mut handles = vec![];
    for _ in 0..24 {
        let intensity = Arc::clone(&intensity);
        let running = Arc::clone(&running);
        
        let handle = thread::spawn(move || {
            let mut acc = 0u64;
            while running.load(Ordering::Relaxed) == 1 {
                let current_intensity = intensity.load(Ordering::Relaxed);
                
                // Work proportional to intensity (0-100)
                for _ in 0..current_intensity {
                    acc = acc.wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    acc ^= acc >> 32;
                }
                
                // Small sleep to allow intensity changes
                if current_intensity < 10 {
                    thread::sleep(Duration::from_micros(100));
                }
            }
            acc
        });
        
        handles.push(handle);
    }
    
    println!("🔺 Triangle wave: 5 cycles, 2s period");
    println!();
    
    let start = Instant::now();
    
    // Generate 5 triangle wave cycles
    for cycle in 0..5 {
        println!("Cycle {}: Ramp UP", cycle + 1);
        
        // Ramp up: 0 -> 100 over 1 second
        for step in 0..=100 {
            intensity.store(step, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(10));
        }
        
        println!("Cycle {}: Ramp DOWN", cycle + 1);
        
        // Ramp down: 100 -> 0 over 1 second
        for step in (0..=100).rev() {
            intensity.store(step, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    running.store(0, Ordering::Relaxed);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n✅ Complete: {:?}", start.elapsed());
    println!("\n🎯 Triangle wave generated - check WiFi signal for correlation");
}
