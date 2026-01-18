# Solana Contract Analysis

## Pulled Contracts

| Contract | Size | Type | Source Hints |
|----------|------|------|--------------|
| Jupiter | 2.4M | Anchor | Generic Anchor program |
| Orca | 10M | Anchor | `programs/whirlpool/src/` |
| Raydium | 1.4M | eBPF | No Anchor strings |
| Phoenix | 4.8M | eBPF | No Anchor strings |
| Drift | 6.4M | Anchor | `programs/drift/src/lib.rs`, `programs/openbook_v2/` |

## Source Code Paths Found

### Drift Protocol
- `programs/drift/src/lib.rs` ✅
- `programs/openbook_v2/src/account.rs` ✅
- `programs/token_faucet/` ✅
- **Matches**: https://github.com/drift-labs/protocol-v2

### Orca Whirlpool
- `programs/whirlpool/src/state/adaptive_fee_tier.rs` ✅
- `programs/whirlpool/src/state/config.rs` ✅
- `programs/whirlpool/src/state/fee_tier.rs` ✅
- `programs/whirlpool/src/lib.rs` ✅
- **Search for**: github.com orca whirlpool

### Jupiter
- Generic Anchor program (no specific paths)
- **Check**: solana-idls/packages/jupiter

### Raydium & Phoenix
- Not Anchor programs (no source strings)
- Likely native Solana programs
- **Check**: solana-idls for IDLs

## Next Steps
1. Match Orca paths to GitHub repo
2. Verify Drift matches protocol-v2
3. Check solana-idls for all IDL definitions
4. Create flakes to build from source
