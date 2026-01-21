# The Complete Labeling Hierarchy

## Level 0: Mes Bootstrap (Baseline)
**File**: `mes-bootstrap-proof/flake.nix`
**Records**: Building Mes from 357-byte seed
**Labels**: The foundation - proves everything from minimal seed

## Level 1: Toolchain Bootstrap (NEW - NEEDED)
**File**: `toolchain-bootstrap/flake.nix` 
**Records**: Building language toolchains using Mes/GCC
- rustc built from source
- ghc built from source  
- python built from source
**Labels**: How each toolchain is constructed

## Level 2: Language Execution (DONE)
**File**: `perf_actual/flake.nix`
**Records**: Language tools compiling user code
- rustc compiling hello.rs
- ghc compiling hello.hs
**Labels**: How each language processes code

## The Hierarchy

```
Mes (357 bytes)
  ↓ builds
GCC/Binutils
  ↓ builds
Rustc/GHC/Python (Level 1 - NEED THIS)
  ↓ compiles
User Code (Level 2 - HAVE THIS)
```

## What We Need

Create `toolchain-bootstrap/flake.nix` that records:
- Building rustc from source (using existing GCC)
- Building ghc from source
- Building python from source
- etc.

All stored in `/nix/store` with perf data.
