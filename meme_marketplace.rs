// Meme marketplace: Atomic swaps and auctions
// Store world in parquet, trade programs as memes

use std::collections::HashMap;

pub type MemeID = u64;
pub type Address = String;

#[derive(Debug, Clone)]
pub struct Meme {
    pub id: MemeID,
    pub godel_number: u64,
    pub emoji: String,
    pub code: Vec<u8>,
    pub complexity: usize,
    pub fitness: f64,
    pub rarity: f64,
    pub generation: usize,
    pub owner: Address,
    pub price: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SwapOffer {
    pub offerer: Address,
    pub offer_meme: MemeID,
    pub want_meme: MemeID,
    pub expiry: i64,
}

#[derive(Debug, Clone)]
pub struct Auction {
    pub meme_id: MemeID,
    pub seller: Address,
    pub start_price: u64,
    pub current_bid: u64,
    pub highest_bidder: Option<Address>,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone)]
pub struct Bid {
    pub bidder: Address,
    pub amount: u64,
    pub timestamp: i64,
}

pub struct Marketplace {
    memes: HashMap<MemeID, Meme>,
    for_sale: HashMap<MemeID, u64>,
    auctions: HashMap<MemeID, Auction>,
    swap_offers: HashMap<MemeID, Vec<SwapOffer>>,
    current_user: Address,
}

impl Marketplace {
    pub fn new(user: Address) -> Self {
        Self {
            memes: HashMap::new(),
            for_sale: HashMap::new(),
            auctions: HashMap::new(),
            swap_offers: HashMap::new(),
            current_user: user,
        }
    }
    
    pub fn add_meme(&mut self, meme: Meme) {
        self.memes.insert(meme.id, meme);
    }
    
    pub fn list_for_sale(&mut self, meme_id: MemeID, price: u64) -> Result<(), String> {
        let meme = self.memes.get_mut(&meme_id).ok_or("Meme not found")?;
        
        if meme.owner != self.current_user {
            return Err("Not owner".to_string());
        }
        
        meme.price = Some(price);
        self.for_sale.insert(meme_id, price);
        
        Ok(())
    }
    
    pub fn buy(&mut self, meme_id: MemeID, buyer: Address) -> Result<(), String> {
        let _price = *self.for_sale.get(&meme_id).ok_or("Not for sale")?;
        let meme = self.memes.get_mut(&meme_id).ok_or("Meme not found")?;
        
        // Transfer ownership
        meme.owner = buyer;
        meme.price = None;
        self.for_sale.remove(&meme_id);
        
        Ok(())
    }
    
    pub fn create_swap_offer(
        &mut self,
        offer: MemeID,
        want: MemeID,
        expiry: i64
    ) -> Result<SwapOffer, String> {
        let meme = self.memes.get(&offer).ok_or("Meme not found")?;
        
        if meme.owner != self.current_user {
            return Err("Not owner".to_string());
        }
        
        let swap = SwapOffer {
            offerer: self.current_user.clone(),
            offer_meme: offer,
            want_meme: want,
            expiry,
        };
        
        self.swap_offers.entry(want).or_default().push(swap.clone());
        
        Ok(swap)
    }
    
    pub fn accept_swap(&mut self, swap: &SwapOffer) -> Result<(), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if now > swap.expiry {
            return Err("Swap expired".to_string());
        }
        
        // Get owners first to avoid double borrow
        let meme1_owner = self.memes.get(&swap.offer_meme).ok_or("Meme not found")?.owner.clone();
        let meme2_owner = self.memes.get(&swap.want_meme).ok_or("Meme not found")?.owner.clone();
        
        if meme1_owner != swap.offerer {
            return Err("Offerer no longer owns meme".to_string());
        }
        if meme2_owner != self.current_user {
            return Err("You don't own wanted meme".to_string());
        }
        
        // Atomic swap
        self.memes.get_mut(&swap.offer_meme).unwrap().owner = meme2_owner;
        self.memes.get_mut(&swap.want_meme).unwrap().owner = meme1_owner;
        
        Ok(())
    }
    
    pub fn create_auction(
        &mut self,
        meme_id: MemeID,
        start_price: u64,
        duration: i64
    ) -> Result<Auction, String> {
        let meme = self.memes.get(&meme_id).ok_or("Meme not found")?;
        
        if meme.owner != self.current_user {
            return Err("Not owner".to_string());
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let auction = Auction {
            meme_id,
            seller: self.current_user.clone(),
            start_price,
            current_bid: start_price,
            highest_bidder: None,
            start_time: now,
            end_time: now + duration,
        };
        
        self.auctions.insert(meme_id, auction.clone());
        
        Ok(auction)
    }
    
    pub fn place_bid(&mut self, meme_id: MemeID, amount: u64) -> Result<(), String> {
        let auction = self.auctions.get_mut(&meme_id).ok_or("Auction not found")?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if now > auction.end_time {
            return Err("Auction ended".to_string());
        }
        
        if amount <= auction.current_bid {
            return Err("Bid too low".to_string());
        }
        
        auction.current_bid = amount;
        auction.highest_bidder = Some(self.current_user.clone());
        
        Ok(())
    }
    
    pub fn finalize_auction(&mut self, meme_id: MemeID) -> Result<(), String> {
        let auction = self.auctions.get(&meme_id).ok_or("Auction not found")?.clone();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if now < auction.end_time {
            return Err("Auction still active".to_string());
        }
        
        if let Some(winner) = auction.highest_bidder {
            let meme = self.memes.get_mut(&meme_id).ok_or("Meme not found")?;
            meme.owner = winner;
        }
        
        self.auctions.remove(&meme_id);
        
        Ok(())
    }
}

pub fn compute_rarity(meme: &Meme) -> f64 {
    let orbit = meme.godel_number % 71;
    
    let mut rarity = 1.0;
    
    // Prime orbits are rarer
    if is_prime(orbit) {
        rarity *= 10.0;
    }
    
    // Low complexity is rare
    if meme.complexity < 10 {
        rarity *= 5.0;
    }
    
    // High fitness is rare
    if meme.fitness > 90.0 {
        rarity *= 8.0;
    }
    
    rarity
}

pub fn compute_price(meme: &Meme) -> u64 {
    let base_price = 100;
    let rarity_multiplier = meme.rarity;
    let fitness_multiplier = meme.fitness / 10.0;
    
    (base_price as f64 * rarity_multiplier * fitness_multiplier) as u64
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n.is_multiple_of(2) { return false; }
    
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n.is_multiple_of(i) { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_marketplace() {
        let mut market = Marketplace::new("alice".to_string());
        
        let meme = Meme {
            id: 1,
            godel_number: 42,
            emoji: "🧬".to_string(),
            code: vec![1, 2, 3],
            complexity: 5,
            fitness: 80.0,
            rarity: 5.0,
            generation: 0,
            owner: "alice".to_string(),
            price: None,
        };
        
        market.add_meme(meme);
        assert!(market.list_for_sale(1, 100).is_ok());
    }
    
    #[test]
    fn test_auction() {
        let mut market = Marketplace::new("alice".to_string());
        
        let meme = Meme {
            id: 1,
            godel_number: 42,
            emoji: "🧬".to_string(),
            code: vec![1, 2, 3],
            complexity: 5,
            fitness: 80.0,
            rarity: 5.0,
            generation: 0,
            owner: "alice".to_string(),
            price: None,
        };
        
        market.add_meme(meme);
        assert!(market.create_auction(1, 100, 3600).is_ok());
    }
}
