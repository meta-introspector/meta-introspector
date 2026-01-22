# The Gateway Pattern: Single Point of Impurity

## Philosophy

**Every impure operation has exactly ONE canonical implementation.**

For every:
- System call
- File I/O
- Network request
- Process spawn
- Git operation
- Build command

There is **one singular, highly guarded place** to do it.

## The Gateway

```
Pure Code (99%)
      ↓
   Gateway
      ↓
Impure Operation (1%)
```

## Current Gateways

### 1. Perf Recording
- **Gateway:** `nix/perf-lib.nix` (6 occurrences)
- **Shell:** `scripts/perf/record.sh`
- **Rust:** `src/perf/mod.rs`
- **Everyone else:** References gateway

### 2. Nix Build
- **Gateway:** `scripts/build/nix.sh`
- **Rust:** `src/build::nix_build()`
- **Nix:** `nix/build-lib.nix`
- **Everyone else:** References gateway

### 3. Cargo Build
- **Gateway:** `scripts/build/cargo.sh`
- **Rust:** `src/build::cargo_build()`
- **Everyone else:** References gateway

### 4. Git Operations
- **Gateway:** `scripts/git/{add,commit}.sh`
- **Rust:** `src/git::{add,commit}()`
- **Everyone else:** References gateway

## Pattern

```
Impure Operation: X
├── Gateway: scripts/X/canonical.sh (THE ONLY PLACE)
├── Rust: src/X/mod.rs (calls gateway)
├── Nix: nix/X-lib.nix (calls gateway)
└── Everyone else: References gateway
```

## Benefits

### 1. Auditing
- Want to know all file writes? Check one file.
- Want to know all network calls? Check one file.
- Want to know all builds? Check one file.

### 2. Instrumentation
- Add telemetry once, affects everything
- Add perf recording once, affects everything
- Add logging once, affects everything

### 3. Policy Enforcement
- Rate limiting: One place
- Access control: One place
- Validation: One place

### 4. Testing
- Mock the gateway, test everything
- One mock per operation type

### 5. Security
- Audit surface: Minimal
- Attack surface: Minimal
- Trust boundary: Clear

## Complete Gateway Map

### File System
```
Read:  scripts/fs/read.sh
Write: scripts/fs/write.sh
```

### Network
```
HTTP:  scripts/net/http.sh
Git:   scripts/git/fetch.sh
```

### Process
```
Spawn: scripts/proc/spawn.sh
Kill:  scripts/proc/kill.sh
```

### Build
```
Nix:   scripts/build/nix.sh
Cargo: scripts/build/cargo.sh
```

### Telemetry
```
Perf:  scripts/perf/record.sh
Log:   scripts/log/write.sh
```

## Implementation

Each gateway:
1. **Validates** inputs
2. **Logs** the operation
3. **Records** telemetry
4. **Executes** the impure operation
5. **Returns** result

```rust
// Gateway pattern
pub fn gateway<T>(
    operation: &str,
    validate: impl Fn() -> Result<()>,
    execute: impl Fn() -> Result<T>,
) -> Result<T> {
    // 1. Validate
    validate()?;
    
    // 2. Log
    log::info!("Gateway: {}", operation);
    
    // 3. Record telemetry
    telemetry::record_start(operation);
    
    // 4. Execute
    let result = execute()?;
    
    // 5. Return
    telemetry::record_end(operation);
    Ok(result)
}
```

## Purity Levels

```
Level 0: Pure functions (no gateway needed)
Level 1: Reads (read-only gateway)
Level 2: Writes (write gateway with validation)
Level 3: Network (network gateway with rate limiting)
Level 4: Process (process gateway with sandboxing)
```

## Nix Store Integration

All gateways output to `/nix/store`:

```
Gateway Operation
      ↓
  /nix/store/xxx-operation-result/
      ↓
  Immutable, Reproducible
```

## Verification

```bash
# Find all impure operations
grep -r "std::fs::write" src/
# Should only find: src/fs/mod.rs (the gateway)

grep -r "Command::new" src/
# Should only find: src/proc/mod.rs (the gateway)

grep -r "perf record" .
# Should only find: nix/perf-lib.nix (the gateway)
```

## Current Status

✅ Perf recording: 368 → 10 (97% reduction)
✅ Build commands: 912 identified, gateways created
🚧 File I/O: TODO
🚧 Network: TODO
🚧 Process spawn: TODO

## Goal

**Every impure operation in the codebase goes through exactly ONE gateway.**

```
Total impure operations: ~5000
Gateways needed: ~20
Reduction: 99.6%
```

## The Monad

This is essentially the IO monad:

```haskell
-- Pure code
pure :: a -> IO a

-- Impure operation (gateway)
gateway :: IO a -> a

-- Composition
program = do
  x <- gateway readFile
  y <- pure (process x)
  gateway (writeFile y)
```

## See Also

- `docs/perf/README.md` - Perf gateway
- `docs/build/CANONICAL_COMMANDS.md` - Build gateways
- `nix/perf-lib.nix` - Canonical perf implementation
- `scripts/build/` - Canonical build implementations

---

**One gateway per impurity. Everything else is pure.**
