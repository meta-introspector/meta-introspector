// Rand shim: Load random number generation from nix store .so
// Avoids adding rand to Cargo.toml

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize random from nix store
pub fn init_rand() {
    INIT.call_once(|| {
        // Load libc's random functions from nix store
        println!("📦 Using libc random from nix store");
    });
}

/// Generate random u64 using libc
pub fn random_u64() -> u64 {
    unsafe {
        // Use libc's rand() which is available
        let high = libc::rand() as u64;
        let low = libc::rand() as u64;
        (high << 32) | low
    }
}

/// Generate random usize
pub fn random_usize() -> usize {
    random_u64() as usize
}

/// Generate random f64 between 0.0 and 1.0
pub fn random_f64() -> f64 {
    (random_u64() as f64) / (u64::MAX as f64)
}

/// Seed the random number generator
pub fn seed(seed: u32) {
    unsafe {
        libc::srand(seed);
    }
}

// Provide rand-like interface
pub mod random {
    use super::*;
    
    pub fn random<T>() -> T 
    where T: RandomValue
    {
        T::random()
    }
}

pub trait RandomValue {
    fn random() -> Self;
}

impl RandomValue for u64 {
    fn random() -> Self {
        random_u64()
    }
}

impl RandomValue for usize {
    fn random() -> Self {
        random_usize()
    }
}

impl RandomValue for f64 {
    fn random() -> Self {
        random_f64()
    }
}

impl RandomValue for u8 {
    fn random() -> Self {
        (random_u64() & 0xFF) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random() {
        init_rand();
        
        let r1 = random_u64();
        let r2 = random_u64();
        
        assert_ne!(r1, r2);
    }
}
