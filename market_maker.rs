// Market Maker: Facilitates meme trades by providing liquidity
// Holds inventory and quotes bid/ask spreads

use crate::meme_marketplace::Meme;
use std::collections::HashMap;

pub struct MarketMaker {
    pub node_id: usize,
    pub inventory: HashMap<u64, Vec<Meme>>,  // meme_id -> copies
    pub balance: u64,
    pub bid_prices: HashMap<u64, u64>,  // meme_id -> bid price
    pub ask_prices: HashMap<u64, u64>,  // meme_id -> ask price
    pub spread: f64,  // Bid-ask spread (0.1 = 10%)
}

impl MarketMaker {
    pub fn new(node_id: usize, initial_balance: u64) -> Self {
        Self {
            node_id,
            inventory: HashMap::new(),
            balance: initial_balance,
            bid_prices: HashMap::new(),
            ask_prices: HashMap::new(),
            spread: 0.2,  // 20% spread
        }
    }
    
    /// Quote prices for a meme based on fitness and inventory
    pub fn quote(&mut self, meme: &Meme) -> (u64, u64) {
        let fair_value = (meme.fitness * 100.0) as u64;
        
        // Adjust for inventory: more inventory = lower ask, higher bid
        let inventory_count = self.inventory.get(&meme.id).map(|v| v.len()).unwrap_or(0);
        let inventory_adjustment = 1.0 - (inventory_count as f64 * 0.05).min(0.5);
        
        let bid = (fair_value as f64 * (1.0 - self.spread / 2.0) * inventory_adjustment) as u64;
        let ask = (fair_value as f64 * (1.0 + self.spread / 2.0) / inventory_adjustment) as u64;
        
        self.bid_prices.insert(meme.id, bid);
        self.ask_prices.insert(meme.id, ask);
        
        (bid, ask)
    }
    
    /// Buy meme from seller at bid price
    pub fn buy(&mut self, meme: Meme) -> Option<u64> {
        let bid = self.bid_prices.get(&meme.id).copied().unwrap_or(0);
        
        if self.balance >= bid {
            self.balance -= bid;
            self.inventory.entry(meme.id).or_insert_with(Vec::new).push(meme);
            Some(bid)
        } else {
            None
        }
    }
    
    /// Sell meme to buyer at ask price
    pub fn sell(&mut self, meme_id: u64) -> Option<(Meme, u64)> {
        let ask = self.ask_prices.get(&meme_id).copied().unwrap_or(0);
        
        if let Some(inventory) = self.inventory.get_mut(&meme_id) {
            if let Some(meme) = inventory.pop() {
                self.balance += ask;
                return Some((meme, ask));
            }
        }
        None
    }
    
    /// Adjust spread based on market conditions
    pub fn adjust_spread(&mut self, volatility: f64) {
        // Higher volatility = wider spread
        self.spread = (0.1 + volatility * 0.5).min(0.5);
    }
    
    /// Report market maker stats
    pub fn report(&self) {
        println!("📊 Market Maker Node {}", self.node_id);
        println!("   Balance: {} coins", self.balance);
        println!("   Inventory: {} unique memes", self.inventory.len());
        println!("   Total holdings: {} memes", 
                 self.inventory.values().map(|v| v.len()).sum::<usize>());
        println!("   Spread: {:.1}%", self.spread * 100.0);
    }
}
