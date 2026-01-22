#!/bin/bash
# Move markdown files to categorized directories

cd /mnt/data1/meta-introspector

mkdir -p docs/{71,theory,build,policy,audit,security,blockchain,transformer,monster,misc}

echo "📦 Moving files to categorized directories..."

# Move by pattern matching
mv 71_*.md docs/71/ 2>/dev/null
mv *HARMONIC*.md docs/71/ 2>/dev/null
mv PRIME_HARMONICS.md docs/71/ 2>/dev/null
mv OGG_PRIME_19_HARMONIC_BREAK.md docs/71/ 2>/dev/null
mv GALOIS_*.md docs/71/ 2>/dev/null
mv COMPOSITE_71_PROOFS.md docs/71/ 2>/dev/null
mv GODEL_SENTINEL_71.md docs/71/ 2>/dev/null
mv GPU_71_PLAN.md docs/71/ 2>/dev/null
mv UNIVERSAL_71_SEMANTICS.md docs/71/ 2>/dev/null
mv CANONICAL_PATH.md docs/71/ 2>/dev/null

# Nix and build
mv NIX_*.md docs/build/ 2>/dev/null
mv BUILD_*.md docs/build/ 2>/dev/null
mv *BOOTSTRAP*.md docs/build/ 2>/dev/null
mv FORCE_REBUILD_GUIDE.md docs/build/ 2>/dev/null
mv DEPLOYMENT_READY.md docs/build/ 2>/dev/null
mv ERROR_PATTERN_ANALYSIS.md docs/build/ 2>/dev/null
mv FAILED_PROJECTS_LIST.md docs/build/ 2>/dev/null
mv FILE_INDEX.md docs/build/ 2>/dev/null
mv COMPLETE_SYSTEM_SUMMARY.md docs/build/ 2>/dev/null
mv CHANGES.md docs/build/ 2>/dev/null
mv analyze_path_errors.md docs/build/ 2>/dev/null
mv BINARY_IO_DOCS.md docs/build/ 2>/dev/null

# Monster and theory
mv MONSTER_*.md docs/theory/ 2>/dev/null
mv CONFORMAL_*.md docs/theory/ 2>/dev/null
mv NINE_MUSES_*.md docs/theory/ 2>/dev/null
mv *HOMOTOPY*.md docs/theory/ 2>/dev/null
mv *PERIODICITY*.md docs/theory/ 2>/dev/null
mv *ORBIT*.md docs/theory/ 2>/dev/null
mv automorphism_group.md docs/theory/ 2>/dev/null
mv coherence_orbit.md docs/theory/ 2>/dev/null
mv CURSED_EVIL_RUNESTONES.md docs/theory/ 2>/dev/null
mv COMPRESSION_CONFORMAL_FIELD.md docs/theory/ 2>/dev/null

# Transformer/ML
mv *TRANSFORMER*.md docs/transformer/ 2>/dev/null
mv *TRAINING*.md docs/transformer/ 2>/dev/null
mv GGUF_71_MODEL.md docs/transformer/ 2>/dev/null

# Blockchain
mv BLOCKCHAIN_71_PLAN.md docs/blockchain/ 2>/dev/null

# Policy
mv *POLICY*.md docs/policy/ 2>/dev/null
mv ANTI_*.md docs/policy/ 2>/dev/null

# Audit
mv *AUDIT*.md docs/audit/ 2>/dev/null
mv *REPORT*.md docs/audit/ 2>/dev/null
mv CLIPPY_REPORT.md docs/audit/ 2>/dev/null

# Security
mv *SECURITY*.md docs/security/ 2>/dev/null

# Everything else to misc
mv *.md docs/misc/ 2>/dev/null

# Keep README.md in root
mv docs/misc/README.md . 2>/dev/null

echo ""
echo "✅ Files organized!"
echo ""
echo "Summary:"
echo "  docs/71/: $(ls docs/71/*.md 2>/dev/null | wc -l) files"
echo "  docs/theory/: $(ls docs/theory/*.md 2>/dev/null | wc -l) files"
echo "  docs/build/: $(ls docs/build/*.md 2>/dev/null | wc -l) files"
echo "  docs/transformer/: $(ls docs/transformer/*.md 2>/dev/null | wc -l) files"
echo "  docs/blockchain/: $(ls docs/blockchain/*.md 2>/dev/null | wc -l) files"
echo "  docs/policy/: $(ls docs/policy/*.md 2>/dev/null | wc -l) files"
echo "  docs/audit/: $(ls docs/audit/*.md 2>/dev/null | wc -l) files"
echo "  docs/security/: $(ls docs/security/*.md 2>/dev/null | wc -l) files"
echo "  docs/misc/: $(ls docs/misc/*.md 2>/dev/null | wc -l) files"
echo ""
echo "Remaining in root: $(ls *.md 2>/dev/null | wc -l) files"
