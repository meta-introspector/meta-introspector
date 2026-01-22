#!/bin/bash
# Organize root markdown files into docs/ subdirectories

cd /mnt/data1/meta-introspector

# Create directories
mkdir -p docs/{theory,71,build,policy,monster,transformer,security,audit}

# 71 and harmonic theory
mv 71_*.md docs/71/ 2>/dev/null
mv *HARMONIC*.md docs/71/ 2>/dev/null
mv PRIME_HARMONICS.md docs/71/ 2>/dev/null
mv OGG_PRIME_19_HARMONIC_BREAK.md docs/71/ 2>/dev/null
mv GALOIS_*.md docs/71/ 2>/dev/null

# Nix and build
mv NIX_*.md docs/nix/ 2>/dev/null
mv BUILD_*.md docs/build/ 2>/dev/null
mv BOOTSTRAP*.md docs/build/ 2>/dev/null
mv FORCE_REBUILD_GUIDE.md docs/build/ 2>/dev/null
mv MEMO_NIX_STORE.md docs/nix/ 2>/dev/null
mv CONSOLIDATION_NIX_STORE_PERF.md docs/nix/perf/ 2>/dev/null

# Monster and mathematical theory
mv MONSTER_*.md docs/theory/ 2>/dev/null
mv CONFORMAL_*.md docs/theory/ 2>/dev/null
mv NINE_MUSES_*.md docs/theory/ 2>/dev/null

# Transformer and ML
mv *TRANSFORMER*.md docs/transformer/ 2>/dev/null
mv *TRAINING*.md docs/transformer/ 2>/dev/null

# Policy
mv *POLICY*.md docs/policy/ 2>/dev/null
mv ANTI_*.md docs/policy/ 2>/dev/null

# Audit and reports
mv *AUDIT*.md docs/audit/ 2>/dev/null
mv *REPORT*.md docs/audit/ 2>/dev/null
mv CLIPPY_REPORT.md docs/audit/ 2>/dev/null

# Security
mv *SECURITY*.md docs/security/ 2>/dev/null

# Theory (mathematical)
mv *HOMOTOPY*.md docs/theory/ 2>/dev/null
mv *PERIODICITY*.md docs/theory/ 2>/dev/null
mv *ORBIT*.md docs/theory/ 2>/dev/null
mv automorphism_group.md docs/theory/ 2>/dev/null
mv coherence_orbit.md docs/theory/ 2>/dev/null

echo "✅ Organized markdown files into docs/ subdirectories"
echo "Remaining in root:"
ls *.md 2>/dev/null | wc -l
