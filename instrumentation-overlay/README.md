# Instrumentation Overlay


## The Cryptographic View

Since we know:
1. **The sender**: The compiler (GCC/LLVM)
2. **The message**: The source code
3. **The process**: The compilation

We can **decode the entire transformation** as a cryptographic message:

```
Source → [Compiler as Cipher] → Binary

With full instrumentation, we capture:
- Every intermediate representation (plaintext at each round)
- Every optimization pass (the key schedule)
- Every memory access (the state transitions)
```

## Usage

### Instrument any package

```nix
{
  inputs.instrumentation.url = "path:./instrumentation-overlay";
  
  outputs = { nixpkgs, instrumentation }:
    let
      pkgs = import nixpkgs {
        overlays = [ instrumentation.overlays.default ];
      };
    in {
      packages.x86_64-linux = {
        # Fully instrumented build
        my-app-traced = pkgs.withFullInstrumentation pkgs.my-app;
      };
    };
}
```

### Instrument llama.cpp

```nix
{
  outputs = { instrumentation }:
    let
      llama-traced = instrumentation.lib.instrumentLlamaCpp {
        pkgs = nixpkgs;
        src = /mnt/data1/2023/11/09/llama.cpp-clean;
      };
    in {
      packages.x86_64-linux.default = llama-traced;
    };
}
```

## Output Structure

```
/nix/store/abc-package-instrumented/
├── bin/                    # Original binaries
├── lib/
└── traces/
    ├── build.perf.data     # Perf recording
    ├── compiler-dumps.tar.gz
    │   ├── *.dump          # GCC tree/RTL/IPA dumps
    │   ├── *.i             # Preprocessed source
    │   ├── *.s             # Assembly
    │   └── *.bc            # LLVM bitcode
    ├── time-traces.tar.gz  # LLVM time traces
    ├── qemu-trace.log      # Full memory trace
    └── metadata.json       # Build metadata
```

## The Cryptographic Decoding

### 1. Source as Plaintext
```c
int main() { return 42; }
```

### 2. Preprocessing (Round 1)
```
Captured in: *.i files
Key: Macro definitions, includes
```

### 3. Parsing (Round 2)
```
Captured in: *.dump (tree-original)
Key: Grammar rules
```

### 4. Optimization (Rounds 3-N)
```
Captured in: *.dump (tree-optimized, rtl-*, ipa-*)
Key: Optimization flags, heuristics
Each pass is a cryptographic round
```

### 5. Code Generation (Final Round)
```
Captured in: *.s (assembly)
Key: Target architecture
```

### 6. Memory Execution (Runtime)
```
Captured in: qemu-trace.log
Every instruction, every memory access
The ciphertext executing
```

## Analysis Tools

### Extract all intermediate representations
```bash
nix build .#my-app-traced
tar xzf result/traces/compiler-dumps.tar.gz
```

### Analyze compilation time
```bash
# LLVM time traces
tar xzf result/traces/time-traces.tar.gz
# View in chrome://tracing
```

### Analyze runtime behavior
```bash
# Perf data
perf report -i result/traces/build.perf.data

# QEMU trace
grep "IN:" result/traces/qemu-trace.log | head -100
```

### Extract orbits
```bash
nix build .#extract-orbits result
# Finds instruction pointer loops
```

## Integration with ZOS

This overlay provides the **complete cryptographic trace** that ZOS analyzes:

1. **Compilation orbits**: From compiler dumps
2. **Optimization rounds**: From pass dumps
3. **Execution orbits**: From QEMU traces
4. **Resonances**: At ZOS primes

## The Transparency Principle

**Everything is visible:**
- No hidden transformations
- No opaque optimizations
- No mysterious runtime behavior

**Because we control:**
- The compiler (instrumented GCC/LLVM)
- The execution (QEMU tracing)
- The analysis (ZOS tools)

**We can decode:**
- Every cryptographic round (compilation pass)
- Every state transition (instruction)
- Every orbit (loop)

This is **complete transparency** - the compiler as an open cryptographic system.
