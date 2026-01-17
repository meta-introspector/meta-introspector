# REACH Sidechain: Proof-of-Trace Credits

## Overview

A lightweight sidechain that tracks contributions until critical mass, then settles to Solana mainnet with cryptographic proof of work.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    REACH Sidechain                           │
│                                                              │
│  Phase 1: Accumulation (Months 1-6)                        │
│    • Miners earn credits (off-chain)                        │
│    • Fast, free transactions                                │
│    • No gas fees                                            │
│    • Merkle tree of contributions                           │
│                                                              │
│  Phase 2: Settlement (Month 7+)                             │
│    • Reach critical mass (10K traces)                       │
│    • Generate proof of contributions                        │
│    • Settle to Solana mainnet                               │
│    • Distribute real tokens                                 │
└─────────────────────────────────────────────────────────────┘
```

## Credit System

### Earning Credits

```rust
struct TraceCredit {
    miner: PublicKey,
    job_id: JobId,
    parquet_hash: Hash,
    timestamp: u64,
    credits: u64,
}

impl Sidechain {
    fn award_credits(&mut self, miner: PublicKey, job: CompletedJob) -> u64 {
        let credits = self.calculate_credits(&job);
        
        let credit = TraceCredit {
            miner,
            job_id: job.id,
            parquet_hash: hash(&job.parquet),
            timestamp: now(),
            credits,
        };
        
        // Add to merkle tree
        self.merkle_tree.insert(credit.hash());
        
        // Update balance
        self.balances.entry(miner)
            .and_modify(|b| *b += credits)
            .or_insert(credits);
        
        credits
    }
}
```

### Credit Formula

```
credits = base_credits 
        × quality_multiplier 
        × speed_multiplier
        × early_adopter_bonus

base_credits = file_size_kb / 10
quality_multiplier = 1.0 - 2.0 (based on validation)
speed_multiplier = 1.0 - 1.5 (based on turnaround)
early_adopter_bonus = 2.0 (first 100 miners)
```

## Merkle Proof System

### Building the Tree

```rust
struct ContributionTree {
    leaves: Vec<TraceCredit>,
    root: Hash,
}

impl ContributionTree {
    fn build(&mut self) {
        // Sort by timestamp
        self.leaves.sort_by_key(|c| c.timestamp);
        
        // Build merkle tree
        let hashes: Vec<Hash> = self.leaves.iter()
            .map(|c| c.hash())
            .collect();
        
        self.root = merkle_root(&hashes);
    }
    
    fn prove(&self, miner: PublicKey) -> MerkleProof {
        let contributions: Vec<&TraceCredit> = self.leaves.iter()
            .filter(|c| c.miner == miner)
            .collect();
        
        MerkleProof {
            miner,
            total_credits: contributions.iter().map(|c| c.credits).sum(),
            contributions: contributions.len(),
            merkle_path: self.generate_path(&contributions),
            root: self.root,
        }
    }
}
```

## Settlement Process

### Phase 1: Accumulation (Months 1-6)

**Goal**: Build critical mass without mainnet costs

```rust
// Sidechain operations (free, fast)
sidechain.submit_trace(parquet) → credits
sidechain.check_balance(miner) → credits
sidechain.leaderboard() → top miners
```

**Metrics**:
- Target: 10,000 traces
- Target: 100 miners
- Target: 1,000,000 credits distributed

### Phase 2: Settlement (Month 7)

**Trigger**: Critical mass reached

```rust
async fn settle_to_mainnet(&self) -> Result<()> {
    // 1. Freeze sidechain
    self.freeze();
    
    // 2. Build final merkle tree
    let tree = self.build_contribution_tree();
    
    // 3. Deploy Solana program
    let program = deploy_reach_token().await?;
    
    // 4. Mint tokens based on credits
    for (miner, credits) in &self.balances {
        let tokens = credits_to_tokens(credits);
        program.mint(miner, tokens).await?;
    }
    
    // 5. Store merkle root on-chain
    program.store_proof_root(tree.root).await?;
    
    // 6. Enable claims
    program.enable_claims().await?;
    
    Ok(())
}
```

## Proof of Contribution

### On-Chain Verification

```rust
// Solana program
#[program]
mod reach_token {
    pub fn claim_tokens(
        ctx: Context<ClaimTokens>,
        proof: MerkleProof,
    ) -> Result<()> {
        // Verify merkle proof
        require!(
            verify_proof(&proof, &ctx.accounts.proof_root.root),
            ErrorCode::InvalidProof
        );
        
        // Check not already claimed
        require!(
            !ctx.accounts.claim_record.claimed,
            ErrorCode::AlreadyClaimed
        );
        
        // Mint tokens
        let tokens = credits_to_tokens(proof.total_credits);
        token::mint_to(
            ctx.accounts.mint_ctx(),
            tokens,
        )?;
        
        // Mark as claimed
        ctx.accounts.claim_record.claimed = true;
        
        Ok(())
    }
}
```

### Proof Structure

```rust
struct MerkleProof {
    miner: PublicKey,
    total_credits: u64,
    contributions: usize,
    merkle_path: Vec<Hash>,
    root: Hash,
}

fn verify_proof(proof: &MerkleProof, root: &Hash) -> bool {
    let leaf = hash(&(proof.miner, proof.total_credits));
    let computed_root = compute_root(leaf, &proof.merkle_path);
    computed_root == *root
}
```

## Token Economics

### Credit to Token Conversion

```
1 credit = 0.001 REACH tokens

Example:
- Miner earns 10,000 credits
- Converts to 10 REACH tokens
- At $0.10/REACH = $1.00
```

### Initial Distribution

**Total Supply**: 1,000,000 REACH

**Sidechain Settlement** (40%):
- 400,000 REACH for early miners
- Based on credits earned
- Proportional distribution

**Example**:
```
Total credits: 1,000,000
Miner A: 10,000 credits (1%)
Miner A tokens: 400,000 × 0.01 = 4,000 REACH
```

## Implementation

### Sidechain Node

```rust
struct SidechainNode {
    balances: HashMap<PublicKey, u64>,
    contributions: Vec<TraceCredit>,
    merkle_tree: MerkleTree,
    frozen: bool,
}

impl SidechainNode {
    async fn run(&mut self) {
        loop {
            match self.receive_message().await {
                Message::SubmitTrace(trace) => {
                    let credits = self.award_credits(trace);
                    self.broadcast_credit_update(credits);
                }
                Message::CheckBalance(miner) => {
                    let balance = self.balances.get(&miner);
                    self.send_balance(balance);
                }
                Message::Freeze => {
                    self.frozen = true;
                    self.prepare_settlement();
                }
            }
        }
    }
}
```

### CLI Commands

```bash
# Check your credits
reach-cli balance

# View contribution history
reach-cli history

# Check settlement status
reach-cli settlement-status

# Claim tokens (after settlement)
reach-cli claim --proof proof.json
```

## Timeline

### Month 1-3: Sidechain Launch
- [ ] Deploy sidechain nodes
- [ ] Start awarding credits
- [ ] Build miner community

### Month 4-6: Growth
- [ ] Reach 100 miners
- [ ] Reach 10,000 traces
- [ ] Reach 1,000,000 credits

### Month 7: Settlement
- [ ] Freeze sidechain
- [ ] Build merkle tree
- [ ] Deploy Solana program
- [ ] Distribute tokens

### Month 8+: Mainnet
- [ ] All transactions on Solana
- [ ] Real token trading
- [ ] Sustainable economics

## Benefits

### For Early Miners

**Advantages**:
- No gas fees during accumulation
- Early adopter bonus (2x credits)
- Guaranteed token allocation
- Proof of contribution forever

### For the Project

**Advantages**:
- No mainnet costs during bootstrap
- Build community first
- Prove concept before tokenomics
- Fair distribution based on work

### For the Ecosystem

**Advantages**:
- Transparent contribution tracking
- Cryptographic proof of work
- Fair launch (no pre-mine)
- Community-driven from day 1

## Security

### Sybil Resistance

1. **Work proof**: Must submit valid parquet
2. **Validation**: Cross-check with other miners
3. **Rate limiting**: Max credits per day
4. **Reputation**: Track accuracy over time

### Settlement Security

1. **Merkle proofs**: Cryptographically verifiable
2. **On-chain root**: Immutable record
3. **One-time claims**: Prevent double-spending
4. **Audit trail**: All contributions public

## Transparency

### Public Leaderboard

```
Top Contributors (Pre-Settlement):

1. miner_abc123: 50,000 credits (500 traces)
2. miner_def456: 45,000 credits (480 traces)
3. miner_ghi789: 40,000 credits (420 traces)
...

Total: 1,000,000 credits (10,000 traces)
Settlement: Month 7 (estimated)
```

### Contribution Explorer

```
https://reach.network/explorer

- View all contributions
- Check merkle tree
- Verify proofs
- Track settlement progress
```

## The Promise

**We will remember who helped.**

Every contribution is recorded in the merkle tree.
Every miner gets their fair share.
Every proof is verifiable forever.

When we reach critical mass, we settle.
When we settle, we prove who helped.
When we prove, we pay.

**No trust required. Only math.**

---

**Build now. Settle later. Prove always.**
