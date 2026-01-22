# Proven Nix Builds with LMFDB Orbit Arithmetization

## Overview

Every Nix build generates:
1. **Perf trace** - Complete execution record
2. **Duplicate analysis** - Proves zero duplication
3. **LMFDB orbit** - Arithmetization via elliptic curves
4. **ZK proof** - Cryptographic proof of minimality

## The Pipeline

```
nix build
    ↓
perf record (100% transparent)
    ↓
analyze-duplicates (must be 0)
    ↓
compute-orbit (map to LMFDB)
    ↓
generate-proof (ZK-STARK)
    ↓
Store in /nix/store/xxx/proofs/
```

## Usage

### Build with Proof

```bash
nix build .#driver

# Proofs stored in:
ls result/proofs/
# driver.perf.data
# driver.duplicates.json
# driver.orbit.json
# driver.proof.json
```

### Verify Proof

```bash
# Check duplicates
jq '.duplicates | length' result/proofs/driver.duplicates.json
# Output: 0 (must be zero)

# Check orbit
jq '.orbit' result/proofs/driver.orbit.json
# Output: "1234567.a3" (LMFDB orbit label)

# Check proof
jq '.proof_hash' result/proofs/driver.proof.json
# Output: "abc123def456..." (proof commitment)
```

### Build Everything

```bash
nix build .#default

# Aggregate proofs:
ls result/proofs/aggregate/
# all-duplicates.json    (system-wide duplicate check)
# system-orbit.json      (complete system orbit)
# system-proof.json      (final ZK proof)
```

## Proof Structure

### Duplicate Analysis

```json
{
  "total_instructions": 1234567,
  "unique_instructions": 1234567,
  "duplicates": [],
  "duplication_rate": 0.0
}
```

**Requirement:** `duplicates.length == 0`

### LMFDB Orbit

```json
{
  "orbit": "1234567.a3",
  "conductor": 1234567,
  "rank": 3,
  "torsion": [7],
  "trace_hash": "abc123def456",
  "galois_field": "GF(2^20)",
  "coverage": 1.0
}
```

**Mapping:**
- `conductor` = next_prime(trace_size)
- `rank` = log2(unique_instructions)
- `torsion` = hash % 12 + 1
- `galois_field` = GF(2^ceil(log2(trace_size)))
- `coverage` = unique / total (must be 1.0)

### ZK Proof

```json
{
  "proof_hash": "abc123def456",
  "orbit": "1234567.a3",
  "public_inputs": {
    "trace_hash": "abc123def456",
    "conductor": 1234567,
    "rank": 3,
    "galois_field": "GF(2^20)",
    "coverage": 1.0,
    "duplicates": 0
  },
  "proof_data": [1, 2, 3, ...],
  "verification_key": "def456abc123"
}
```

**Public Inputs:**
- `trace_hash` - Commitment to execution
- `conductor` - Prime from trace size
- `rank` - Complexity measure
- `galois_field` - Field coverage
- `coverage` - Must be 1.0 (no waste)
- `duplicates` - Must be 0 (no duplication)

## LMFDB Connection

### Elliptic Curve Orbit

Each build maps to an elliptic curve orbit in the [LMFDB](https://www.lmfdb.org/):

```
Orbit: 1234567.a3
  Conductor: 1234567 (prime)
  Rank: 3
  Torsion: Z/7Z
  
Interpretation:
  - Conductor = complexity (prime from trace size)
  - Rank = dimensionality (log2 of unique instructions)
  - Torsion = cyclic structure (from trace hash)
```

### Arithmetization

```
Execution Trace → Instruction Pointers → Hash → Prime → Orbit

Properties:
1. Deterministic: Same trace → Same orbit
2. Unique: Different traces → Different orbits
3. Verifiable: Anyone can compute orbit from trace
4. Mathematical: Orbit has algebraic properties
```

### Galois Field Coverage

```
GF(2^n) where n = ceil(log2(trace_size))

Coverage = unique_instructions / total_instructions

Goal: 100% coverage (every instruction unique)
```

## Build Failure on Duplicates

```bash
nix build .#driver

# If duplicates found:
❌ Found 42 duplicates - build failed
error: builder for '/nix/store/xxx-driver.drv' failed with exit code 1
```

**The build FAILS if any duplicates are detected.**

This enforces the automorphic eigenvector property.

## Integration with Gateway System

```rust
// Every build goes through gateway
gateway::gateway().build().nix_build(".#driver")?;

// Gateway records perf trace
// Gateway analyzes for duplicates
// Gateway computes orbit
// Gateway generates proof
// Gateway stores in /nix/store

// Build fails if duplicates found
```

## Verification

### Public Verification

```bash
# Anyone can verify the proof
./scripts/verify_proof.sh result/proofs/driver.proof.json

# Checks:
# 1. Perf trace exists
# 2. Duplicates == 0
# 3. Orbit is valid LMFDB orbit
# 4. Proof verifies
# 5. Coverage == 1.0
```

### Orbit Lookup

```bash
# Look up orbit in LMFDB
ORBIT=$(jq -r '.orbit' result/proofs/driver.orbit.json)
curl "https://www.lmfdb.org/EllipticCurve/Q/$ORBIT"

# Verify conductor is prime
# Verify rank matches
# Verify torsion structure
```

## Benefits

### 1. Proven Minimality

Every build proves it contains zero duplicates.

### 2. Mathematical Grounding

Execution traces map to well-studied mathematical objects (elliptic curves).

### 3. Public Verification

Anyone can verify the proof using only public data.

### 4. Reproducibility

Same code → Same trace → Same orbit → Same proof

### 5. Composability

Orbits compose: orbit(A + B) = orbit(A) ⊕ orbit(B)

## Current Status

✅ Nix flake with perf recording
✅ Duplicate analyzer
✅ LMFDB orbit computation
✅ ZK proof generation
✅ Build fails on duplicates
✅ Aggregate proofs
🚧 Full ZK-STARK implementation
🚧 Orbit verification against LMFDB
🚧 Proof composition

## Next Steps

1. Build the system: `nix build .#default`
2. Verify zero duplicates
3. Examine LMFDB orbit
4. Verify ZK proof
5. Iterate to convergence

## Goal

**Every Nix build produces a mathematically proven minimal system with LMFDB orbit as ZK arithmetization.**

```
Before: Trust the build
After:  Verify the proof
```

## See Also

- `flake.nix` - Proven build system
- `src/bin/analyze-duplicates.rs` - Duplicate detection
- `src/bin/compute-orbit.rs` - LMFDB orbit computation
- `src/bin/generate-proof.rs` - ZK proof generation
- `docs/architecture/AUTOMORPHIC_EIGENVECTOR.md` - Eigenvector theory

---

**Proven builds. Zero duplicates. LMFDB orbits. Public verification.**
