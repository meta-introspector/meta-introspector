#!/usr/bin/env bash
set -euo pipefail

echo "🎭 Complete Bootstrap Performance - From 357 bytes to ZOS"
echo "=========================================================="

OUT="zos-results/complete-bootstrap"
mkdir -p "$OUT"

# Act 1: MES (357 bytes → C compiler)
echo "🎬 Act 1: MES from seed"
perf record -g -o "$OUT/01-mes.perf.data" -- \
strace -f -o "$OUT/01-mes.strace" -- \
  guix build --no-substitutes mes 2>&1 | tee "$OUT/01-mes.log"

# Act 2: Nix (C compiler → package manager)
echo "🎬 Act 2: Nix from MES"
perf record -g -o "$OUT/02-nix.perf.data" -- \
strace -f -o "$OUT/02-nix.strace" -- \
  guix build --no-substitutes nix 2>&1 | tee "$OUT/02-nix.log"

# Act 3: LLVM (package manager → optimizer)
echo "🎬 Act 3: LLVM from Nix"
perf record -g -o "$OUT/03-llvm.perf.data" -- \
strace -f -o "$OUT/03-llvm.strace" -- \
  guix build --no-substitutes llvm 2>&1 | tee "$OUT/03-llvm.log"

# Act 4: Rust (optimizer → systems language)
echo "🎬 Act 4: Rust from LLVM"
perf record -g -o "$OUT/04-rust.perf.data" -- \
strace -f -o "$OUT/04-rust.strace" -- \
  guix build --no-substitutes rust 2>&1 | tee "$OUT/04-rust.log"

# Act 5: ZOS (systems language → self-analysis)
echo "🎬 Act 5: ZOS from Rust"
perf record -g -o "$OUT/05-zos.perf.data" -- \
strace -f -o "$OUT/05-zos.strace" -- \
  cargo build --release 2>&1 | tee "$OUT/05-zos.log"

echo ""
echo "🎧 Reception: Analyzing all traces"
for stage in 01-mes 02-nix 03-llvm 04-rust 05-zos; do
  echo "  📊 $stage..."
  perf report -i "$OUT/$stage.perf.data" --stdio --no-children \
    > "$OUT/$stage-reception.txt" 2>/dev/null || true
done

echo ""
echo "🔬 Introspection: ZOS analyzing the complete bootstrap"
if [ -f target/release/fold_traces ]; then
  ./target/release/fold_traces "$OUT" > "$OUT/complete-bootstrap.parquet" 2>&1 || true
fi

echo ""
echo "✅ Complete bootstrap recorded:"
echo "   357 bytes → MES → Nix → LLVM → Rust → ZOS"
echo "   All traces in: $OUT/"
