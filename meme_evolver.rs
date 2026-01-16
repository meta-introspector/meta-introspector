// Meme Evolution: Buy, evolve, combine, sell for profit

use crate::meme_marketplace::Meme;
use crate::rand_shim::{random_u64, random_f64};

pub struct MemeEvolver {
    pub node_id: usize,
}

impl MemeEvolver {
    pub fn new(node_id: usize) -> Self {
        Self { node_id }
    }
    
    /// Evolve a meme to next level: increase fitness, complexity
    pub fn evolve(&self, meme: &Meme) -> Meme {
        let mut evolved = meme.clone();
        
        // Evolution increases fitness and complexity
        evolved.fitness *= 1.5;  // 50% fitness boost
        evolved.complexity += 10;
        evolved.generation += 1;
        evolved.godel_number = random_u64();  // New Gödel number
        
        // Code grows slightly (mutation)
        let mut new_code = evolved.code.clone();
        new_code.extend_from_slice(&[random_u64() as u8; 8]);
        evolved.code = new_code;
        
        println!("  [Node {}] 🧬 Evolved meme {} → fitness {:.2} → {:.2}", 
                 self.node_id, meme.id, meme.fitness, evolved.fitness);
        
        evolved
    }
    
    /// Combine two memes: create hybrid with best traits
    pub fn combine(&self, meme1: &Meme, meme2: &Meme) -> Meme {
        let mut hybrid = Meme {
            id: random_u64(),
            godel_number: meme1.godel_number ^ meme2.godel_number,  // XOR Gödel numbers
            emoji: format!("{}+{}", meme1.emoji, meme2.emoji),
            code: Vec::new(),
            complexity: (meme1.complexity + meme2.complexity) / 2,
            fitness: (meme1.fitness + meme2.fitness) * 1.2,  // 20% synergy bonus
            rarity: (meme1.rarity + meme2.rarity) / 2.0,
            generation: meme1.generation.max(meme2.generation) + 1,
            owner: meme1.owner.clone(),
            price: None,
        };
        
        // Combine code: take best parts
        let split = meme1.code.len() / 2;
        hybrid.code.extend_from_slice(&meme1.code[..split]);
        hybrid.code.extend_from_slice(&meme2.code[split.min(meme2.code.len())..]);
        
        println!("  [Node {}] 🔬 Combined memes {} + {} → hybrid {} (fitness {:.2})", 
                 self.node_id, meme1.id, meme2.id, hybrid.id, hybrid.fitness);
        
        hybrid
    }
    
    /// Calculate profit potential: evolved_value - cost
    pub fn profit_potential(&self, meme: &Meme, buy_price: u64) -> i64 {
        let evolved_value = (meme.fitness * 1.5 * 100.0) as u64;
        evolved_value as i64 - buy_price as i64
    }
    
    /// Should we buy this meme to evolve and flip?
    pub fn should_buy_to_flip(&self, meme: &Meme, price: u64, balance: u64) -> bool {
        if balance < price {
            return false;
        }
        
        let profit = self.profit_potential(meme, price);
        
        // Buy if profit > 50% of price
        profit > (price as i64 / 2)
    }
}

/// Track a profitable trade sequence
#[derive(Debug, Clone)]
pub struct TradeSequence {
    pub node_id: usize,
    pub bought_meme_id: u64,
    pub buy_price: u64,
    pub evolved_meme_id: u64,
    pub sell_price: u64,
    pub profit: i64,
    pub strategy: String,  // "evolve" or "combine"
}

impl TradeSequence {
    pub fn report(&self) {
        println!("\n💰 Profitable Trade by Node {}:", self.node_id);
        println!("   Strategy: {}", self.strategy);
        println!("   Bought meme {} for {} coins", self.bought_meme_id, self.buy_price);
        println!("   Evolved to meme {}", self.evolved_meme_id);
        println!("   Sold for {} coins", self.sell_price);
        println!("   Profit: {} coins ({:.1}% ROI)", 
                 self.profit, 
                 (self.profit as f64 / self.buy_price as f64) * 100.0);
    }
}
