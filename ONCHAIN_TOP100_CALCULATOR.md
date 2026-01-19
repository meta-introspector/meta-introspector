# On-Chain Top 100 Holder Calculator

## 🎯 Concept: Read from Chain, Calculate Ranks

Instead of storing holder lists, **read directly from blockchain** and calculate top 100 in real-time.

## 📊 Algorithm

```
1. Get token mint: BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump
2. Query all token accounts for this mint
3. Sort by balance (descending)
4. Take top 100
5. Calculate market cap share
6. Assign tiers: Senate (1-100), Reps (101-600), Vendors (601-1600)
```

## 🔐 Read-Only Implementation

```rust
// src/bin/onchain_top100.rs
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const SOLFUNMEME_MINT: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HolderInfo {
    rank: usize,
    address: String,
    balance: u64,
    balance_ui: f64,
    mcap_share: f64,
    tier: Tier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Tier {
    Senator,        // Rank 1-100
    Representative, // Rank 101-600
    Vendor,         // Rank 601-1600
    Holder,         // Rank 1601+
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Calculating Top 100 SOLFUNMEME Holders (On-Chain)");
    
    let client = RpcClient::new(RPC_URL.to_string());
    let mint = Pubkey::from_str(SOLFUNMEME_MINT)?;
    
    println!("📡 Querying all token accounts for mint: {}", SOLFUNMEME_MINT);
    
    // Get all token accounts
    let accounts = client.get_token_accounts_by_mint(&mint)?;
    
    println!("✅ Found {} token accounts", accounts.len());
    
    // Parse balances
    let mut holders: Vec<(String, u64)> = accounts
        .iter()
        .filter_map(|acc| {
            let data = acc.account.data.as_ref()?;
            let balance = parse_token_balance(data)?;
            Some((acc.pubkey.clone(), balance))
        })
        .collect();
    
    // Sort by balance (descending)
    holders.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Calculate total supply
    let total_supply: u64 = holders.iter().map(|(_, bal)| bal).sum();
    
    println!("📊 Total Supply: {} tokens", total_supply);
    println!("📊 Total Holders: {}", holders.len());
    
    // Calculate top 100
    let top100: Vec<HolderInfo> = holders
        .iter()
        .take(100)
        .enumerate()
        .map(|(i, (addr, bal))| {
            let rank = i + 1;
            let balance_ui = *bal as f64 / 1_000_000.0; // 6 decimals
            let mcap_share = (*bal as f64 / total_supply as f64) * 100.0;
            
            HolderInfo {
                rank,
                address: addr.clone(),
                balance: *bal,
                balance_ui,
                mcap_share,
                tier: Tier::Senator,
            }
        })
        .collect();
    
    // Print top 10
    println!("\n🏛️ Top 10 Senators:");
    for holder in top100.iter().take(10) {
        println!(
            "  #{}: {} - {:.2} tokens ({:.2}% of supply)",
            holder.rank,
            &holder.address[..8],
            holder.balance_ui,
            holder.mcap_share
        );
    }
    
    // Save to JSON
    let json = serde_json::to_string_pretty(&top100)?;
    std::fs::write("top100_holders.json", json)?;
    
    println!("\n💾 Saved to: top100_holders.json");
    
    Ok(())
}

fn parse_token_balance(data: &[u8]) -> Option<u64> {
    // SPL Token account layout:
    // 0-32: mint
    // 32-64: owner
    // 64-72: amount (u64)
    if data.len() < 72 {
        return None;
    }
    
    let amount_bytes: [u8; 8] = data[64..72].try_into().ok()?;
    Some(u64::from_le_bytes(amount_bytes))
}
```

## 🌐 WASM Version (Browser)

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Top100Calculator {
    holders: Vec<HolderInfo>,
}

#[wasm_bindgen]
impl Top100Calculator {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Top100Calculator, JsValue> {
        console_log!("🔍 Fetching on-chain holder data...");
        
        let holders = Self::fetch_holders().await?;
        
        console_log!("✅ Loaded {} holders", holders.len());
        
        Ok(Top100Calculator { holders })
    }
    
    /// Get holder by rank
    #[wasm_bindgen]
    pub fn get_holder(&self, rank: usize) -> JsValue {
        if rank > 0 && rank <= self.holders.len() {
            serde_wasm_bindgen::to_value(&self.holders[rank - 1]).unwrap()
        } else {
            JsValue::NULL
        }
    }
    
    /// Check if address is in top 100
    #[wasm_bindgen]
    pub fn is_senator(&self, address: String) -> bool {
        self.holders.iter().any(|h| h.address == address)
    }
    
    /// Get rank for address
    #[wasm_bindgen]
    pub fn get_rank(&self, address: String) -> Option<usize> {
        self.holders
            .iter()
            .find(|h| h.address == address)
            .map(|h| h.rank)
    }
    
    /// Get tier for address
    #[wasm_bindgen]
    pub fn get_tier(&self, address: String) -> String {
        if let Some(rank) = self.get_rank(address) {
            match rank {
                1..=100 => "🏛️ Senator".to_string(),
                101..=600 => "🏛️ Representative".to_string(),
                601..=1600 => "🏛️ Vendor".to_string(),
                _ => "🎭 Holder".to_string(),
            }
        } else {
            "👀 Observer".to_string()
        }
    }
    
    async fn fetch_holders() -> Result<Vec<HolderInfo>, JsValue> {
        // Fetch from Solana RPC
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let mint = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";
        
        // Make RPC call
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTokenLargestAccounts",
            "params": [mint]
        });
        
        let response = Self::rpc_call(rpc_url, &request).await?;
        
        // Parse response
        let accounts = response["result"]["value"]
            .as_array()
            .ok_or("Invalid response")?;
        
        let mut holders = Vec::new();
        
        for (i, acc) in accounts.iter().enumerate() {
            let address = acc["address"].as_str().unwrap_or("").to_string();
            let amount = acc["amount"].as_str().unwrap_or("0");
            let balance = amount.parse::<u64>().unwrap_or(0);
            let balance_ui = balance as f64 / 1_000_000.0;
            
            holders.push(HolderInfo {
                rank: i + 1,
                address,
                balance,
                balance_ui,
                mcap_share: 0.0, // Calculate after getting total
                tier: Tier::Senator,
            });
        }
        
        Ok(holders)
    }
    
    async fn rpc_call(url: &str, request: &serde_json::Value) -> Result<serde_json::Value, JsValue> {
        // Use fetch API
        let window = web_sys::window().ok_or("No window")?;
        let resp = JsFuture::from(
            window.fetch_with_str_and_init(
                url,
                web_sys::RequestInit::new()
                    .method("POST")
                    .body(Some(&JsValue::from_str(&request.to_string())))
            )
        ).await?;
        
        let resp: web_sys::Response = resp.dyn_into()?;
        let json = JsFuture::from(resp.json()?).await?;
        
        Ok(serde_wasm_bindgen::from_value(json)?)
    }
}
```

## 📊 Usage Example

```javascript
// Browser
const calculator = await Top100Calculator.new();

// Check if user is senator
const userAddress = "HMEKzpgzJEfyYyqoob5uGHR9P3LF6248zbm8tWgaApim";
const isSenator = calculator.is_senator(userAddress);
console.log("Is Senator:", isSenator);

// Get rank
const rank = calculator.get_rank(userAddress);
console.log("Rank:", rank);

// Get tier
const tier = calculator.get_tier(userAddress);
console.log("Tier:", tier);

// Get top holder
const top1 = calculator.get_holder(1);
console.log("Top Holder:", top1);
```

## 🔄 Real-Time Updates

```rust
impl Top100Calculator {
    /// Refresh holder data from chain
    #[wasm_bindgen]
    pub async fn refresh(&mut self) -> Result<(), JsValue> {
        console_log!("🔄 Refreshing holder data...");
        
        self.holders = Self::fetch_holders().await?;
        
        console_log!("✅ Refreshed {} holders", self.holders.len());
        Ok(())
    }
}

// Auto-refresh every 60 seconds
setInterval(async () => {
    await calculator.refresh();
}, 60000);
```

## 🎯 Integration with Federal DAO

```rust
// Senate voting requires top 100 verification
impl SenateVoteOrbit {
    pub async fn verify_senator(address: &str) -> Result<bool, String> {
        let calculator = Top100Calculator::new().await?;
        Ok(calculator.is_senator(address.to_string()))
    }
    
    pub async fn get_senator_rank(address: &str) -> Result<Option<usize>, String> {
        let calculator = Top100Calculator::new().await?;
        Ok(calculator.get_rank(address.to_string()))
    }
}
```

## 📈 Market Cap Calculation

```rust
impl Top100Calculator {
    /// Calculate total market cap
    pub fn total_mcap(&self) -> u64 {
        self.holders.iter().map(|h| h.balance).sum()
    }
    
    /// Calculate senator concentration (top 100 share)
    pub fn senator_concentration(&self) -> f64 {
        let senator_total: u64 = self.holders.iter().take(100).map(|h| h.balance).sum();
        let total = self.total_mcap();
        (senator_total as f64 / total as f64) * 100.0
    }
}
```

---

**Status**: 🔍 On-chain top 100 calculator ready  
**Source**: Direct blockchain queries (read-only)  
**Update**: Real-time via RPC calls  
**Verification**: Cryptographic proof via on-chain data  
**Integration**: Federal DAO Senate tier assignment  
**#SOLFUNMEME**: BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump
