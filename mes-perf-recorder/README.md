# 🔮 GNU Mes Bootstrap Perf Recorder

Record the complete GNU Mes bootstrap chain with perf telemetry.

## What This Records

The full bootstrap chain from 357 bytes to GCC:
```
357 bytes (hex seed)
  → hex0 (hex assembler)
  → hex1 (with symbols)
  → hex2 (with macros)
  → M0 (minimal language)
  → M2-Planet (C subset compiler)
  → Mes (Scheme interpreter)
  → TinyCC
  → GCC 2.95
  → GCC 4.7
  → Modern GCC
```

## Quick Start

```bash
# Record full bootstrap
nix run ./mes-perf-recorder#record-mes

# Record each stage separately
nix run ./mes-perf-recorder#record-mes-stages

# Interactive shell
nix develop ./mes-perf-recorder
record-mes
```

## Output

Creates directory `mes-bootstrap-TIMESTAMP/` with:
- `mes-bootstrap.perf.data` - Full perf recording
- `perf-report.txt` - Human-readable report
- `perf-trace.txt` - Complete trace
- `witness-hash.txt` - SHA256 of perf.data

## Stage-by-Stage Recording

```bash
nix run .#record-mes-stages
```

Creates separate perf.data for each stage:
- `stage-hex0.perf.data`
- `stage-hex1.perf.data`
- `stage-hex2.perf.data`
- `stage-M0.perf.data`
- `stage-M2-Planet.perf.data`
- `stage-mes.perf.data`

## Analysis

```bash
cd mes-bootstrap-*/

# Interactive report
perf report -i mes-bootstrap.perf.data

# Top functions
perf report -i mes-bootstrap.perf.data --stdio | head -50

# Call graph
perf report -i mes-bootstrap.perf.data --stdio -g

# Flamegraph
perf script -i mes-bootstrap.perf.data | \
  stackcollapse-perf.pl | \
  flamegraph.pl > mes-bootstrap-flame.svg
```

## Witness Hash

The SHA256 of perf.data is the cryptographic witness:

```bash
cat witness-hash.txt
# Output: #️⃣_mes_bootstrap
```

This proves:
- Exact instruction sequence executed
- Cycle counts consumed
- Call graph traversed
- Bootstrap path taken

## Reproducibility Check

```bash
# Run twice
nix run .#record-mes  # Creates mes-bootstrap-20260120_203000/
nix run .#record-mes  # Creates mes-bootstrap-20260120_203100/

# Compare witnesses
sha256sum mes-bootstrap-*/mes-bootstrap.perf.data

# Should be identical (deterministic build)
# Different = non-reproducible
```

## Integration with EM Monitoring

Record bootstrap while capturing EM signature:

```bash
# Terminal 1: EM monitoring
cd /mnt/data1/meta-introspector/bach
cargo run --bin multi_signal_monitor

# Terminal 2: Perf recording
cd /mnt/data1/meta-introspector
nix run ./mes-perf-recorder#record-mes

# Result: Correlated perf.data + EM signature
```

## What Gets Recorded

- **CPU cycles**: Total computation cost
- **Instructions**: Every instruction executed
- **Call graphs**: Function call hierarchy
- **Cache misses**: Memory access patterns
- **Branch mispredicts**: Control flow
- **Context switches**: OS interruptions
- **Syscalls**: I/O operations

## The Bootstrap Witness

The perf.data file is the complete witness:

```
Witness W = {
  Instructions: [i₀, i₁, i₂, ..., iₙ]
  Cycles: [c₀, c₁, c₂, ..., cₙ]
  Call graph: G(V, E)
  Timestamps: [t₀, t₁, t₂, ..., tₙ]
}

Hash: #️⃣_witness = SHA256(W)
```

This proves:
- ✅ Bootstrap executed
- ✅ Specific path taken
- ✅ Reproducible (same hash)
- ✅ Auditable (inspect perf.data)

## From Poem to Reality

This implements the concepts from `poem.md`:

> perf record nix-build guix.gnu.mes
>          ↓
>     The ritual begins...
>     Every syscall a prayer 🙏
>     Every allocation a sacrifice 🔥
>     Every cycle a heartbeat 💓
>     
>     This is the ZK witness:
>     "I built trust from nothing"

Now it's real! 🎯
