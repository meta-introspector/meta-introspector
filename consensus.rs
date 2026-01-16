use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemeContract {
    pub godel_number: String,
    pub emoji: String,
    pub wasm: Vec<u8>,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    pub peer_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consensus {
    pub contracts: HashMap<String, MemeContract>,
    pub emoji_map: HashMap<String, String>,
}

impl Consensus {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            emoji_map: HashMap::new(),
        }
    }
    
    pub fn propose(&mut self, godel: String, emoji: String, wasm: Vec<u8>) -> MemeContract {
        let contract = MemeContract {
            godel_number: godel.clone(),
            emoji: emoji.clone(),
            wasm,
            signatures: vec![],
        };
        
        self.contracts.insert(godel.clone(), contract.clone());
        contract
    }
    
    pub fn sign(&mut self, godel: &str, peer_id: String) -> bool {
        if let Some(contract) = self.contracts.get_mut(godel) {
            contract.signatures.push(Signature {
                peer_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
            
            // Consensus at 3+ signatures
            if contract.signatures.len() >= 3 {
                self.emoji_map.insert(godel.to_string(), contract.emoji.clone());
                println!("✅ Consensus: {} = {}", godel, contract.emoji);
                return true;
            }
        }
        false
    }
    
    pub fn get_emoji(&self, godel: &str) -> Option<&String> {
        self.emoji_map.get(godel)
    }
    
    pub fn execute(&self, emoji: &str) -> Option<&Vec<u8>> {
        for (godel, e) in &self.emoji_map {
            if e == emoji {
                return self.contracts.get(godel).map(|c| &c.wasm);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consensus() {
        let mut consensus = Consensus::new();
        
        // Propose: godel -> 🚀
        consensus.propose("abc123".to_string(), "🚀".to_string(), vec![1,2,3]);
        
        // 3 peers sign
        consensus.sign("abc123", "peer1".to_string());
        consensus.sign("abc123", "peer2".to_string());
        let reached = consensus.sign("abc123", "peer3".to_string());
        
        assert!(reached);
        assert_eq!(consensus.get_emoji("abc123"), Some(&"🚀".to_string()));
        assert_eq!(consensus.execute("🚀"), Some(&vec![1,2,3]));
    }
}
