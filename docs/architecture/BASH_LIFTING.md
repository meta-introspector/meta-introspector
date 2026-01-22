# Bash to Rust Lifting via Perf + Shellcheck + Lean4

## Goal

Lift all bash scripts into Rust via the driver binary.

## The Pipeline

```
Bash Script
    ↓
[1. Perf Record] → Execution trace
    ↓
[2. Shellcheck] → AST + semantics
    ↓
[3. Lean4 Proof] → Verified translation
    ↓
Rust Code (in driver binary)
```

## Step 1: Perf Recording

Record execution trace of bash script:

```bash
perf record -o script.perf.data bash script.sh
```

**Captures:**
- Syscalls made
- Files accessed
- Commands executed
- Execution order

## Step 2: Shellcheck Analysis

Parse bash script to AST:

```bash
shellcheck -f json script.sh > script.ast.json
```

**Extracts:**
- Command structure
- Variable usage
- Control flow
- Dependencies

## Step 3: Lean4 Proof

Prove the Rust translation is equivalent:

```lean
theorem bash_to_rust_equiv (bash_script : BashAST) (rust_code : RustAST) :
  ∀ input, eval_bash bash_script input = eval_rust rust_code input
```

**Verifies:**
- Same syscalls
- Same order
- Same results
- Provably equivalent

## Step 4: Generate Rust

```rust
// Generated from script.sh
pub fn script_sh(args: &[&str]) -> Result<(), String> {
    // Lifted from bash, proven equivalent
    gateway::gateway().fs().read("input.txt")?;
    gateway::gateway().build().cargo_build(&["--release"])?;
    gateway::gateway().git().commit("message")?;
    Ok(())
}
```

## Integration with Driver

```rust
// In driver binary
match command {
    "script.sh" => lifted::script_sh(&args),
    "other.sh" => lifted::other_sh(&args),
    // All bash scripts become Rust functions
}
```

## Benefits

### 1. Type Safety
Bash: `$var` (string, maybe)
Rust: `var: String` (guaranteed)

### 2. Performance
Bash: Spawn processes
Rust: Direct syscalls

### 3. Verification
Bash: Hope it works
Rust: Proven equivalent via Lean4

### 4. Single Binary
Before: 100+ bash scripts
After: 1 driver binary with 100 functions

## The Lifting Process

### Input: Bash Script
```bash
#!/bin/bash
nix build .#default
cargo build --release
git add .
git commit -m "update"
```

### Step 1: Perf Trace
```
syscall: execve("nix", ["build", ".#default"])
syscall: execve("cargo", ["build", "--release"])
syscall: execve("git", ["add", "."])
syscall: execve("git", ["commit", "-m", "update"])
```

### Step 2: Shellcheck AST
```json
{
  "commands": [
    {"cmd": "nix", "args": ["build", ".#default"]},
    {"cmd": "cargo", "args": ["build", "--release"]},
    {"cmd": "git", "args": ["add", "."]},
    {"cmd": "git", "args": ["commit", "-m", "update"]}
  ]
}
```

### Step 3: Lean4 Proof
```lean
def bash_script : BashAST := ...
def rust_code : RustAST := ...

theorem equiv : bash_script ≈ rust_code := by
  -- Prove each command maps correctly
  -- Prove order is preserved
  -- Prove results are identical
```

### Step 4: Generated Rust
```rust
pub fn script(args: &[&str]) -> Result<(), String> {
    gateway::gateway().build().nix_build(".#default")?;
    gateway::gateway().build().cargo_build(&["--release"])?;
    gateway::gateway().git().add(&["."])?;
    gateway::gateway().git().commit("update")?;
    Ok(())
}
```

## Nix Integration

```nix
{
  inputs.driver.url = "github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=driver";
  
  outputs = { driver, ... }: {
    # Use driver instead of raw commands
    buildPhase = ''
      ${driver.packages.x86_64-linux.default}/bin/driver nix build .#default
      ${driver.packages.x86_64-linux.default}/bin/driver cargo build --release
    '';
  };
}
```

## Legacy Bash Compatibility

```bash
#!/bin/bash
# Legacy script - automatically uses driver

# Source driver aliases
source scripts/driver.sh

# Now all commands go through driver
nix build .#default      # → driver nix build .#default
cargo build --release    # → driver cargo build --release
git commit -m "update"   # → driver git commit -m "update"
```

## The Complete Stack

```
Bash Script (legacy)
    ↓
Driver Binary (Rust)
    ↓
Gateway Traits
    ↓
ZK Proof Generator
    ↓
Syscall (any kernel)
    ↓
Proof (stored in /nix/store)
```

## Verification

Every bash script execution produces:
1. **Perf trace** - What actually happened
2. **Shellcheck AST** - What was intended
3. **Lean4 proof** - They match
4. **ZK proof** - Syscalls were correct

## Current Status

✅ Gateway traits defined
✅ Driver binary structure
✅ Bash wrapper created
✅ Nix integration planned
🚧 Perf + Shellcheck + Lean4 pipeline
🚧 Bash lifting automation
🚧 Proof generation

## Next Steps

1. Build driver binary
2. Lift one bash script as proof of concept
3. Automate lifting pipeline
4. Replace all bash scripts with driver calls
5. Verify with Lean4

## Goal

**One binary. All tools. All proven. All through gateways.**

```
Before: 100+ binaries, 100+ scripts, 5000+ syscalls
After:  1 binary, 100 functions, 20 gateways
```

## See Also

- `src/bin/driver.rs` - Unified driver binary
- `src/gateway/mod.rs` - Gateway trait system
- `docs/architecture/KERNEL_ABSTRACTION.md` - ZK proof abstraction
- `docs/architecture/GATEWAY_PATTERN.md` - Gateway pattern

---

**One binary to rule them all. Every syscall proven. The kernel abstracted.**
