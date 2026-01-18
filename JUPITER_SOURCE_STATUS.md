# Jupiter Source Code Status

## What's Open Source ✅

### SDKs and Tools (github.com/jup-ag)
- `@jup-ag/core` - NPM package for routing
- `@jup-ag/react-hook` - React integration
- `@jup-ag/api` - Quote API bindings
- `jupiter-core-example` - Example implementations
- Integrated AMM SDKs (forks):
  - whirlpool-sdk
  - raydium-clmm-sdk
  - lifinity-amm-v2
  - cykura-sdk, dradex-sdk, invariant-protocol, etc.

### Documentation
- https://station.jup.ag/docs - Developer docs
- Quote API (Rust and Node.js bindings)
- CLI tools for fee wallet management

## What's NOT Open Source ❌

### Core On-Chain Program
- **JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB** - Closed source
- The actual aggregator smart contract is proprietary
- Only the routing/SDK layer is open

## Why This Matters

Jupiter's business model:
1. **Open SDK** - Anyone can integrate Jupiter routing
2. **Closed program** - Jupiter controls the on-chain aggregation logic
3. **Fee capture** - Jupiter earns fees through the closed program

## What We Have

From decompilation:
- ✅ Generic Anchor program structure
- ✅ Error messages and constraints
- ✅ eBPF assembly (143K lines)
- ❌ No source code paths (unlike Drift/Orca)

## Comparison

| Protocol | Program | SDK | Status |
|----------|---------|-----|--------|
| Drift | ✅ Open | ✅ Open | Fully open source |
| Orca | ✅ Open | ✅ Open | Fully open source |
| Jupiter | ❌ Closed | ✅ Open | Hybrid model |
| Raydium | ❌ Closed | ✅ Open | Hybrid model |
| Phoenix | ❌ Closed | ✅ Open | Hybrid model |

## Conclusion

**Jupiter's core program is NOT open source.** We can:
1. Use their SDK to integrate
2. Analyze the binary (already done)
3. Build Drift and Orca from source ✅
4. Use Jupiter via their API/SDK only
