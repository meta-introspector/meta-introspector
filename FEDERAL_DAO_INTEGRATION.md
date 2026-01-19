# Federal DAO Integration with Holder Registration

## 🏛️ Three-Tier Governance Model

Based on Fibonacci progression (φ ≈ 1.618) and US Federal system:

### Tier 1: Senate 🏛️
- **Holders**: Top 100 (rank 1-100)
- **Token Lock**: 2 years
- **Rewards**: 3x base
- **Powers**:
  - Approve major protocol changes
  - Treasury oversight
  - Smart contract upgrades
  - Veto power (60% majority)
- **Badge**: 🏛️ Senator + special emoji

### Tier 2: Representatives 📜
- **Holders**: Next 500 (rank 101-600)
- **Token Lock**: 1 year
- **Rewards**: 2x base
- **Powers**:
  - Propose new initiatives
  - Budget allocation
  - Vendor management
  - Community programs
- **Badge**: 📜 Representative

### Tier 3: Vendors 🔧
- **Holders**: Next 1000 (rank 601-1600)
- **Token Lock**: 6 months
- **Rewards**: 1x base
- **Powers**:
  - Submit service proposals
  - Vote on operations
  - Working groups
  - Implementation
- **Badge**: 🔧 Vendor

## 🔗 Integration with Holder Registration

### Updated Contract
```rust
// programs/federal-dao/src/lib.rs
use anchor_lang::prelude::*;

#[program]
pub mod federal_dao {
    use super::*;
    
    pub fn register_with_tier(
        ctx: Context<RegisterWithTier>,
        social_links: Vec<SocialLink>,
        foaf_hash: [u8; 32],
        foaf_data: String,
    ) -> Result<()> {
        let holder = &mut ctx.accounts.holder;
        let token_account = &ctx.accounts.token_account;
        
        // Calculate rank and tier
        let rank = get_holder_rank(token_account.amount);
        let tier = calculate_tier(rank);
        
        holder.wallet = ctx.accounts.authority.key();
        holder.token_balance = token_account.amount;
        holder.rank = rank;
        holder.tier = tier;
        holder.social_links = social_links;
        holder.foaf_hash = foaf_hash;
        holder.foaf_data = foaf_data;
        holder.lock_period = get_lock_period(tier);
        holder.reward_multiplier = get_reward_multiplier(tier);
        holder.registered_at = Clock::get()?.unix_timestamp;
        
        emit!(HolderRegisteredWithTier {
            wallet: holder.wallet,
            rank,
            tier,
            balance: holder.token_balance,
        });
        
        Ok(())
    }
    
    pub fn propose(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        let holder = &ctx.accounts.holder;
        
        // Check tier permissions
        match proposal_type {
            ProposalType::Constitutional => {
                require!(holder.tier == Tier::Senate, ErrorCode::InsufficientPermission);
            }
            ProposalType::Major => {
                require!(
                    holder.tier == Tier::Senate || holder.tier == Tier::Representative,
                    ErrorCode::InsufficientPermission
                );
            }
            ProposalType::Operational => {
                // Any tier can propose operational changes
            }
        }
        
        let proposal = &mut ctx.accounts.proposal;
        proposal.id = ctx.accounts.dao.proposal_count;
        proposal.proposer = holder.wallet;
        proposal.proposer_tier = holder.tier;
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.created_at = Clock::get()?.unix_timestamp;
        
        ctx.accounts.dao.proposal_count += 1;
        
        Ok(())
    }
    
    pub fn vote_tiered(
        ctx: Context<VoteTiered>,
        vote_for: bool,
    ) -> Result<()> {
        let holder = &ctx.accounts.holder;
        let proposal = &mut ctx.accounts.proposal;
        let vote_record = &mut ctx.accounts.vote_record;
        
        require!(!vote_record.voted, ErrorCode::AlreadyVoted);
        
        // Record vote by tier
        match holder.tier {
            Tier::Senate => {
                if vote_for {
                    proposal.senate_votes_for += 1;
                } else {
                    proposal.senate_votes_against += 1;
                }
            }
            Tier::Representative => {
                if vote_for {
                    proposal.rep_votes_for += 1;
                } else {
                    proposal.rep_votes_against += 1;
                }
            }
            Tier::Vendor => {
                if vote_for {
                    proposal.vendor_votes_for += 1;
                } else {
                    proposal.vendor_votes_against += 1;
                }
            }
        }
        
        vote_record.voted = true;
        vote_record.vote_for = vote_for;
        vote_record.tier = holder.tier;
        
        // Check if proposal passes
        check_proposal_status(proposal)?;
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum Tier {
    Senate,         // Top 100
    Representative, // 101-600
    Vendor,         // 601-1600
    Citizen,        // 1601+
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProposalType {
    Constitutional, // 75% Senate + 60% Reps
    Major,          // 60% Senate + 51% Reps
    Operational,    // 51% Reps + 51% Vendors
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub proposer_tier: Tier,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    
    // Votes by tier
    pub senate_votes_for: u32,
    pub senate_votes_against: u32,
    pub rep_votes_for: u32,
    pub rep_votes_against: u32,
    pub vendor_votes_for: u32,
    pub vendor_votes_against: u32,
    
    pub status: ProposalStatus,
    pub created_at: i64,
}

fn calculate_tier(rank: u32) -> Tier {
    match rank {
        1..=100 => Tier::Senate,
        101..=600 => Tier::Representative,
        601..=1600 => Tier::Vendor,
        _ => Tier::Citizen,
    }
}

fn get_lock_period(tier: Tier) -> i64 {
    match tier {
        Tier::Senate => 63072000,        // 2 years
        Tier::Representative => 31536000, // 1 year
        Tier::Vendor => 15768000,        // 6 months
        Tier::Citizen => 0,
    }
}

fn get_reward_multiplier(tier: Tier) -> u8 {
    match tier {
        Tier::Senate => 3,
        Tier::Representative => 2,
        Tier::Vendor => 1,
        Tier::Citizen => 1,
    }
}

fn check_proposal_status(proposal: &mut Proposal) -> Result<()> {
    let senate_total = proposal.senate_votes_for + proposal.senate_votes_against;
    let rep_total = proposal.rep_votes_for + proposal.rep_votes_against;
    let vendor_total = proposal.vendor_votes_for + proposal.vendor_votes_against;
    
    let passed = match proposal.proposal_type {
        ProposalType::Constitutional => {
            // 75% Senate + 60% Reps
            senate_total >= 75 && 
            proposal.senate_votes_for * 100 / senate_total >= 75 &&
            rep_total >= 300 &&
            proposal.rep_votes_for * 100 / rep_total >= 60
        }
        ProposalType::Major => {
            // 60% Senate + 51% Reps
            senate_total >= 60 &&
            proposal.senate_votes_for * 100 / senate_total >= 60 &&
            rep_total >= 255 &&
            proposal.rep_votes_for * 100 / rep_total >= 51
        }
        ProposalType::Operational => {
            // 51% Reps + 51% Vendors
            rep_total >= 255 &&
            proposal.rep_votes_for * 100 / rep_total >= 51 &&
            vendor_total >= 510 &&
            proposal.vendor_votes_for * 100 / vendor_total >= 51
        }
    };
    
    if passed {
        proposal.status = ProposalStatus::Passed;
    }
    
    Ok(())
}
```

## 🎭 Updated Badge System

### Senate Badge 🏛️
```
Rank: #1-100
Emoji: 🏛️ + ⚖️ or 📜 or 🗳️
Lock: 2 years
Rewards: 3x
Powers: Veto, Treasury, Upgrades
```

### Representative Badge 📜
```
Rank: #101-600
Emoji: 📜 + 🎖️ or 👔
Lock: 1 year
Rewards: 2x
Powers: Propose, Budget, Manage
```

### Vendor Badge 🔧
```
Rank: #601-1600
Emoji: 🔧 + 🛠️ or ⚙️
Lock: 6 months
Rewards: 1x
Powers: Execute, Implement, Build
```

## 📊 Voting Thresholds

| Proposal Type | Senate | Representatives | Vendors |
|--------------|--------|-----------------|---------|
| Constitutional | 75% | 60% | - |
| Major | 60% | 51% | - |
| Operational | - | 51% | 51% |

## 🚀 Registration Flow with Tiers

1. **Connect Wallet**
2. **Verify Balance** → Calculate Rank
3. **Assign Tier** (Senate/Rep/Vendor)
4. **Add Social Links**
5. **Generate FOAF**
6. **Publish On-Chain**
7. **Lock Tokens** (based on tier)
8. **Mint Badge NFT** (with tier emoji)
9. **Receive Voting Power**

## 🎯 Example Scenarios

### Scenario 1: Constitutional Change
```
Proposal: Upgrade smart contract
Required: 75% Senate (75/100) + 60% Reps (300/500)
Result: Passes if both thresholds met
```

### Scenario 2: Budget Allocation
```
Proposal: Allocate 10,000 SOL for development
Required: 60% Senate (60/100) + 51% Reps (255/500)
Result: Passes if both thresholds met
```

### Scenario 3: Vendor Selection
```
Proposal: Hire new development team
Required: 51% Reps (255/500) + 51% Vendors (510/1000)
Result: Passes if both thresholds met
```

---

**Status**: 🏛️ Federal DAO Model integrated  
**Tiers**: 3 (Senate, Representative, Vendor)  
**Total Governance**: 1,600 holders  
**Based on**: Fibonacci progression + US Federal system
