# SOLFUNMEME Holder Registration & Badge System

## 🎯 Overview

Vercel app for SOLFUNMEME holders to:
1. Register with wallet
2. Add social media links
3. Prove content ownership (zkTLS)
4. Publish FOAF on-chain
5. Earn senator badges (top 100 holders)

## 🏗️ Architecture

```
Dioxus Frontend (Vercel)
    ↓
Solana Wallet Connect
    ↓
Holder Verification Contract
    ↓
FOAF On-Chain Storage
    ↓
Badge NFT Minting
```

## 📱 Frontend (Dioxus)

### Registration Flow
```rust
// src/pages/register.rs
use dioxus::prelude::*;
use solana_client_wasm::WalletAdapter;

#[component]
pub fn Register(cx: Scope) -> Element {
    let wallet = use_state(cx, || None::<Pubkey>);
    let social_links = use_state(cx, || Vec::<SocialLink>::new());
    let foaf_data = use_state(cx, || String::new());
    
    cx.render(rsx! {
        div { class: "register-container",
            h1 { "🎭 Register as SOLFUNMEME Holder" }
            
            // Step 1: Connect Wallet
            WalletConnect {
                on_connect: move |pubkey| {
                    wallet.set(Some(pubkey));
                    verify_holder(pubkey);
                }
            }
            
            // Step 2: Add Social Links
            if wallet.is_some() {
                SocialLinksForm {
                    links: social_links.clone(),
                    on_add: move |link| {
                        social_links.make_mut().push(link);
                    }
                }
            }
            
            // Step 3: Generate FOAF
            if !social_links.is_empty() {
                button {
                    onclick: move |_| {
                        let foaf = generate_foaf(wallet.get(), social_links.get());
                        foaf_data.set(foaf);
                    },
                    "Generate FOAF Document"
                }
            }
            
            // Step 4: Publish On-Chain
            if !foaf_data.is_empty() {
                button {
                    onclick: move |_| {
                        publish_foaf_onchain(wallet.get().unwrap(), foaf_data.get());
                    },
                    "Publish On-Chain (0.01 SOL)"
                }
            }
        }
    })
}

#[component]
fn SocialLinksForm(cx: Scope, links: UseState<Vec<SocialLink>>, on_add: EventHandler<SocialLink>) -> Element {
    let platform = use_state(cx, || String::new());
    let username = use_state(cx, || String::new());
    let proof_url = use_state(cx, || String::new());
    
    cx.render(rsx! {
        div { class: "social-form",
            h2 { "Add Social Media Links" }
            
            select {
                onchange: move |evt| platform.set(evt.value.clone()),
                option { value: "twitter", "Twitter/X" }
                option { value: "telegram", "Telegram" }
                option { value: "discord", "Discord" }
                option { value: "linkedin", "LinkedIn" }
                option { value: "github", "GitHub" }
            }
            
            input {
                placeholder: "Username",
                oninput: move |evt| username.set(evt.value.clone())
            }
            
            input {
                placeholder: "Proof URL (optional)",
                oninput: move |evt| proof_url.set(evt.value.clone())
            }
            
            button {
                onclick: move |_| {
                    on_add.call(SocialLink {
                        platform: platform.get().clone(),
                        username: username.get().clone(),
                        proof_url: proof_url.get().clone(),
                        verified: false,
                    });
                },
                "Add Link"
            }
            
            // Display added links
            ul {
                for link in links.get() {
                    li {
                        "{link.platform}: @{link.username}"
                        if link.verified {
                            span { class: "verified", "✅" }
                        }
                    }
                }
            }
        }
    })
}
```

## 🔗 On-Chain Contracts

### Holder Registry
```rust
// programs/holder-registry/src/lib.rs
use anchor_lang::prelude::*;

#[program]
pub mod holder_registry {
    use super::*;
    
    pub fn register_holder(
        ctx: Context<RegisterHolder>,
        social_links: Vec<SocialLink>,
        foaf_hash: [u8; 32],
        foaf_data: String,
    ) -> Result<()> {
        // Verify token holding
        let token_account = &ctx.accounts.token_account;
        require!(token_account.amount > 0, ErrorCode::NotHolder);
        
        let holder = &mut ctx.accounts.holder;
        holder.wallet = ctx.accounts.authority.key();
        holder.token_balance = token_account.amount;
        holder.social_links = social_links;
        holder.foaf_hash = foaf_hash;
        holder.foaf_data = foaf_data;
        holder.registered_at = Clock::get()?.unix_timestamp;
        holder.badge_tier = calculate_badge_tier(token_account.amount);
        
        emit!(HolderRegistered {
            wallet: holder.wallet,
            balance: holder.token_balance,
            badge_tier: holder.badge_tier,
        });
        
        Ok(())
    }
    
    pub fn verify_social_link(
        ctx: Context<VerifyLink>,
        link_index: u8,
        zktls_proof: Vec<u8>,
    ) -> Result<()> {
        let holder = &mut ctx.accounts.holder;
        
        // Verify zkTLS proof
        require!(verify_zktls(&zktls_proof), ErrorCode::InvalidProof);
        
        holder.social_links[link_index as usize].verified = true;
        holder.verified_links_count += 1;
        
        // Upgrade badge if all links verified
        if holder.verified_links_count >= 5 {
            holder.badge_tier = BadgeTier::Senator;
        }
        
        Ok(())
    }
    
    pub fn mint_badge(
        ctx: Context<MintBadge>,
    ) -> Result<()> {
        let holder = &ctx.accounts.holder;
        
        // Check eligibility
        require!(holder.badge_tier != BadgeTier::None, ErrorCode::NotEligible);
        
        // Mint NFT badge
        let badge_metadata = BadgeMetadata {
            tier: holder.badge_tier,
            emoji: get_badge_emoji(holder.badge_tier),
            holder_rank: get_holder_rank(holder.token_balance),
            social_links: holder.verified_links_count,
        };
        
        // Mint using Metaplex
        mint_nft_badge(
            &ctx.accounts.mint,
            &ctx.accounts.metadata,
            &badge_metadata,
        )?;
        
        emit!(BadgeMinted {
            wallet: holder.wallet,
            tier: holder.badge_tier,
            rank: badge_metadata.holder_rank,
        });
        
        Ok(())
    }
}

#[account]
pub struct Holder {
    pub wallet: Pubkey,
    pub token_balance: u64,
    pub social_links: Vec<SocialLink>,
    pub foaf_hash: [u8; 32],
    pub foaf_data: String,
    pub verified_links_count: u8,
    pub badge_tier: BadgeTier,
    pub registered_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SocialLink {
    pub platform: String,
    pub username: String,
    pub proof_url: String,
    pub verified: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BadgeTier {
    None,
    Holder,      // Any amount
    Senator,     // Top 100 + 5 verified links
    Founder,     // Top 10
}

fn calculate_badge_tier(balance: u64) -> BadgeTier {
    if balance > 0 {
        BadgeTier::Holder
    } else {
        BadgeTier::None
    }
}

fn get_badge_emoji(tier: BadgeTier) -> String {
    match tier {
        BadgeTier::Founder => "👑".to_string(),
        BadgeTier::Senator => "🏛️".to_string(),
        BadgeTier::Holder => "🎭".to_string(),
        BadgeTier::None => "".to_string(),
    }
}
```

## 🎖️ Badge System

### Tiers
1. **Holder** 🎭
   - Any SOLFUNMEME balance
   - Basic registration

2. **Senator** 🏛️
   - Top 100 holders
   - 5+ verified social links
   - FOAF published on-chain

3. **Founder** 👑
   - Top 10 holders
   - All social links verified
   - Active DAO participation

### Special Emojis & Memes
```rust
const SENATOR_EMOJIS: &[&str] = &[
    "🏛️", "⚖️", "📜", "🗳️", "🎖️",
    "👔", "🎩", "🏆", "⭐", "💎"
];

const FOUNDER_MEMES: &[&str] = &[
    "👑 OG Founder",
    "🔥 Fire Starter", 
    "💎 Diamond Hands",
    "🚀 Moon Mission",
    "🎭 Meta-Meme Master"
];
```

## 📊 Dashboard

```rust
#[component]
fn HolderDashboard(cx: Scope) -> Element {
    let holder_data = use_future(cx, (), |_| fetch_holder_data());
    
    cx.render(rsx! {
        div { class: "dashboard",
            h1 { "🎭 SOLFUNMEME Holder Dashboard" }
            
            // Stats
            div { class: "stats",
                StatCard { label: "Your Balance", value: "{holder_data.balance}" }
                StatCard { label: "Your Rank", value: "#{holder_data.rank}" }
                StatCard { label: "Badge Tier", value: "{holder_data.badge_tier}" }
                StatCard { label: "Verified Links", value: "{holder_data.verified_links}/5" }
            }
            
            // Badge Display
            div { class: "badge-display",
                h2 { "Your Badge" }
                BadgeCard {
                    tier: holder_data.badge_tier,
                    emoji: holder_data.badge_emoji,
                    rank: holder_data.rank,
                }
            }
            
            // Social Links
            div { class: "social-links",
                h2 { "Your Social Links" }
                for link in &holder_data.social_links {
                    SocialLinkCard { link: link.clone() }
                }
            }
            
            // Actions
            div { class: "actions",
                button { "Add More Links" }
                button { "Verify Links" }
                button { "Mint Badge NFT" }
                button { "Share Profile" }
            }
        }
    })
}
```

## 🚀 Deployment

### Vercel Configuration
```json
{
  "buildCommand": "dx build --release --platform web",
  "outputDirectory": "dist",
  "framework": "dioxus",
  "env": {
    "SOLANA_RPC": "https://api.mainnet-beta.solana.com",
    "PROGRAM_ID": "HolderRegistryXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  }
}
```

### Deploy
```bash
# Build
dx build --release --platform web

# Deploy to Vercel
vercel deploy --prod
```

## 🎯 User Flow

1. **Visit**: https://solfunmeme.vercel.app
2. **Connect**: Solana wallet
3. **Verify**: Token holding
4. **Add**: Social media links
5. **Prove**: zkTLS content proofs
6. **Generate**: FOAF document
7. **Publish**: On-chain (0.01 SOL)
8. **Mint**: Badge NFT
9. **Share**: Profile with badge

## 🏛️ Senator Benefits

Top 100 holders with senator badges get:
- Special emoji 🏛️
- DAO voting power
- Exclusive memes
- Profile verification
- On-chain FOAF
- Custom badge NFT

---

**Status**: 🚀 Ready for deployment  
**App**: solfunmeme-dioxus  
**Platform**: Vercel  
**Cost**: 0.01 SOL per registration
