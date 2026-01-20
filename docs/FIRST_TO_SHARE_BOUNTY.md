# First-to-Share Bounty System

## Incentive Model

```
┌─────────────────────────────────────────────────────────────┐
│ BOUNTY TIERS                                                │
│                                                             │
│ 🥇 First to share viral moment:     1.0 SOL                │
│ 🥈 First to share whale transaction: 0.5 SOL               │
│ 🥉 First to share price spike:      0.3 SOL                │
│ 💎 First to share unique pattern:   0.1 SOL                │
│                                                             │
│ + Bonus if DAO approves archival:   +0.5 SOL               │
└─────────────────────────────────────────────────────────────┘
```

## Flow

```
User A detects viral tweet at 14:30:00
  → Submits to server
  → Server checks: Is this first submission?
  → YES → Pay 1.0 SOL immediately
  → Create DAO proposal
  → If approved → Pay bonus 0.5 SOL
  → Total: 1.5 SOL

User B submits same tweet at 14:30:05
  → Server checks: Already submitted by User A
  → NO PAYMENT
  → But still creates vote weight for proposal
```

## Smart Contract

```rust
// Bounty tracking contract
pub struct BountyTracker {
    submissions: HashMap<ContentHash, Submission>,
    bounty_pool: u64,
}

pub struct Submission {
    submitter: Pubkey,
    timestamp: u64,
    content_hash: [u8; 32],
    bounty_paid: u64,
    dao_approved: bool,
}

impl BountyTracker {
    pub fn submit_moment(&mut self, moment: TwitterMoment, submitter: Pubkey) -> Result<u64> {
        let hash = hash_content(&moment);
        
        // Check if first submission
        if self.submissions.contains_key(&hash) {
            return Err("Already submitted");
        }
        
        // Calculate bounty
        let bounty = calculate_bounty(&moment);
        
        // Record submission
        self.submissions.insert(hash, Submission {
            submitter,
            timestamp: Clock::get()?.unix_timestamp as u64,
            content_hash: hash,
            bounty_paid: bounty,
            dao_approved: false,
        });
        
        // Pay immediately
        transfer_sol(self.bounty_pool, submitter, bounty)?;
        
        Ok(bounty)
    }
    
    pub fn dao_approved(&mut self, content_hash: [u8; 32]) -> Result<()> {
        let submission = self.submissions.get_mut(&content_hash)
            .ok_or("Not found")?;
        
        if submission.dao_approved {
            return Err("Already paid bonus");
        }
        
        // Pay bonus
        let bonus = 0.5 * LAMPORTS_PER_SOL;
        transfer_sol(self.bounty_pool, submission.submitter, bonus)?;
        
        submission.dao_approved = true;
        Ok(())
    }
}

fn calculate_bounty(moment: &TwitterMoment) -> u64 {
    match moment.engagement {
        e if e > 10000 => 1.0 * LAMPORTS_PER_SOL,  // Viral
        e if e > 5000  => 0.5 * LAMPORTS_PER_SOL,  // High engagement
        e if e > 1000  => 0.3 * LAMPORTS_PER_SOL,  // Medium
        _              => 0.1 * LAMPORTS_PER_SOL,  // Low
    }
}
```

## Browser Extension Update

```javascript
// content_scripts/twitter_monitor.js

async function exportMoment(moment) {
  const wasm = await import('./wasm/twitter_exporter.js');
  await wasm.default();
  
  // Export with wallet signature
  const wallet = await getWalletAdapter();
  const signature = await wallet.signMessage(JSON.stringify(moment));
  
  const result = await wasm.export_twitter_moment(
    JSON.stringify(moment),
    signature,
    wallet.publicKey.toString(),
    'https://zos-server.solfunmeme.com/api/moments'
  );
  
  if (result.bounty_paid) {
    showNotification(`🎉 Earned ${result.bounty_paid} SOL for first submission!`);
  } else {
    showNotification(`Already submitted by another user`);
  }
}
```

## Server Plugin Update

```rust
// tools/so-plugins/block-collector/src/lib.rs

#[no_mangle]
pub extern "C" fn submit_twitter_moment(
    moment_json: *const c_char,
    signature: *const c_char,
    submitter_pubkey: *const c_char
) -> *const c_char {
    let moment: TwitterMoment = parse_json(moment_json);
    let sig = parse_signature(signature);
    let pubkey = parse_pubkey(submitter_pubkey);
    
    // Verify signature
    if !verify_signature(&moment, &sig, &pubkey) {
        return error("Invalid signature");
    }
    
    // Check if first submission
    let content_hash = hash_content(&moment);
    if is_duplicate(content_hash) {
        return error("Already submitted");
    }
    
    // Calculate and pay bounty
    let bounty = calculate_bounty(&moment);
    let tx_sig = pay_bounty(pubkey, bounty);
    
    // Record submission
    record_submission(content_hash, pubkey, bounty);
    
    // Create DAO proposal
    create_dao_proposal(&moment);
    
    let response = serde_json::json!({
        "status": "accepted",
        "bounty_paid": bounty,
        "tx_signature": tx_sig,
        "first_submission": true
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}
```

## Leaderboard

```rust
pub struct Leaderboard {
    top_earners: Vec<(Pubkey, u64)>,
    total_bounties_paid: u64,
}

impl Leaderboard {
    pub fn get_top_10(&self) -> Vec<LeaderboardEntry> {
        self.top_earners.iter()
            .take(10)
            .map(|(pubkey, earnings)| LeaderboardEntry {
                wallet: pubkey.to_string(),
                total_earned: *earnings,
                submissions: self.count_submissions(pubkey),
            })
            .collect()
    }
}
```

## UI in solfunmeme-dioxus

```rust
fn Leaderboard(cx: Scope) -> Element {
    let leaderboard = use_future(cx, (), |_| async {
        fetch_leaderboard().await
    });
    
    cx.render(rsx! {
        div { class: "leaderboard",
            h2 { "🏆 Top Earners" }
            
            match leaderboard.value() {
                Some(Ok(data)) => rsx! {
                    for (rank, entry) in data.iter().enumerate() {
                        div { class: "entry",
                            span { "{rank + 1}. {entry.wallet}" }
                            span { "{entry.total_earned} SOL" }
                            span { "{entry.submissions} submissions" }
                        }
                    }
                },
                _ => rsx! { div { "Loading..." } }
            }
        }
    })
}
```

## Economics

### Bounty Pool Funding
1. **DAO Treasury**: 10% of token supply
2. **Revenue Share**: 50% of dataset sales
3. **Donations**: Community contributions

### Payout Structure
```
Immediate Payment (First-to-share):
- Viral moment (>10k engagement): 1.0 SOL
- High engagement (>5k):          0.5 SOL
- Medium engagement (>1k):         0.3 SOL
- Low engagement:                  0.1 SOL

Bonus Payment (DAO approved):
- +0.5 SOL if proposal passes
- +0.2 SOL if >75% approval
```

### Example Earnings

**User A** (Active Twitter monitor):
- Submits 10 viral moments/month
- 7 get DAO approved
- Earnings: (10 × 1.0) + (7 × 0.5) = 13.5 SOL/month

**User B** (Whale tracker):
- Submits 50 whale transactions/month
- 30 get DAO approved
- Earnings: (50 × 0.5) + (30 × 0.5) = 40 SOL/month

## Anti-Gaming Measures

1. **Signature Required**: Must sign with wallet
2. **Content Hash**: Prevents duplicate submissions
3. **Timestamp Check**: Must be recent (<5 min old)
4. **Rate Limiting**: Max 100 submissions/day per wallet
5. **DAO Approval**: Bonus only if community approves

## Value Proposition

### For Contributors
- Earn SOL for valuable data
- First-mover advantage
- Passive income from monitoring

### For Token Holders
- Curated data from incentivized contributors
- Quality over quantity
- Community-driven dataset

### For Network
- Distributed data collection
- Real-time viral moment detection
- Competitive discovery

## Next Steps

1. [ ] Deploy bounty contract
2. [ ] Add signature verification to plugin
3. [ ] Build leaderboard UI
4. [ ] Test with real Twitter data
5. [ ] Launch bounty program

---

**Key Innovation**: Pay for valuable data discovery, not just block collection. First-to-share = highest reward.
