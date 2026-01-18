# Jupiter eBPF Reverse Engineering Plan

## Goal
Understand Jupiter's closed-source aggregator by tracing eBPF execution and analyzing on-chain data.

## What We Have
1. ✅ Jupiter program binary (2.4M, eBPF)
2. ✅ Disassembly (143K lines)
3. ✅ Error codes and constraints
4. ✅ Anchor structure hints
5. ✅ On-chain transaction logs

## Analysis Approach

### Phase 1: Static Analysis (Done)
- [x] Decompile to eBPF assembly
- [x] Extract strings and error messages
- [x] Identify Anchor structure
- [x] Map constraints

### Phase 2: Dynamic Analysis (Current)
- [ ] Trace real Jupiter transactions
- [ ] Map instruction discriminators to eBPF functions
- [ ] Analyze execution logs
- [ ] Track state changes (token balances)
- [ ] Identify CPI calls to AMMs

### Phase 3: Pattern Recognition
- [ ] Identify routing algorithm patterns
- [ ] Map fee calculation logic
- [ ] Understand slippage protection
- [ ] Reverse engineer price impact calculations

### Phase 4: Reconstruction
- [ ] Document instruction formats
- [ ] Create IDL from analysis
- [ ] Build reference implementation
- [ ] Validate against on-chain behavior

## Tools Created

1. **analyze_jupiter_ebpf.sh** - Static analysis
   - Extracts instructions, errors, constraints
   - Creates trace template

2. **trace_jupiter.py** - Dynamic analysis
   - Fetches transactions from Solana
   - Maps discriminators to eBPF
   - Analyzes execution logs
   - Tracks state changes

3. **decompile_solana_contracts.sh** - Binary analysis
   - Disassembles all contracts
   - Extracts symbols and strings

## Data Sources

### On-Chain
- Transaction logs (via `solana transaction`)
- Account state changes
- Token balance deltas
- CPI calls to other programs

### Off-Chain
- Solscan transaction explorer
- Jupiter API responses
- SDK behavior

## Next Steps

1. **Get sample transactions**:
   ```bash
   # Find recent Jupiter swaps on Solscan
   # https://solscan.io/account/JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB
   ```

2. **Trace execution**:
   ```bash
   python3 trace_jupiter.py <signature>
   ```

3. **Map eBPF flow**:
   - Match discriminator to entry point
   - Follow execution through disassembly
   - Identify key decision points

4. **Build knowledge base**:
   - Document each instruction type
   - Map routing strategies
   - Understand fee structures

## Legal/Ethical Notes

- Jupiter is closed source by choice (business model)
- Reverse engineering for understanding is generally legal
- Cannot redistribute or create competing closed-source clone
- Can create open-source alternative with different implementation
- Analysis for educational/research purposes is acceptable

## Expected Outcomes

1. **Instruction IDL** - Complete interface definition
2. **Routing algorithm** - High-level understanding
3. **Fee structure** - How Jupiter captures value
4. **Integration guide** - How to interact with the program
5. **Reference docs** - Community knowledge base
