# ZOS Server Integration Plan

**Objective**: Use existing ~/zos-server Rust code as .so library for evolution server

## Current State

**Python Evolution Server**: `scripts/build/evolution_server.py`
- Monitors build errors
- Calls Gemini for fixes
- Tracks iterations

**Existing Rust Code**: `~/zos-server/*.rs`
- Multiple Rust modules
- No Cargo.toml structure
- Needs organization

## Plan: Lift Python → Rust via Existing Code

### Step 1: Organize ~/zos-server into Library

```bash
cd ~/zos-server
cargo init --lib

# Move relevant files to src/
mv automorphic_field.rs src/
mv audited_automorphic_system.rs src/
# ... etc
```

### Step 2: Create .so Library

```toml
# ~/zos-server/Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### Step 3: Lift evolution_server.py

Use existing lifting pipeline:
```bash
cd /mnt/data1/meta-introspector
python3 scripts/build/lift_python.py scripts/build/evolution_server.py
```

This generates:
- Test cases
- Perf traces
- Rust code + proof

### Step 4: Integrate with ~/zos-server

```rust
// ~/zos-server/src/evolution.rs
use crate::automorphic_field::*;

pub async fn run_evolution_server() {
    // Use existing zos-server code
    // No duplication
}
```

### Step 5: Build .so

```bash
cd ~/zos-server
cargo build --release
# Produces: target/release/libzos_server.so
```

### Step 6: Load from Python (temporary)

```python
# scripts/build/evolution_server.py
import ctypes
zos = ctypes.CDLL("~/zos-server/target/release/libzos_server.so")
```

### Step 7: Full Rust Binary

Eventually replace Python entirely:
```bash
cargo build --release --bin zos-server
./target/release/zos-server
```

## No Duplication

✅ Use existing ~/zos-server code
✅ Lift Python via pipeline (not manual)
✅ Generate .so library
✅ Integrate, don't duplicate

## Next Action

Run lifting pipeline on evolution_server.py:
```bash
./scripts/crq002_phase1.sh  # Or similar for evolution server
```
