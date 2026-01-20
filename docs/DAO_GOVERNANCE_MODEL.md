# SOLFUNMEME DAO-Governed Block Collection

## Value Proposition for Token Holders

### The Problem
Blockchain data collection requires resources:
- RPC bandwidth (limited)
- Storage (limited)
- Compute (limited)
- Payment budget (limited)

### The Solution: DAO Governance
**Token holders vote on which transactions to include in the archive.**

## Governance Model

```
┌─────────────────────────────────────────────────────────────┐
│ 1. RESOURCE LIMITS                                          │
│                                                             │
│  Daily Budget:                                              │
│  - 10,000 RPC calls                                         │
│  - 100 GB storage                                           │
│  - 10 SOL payment budget                                    │
│                                                             │
│  = Can process ~10,000 blocks/day                           │
│  = But SOLFUNMEME has ~100,000 transactions/day             │
│                                                             │
│  ❓ Which 10,000 blocks to archive?                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. DAO VOTING                                               │
│                                                             │
│  Token Holders Vote:                                        │
│                                                             │
│  Proposal #1: "Archive all swaps > 1000 SFM"               │
│  ✅ Yes: 60% (600,000 SFM)                                  │
│  ❌ No:  40% (400,000 SFM)                                  │
│  → APPROVED                                                 │
│                                                             │
│  Proposal #2: "Archive all whale wallets"                  │
│  ✅ Yes: 75% (750,000 SFM)                                  │
│  ❌ No:  25% (250,000 SFM)                                  │
│  → APPROVED                                                 │
│                                                             │
│  Proposal #3: "Archive random sample"                      │
│  ✅ Yes: 30% (300,000 SFM)                                  │
│  ❌ No:  70% (700,000 SFM)                                  │
│  → REJECTED                                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. PRIORITY QUEUE                                           │
│                                                             │
│  Server Plugin Implements Approved Rules:                   │
│                                                             │
│  Priority 1: Swaps > 1000 SFM (75% weight)                 │
│  Priority 2: Whale wallets (60% weight)                    │
│  Priority 3: Everything else (if budget remains)           │
│                                                             │
│  fn should_archive(tx: &Transaction) -> bool {             │
│    if tx.amount > 1000 { return true; }                    │
│    if WHALE_LIST.contains(tx.wallet) { return true; }      │
│    if budget_remaining() > 0 { return true; }              │
│    false                                                    │
│  }                                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. VALUE TO HOLDERS                                         │
│                                                             │
│  ✅ Voting Rights = Governance Power                        │
│  ✅ Curated Dataset = Higher Quality                        │
│  ✅ Limited Resources = Scarcity Value                      │
│  ✅ Community Decides = Decentralization                    │
│                                                             │
│  Example Use Cases:                                         │
│  - Traders vote to archive price-moving transactions       │
│  - Researchers vote to archive specific patterns           │
│  - Community votes to archive meme moments                  │
└─────────────────────────────────────────────────────────────┘
```

## Implementation

### DAO Contract (Solana)
```rust
// Proposal structure
struct Proposal {
    id: u64,
    description: String,
    filter_rule: FilterRule,
    yes_votes: u64,
    no_votes: u64,
    status: ProposalStatus,
}

enum FilterRule {
    AmountThreshold(u64),
    WalletList(Vec<Pubkey>),
    TimeRange(u64, u64),
    Custom(String),
}

// Vote with SFM tokens
fn vote(proposal_id: u64, vote: bool, amount: u64) {
    // Weight by token holdings
}
```

### Server Plugin Integration
```rust
// tools/so-plugins/block-collector/src/lib.rs

fn should_archive(block: &Block) -> bool {
    // Load approved DAO rules
    let rules = load_dao_rules();
    
    for rule in rules {
        if rule.matches(block) {
            return true;
        }
    }
    
    // Check budget
    budget_remaining() > 0
}

#[no_mangle]
pub extern "C" fn submit_block(block_json_ptr: *const c_char) -> *const c_char {
    let block: Block = parse_json(block_json_ptr);
    
    // DAO governance check
    if !should_archive(&block) {
        return error("block not approved by DAO");
    }
    
    // Verify, store, pay
    verify_and_store(&block);
}
```

## Nix Build

```bash
# Build both plugins
nix build .#solana-rpc-wasm
nix build .#block-collector-plugin

# Outputs:
# result/solana_rpc_wasm.js (WASM)
# result/lib/libblock_collector_plugin.so
```

## Economic Model

### Token Utility
1. **Governance**: 1 SFM = 1 vote
2. **Priority**: Higher holders get priority in queue
3. **Revenue Share**: DAO treasury from data sales

### Revenue Streams
1. Premium API access to curated data
2. Custom dataset requests
3. Analytics subscriptions

### Value Accrual
- More holders → More governance → Better curation → Higher value dataset → More revenue → Higher token value

## Example DAO Proposals

### Proposal: Archive Top 100 Holders
```json
{
  "id": 1,
  "title": "Archive all transactions from top 100 SFM holders",
  "filter": {
    "type": "WalletList",
    "wallets": ["HMEKzpgz...", "BwUTq7fS..."]
  },
  "budget_impact": "~2000 blocks/day",
  "rationale": "Track whale activity for market analysis"
}
```

### Proposal: Archive Meme Moments
```json
{
  "id": 2,
  "title": "Archive blocks during viral Twitter moments",
  "filter": {
    "type": "TimeRange",
    "start": 1737318000,
    "end": 1737321600
  },
  "budget_impact": "~500 blocks",
  "rationale": "Preserve historical meme moments"
}
```

## Next Steps

1. [ ] Deploy DAO contract on Solana
2. [ ] Add governance check to plugin
3. [ ] Build voting UI in solfunmeme-dioxus
4. [ ] Test with Nix build
5. [ ] Launch governance

---

**Key Insight**: Limited resources + DAO governance = Token utility + Value accrual
