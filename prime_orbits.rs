use std::f64::consts::PI;

fn main() {
    println!("🌀 Prime Orbits and ZOS Resonances");
    
    // Calculate orbits for ZOS primes
    let zos_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
    
    for &p in &zos_primes {
        let orbit = calculate_orbit(p);
        println!("\nPrime {}: orbit radius = {:.4}", p, orbit.radius);
        println!("  Period: {:.4}", orbit.period);
        println!("  Frequency: {:.4}", orbit.frequency);
        println!("  Resonance: {:.4}", orbit.resonance);
    }
    
    // Find resonances
    let resonances = find_resonances(&zos_primes);
    println!("\n🎵 Resonances found: {}", resonances.len());
    
    for res in resonances {
        println!("  {} ↔ {}: strength = {:.4}", res.p1, res.p2, res.strength);
    }
}

fn calculate_orbit(p: u64) -> Orbit {
    let p_f = p as f64;
    
    // Orbit radius: r = p / (2π)
    let radius = p_f / (2.0 * PI);
    
    // Period: T = 2π√(p)
    let period = 2.0 * PI * p_f.sqrt();
    
    // Frequency: f = 1/T
    let frequency = 1.0 / period;
    
    // Resonance strength: based on prime gaps
    let resonance = calculate_resonance(p);
    
    Orbit {
        prime: p,
        radius,
        period,
        frequency,
        resonance,
    }
}

fn calculate_resonance(p: u64) -> f64 {
    // Resonance based on position in ZOS sequence
    match p {
        2 => 1.0,      // Fundamental
        3 => 0.666,    // 2/3 harmonic
        5 => 0.4,      // 2/5 harmonic
        7 => 0.285,    // 2/7 harmonic
        11 => 0.181,   // 2/11 harmonic
        37 => 0.054,   // 2/37 - the break
        71 => 0.028,   // 2/71 - the boundary
        _ => 2.0 / p as f64,
    }
}

fn find_resonances(primes: &[u64]) -> Vec<Resonance> {
    let mut resonances = Vec::new();
    
    for i in 0..primes.len() {
        for j in i+1..primes.len() {
            let p1 = primes[i];
            let p2 = primes[j];
            
            let orbit1 = calculate_orbit(p1);
            let orbit2 = calculate_orbit(p2);
            
            // Check for resonance: frequency ratio is rational
            let ratio = orbit1.frequency / orbit2.frequency;
            
            // Find closest rational approximation
            if let Some((n, d)) = approximate_rational(ratio, 10) {
                if n <= 5 && d <= 5 {
                    // Strong resonance
                    let strength = 1.0 / (n + d) as f64;
                    resonances.push(Resonance {
                        p1,
                        p2,
                        ratio: (n, d),
                        strength,
                    });
                }
            }
        }
    }
    
    resonances.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
    resonances
}

fn approximate_rational(x: f64, max_denom: u64) -> Option<(u64, u64)> {
    // Continued fraction approximation
    let mut a = x.floor() as u64;
    let mut h1 = 1u64;
    let mut h2 = 0u64;
    let mut k1 = 0u64;
    let mut k2 = 1u64;
    let mut b = x - a as f64;
    
    for _ in 0..20 {
        if b < 1e-10 {
            return Some((a * h1 + h2, a * k1 + k2));
        }
        
        b = 1.0 / b;
        a = b.floor() as u64;
        
        let h = a * h1 + h2;
        let k = a * k1 + k2;
        
        if k > max_denom {
            return Some((h1, k1));
        }
        
        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;
        
        b = b - a as f64;
    }
    
    Some((h1, k1))
}

#[derive(Debug)]
struct Orbit {
    prime: u64,
    radius: f64,
    period: f64,
    frequency: f64,
    resonance: f64,
}

#[derive(Debug)]
struct Resonance {
    p1: u64,
    p2: u64,
    ratio: (u64, u64),
    strength: f64,
}
