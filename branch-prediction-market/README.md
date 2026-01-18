# Branch Prediction Market

**Bet on which eBPF branches will execute in Jupiter transactions tomorrow!**

## Overview

A Solana prediction market where users bet on the execution behavior of Jupiter's closed-source aggregator program. By analyzing on-chain transaction logs and mapping them to our extracted control flow graph, we can settle markets based on real execution data.

## Features

- 📊 **Binary Markets**: Will branch X execute? (Yes/No)
- 🔢 **Frequency Markets**: How many times will it execute?
- 🛤️ **Path Markets**: Will execution follow path A→B→C?
- 🏆 **Tournament Markets**: Which branch executes most?

## How It Works

1. **CFG Extraction**: We've extracted 38,173 basic blocks from Jupiter's eBPF binary
2. **Market Creation**: Anyone can create a prediction market for a specific branch
3. **Betting**: Users bet SOL on yes/no outcomes
4. **Oracle**: After time window, oracle analyzes on-chain transactions
5. **Settlement**: Winners split the pool proportionally

## Smart Contract

Built with Anchor framework for Solana.

### Instructions

- `create_market` - Create prediction market for a branch
- `place_bet` - Bet SOL on outcome (yes/no)
- `submit_report` - Oracle submits execution count
- `settle_market` - Settle based on oracle report
- `claim_winnings` - Winners claim their share

### Accounts

- `PredictionMarket` - Market state (branch, time window, bets, outcome)
- `UserBet` - Individual bet (user, amount, prediction)
- `OracleReport` - Execution data (count, timestamp)

## Build & Deploy

```bash
# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## Analysis Tools

Located in parent directory:

- `build_jupiter_cfg.py` - Extract CFG from Jupiter binary
- `predict_jupiter_branches.py` - Learn patterns from traces
- `trace_jupiter.py` - Analyze individual transactions

## Use Cases

### Research
- Understand Jupiter's routing algorithm
- Identify common execution paths
- Study market condition impacts

### Trading
- Predict high-volume periods
- Anticipate routing decisions
- Hedge against slippage

### Development
- Test routing strategies
- Benchmark performance
- Optimize gas usage

## Economics

### Market Creation
- Fee: 0.1 SOL
- Creator earns from trading fees

### Betting
- Minimum: 0.01 SOL
- Trading fee: 1%
- Winners split pool proportionally

### Oracle
- Report fee: 0.01 SOL
- Incentivized for accuracy
- Future: Slashing for false reports

## Roadmap

### Phase 1: MVP ✅
- [x] CFG extraction (38K blocks)
- [x] Basic Anchor program
- [x] Binary markets
- [x] Manual oracle

### Phase 2: Oracle Network
- [ ] Multiple reporters
- [ ] Consensus mechanism
- [ ] Automated data collection
- [ ] Slashing for false reports

### Phase 3: Advanced Markets
- [ ] Frequency ranges
- [ ] Path markets
- [ ] Multi-outcome markets
- [ ] AMM for continuous trading

### Phase 4: Integration
- [ ] Jupiter SDK integration
- [ ] Real-time tracking
- [ ] Historical analysis
- [ ] ML predictions

## Legal

- Jupiter's on-chain program is public data
- Analysis for educational/research purposes
- Cannot use Jupiter trademark/branding
- Must attribute source properly

## Contributing

This is an experimental project. Contributions welcome!

1. Fork the repo
2. Create feature branch
3. Submit PR

## License

AGPL-3.0

## Links

- [Jupiter Program](https://solscan.io/account/JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB)
- [CFG Analysis](../JUPITER_CFG_ANALYSIS.md)
- [Legal Clarification](../JUPITER_LEGAL_CLARIFICATION.md)
- [Full Specification](../BRANCH_PREDICTION_MARKET.md)
