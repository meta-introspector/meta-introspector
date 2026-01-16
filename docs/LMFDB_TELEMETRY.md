# LMFDB Telemetry System

Overpowered build.rs that generates wrappers for 37,756 symbols from 92 .so files, filtered by LMFDB mathematical complexity.

## Overview

- **Symbol Extraction**: Uses goblin to parse 92 .so files from real Nix Rust build
- **LMFDB Ranking**: Assigns conductor values (3000-10000+) based on mathematical complexity
- **Harmonic Filtering**: Select symbols by semantic categories (strings, memory, crypto, etc.)
- **Safe Telemetry**: Custom print macros using raw syscalls to avoid recursion
- **Feature Gating**: Enable wrappers incrementally for testing

## Quick Start

```bash
# Default: Top 10% most complex symbols
cargo build

# All symbols (100%)
LMFDB_FILTER_PERCENT=100 cargo build

# String functions only
LMFDB_HARMONIC_FILTER=strings LMFDB_FILTER_PERCENT=100 cargo build

# Memory functions only, top 50%
LMFDB_HARMONIC_FILTER=memory LMFDB_FILTER_PERCENT=50 cargo build
```

## LMFDB Conductor Calculation

Symbols are ranked by conductor (mathematical complexity):

```
Base: 3000
+ Length × 10
+ Namespace depth (::) × 100
+ Templates (<>) × 200
+ Pointers/refs (*&) × 50
+ Crypto keywords: +2000
+ Memory keywords: +1000
+ I/O keywords: +800
```

**Examples**:
- `getpid` → 3070 (simple)
- `std::string::String::new` → 3500 (moderate)
- `SHA256_Transform<T>` → 7200 (complex)

## Harmonic Filters

Select symbols by semantic category:

| Filter | Description | Example Symbols |
|--------|-------------|-----------------|
| `constants` | AST complexity < 2, simple constants | `*_it`, `CONST_*`, short names |
| `strings` | String manipulation | `str*`, `String*`, `*string*` |
| `memory` | Memory management | `malloc`, `free`, `alloc`, `realloc` |
| `io` | I/O operations | `read`, `write`, `open`, `close` |
| `crypto` | Cryptographic functions | `SHA*`, `AES*`, `hash*`, `crypt*` |
| `simple` | Low complexity (conductor < 4000) | Basic functions |
| `complex` | High complexity (conductor > 6000) | Template-heavy, nested |

## Environment Variables

```bash
# Harmonic filter (semantic category)
export LMFDB_HARMONIC_FILTER=strings  # or memory, io, crypto, constants, simple, complex

# Percentage filter (top N% by conductor)
export LMFDB_FILTER_PERCENT=100  # 1-100, default 10

# Example: Top 25% of crypto functions
LMFDB_HARMONIC_FILTER=crypto LMFDB_FILTER_PERCENT=25 cargo build
```

## Safe Print Library

Custom print macros using raw `SYS_write` syscall to avoid recursion:

```rust
use safe_print::{safe_eprintln};

// Safe - won't recurse even if called from wrapped functions
safe_eprintln(&format!("Count: {}", n));
```

**Why needed**: Standard `eprintln!` internally calls `getpid`, causing infinite recursion when wrapping libc functions.

## Feature-Gated Wrappers (nix-telemetry)

Test wrappers incrementally:

```bash
cd /mnt/data1/nix-telemetry

# No wrappers (baseline)
cargo build --release --no-default-features

# Single wrapper
cargo build --release --no-default-features --features wrap_getpid

# All safe wrappers
cargo build --release --no-default-features --features all_safe

# Test
LD_PRELOAD=./target/release/libnix_telemetry.so ls
```

**Features**:
- `wrap_getpid`, `wrap_getuid` - Process info (safest)
- `wrap_open`, `wrap_close` - File operations
- `wrap_read`, `wrap_write` - I/O operations
- `wrap_malloc`, `wrap_free` - Memory (dangerous - causes recursion)
- `all_safe` - All except malloc/free

## Real Build Data

From `data/build_analysis/real_build_1768332029_*.json`:

- **32 binaries** executed during Nix Rust build
- **92 unique .so libraries** loaded
- **37,756 symbols** extracted via goblin
- **3,775 symbols** after 10% filter (default)

## Results

### Symbol Distribution by Conductor

```
Tier 1 (10000+): Crypto, complex templates    ~500 symbols
Tier 2 (8000+):  Memory management, I/O       ~1200 symbols
Tier 3 (6000+):  String operations            ~2500 symbols
Tier 4 (4000+):  Simple functions             ~8000 symbols
Tier 5 (3000+):  Constants, basic ops         ~25000 symbols
```

### Harmonic Filter Results

```
strings:   ~4200 symbols (11%)
memory:    ~1800 symbols (5%)
io:        ~2100 symbols (6%)
crypto:    ~650 symbols (2%)
constants: ~8500 symbols (23%)
simple:    ~15000 symbols (40%)
complex:   ~5000 symbols (13%)
```

## Nix Rust Bootstrap Capture

```bash
# Build telemetry library
cd /mnt/data1/nix-telemetry
cargo build --release --no-default-features --features all_safe

# Run Nix build with telemetry
cd /mnt/data1/meta-introspector/rust-overlay-test
LD_PRELOAD=/mnt/data1/nix-telemetry/target/release/libnix_telemetry.so \
  nix build .#rustNightlyProfiling --print-build-logs
```

## Architecture

```
build.rs (meta-introspector)
├── Read real_build_*_libraries.json (92 .so files)
├── Extract symbols with goblin (37,756 total)
├── Calculate LMFDB conductor for each
├── Apply harmonic filter (semantic)
├── Apply percentage filter (top N%)
└── Generate symbol_wrappers.rs macros

nix-telemetry (LD_PRELOAD library)
├── safe_print.rs - Raw syscall printing
├── lib.rs - Feature-gated wrappers
└── Cargo.toml - Feature flags
```

## Key Discoveries

1. **Recursion Issue**: `eprintln!` calls `getpid` internally → infinite loop
2. **Solution**: Raw `SYS_write` syscall with recursion guard
3. **130,500 calls**: Captured before segfault, proving the recursion
4. **LMFDB Filtering**: Reduces noise by 90%, focuses on complex operations

## Future Work

- [ ] Stream telemetry to Parquet files
- [ ] Add more harmonic filters (networking, threading, etc.)
- [ ] Correlate symbols with LMFDB elliptic curves
- [ ] Generate wrappers for all 37k symbols with safe print
- [ ] Capture full Nix Rust bootstrap build

## References

- LMFDB: L-functions and Modular Forms Database
- Goblin: Pure Rust ELF parser
- Real build data: `data/build_analysis/real_build_1768332029_*.json`
