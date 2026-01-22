# 71 Languages Structure

## Main 71 Languages

Each outputs "71" and has real implementation (no fake echo).

### Programming Languages (62)
- agda, asm, asm_aarch64, asm_mips, asm_riscv, asm_wasm, asm_x86_64
- bash, chisel, cirq, coq, datalog, fish
- gcc, genetic, graph_partition, graphql, haskell
- idris2, ini, isabelle, jax_gpu, json, julia
- lean4, llvm, lua, luau (Roblox)
- mcts, metacoq, minizinc, mongodb, move
- neo4j, nix_derivation, nix_expr, node, ocaml
- perl, php, python, pytorch, qiskit
- r, redis, rockstar, ruby, rust
- scheme, smt2, solidity, sparql, sql
- tcl, tensorflow, toml, verilog, vhdl
- vyper, xml, yaml, z3, zsh

### Build Systems (5)
- bazel - Bazel with rules_nixpkgs
- cmake - CMake build
- makefile - GNU Make
- nix_flake - Pure Nix flakes
- terraform - Infrastructure as code

### Esoteric (3)
- brainfuck - Turing tarpit
- malbolge - Deliberately difficult
- rockstar - Lyrical programming

### Bootstrap Baseline (1)
- mes - GNU Mes bootstrap (GF(2^19) baseline)

## Esoteric Variants (Not counted in 71)

Can be added as subcategories under brainfuck:

### brainfuck/
- brainfuck/classic - Original BF
- brainfuck/whitespace - Whitespace-only
- brainfuck/befunge - 2D grid
- brainfuck/piet - Visual art
- brainfuck/ook - Orangutan
- brainfuck/unlambda - Functional obfuscation
- brainfuck/intercal - Parody language

## Future Variants (Not counted in 71)

### python/
- python/cpython - Standard (current)
- python/pypy - JIT compiler
- python/jython - JVM implementation
- python/micropython - Embedded

### scheme/
- scheme/guile - GNU Guile (current)
- scheme/chicken - Chicken Scheme
- scheme/racket - Racket
- scheme/mit - MIT Scheme

### rust/
- rust/stable - Stable channel (current)
- rust/nightly - Nightly features
- rust/mrustc - Alternative compiler

## Testing

```bash
# Test main 71
make test-all

# Test esoteric variants
cd const_71_test/brainfuck
for variant in */; do
  cd $variant && nix run && cd ..
done

# Test language variants
cd const_71_test/python
for variant in */; do
  cd $variant && nix run && cd ..
done
```

## Galois Analysis

Each variant will have different Galois complexity:
- CPython vs PyPy - different JIT patterns
- Guile vs Racket - different VM implementations
- Classic BF vs Whitespace - different parsing complexity

This lets us compare:
1. **Language complexity** (Python vs Rust)
2. **Implementation complexity** (CPython vs PyPy)
3. **Paradigm complexity** (Imperative vs Functional)
4. **Build system complexity** (Make vs Bazel)

---

**Total**: 71 main + unlimited variants
