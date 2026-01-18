// Bach - Prime Resonance Detector
// 24 CPU cores computing at different prime cycle lengths
// Creates interference patterns like Bach's fugues - multiple voices in counterpoint

use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("🎼 BACH - Prime Resonance Detector");
    println!("24 voices in computational counterpoint\n");

    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 
        41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89
    ];

    let running = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];

    println!("🎵 Voices:");
    for (i, &prime) in primes.iter().enumerate() {
        println!("  Voice {}: {} cycle", i, prime);
    }
    println!();

    let start = Instant::now();

    for (voice, &prime_cycle) in primes.iter().enumerate() {
        let running = Arc::clone(&running);
        
        let handle = thread::spawn(move || {
            let mut iteration = 0u64;
            let mut acc = 0u64;
            
            while running.load(Ordering::Relaxed) {
                for _ in 0..prime_cycle {
                    acc = acc.wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    acc ^= acc >> 32;
                }
                iteration += 1;
                
                if iteration % (prime_cycle as u64 * 10000) == 0 {
                    println!("Voice {} ({}Hz): {} @ {}ms", 
                        voice, prime_cycle, iteration, start.elapsed().as_millis());
                }
            }
            acc
        });
        
        handles.push(handle);
    }

    println!("⏱️  Playing for 10 seconds...\n");
    thread::sleep(Duration::from_secs(10));
    
    running.store(false, Ordering::Relaxed);
    
    println!("\n🛑 Stopping...");
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n✅ Complete: {:?}", start.elapsed());
    println!("\n🎯 Interference pattern created:");
    println!("   24 prime cycles = unique EM signature");
    println!("   Beat frequencies from prime relationships");
}
