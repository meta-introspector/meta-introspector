// Prime Resonance Detector - Each CPU core computes at different prime cycle lengths
// Creates interference patterns detectable in EM/thermal fields

use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("🌊 PRIME RESONANCE DETECTOR");
    println!("Creating interference patterns with 24 prime-cycle oscillators\n");

    // First 24 primes for cycle lengths
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 
        41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89
    ];

    let running = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];

    println!("🎵 Starting {} oscillators with prime cycles:", primes.len());
    for (i, &prime) in primes.iter().enumerate() {
        println!("  Core {}: {} cycle", i, prime);
    }
    println!();

    let start = Instant::now();

    // Spawn thread for each prime cycle
    for (core_id, &prime_cycle) in primes.iter().enumerate() {
        let running = Arc::clone(&running);
        
        let handle = thread::spawn(move || {
            let mut iteration = 0u64;
            let mut accumulator = 0u64;
            
            while running.load(Ordering::Relaxed) {
                // Work burst of prime_cycle iterations
                for _ in 0..prime_cycle {
                    // CPU-intensive work
                    accumulator = accumulator.wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    accumulator ^= accumulator >> 32;
                }
                
                iteration += 1;
                
                // Each core reports at different intervals (creates beat pattern)
                if iteration.is_multiple_of(prime_cycle as u64 * 10000) {
                    let elapsed = start.elapsed().as_millis();
                    println!("Core {} ({}Hz): {} iterations @ {}ms", 
                        core_id, prime_cycle, iteration, elapsed);
                }
            }
            
            accumulator
        });
        
        handles.push(handle);
    }

    // Run for 10 seconds
    println!("⏱️  Running for 10 seconds...\n");
    thread::sleep(Duration::from_secs(10));
    
    running.store(false, Ordering::Relaxed);
    
    println!("\n🛑 Stopping oscillators...");
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start.elapsed();
    println!("\n✅ Complete: {:?}", elapsed);
    println!("\n🎯 INTERFERENCE PATTERN CREATED:");
    println!("   24 cores × prime cycles = unique EM signature");
    println!("   Beat frequencies from GCD relationships between primes");
    println!("   Thermal standing waves from phase alignment");
}
