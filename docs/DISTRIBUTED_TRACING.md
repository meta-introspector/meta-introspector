# Distributed Reachability Network

## Vision

A peer-to-peer network where contributors earn rewards for donating compute resources to trace Rust compilations.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                 Distributed Tracing Network                  │
│                                                              │
│  Developer                    P2P Network              Miner │
│     │                              │                     │   │
│     │ Submit job                   │                     │   │
│     ├──────────────────────────────┤                     │   │
│     │                              │ Request work        │   │
│     │                              ├─────────────────────┤   │
│     │                              │ Trace compilation   │   │
│     │                              │ (QEMU + plugin)     │   │
│     │                              ├─────────────────────┤   │
│     │                              │ Submit parquet      │   │
│     │ Receive parquet + pay reward │                     │   │
│     ├──────────────────────────────┤                     │   │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Tracing Node (Miner)

Runs QEMU + reachability plugin, earns tokens:

```rust
struct TracingNode {
    qemu_path: PathBuf,
    plugin_path: PathBuf,
    wallet: SolanaWallet,
    libp2p_peer: PeerId,
}

impl TracingNode {
    async fn mine_traces(&mut self) -> Result<()> {
        loop {
            // 1. Request work from network
            let job = self.request_job().await?;
            
            // 2. Run QEMU trace
            let parquet = self.trace_compilation(&job).await?;
            
            // 3. Submit result
            let reward = self.submit_result(parquet).await?;
            
            // 4. Receive payment
            self.wallet.receive(reward).await?;
            
            println!("✅ Earned {} tokens", reward);
        }
    }
}
```

### 2. Job Coordinator

Distributes work, validates results:

```rust
struct JobCoordinator {
    pending_jobs: Vec<TraceJob>,
    completed_traces: HashMap<JobId, ParquetFile>,
    reward_pool: TokenAmount,
}

impl JobCoordinator {
    async fn submit_job(&mut self, source: PathBuf, reward: TokenAmount) -> JobId {
        let job = TraceJob {
            id: JobId::new(),
            source,
            reward,
            submitted_at: Timestamp::now(),
        };
        
        self.pending_jobs.push(job.clone());
        self.broadcast_job(job).await;
        
        job.id
    }
    
    async fn validate_result(&self, job_id: JobId, parquet: ParquetFile) -> bool {
        // Verify parquet is valid
        // Check against multiple submissions
        // Ensure no fake data
        true
    }
}
```

### 3. P2P Protocol

libp2p-based network:

```rust
#[derive(NetworkBehaviour)]
struct ReachabilityBehaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    request_response: RequestResponse<TraceProtocol>,
}

// Message types
enum TraceMessage {
    JobAvailable(TraceJob),
    RequestJob,
    SubmitResult { job_id: JobId, parquet: Vec<u8> },
    PaymentProof { tx_hash: String, amount: u64 },
}
```

## Economic Model

### Earning Rewards

**Miners earn tokens for**:
- Tracing compilations (base reward)
- Fast turnaround (speed bonus)
- High-quality traces (accuracy bonus)
- Rare architectures (diversity bonus)

**Reward Formula**:
```
reward = base_reward 
       × (1 + speed_bonus) 
       × (1 + accuracy_bonus) 
       × (1 + diversity_bonus)
```

### Pricing

**Job submitters pay**:
- Per source file size
- Per expected trace size
- Priority multiplier (urgent jobs)

**Example**:
```
Small crate (10KB):  0.1 tokens
Medium crate (100KB): 1.0 tokens
Large crate (1MB):   10.0 tokens
Rustc (500MB):      5000.0 tokens
```

### Token Economics

**Token**: REACH (Reachability Token)

**Supply**: 
- Initial: 1,000,000 REACH
- Inflation: 5% annually (mining rewards)
- Burn: 1% of job fees

**Distribution**:
- 40% Mining rewards
- 30% Development fund
- 20% Early contributors
- 10% Reserve

## Implementation

### Phase 1: Local Network (Week 1-2)

```bash
# Start coordinator
reach-coordinator --port 3000

# Start miner
reach-miner --coordinator localhost:3000 --wallet wallet.json

# Submit job
reach-submit job.rs --reward 1.0
```

### Phase 2: P2P Network (Week 3-4)

```bash
# Start node (auto-discovers peers)
reach-node --wallet wallet.json

# Submit to network
reach-submit job.rs --reward 1.0 --network mainnet
```

### Phase 3: Solana Integration (Week 5-6)

```bash
# Create wallet
solana-keygen new -o reach-wallet.json

# Fund wallet
solana airdrop 1 reach-wallet.json

# Start earning
reach-miner --wallet reach-wallet.json --network mainnet
```

## Incentive Alignment

### For Miners

**Benefits**:
- Earn passive income
- Contribute to open source
- Learn about compilation
- Access to ecosystem data

**Requirements**:
- QEMU + plugin installed
- Stable internet connection
- Honest behavior (no fake traces)

### For Developers

**Benefits**:
- Fast trace generation
- Distributed compute
- No local QEMU overhead
- Access to rare architectures

**Cost**:
- Pay per trace
- Cheaper than cloud compute
- Supports ecosystem

### For Ecosystem

**Benefits**:
- Complete Rust provenance data
- Distributed infrastructure
- Economic sustainability
- Community participation

## Security

### Preventing Cheating

1. **Multiple submissions**: Same job to multiple miners
2. **Validation**: Cross-check parquet files
3. **Reputation**: Track miner accuracy
4. **Slashing**: Penalize fake submissions

### Privacy

1. **Optional encryption**: Encrypt source before submission
2. **Trusted miners**: Whitelist for sensitive code
3. **Local tracing**: Keep sensitive code local

## Scaling

### Current (QEMU)
- 1 trace = 10-100x slower than native
- 1 node = ~10 traces/day
- 100 nodes = ~1000 traces/day

### Future (Rustc Backend)
- 1 trace = 1-2x slower than native
- 1 node = ~1000 traces/day
- 100 nodes = ~100K traces/day

### Target
- 1M crates on crates.io
- 100K active crates
- 10K daily compilations
- Need: ~100 nodes

## Roadmap

### Month 1: Prototype
- [ ] Local coordinator
- [ ] Single miner
- [ ] Mock payments

### Month 2: P2P Network
- [ ] libp2p integration
- [ ] Peer discovery
- [ ] Job distribution

### Month 3: Token Integration
- [ ] Solana smart contract
- [ ] Wallet integration
- [ ] Real payments

### Month 4: Production
- [ ] Mainnet launch
- [ ] 10+ miners
- [ ] First paid traces

### Month 6: Scale
- [ ] 100+ miners
- [ ] 1000+ traces/day
- [ ] Ecosystem integration

## Get Started

### As a Miner

```bash
# Install
cargo install reach-miner

# Configure
reach-miner init --wallet wallet.json

# Start earning
reach-miner start
```

### As a Developer

```bash
# Install
cargo install reach-cli

# Submit job
reach-cli trace my-crate/ --reward 1.0

# Wait for result
reach-cli status <job-id>

# Download parquet
reach-cli download <job-id> output.parquet
```

## Economics Example

### Scenario: Tracing Top 100 Crates

**Cost**:
- 100 crates × 1.0 REACH = 100 REACH
- At $0.10/REACH = $10

**Miner Earnings**:
- 100 traces × 0.9 REACH (after fees) = 90 REACH
- Distributed to 10 miners = 9 REACH each
- At $0.10/REACH = $0.90 per miner

**Time**:
- 100 traces / 10 miners = 10 traces per miner
- At 1 hour per trace = 10 hours
- Earnings: $0.90 / 10 hours = $0.09/hour

**With Rustc Backend** (100x faster):
- 10 hours → 6 minutes
- $0.09/hour → $9/hour
- Competitive with cloud compute!

## Vision

**Year 1**: 100 miners, 10K traces
**Year 2**: 1000 miners, 100K traces
**Year 3**: Complete crates.io traced
**Year 5**: Standard Rust infrastructure

**Ultimate Goal**: Self-sustaining distributed tracing network that makes Reachable Rust economically viable.

## Join the Network

- **Mine**: Earn tokens by tracing
- **Submit**: Pay for fast traces
- **Develop**: Build on the protocol
- **Invest**: Support the ecosystem

See: [DISTRIBUTED_TRACING.md](DISTRIBUTED_TRACING.md)

---

**Faster progress through economic incentives.**
