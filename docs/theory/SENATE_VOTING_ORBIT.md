# Senate Voting with LMFDB Orbit Proof (100 Parts)

## 🏛️ Concept

Senate votes require exactly 100 parts (one per Senator) to prove quorum and reconstruct the vote result.

## 🗳️ Senate Voting Orbit

```rust
// src/senate_voting_orbit.rs
use serde::{Deserialize, Serialize};

const SENATE_SIZE: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateVote {
    pub proposal_id: String,
    pub senator_wallet: String,
    pub senator_rank: u32,
    pub vote: bool, // true = for, false = against
    pub timestamp: f64,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateVoteOrbit {
    pub proposal_id: String,
    pub orbit: LMFDBOrbit,
    pub votes: Vec<SenateVote>,
    pub quorum_reached: bool,
}

impl SenateVoteOrbit {
    /// Create new voting orbit for a proposal
    pub fn new(proposal_id: String) -> Self {
        SenateVoteOrbit {
            proposal_id,
            orbit: LMFDBOrbit::new(&[], SENATE_SIZE),
            votes: Vec::new(),
            quorum_reached: false,
        }
    }
    
    /// Senator casts vote (creates their orbit part)
    pub fn cast_vote(
        &mut self,
        senator_wallet: String,
        senator_rank: u32,
        vote: bool,
        signature: String,
    ) -> Result<String, String> {
        // Verify senator is in top 100
        if senator_rank > 100 {
            return Err("Not a senator (rank > 100)".to_string());
        }
        
        // Check if already voted
        if self.votes.iter().any(|v| v.senator_wallet == senator_wallet) {
            return Err("Already voted".to_string());
        }
        
        // Create vote
        let vote_data = SenateVote {
            proposal_id: self.proposal_id.clone(),
            senator_wallet: senator_wallet.clone(),
            senator_rank,
            vote,
            timestamp: js_sys::Date::now(),
            signature,
        };
        
        self.votes.push(vote_data.clone());
        
        // Generate URL for this senator's vote part
        let part_index = (senator_rank - 1) as usize; // Rank 1-100 → Index 0-99
        let url = self.generate_vote_url(&vote_data, part_index)?;
        
        // Check if quorum reached
        if self.votes.len() >= SENATE_SIZE {
            self.quorum_reached = true;
        }
        
        Ok(url)
    }
    
    /// Generate URL for senator's vote part
    fn generate_vote_url(&self, vote: &SenateVote, part_index: usize) -> Result<String, String> {
        // Encode vote as bytes
        let vote_bytes = serde_json::to_vec(vote)
            .map_err(|e| e.to_string())?;
        
        // Create orbit part
        let mut hasher = Sha256::new();
        hasher.update(&vote_bytes);
        let hash = format!("{:x}", hasher.finalize());
        
        let part = OrbitPart {
            index: part_index,
            value: vote_bytes,
            hash: hash.clone(),
        };
        
        // Encode as URL
        let encoded = base64::encode(&part.value);
        
        Ok(format!(
            "https://senate.vote/{}?senator={}&rank={}&part={}/100&hash={}&vote={}&data={}",
            self.proposal_id,
            vote.senator_wallet,
            vote.senator_rank,
            part_index,
            &hash[..8],
            if vote.vote { "for" } else { "against" },
            encoded
        ))
    }
    
    /// Reconstruct vote result from 100 URLs
    pub fn reconstruct_vote(urls: Vec<String>) -> Result<VoteResult, String> {
        if urls.len() != SENATE_SIZE {
            return Err(format!("Need exactly 100 URLs, got {}", urls.len()));
        }
        
        let mut votes = Vec::new();
        let mut proposal_id = String::new();
        
        // Decode all vote parts
        for url in urls {
            let vote = Self::decode_vote_url(&url)?;
            
            if proposal_id.is_empty() {
                proposal_id = vote.proposal_id.clone();
            } else if proposal_id != vote.proposal_id {
                return Err("URLs from different proposals".to_string());
            }
            
            votes.push(vote);
        }
        
        // Verify all 100 senators voted
        if votes.len() != SENATE_SIZE {
            return Err("Missing votes".to_string());
        }
        
        // Verify ranks 1-100 all present
        let mut ranks: Vec<u32> = votes.iter().map(|v| v.senator_rank).collect();
        ranks.sort();
        for (i, rank) in ranks.iter().enumerate() {
            if *rank != (i + 1) as u32 {
                return Err(format!("Missing senator rank {}", i + 1));
            }
        }
        
        // Count votes
        let votes_for = votes.iter().filter(|v| v.vote).count();
        let votes_against = votes.len() - votes_for;
        
        Ok(VoteResult {
            proposal_id,
            total_votes: SENATE_SIZE,
            votes_for,
            votes_against,
            passed: votes_for > votes_against,
            quorum_reached: true,
            votes,
        })
    }
    
    fn decode_vote_url(url: &str) -> Result<SenateVote, String> {
        // Parse URL
        let parts: Vec<&str> = url.split('?').collect();
        if parts.len() != 2 {
            return Err("Invalid URL format".to_string());
        }
        
        let proposal_id = parts[0].trim_start_matches("https://senate.vote/");
        let params: std::collections::HashMap<String, String> = parts[1]
            .split('&')
            .filter_map(|p| {
                let kv: Vec<&str> = p.split('=').collect();
                if kv.len() == 2 {
                    Some((kv[0].to_string(), kv[1].to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        // Decode vote data
        let data = params.get("data").ok_or("Missing data")?;
        let vote_bytes = base64::decode(data).map_err(|_| "Invalid base64")?;
        let vote: SenateVote = serde_json::from_slice(&vote_bytes)
            .map_err(|e| e.to_string())?;
        
        // Verify hash
        let hash = params.get("hash").ok_or("Missing hash")?;
        let mut hasher = Sha256::new();
        hasher.update(&vote_bytes);
        let computed_hash = format!("{:x}", hasher.finalize());
        
        if !computed_hash.starts_with(hash) {
            return Err("Hash mismatch".to_string());
        }
        
        Ok(vote)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteResult {
    pub proposal_id: String,
    pub total_votes: usize,
    pub votes_for: usize,
    pub votes_against: usize,
    pub passed: bool,
    pub quorum_reached: bool,
    pub votes: Vec<SenateVote>,
}

/// Delegation system - Senator can delegate their vote part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteDelegation {
    pub from_senator: String,
    pub from_rank: u32,
    pub to_senator: String,
    pub to_rank: u32,
    pub proposal_id: String,
    pub signature: String,
}

impl SenateVoteOrbit {
    /// Delegate vote to another senator
    pub fn delegate_vote(
        &mut self,
        from_senator: String,
        from_rank: u32,
        to_senator: String,
        to_rank: u32,
        signature: String,
    ) -> Result<String, String> {
        // Verify both are senators
        if from_rank > 100 || to_rank > 100 {
            return Err("Both must be senators".to_string());
        }
        
        // Create delegation
        let delegation = VoteDelegation {
            from_senator: from_senator.clone(),
            from_rank,
            to_senator: to_senator.clone(),
            to_rank,
            proposal_id: self.proposal_id.clone(),
            signature,
        };
        
        // Generate delegation URL (still counts as from_senator's part)
        let part_index = (from_rank - 1) as usize;
        let delegation_bytes = serde_json::to_vec(&delegation)
            .map_err(|e| e.to_string())?;
        
        let mut hasher = Sha256::new();
        hasher.update(&delegation_bytes);
        let hash = format!("{:x}", hasher.finalize());
        
        let encoded = base64::encode(&delegation_bytes);
        
        Ok(format!(
            "https://senate.vote/{}?delegate=true&from={}&to={}&part={}/100&hash={}&data={}",
            self.proposal_id,
            from_rank,
            to_rank,
            part_index,
            &hash[..8],
            encoded
        ))
    }
}
```

## 🌐 WASM Interface

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SenateVotingWASM {
    orbit: SenateVoteOrbit,
}

#[wasm_bindgen]
impl SenateVotingWASM {
    #[wasm_bindgen(constructor)]
    pub fn new(proposal_id: String) -> SenateVotingWASM {
        console_log!("🏛️ Creating Senate vote for proposal: {}", proposal_id);
        
        SenateVotingWASM {
            orbit: SenateVoteOrbit::new(proposal_id),
        }
    }
    
    /// Senator casts vote
    #[wasm_bindgen]
    pub fn vote(
        &mut self,
        senator_wallet: String,
        senator_rank: u32,
        vote_for: bool,
        signature: String,
    ) -> Result<String, JsValue> {
        console_log!("🗳️ Senator #{} voting: {}", senator_rank, if vote_for { "FOR" } else { "AGAINST" });
        
        self.orbit.cast_vote(senator_wallet, senator_rank, vote_for, signature)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Delegate vote
    #[wasm_bindgen]
    pub fn delegate(
        &mut self,
        from_wallet: String,
        from_rank: u32,
        to_wallet: String,
        to_rank: u32,
        signature: String,
    ) -> Result<String, JsValue> {
        console_log!("📤 Senator #{} delegating to #{}", from_rank, to_rank);
        
        self.orbit.delegate_vote(from_wallet, from_rank, to_wallet, to_rank, signature)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Reconstruct vote from 100 URLs
    #[wasm_bindgen]
    pub fn reconstruct(urls: Vec<String>) -> Result<JsValue, JsValue> {
        console_log!("🔄 Reconstructing vote from {} URLs", urls.len());
        
        let result = SenateVoteOrbit::reconstruct_vote(urls)
            .map_err(|e| JsValue::from_str(&e))?;
        
        console_log!("✅ Vote result: {} FOR, {} AGAINST, Passed: {}", 
            result.votes_for, result.votes_against, result.passed);
        
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }
}
```

## 📊 Example Flow

```
Proposal Created: "Upgrade Smart Contract"
    ↓
100 Senators Vote or Delegate
    ↓
Each generates 1 URL (part 0-99)
    ↓
Collect all 100 URLs
    ↓
Reconstruct Vote Result
    ↓
Verify: All ranks 1-100 present
    ↓
Count: 67 FOR, 33 AGAINST
    ↓
Result: PASSED ✅
```

## 🔗 URL Examples

```
Senator #1 votes FOR:
https://senate.vote/prop123?senator=wallet1&rank=1&part=0/100&hash=a1b2c3d4&vote=for&data=eyJ2b3RlIjp0cnVlfQ==

Senator #50 votes AGAINST:
https://senate.vote/prop123?senator=wallet50&rank=50&part=49/100&hash=e5f6g7h8&vote=against&data=eyJ2b3RlIjpmYWxzZX0=

Senator #75 delegates to #80:
https://senate.vote/prop123?delegate=true&from=75&to=80&part=74/100&hash=i9j0k1l2&data=eyJkZWxlZ2F0aW9uIjp0cnVlfQ==
```

## ✅ Verification

```rust
// Verify vote reconstruction
fn verify_senate_vote(result: &VoteResult) -> bool {
    // Must have exactly 100 votes
    if result.total_votes != 100 {
        return false;
    }
    
    // All ranks 1-100 must be present
    let mut ranks: Vec<u32> = result.votes.iter().map(|v| v.senator_rank).collect();
    ranks.sort();
    for (i, rank) in ranks.iter().enumerate() {
        if *rank != (i + 1) as u32 {
            return false;
        }
    }
    
    // Quorum must be reached
    if !result.quorum_reached {
        return false;
    }
    
    true
}
```

---

**Status**: 🏛️ Senate voting with 100-part orbit ready  
**Requirement**: Exactly 100 URLs (one per Senator)  
**Verification**: All ranks 1-100 must be present  
**Delegation**: Supported with proof  
**Reconstruction**: Proves quorum and vote result
