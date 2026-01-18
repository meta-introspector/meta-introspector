# Contract Activity Prediction Markets

## Concept

**Bet on which Solana contracts will be most active tomorrow!**

Combines:
1. Historical on-chain activity data
2. Branch execution patterns
3. Market sentiment
4. Time-series prediction

## Market Types

### 1. Top 20 Most Active Contracts

**Question**: Which contracts will have most transactions tomorrow?

**Ranking Markets**:
```
1st place: Jupiter (0.35 SOL/share)
2nd place: Orca (0.20 SOL/share)
3rd place: Raydium (0.15 SOL/share)
...
20th place: Drift (0.01 SOL/share)
```

**Settlement**: Count transactions in 24h window, rank by volume

### 2. Activity Threshold Markets

**Question**: Will Jupiter have >100K transactions tomorrow?

**Binary Markets**:
- Jupiter >100K: Yes/No
- Orca >50K: Yes/No
- Raydium >75K: Yes/No

### 3. Relative Activity Markets

**Question**: Will Jupiter have more transactions than Orca?

**Comparison Markets**:
- Jupiter vs Orca
- Raydium vs Phoenix
- Drift vs Mango

### 4. Activity Range Markets

**Question**: How many transactions will Jupiter have?

**Range Markets**:
- 0-50K: 0.10 SOL
- 50K-100K: 0.35 SOL
- 100K-200K: 0.40 SOL
- 200K+: 0.15 SOL

## Data Sources

### Historical Activity
```python
def get_contract_activity(program_id, days=30):
    """Get historical transaction counts"""
    # Query Solana RPC or indexer
    txs_per_day = []
    for day in range(days):
        count = get_transaction_count(program_id, day)
        txs_per_day.append(count)
    return txs_per_day

# Top 20 Solana contracts by activity
TOP_CONTRACTS = [
    "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB",  # Jupiter
    "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",  # Orca
    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",  # Raydium
    "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY",  # Phoenix
    "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",  # Drift
    # ... 15 more
]
```

### Branch Execution Correlation
```python
def correlate_branches_to_activity(program_id):
    """Find which branches correlate with high activity"""
    activity = get_contract_activity(program_id)
    branch_counts = get_branch_execution_counts(program_id)
    
    # Find correlation
    correlations = {}
    for branch_addr, counts in branch_counts.items():
        corr = pearson_correlation(activity, counts)
        correlations[branch_addr] = corr
    
    return correlations
```

### Market Indicators
```python
def get_market_indicators():
    """External factors affecting activity"""
    return {
        'sol_price': get_sol_price(),
        'total_volume': get_dex_volume(),
        'gas_price': get_priority_fees(),
        'time_of_day': get_time_of_day(),
        'day_of_week': get_day_of_week(),
    }
```

## Smart Contract Design

### Accounts

```rust
pub struct ContractActivityMarket {
    pub market_type: MarketType,
    pub contracts: Vec<Pubkey>,  // Top 20 contracts
    pub time_window: TimeWindow,
    pub total_pool: u64,
    pub settled: bool,
    pub results: Option<Vec<ActivityResult>>,
}

pub enum MarketType {
    TopN { n: u8 },              // Top N ranking
    Threshold { threshold: u64 }, // Above/below threshold
    Comparison { a: Pubkey, b: Pubkey }, // A vs B
    Range { ranges: Vec<Range> }, // Activity ranges
}

pub struct ActivityResult {
    pub contract: Pubkey,
    pub transaction_count: u64,
    pub rank: u8,
}

pub struct ContractBet {
    pub market: Pubkey,
    pub user: Pubkey,
    pub contract: Pubkey,  // Which contract they bet on
    pub amount: u64,
    pub claimed: bool,
}
```

### Instructions

```rust
pub enum Instruction {
    /// Create top-N ranking market
    CreateRankingMarket {
        contracts: Vec<Pubkey>,
        n: u8,
        time_window: TimeWindow,
    },
    
    /// Bet on a contract's rank
    BetOnRank {
        contract: Pubkey,
        predicted_rank: u8,
        amount: u64,
    },
    
    /// Submit activity data (oracle)
    SubmitActivity {
        results: Vec<ActivityResult>,
    },
    
    /// Settle market
    SettleMarket,
    
    /// Claim winnings
    ClaimWinnings,
}
```

## Oracle Implementation

### Data Collection
```python
class ActivityOracle:
    def collect_activity_data(self, contracts, start_time, end_time):
        """Collect transaction counts for all contracts"""
        results = []
        
        for contract in contracts:
            # Query Solana RPC
            signatures = get_signatures_for_address(
                contract,
                start_time,
                end_time
            )
            
            count = len(signatures)
            results.append({
                'contract': contract,
                'transaction_count': count,
                'signatures': signatures[:100]  # Proof sample
            })
        
        # Rank by activity
        results.sort(key=lambda x: x['transaction_count'], reverse=True)
        for i, result in enumerate(results):
            result['rank'] = i + 1
        
        return results
```

### Verification
```python
def verify_activity_report(report):
    """Verify oracle report on-chain"""
    for result in report:
        # Sample check: verify first 10 signatures
        for sig in result['signatures'][:10]:
            tx = get_transaction(sig)
            assert result['contract'] in tx.accounts
            assert start_time <= tx.timestamp <= end_time
    
    return True
```

## Prediction Models

### Time Series Forecasting
```python
def predict_activity(contract, historical_data):
    """Predict tomorrow's activity using time series"""
    from statsmodels.tsa.arima.model import ARIMA
    
    # Fit ARIMA model
    model = ARIMA(historical_data, order=(7, 1, 1))
    fitted = model.fit()
    
    # Forecast next day
    forecast = fitted.forecast(steps=1)
    return forecast[0]
```

### Branch-Based Prediction
```python
def predict_from_branches(contract, branch_patterns):
    """Predict activity based on branch execution patterns"""
    # Get recent branch execution
    recent_branches = get_recent_branch_counts(contract, days=7)
    
    # Find similar historical patterns
    similar_days = find_similar_patterns(recent_branches, historical_data)
    
    # Average activity on similar days
    predicted = np.mean([day.activity for day in similar_days])
    return predicted
```

### Ensemble Model
```python
def ensemble_prediction(contract):
    """Combine multiple prediction methods"""
    historical = get_historical_activity(contract, days=30)
    branches = get_branch_patterns(contract, days=7)
    market = get_market_indicators()
    
    # Multiple models
    pred_ts = predict_activity(contract, historical)
    pred_branch = predict_from_branches(contract, branches)
    pred_market = predict_from_market(contract, market)
    
    # Weighted average
    prediction = (
        0.4 * pred_ts +
        0.3 * pred_branch +
        0.3 * pred_market
    )
    
    return prediction
```

## Market Mechanics

### Ranking Market Payout
```python
def calculate_ranking_payout(bet, actual_rank):
    """Payout based on prediction accuracy"""
    predicted_rank = bet.predicted_rank
    
    # Exact match: 100% of pool share
    if predicted_rank == actual_rank:
        return bet.amount * 10
    
    # Off by 1: 50% of pool share
    elif abs(predicted_rank - actual_rank) == 1:
        return bet.amount * 5
    
    # Off by 2: 25% of pool share
    elif abs(predicted_rank - actual_rank) == 2:
        return bet.amount * 2.5
    
    # Wrong: lose bet
    else:
        return 0
```

### Dynamic Odds
```python
def calculate_odds(contract, current_bets):
    """Calculate current odds based on betting pool"""
    total_pool = sum(bet.amount for bet in current_bets)
    contract_pool = sum(
        bet.amount for bet in current_bets 
        if bet.contract == contract
    )
    
    # Odds = total_pool / contract_pool
    odds = total_pool / contract_pool if contract_pool > 0 else 1.0
    return odds
```

## Integration with Branch Markets

### Combined Markets
```python
class CombinedMarket:
    """Bet on both activity AND branch execution"""
    
    def create_combined_market(self):
        # Market: Jupiter will be #1 AND branch 0x372b will execute >1000 times
        return {
            'activity_condition': {
                'contract': 'Jupiter',
                'rank': 1
            },
            'branch_condition': {
                'contract': 'Jupiter',
                'branch': 0x372b,
                'threshold': 1000
            },
            'payout': '10x if both true, 0 otherwise'
        }
```

### Correlation Analysis
```python
def analyze_activity_branch_correlation():
    """Find which branches predict high activity"""
    contracts = get_top_contracts(20)
    
    for contract in contracts:
        activity = get_daily_activity(contract, days=30)
        branches = get_daily_branch_counts(contract, days=30)
        
        # Find predictive branches
        for branch_addr, counts in branches.items():
            corr = correlation(activity, counts)
            if corr > 0.7:
                print(f"Branch {branch_addr:x} predicts activity (r={corr:.2f})")
```

## Use Cases

### 1. Trading Strategy
- Predict high-volume days
- Anticipate liquidity
- Time trades optimally

### 2. Infrastructure Planning
- Scale RPC nodes
- Allocate compute resources
- Optimize caching

### 3. Market Research
- Understand user behavior
- Identify trends
- Competitive analysis

### 4. Risk Management
- Hedge against volatility
- Diversify exposure
- Manage slippage

## Roadmap

### Phase 1: Activity Markets
- [x] Top 20 contract tracking
- [ ] Ranking markets
- [ ] Threshold markets
- [ ] Historical data collection

### Phase 2: Prediction Models
- [ ] Time series forecasting
- [ ] Branch correlation analysis
- [ ] Ensemble predictions
- [ ] ML-based models

### Phase 3: Combined Markets
- [ ] Activity + branch markets
- [ ] Multi-contract markets
- [ ] Conditional markets
- [ ] Derivative markets

### Phase 4: Platform
- [ ] Real-time dashboards
- [ ] API for predictions
- [ ] Automated trading
- [ ] Market analytics

## Example Markets

### Tomorrow's Top 5
```
Bet on which contracts will be top 5 most active tomorrow:

Current odds:
1. Jupiter: 1.2x
2. Orca: 2.5x
3. Raydium: 3.0x
4. Phoenix: 5.0x
5. Drift: 4.0x
6. Mango: 8.0x
...

Total pool: 1,000 SOL
Time remaining: 6 hours
```

### Activity Threshold
```
Will Jupiter have >100K transactions tomorrow?

Yes: 0.65 SOL/share (65% probability)
No: 0.35 SOL/share (35% probability)

Historical average: 85K transactions/day
Last 7 days: 92K avg
Prediction model: 105K ± 15K
```

### Head-to-Head
```
Which will have more transactions tomorrow?

Jupiter vs Orca

Jupiter: 0.70 SOL/share
Orca: 0.30 SOL/share

Historical win rate: Jupiter 75%, Orca 25%
```

## Next Steps

1. **Collect historical data** for top 20 contracts
2. **Build prediction models** (time series, branches, ensemble)
3. **Create ranking markets** smart contract
4. **Deploy oracle** for activity tracking
5. **Launch test markets** on devnet
6. **Integrate with branch markets** for combined predictions
