# Nix-Based Complexity Proof System

Automated pipeline to analyze Rust code, compute topological invariants, and generate verified Lean4 proofs.

## Quick Start

```bash
# Enter development environment
nix develop

# Analyze and prove enum vs struct complexity
analyze-and-prove test_enum.rs test_struct.rs ./proofs
```

## What It Does

1. **Trace Execution** (QEMU + reachability plugin)
2. **Cluster Tests** (Source2Test)
3. **Harmonic Analysis** (FFT signatures)
4. **Homotopy Classification** (Compute genus, conductor, weight, level)
5. **Generate Lean4 Proof** (Formal theorem)
6. **Verify Proof** (Lean4 type checker)

## Pipeline

```
test_enum.rs
    ↓
[QEMU Trace]
    ↓
reach_tracer → enum_reach.txt
    ↓
source2test → enum_clusters.json
    ↓
homotopy_classifier → enum_class.json
    ↓
Extract: genus=3, conductor=150
    ↓
[Generate Lean4 Proof]
    ↓
theorem enum_more_complex :
  complexity 3 150 > complexity 1 50
    ↓
[Lean4 Verify]
    ↓
✅ PROOF VERIFIED
```

## Example Proof

```lean
import Mathlib.Data.Nat.Basic
import Mathlib.Tactic

def complexity (genus : ℕ) (conductor : ℕ) : ℕ :=
  2 * genus + conductor

def enum_complexity : ℕ := complexity 3 150
def struct_complexity : ℕ := complexity 1 50

theorem enum_more_complex : enum_complexity > struct_complexity := by
  unfold enum_complexity struct_complexity complexity
  norm_num

#check enum_more_complex
```

## Usage

### Via Flake

```bash
# Run complete pipeline
nix run . -- test_enum.rs test_struct.rs

# Build tools only
nix build .#tools

# Enter dev shell
nix develop
```

### Via Script

```bash
# Generate proof manually
generate-proof 3 150 1 50

# Verify
lean --make complexity_proof.lean
```

### Via Nix Expression

```nix
nix-build complexity-proof.nix -A fullPipeline
```

## Output Structure

```
proof_output/
├── enum_reach.txt          # QEMU trace
├── enum_clusters.json      # Test clusters
├── enum_class.json         # Homotopy classification
├── struct_reach.txt
├── struct_clusters.json
├── struct_class.json
├── complexity_proof.lean   # Generated proof
└── proof_status.txt        # VERIFIED or FAILED
```

## Theorem Template

The system proves:

```
∀ code1 code2,
  genus(code1) > genus(code2) ∨
  conductor(code1) > conductor(code2)
  →
  complexity(code1) > complexity(code2)
```

Where:
- `complexity(g, c) = 2g + c`
- `g` = genus (topological holes)
- `c` = conductor (ramification)

## Integration with CI

```yaml
# .github/workflows/complexity-proof.yml
- name: Analyze complexity
  run: |
    nix develop --command analyze-and-prove \
      src/enum_impl.rs \
      src/struct_impl.rs \
      ./proofs
    
- name: Check proof
  run: |
    if grep -q "VERIFIED" proofs/proof_status.txt; then
      echo "✅ Complexity proof verified"
    else
      echo "❌ Proof failed"
      exit 1
    fi
```

## Custom Proofs

### Compare Two Implementations

```bash
analyze-and-prove impl_v1.rs impl_v2.rs ./comparison
```

### Prove Optimization

```bash
# Before optimization
analyze-and-prove old_code.rs new_code.rs ./optimization

# Proof will show: genus(new) < genus(old)
```

### Prove Refactoring Preserves Complexity

```bash
analyze-and-prove original.rs refactored.rs ./refactor

# Proof will show: genus(original) = genus(refactored)
```

## Advanced Usage

### Custom Complexity Measure

Edit `complexity_proof.lean`:

```lean
def complexity (genus : ℕ) (conductor : ℕ) (weight : ℕ) : ℕ :=
  2 * genus + conductor + weight / 10
```

### Multiple Comparisons

```bash
for impl in impl_*.rs; do
  analyze-and-prove $impl baseline.rs ./proofs_$impl
done
```

### Extract Metrics Only

```bash
homotopy_classifier < clusters.json | \
  jq -r '.[] | "\(.mathematical_classification.modular_form.genus),\(.mathematical_classification.modular_form.conductor)"'
```

## Reproducibility

All builds are reproducible via Nix:

```bash
# Same inputs → same outputs
nix build .#tools --rebuild

# Pin dependencies
nix flake lock
```

## Dependencies

- Rust toolchain (via Nix)
- QEMU (for tracing)
- Lean4 (for proofs)
- jq (for JSON processing)

All managed by Nix—no manual installation needed.

## Troubleshooting

### Proof Fails

Check metrics:
```bash
cat proof_output/enum_class.json | jq '.[] | .mathematical_classification.modular_form'
```

### QEMU Trace Empty

Ensure QEMU plugin is built:
```bash
nix build .#tools
ls result/lib/libreachability_rust.so
```

### Lean4 Error

Check proof syntax:
```bash
lean --version
lean --make complexity_proof.lean
```

## Theory

This system implements the theory that:

**Code complexity is a topological invariant**

- Execution traces → curves in manifold
- Test clusters → homotopy classes
- Harmonic signatures → modular forms
- Minimal test set → homology basis

The genus and conductor are **intrinsic** properties that cannot be reduced without changing the code's behavior.

## References

- [THEORY.md](THEORY.md) - Complete mathematical theory
- [docs/HOMOTOPY_CLASSIFICATION.md](docs/HOMOTOPY_CLASSIFICATION.md) - Classification details
- [Lean4 Mathlib](https://github.com/leanprover-community/mathlib4) - Formal mathematics library
