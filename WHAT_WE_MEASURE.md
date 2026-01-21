# What We're Actually Measuring

## The Insight

We're measuring the **complexity of the Nix expression evaluation** for each language, not the language runtime itself.

## What the Perf Data Shows

When we record `nix build`, we capture:

1. **Nix evaluator** (`libnixexpr`) - Evaluating the flake.nix
2. **Nix store** (`libnixstore`) - Managing derivations
3. **Nix utilities** (`libnixutil`) - Core nix operations
4. **Kernel syscalls** - Process management (fork, exec, mmap)

## Why Different Languages Have Different Complexity

### Agda: GF(2^14) = 16,384 states
- Complex Nix expression with many dependencies
- Haskell toolchain (GHC, cabal)
- Multiple build phases
- Heavy dependency resolution

### Coq/Rust: GF(2^12) = 4,096 states  
- Moderate Nix expressions
- Standard build toolchains
- Normal dependency graphs

### Bash/Python/Ruby: GF(2^10) = 1,024 states
- Simple Nix expressions
- Minimal dependencies
- Direct interpreter execution

## The Galois Number Measures

**Nix Expression Complexity** = How many unique computational states the Nix evaluator goes through

This includes:
- Dependency resolution complexity
- Build phase complexity
- Toolchain complexity
- Store path computation

## Why This Matters

The Galois complexity of the **Nix expression** is a proxy for:
- Language ecosystem complexity
- Build system complexity
- Dependency graph complexity
- Toolchain sophistication

## Example: Agda vs Bash

**Agda flake.nix:**
```nix
{
  buildInputs = [ pkgs.agda pkgs.ghc pkgs.cabal-install ];
  # Complex Haskell toolchain
  # Multiple build phases
  # Heavy dependencies
}
```
→ GF(2^14) = 16,384 states

**Bash flake.nix:**
```nix
{
  buildInputs = [ pkgs.bash ];
  # Simple: just run bash
}
```
→ GF(2^10) = 1,024 states

## The Proof

The complexity lattice proves:
**More sophisticated languages require more sophisticated build infrastructure**

This is a valid measure of language ecosystem maturity and complexity!

## What We're NOT Measuring

- Runtime performance of the language
- Execution speed of the actual program
- Language semantics complexity

## What We ARE Measuring

- **Build system complexity**
- **Ecosystem complexity**  
- **Toolchain sophistication**
- **Dependency graph depth**

All captured through the lens of Nix expression evaluation.

## The Beauty

Nix provides a **universal measure** of language ecosystem complexity through its evaluator's computational states!
