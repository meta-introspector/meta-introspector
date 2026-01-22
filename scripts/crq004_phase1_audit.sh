#!/usr/bin/env bash
# CRQ-004 Phase 1: Audit llama.cpp existing work

set -euo pipefail

LLAMA_DIR="/mnt/data1/2023/11/09/llama.cpp"
OUTPUT_DIR="data/crq004"

mkdir -p "$OUTPUT_DIR"

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║         CRQ-004: llama.cpp-clean Audit                               ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo

cd "$LLAMA_DIR"

# 1. Branches
echo "📋 Branches:"
git branch -a | tee "$OUTPUT_DIR/branches.txt"
echo

# 2. Dirty files
echo "🗑️  Dirty files (to clean):"
find . -name "*.py~" -o -name "#*#" -o -name ".#*" | tee "$OUTPUT_DIR/dirty_files.txt"
echo "Count: $(wc -l < "$OUTPUT_DIR/dirty_files.txt")"
echo

# 3. Instrumentation code
echo "🔬 Instrumentation files:"
find . -name "*trace*" -o -name "*perf*" -o -name "*instrument*" | grep -v ".git" | tee "$OUTPUT_DIR/instrumentation.txt"
echo

# 4. Nix files
echo "❄️  Nix files:"
find . -name "*.nix" | tee "$OUTPUT_DIR/nix_files.txt"
echo

# 5. Existing traces
echo "📊 Existing traces:"
find . -name "*.perf.data" -o -name "*.perf.script" 2>/dev/null | tee "$OUTPUT_DIR/existing_traces.txt" || echo "None found"
echo

# 6. Models
echo "🤖 Models:"
find . -name "*.gguf" -o -name "*.bin" 2>/dev/null | head -10 | tee "$OUTPUT_DIR/models.txt" || echo "None found"
echo

# 7. Branch summary
echo "📝 Branch analysis:"
cat > "$OUTPUT_DIR/branch_summary.txt" << 'SUMMARY'
Branches to review:
- feature/zos: ZOS integration work
- feature/save_temps: Save intermediate files
- feature/boost_python: Python bindings
- feature/ocaml: OCaml bindings
- feature/refl-cpp: Reflection work

Action items:
1. Merge useful work from feature branches
2. Archive experimental branches
3. Clean up dirty files
4. Document instrumentation approach
SUMMARY
cat "$OUTPUT_DIR/branch_summary.txt"
echo

# 8. Generate cleanup script
cat > "$OUTPUT_DIR/cleanup.sh" << 'CLEANUP'
#!/bin/bash
# Generated cleanup script

cd /mnt/data1/2023/11/09/llama.cpp

# Remove dirty files
find . -name "*.py~" -delete
find . -name "#*#" -delete
find . -name ".#*" -delete

echo "✅ Cleaned up dirty files"
CLEANUP
chmod +x "$OUTPUT_DIR/cleanup.sh"

echo "✅ Audit complete!"
echo "📁 Output: $OUTPUT_DIR/"
echo
echo "Next steps:"
echo "1. Review branch_summary.txt"
echo "2. Run cleanup.sh to remove dirty files"
echo "3. Start Phase 2: Clean Nix build"
