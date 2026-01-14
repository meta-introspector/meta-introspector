#!/bin/bash
# Incremental ELF file list updater
# Scans for new binaries and adds them to the list

CURRENT_LIST="elf_files_filtered.txt"
UPDATED_LIST="elf_files_updated.txt"
NEW_ONLY="elf_files_new.txt"

echo "🔄 Incremental ELF File List Updater"
echo "===================================="

# Start with current list
cp "$CURRENT_LIST" "$UPDATED_LIST"
BASELINE=$(wc -l < "$CURRENT_LIST")

echo "Baseline: $BASELINE files"
echo ""

# Scan for new files in various locations
echo "📂 Scanning for new binaries..."

# 1. Const test binaries
if [ -d "const_71_analysis" ]; then
    echo "  - const_71_analysis/"
    find const_71_analysis -name "binary_*" -type f >> "$UPDATED_LIST"
fi

# 2. Recent nix store additions (last 24 hours)
echo "  - /nix/store (recent)"
find /nix/store -type f \( -name "*.so*" -o -executable \) -mtime -1 2>/dev/null >> "$UPDATED_LIST"

# 3. Build output directories
for dir in result result-* const_71_test/*/result; do
    if [ -d "$dir" ]; then
        echo "  - $dir"
        find "$dir" -type f -executable 2>/dev/null >> "$UPDATED_LIST"
    fi
done

# 4. Markov analysis output
if [ -d "markov_results" ]; then
    echo "  - markov_results/"
    find markov_results -type f -executable 2>/dev/null >> "$UPDATED_LIST"
fi

# Remove duplicates and sort
sort -u "$UPDATED_LIST" -o "$UPDATED_LIST"

# Find what's new
comm -13 <(sort "$CURRENT_LIST") <(sort "$UPDATED_LIST") > "$NEW_ONLY"

NEW_COUNT=$(wc -l < "$UPDATED_LIST")
ADDED=$(wc -l < "$NEW_ONLY")

echo ""
echo "===================================="
echo "📊 Results:"
echo "  Baseline: $BASELINE files"
echo "  Updated:  $NEW_COUNT files"
echo "  Added:    $ADDED new files"
echo ""

if [ $ADDED -gt 0 ]; then
    echo "🆕 New files:"
    head -20 "$NEW_ONLY" | while read file; do
        SIZE=$(stat -c%s "$file" 2>/dev/null || echo "?")
        echo "  - $(basename "$file") ($SIZE bytes)"
    done
    
    if [ $ADDED -gt 20 ]; then
        echo "  ... and $((ADDED - 20)) more"
    fi
    
    echo ""
    echo "✅ Updated list saved to: $UPDATED_LIST"
    echo "   New files only: $NEW_ONLY"
else
    echo "ℹ️  No new files found"
fi

echo ""
echo "To use updated list:"
echo "  mv $UPDATED_LIST $CURRENT_LIST"
echo "  cargo run --release -p markov_resonance_analyzer -- $CURRENT_LIST"
