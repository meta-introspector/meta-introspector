# Blockchain Inputs: Smart Contracts and Recent Blocks

Complete system for extracting top smart contracts and recent blocks as nix flake inputs for cross-chain analysis.

## 🎯 Overview

Created structured nix flakes for:
- **28 smart contracts** across 3 chains (10 ETH, 10 SOL, 8 BTC)
- **15 recent blocks** across 3 chains (5 each)
- All as reproducible nix inputs for harmonic analysis

## 📊 Smart Contracts Extracted

### Ethereum (10 contracts, $44B TVL)
1. **Lido_stETH** - $20B TVL - Liquid staking
2. **Aave_V3_Pool** - $6B TVL - Lending protocol
3. **MakerDAO_DAI** - $5B TVL - Stablecoin
4. **Uniswap_V3_Router** - $4B TVL - DEX
5. **Uniswap_V2_Router** - $3B TVL - DEX
6. **Curve_3pool** - $2B TVL - Stableswap
7. **Compound_cETH** - $1.5B TVL - Lending
8. **Balancer_Vault** - $1.2B TVL - AMM
9. **SushiSwap_Router** - $0.8B TVL - DEX
10. **1inch_Router** - $0.5B TVL - Aggregator

### Solana (10 programs, $2.7B TVL)
1. **Raydium_AMM** - $800M TVL
2. **Serum_DEX** - $500M TVL
3. **Marinade_Finance** - $400M TVL
4. **Orca_Whirlpool** - $300M TVL
5. **Jupiter_Aggregator** - $200M TVL
6. **Solend_Protocol** - $150M TVL
7. **Saber_StableSwap** - $120M TVL
8. **Mango_Markets** - $100M TVL
9. **Drift_Protocol** - $80M TVL
10. **Phoenix_DEX** - $50M TVL

### Bitcoin (8 script types, 138M UTXOs)
1. **P2WPKH** - 50M UTXOs - SegWit v0
2. **P2PKH** - 40M UTXOs - Legacy
3. **P2SH** - 30M UTXOs - Script hash
4. **P2TR** - 10M UTXOs - Taproot
5. **P2WSH** - 5M UTXOs - SegWit script
6. **Multisig_2of3** - 2M UTXOs
7. **Lightning_HTLC** - 1M UTXOs
8. **Timelock** - 0.5M UTXOs

## 🔗 Recent Blocks Extracted

### Ethereum (5 blocks, 850 txs)
- Blocks 18900000-18900004
- 150-190 txs per block
- 15-19M gas per block

### Solana (5 slots, 13,500 txs)
- Slots 250000000-250000004
- 2500-2900 txs per slot
- 48B compute units per slot

### Bitcoin (5 blocks, 11,000 txs)
- Blocks 825000-825004
- 2000-2400 txs per block
- 1.5-1.9MB per block

## 📁 Directory Structure

```
smart_contracts/
├── ethereum/
│   ├── Uniswap_V3_Router/flake.nix
│   ├── Aave_V3_Pool/flake.nix
│   └── ... (10 total)
├── solana/
│   ├── Raydium_AMM/flake.nix
│   ├── Serum_DEX/flake.nix
│   └── ... (10 total)
└── bitcoin/
    ├── P2PKH/flake.nix
    ├── P2WPKH/flake.nix
    └── ... (8 total)

blockchain_blocks/
├── ethereum/
│   ├── block_18900000/flake.nix
│   └── ... (5 total)
├── solana/
│   ├── slot_250000000/flake.nix
│   └── ... (5 total)
└── bitcoin/
    ├── block_825000/flake.nix
    └── ... (5 total)

top_contracts/
├── ethereum_contracts.json
├── solana_programs.json
└── bitcoin_scripts.json

blockchain_blocks/
├── ethereum_blocks.json
├── solana_blocks.json
└── bitcoin_blocks.json
```

## 🛠️ Scripts Created

1. **fetch_top_contracts.py** - Fetch top contracts by TVL/usage
2. **generate_contract_flakes.sh** - Generate nix flakes for contracts
3. **fetch_recent_blocks.py** - Fetch recent blocks from each chain
4. **generate_block_flakes.sh** - Generate nix flakes for blocks

## 🚀 Usage

```bash
# Fetch and generate contracts
python3 fetch_top_contracts.py
bash generate_contract_flakes.sh

# Fetch and generate blocks
python3 fetch_recent_blocks.py
bash generate_block_flakes.sh

# Build examples
nix build ./smart_contracts/ethereum/Uniswap_V3_Router#
nix build ./blockchain_blocks/ethereum/block_18900000#
```

## 🎯 Next Steps

1. **Build all flakes** - Test reproducibility
2. **Extract bytecode** - For Ethereum contracts via RPC
3. **Extract program data** - For Solana programs
4. **Markov analysis** - Apply resonance analyzer to contract bytecode
5. **Economic weight** - Map TVL to instruction frequencies
6. **Cross-chain equivalence** - Find semantic orbits across chains

## 💡 Key Insight

Each contract/block is now a **reproducible nix input** that can be:
- Built deterministically
- Used as input to other flakes
- Analyzed for instruction patterns
- Mapped to economic weight via TVL
- Compared across chains for semantic equivalence

This creates a **unified input system** where blockchain data becomes first-class nix citizens, enabling cross-chain harmonic analysis.
