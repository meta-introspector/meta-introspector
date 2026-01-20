# SOLFUNMEME Social-Price-Governance Data Pipeline

## Vision

Unified dataset correlating:
- **Social Media** (Twitter, Telegram, Discord)
- **Token Price** (Solana on-chain data)
- **Sentiment Analysis** (ZK-proven)
- **DAO Governance** (Vote correlation)

## Data Sources

### Social Media Feeds
```
social/
├── twitter/
│   └── @zos_sfm_2025.json
├── telegram/
│   └── introsp3ctor_2025.json
└── discord/
    └── WASKdrBBzu_2025.json
```

### Price Data
```
price/
├── raydium_swaps.parquet
├── token_balances.parquet
└── liquidity_snapshots.parquet
```

### Sentiment + ZK Proofs
```
sentiment/
├── posts_with_sentiment.parquet
└── zk_sentiment_proofs.parquet
```

### DAO Votes
```
governance/
├── proposals.parquet
└── votes.parquet
```

## Schema Design

### Unified Event Stream
```rust
struct SocialPriceEvent {
    timestamp: u64,
    event_type: EventType,
    
    // Social
    platform: Option<String>,      // "twitter", "telegram", "discord"
    author: Option<String>,
    content: Option<String>,
    sentiment_score: Option<f64>,  // -1.0 to 1.0
    sentiment_proof: Option<String>, // ZK proof hash
    
    // Price
    price_sol: Option<f64>,
    price_usd: Option<f64>,
    volume_24h: Option<u64>,
    liquidity: Option<u64>,
    
    // Governance
    proposal_id: Option<String>,
    vote_type: Option<String>,     // "yes", "no", "abstain"
    voter_weight: Option<u64>,
}

enum EventType {
    SocialPost,
    PriceUpdate,
    Swap,
    Vote,
}
```

## Pipeline Architecture

```
┌─────────────┐
│   Twitter   │──┐
│  Telegram   │──┼──> Social Scraper ──> Sentiment Analysis ──> ZK Proof Gen
│   Discord   │──┘                                                    │
└─────────────┘                                                       │
                                                                      ▼
┌─────────────┐                                              ┌──────────────┐
│   Solana    │──> RPC Poller ──> Price Extractor ────────> │ Unified      │
│  Blockchain │                                              │ Parquet      │
└─────────────┘                                              │ Dataset      │
                                                             └──────────────┘
┌─────────────┐                                                      ▲
│ DAO Contract│──> Vote Scraper ──> Vote Analyzer ──────────────────┘
└─────────────┘
```

## Tools Structure

```
tools/
├── social-scraper/          # Download social feeds
│   ├── twitter.rs
│   ├── telegram.rs
│   └── discord.rs
├── sentiment-analyzer/      # Analyze sentiment
│   ├── model.rs
│   └── zk_prover.rs
├── price-correlator/        # Correlate price with events
│   └── correlate.rs
└── dao-analyzer/            # DAO vote analysis
    └── votes.rs
```

## Implementation Plan

### Phase 1: Data Collection (Week 1)
```bash
# Download social feeds
cargo run --bin social-scraper -- \
  --twitter @zos_sfm \
  --telegram introsp3ctor \
  --discord WASKdrBBzu \
  --start 2025-01-01 \
  --end 2026-01-19 \
  --output social/

# Already have: Solana transaction data
# From: hf_dataset/ (3671 JSON files)
```

### Phase 2: Sentiment + ZK (Week 2)
```bash
# Analyze sentiment with ZK proofs
cargo run --bin sentiment-analyzer -- \
  --input social/ \
  --output sentiment/ \
  --zk-circuit sentiment.r1cs
```

### Phase 3: Correlation (Week 3)
```bash
# Correlate social events with price
cargo run --bin price-correlator -- \
  --social sentiment/ \
  --price hf_dataset/ \
  --output unified/events.parquet
```

### Phase 4: DAO Integration (Week 4)
```bash
# Scrape DAO votes and correlate
cargo run --bin dao-analyzer -- \
  --contract <DAO_ADDRESS> \
  --events unified/events.parquet \
  --output governance/
```

## ZK Sentiment Proof

### Circuit Design
```
Input (public):
  - post_hash: Hash of social media post
  - timestamp: Unix timestamp
  - platform: "twitter" | "telegram" | "discord"

Input (private):
  - content: Full post content
  - sentiment_model_weights: ML model weights

Output (public):
  - sentiment_score: -1.0 to 1.0
  - proof: ZK-SNARK proof

Constraints:
  1. Hash(content) == post_hash
  2. sentiment_score = SentimentModel(content, weights)
  3. -1.0 <= sentiment_score <= 1.0
```

### Why ZK?
- **Privacy**: Don't reveal full post content
- **Verifiability**: Anyone can verify sentiment without re-running model
- **Immutability**: Proof is cryptographic evidence

## Analysis Queries

### Correlation Analysis
```sql
-- Price impact of social posts
SELECT 
  DATE(timestamp) as date,
  AVG(sentiment_score) as avg_sentiment,
  AVG(price_usd) as avg_price,
  COUNT(*) as post_count
FROM unified_events
WHERE event_type = 'SocialPost'
GROUP BY date
ORDER BY date;

-- DAO votes vs sentiment
SELECT 
  proposal_id,
  AVG(sentiment_score) as community_sentiment,
  SUM(CASE WHEN vote_type = 'yes' THEN voter_weight ELSE 0 END) as yes_weight,
  SUM(CASE WHEN vote_type = 'no' THEN voter_weight ELSE 0 END) as no_weight
FROM unified_events
GROUP BY proposal_id;
```

## Deliverables

### Datasets
1. `social_posts_2025.parquet` - All social media posts
2. `sentiment_zk_proofs.parquet` - ZK-proven sentiment scores
3. `price_events_2025.parquet` - All price updates
4. `dao_votes_2025.parquet` - All DAO votes
5. `unified_events_2025.parquet` - Combined timeline

### Archives
- `solfunmeme-social-2025.nar.zst` - Nix archive
- Upload to HuggingFace: `introspector/solfunmeme-social`
- Upload to Archive.org
- Pin to IPFS

### Visualizations
- Sentiment vs Price chart
- Vote correlation heatmap
- Social activity timeline

## Next Steps

1. [ ] Create `tools/social-scraper/` package
2. [ ] Implement Twitter API integration
3. [ ] Implement Telegram bot for history
4. [ ] Implement Discord webhook scraper
5. [ ] Design ZK sentiment circuit
6. [ ] Build correlation engine
7. [ ] Create DAO vote scraper
8. [ ] Generate unified Parquet dataset
9. [ ] Upload to HuggingFace
10. [ ] Write analysis notebook

## Use Cases

### For Traders
- Predict price movements from sentiment
- Identify whale activity correlated with posts
- Track community engagement metrics

### For DAO Members
- See how sentiment affects votes
- Identify influential community members
- Track proposal success factors

### For Researchers
- Study meme coin social dynamics
- Analyze ZK-ML applications
- Research DAO governance patterns

---

**Status**: 🟡 Design Phase  
**Priority**: High  
**Timeline**: 4 weeks  
**Dependencies**: Social API access, ZK circuit library
