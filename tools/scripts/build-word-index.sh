#!/usr/bin/env bash
set -euo pipefail

echo "📚 Building ZOS Word Index"
echo "=========================="

OUT="zos-results/word-index"
mkdir -p "$OUT"

# Index all documentation (use git ls-files, not find)
echo "Indexing documentation..."
git ls-files 'zos/*.md' | while read file; do
    tr '[:upper:]' '[:lower:]' < "$file" | \
    grep -oE '\w+' | \
    sort | uniq -c | sort -rn > "$OUT/$(basename $file .md).words"
done

# Index Rust files (use git ls-files)
echo "Indexing Rust files..."
git ls-files '*.rs' | while read file; do
    tr '[:upper:]' '[:lower:]' < "$file" | \
    grep -oE '\w+' | \
    sort | uniq -c | sort -rn
done | awk '{print $2, $1}' | \
    awk '{count[$1]+=$2} END {for(word in count) print count[word], word}' | \
    sort -rn > "$OUT/rust.words"

# Index Nix files (use git ls-files)
echo "Indexing Nix files..."
git ls-files '*.nix' | while read file; do
    tr '[:upper:]' '[:lower:]' < "$file" | \
    grep -oE '\w+' | \
    sort | uniq -c | sort -rn
done | awk '{print $2, $1}' | \
    awk '{count[$1]+=$2} END {for(word in count) print count[word], word}' | \
    sort -rn > "$OUT/nix.words"

# Index Shell files (use git ls-files)
echo "Indexing Shell files..."
git ls-files '*.sh' | while read file; do
    tr '[:upper:]' '[:lower:]' < "$file" | \
    grep -oE '\w+' | \
    sort | uniq -c | sort -rn
done | awk '{print $2, $1}' | \
    awk '{count[$1]+=$2} END {for(word in count) print count[word], word}' | \
    sort -rn > "$OUT/shell.words"

# Combine all word counts
echo "Combining word counts..."
cat "$OUT"/*.words | \
    awk '{print $2, $1}' | \
    awk '{count[$1]+=$2} END {for(word in count) print count[word], word}' | \
    sort -rn > "$OUT/total.words"

# Extract key terms (words appearing in multiple docs)
echo "Finding key terms..."
for word in $(head -100 "$OUT/total.words" | awk '{print $2}'); do
    count=$(grep -l "$word" zos/*.md 2>/dev/null | wc -l)
    if [ "$count" -gt 3 ]; then
        echo "$count $word"
    fi
done | sort -rn > "$OUT/key-terms.txt"

# Build term-document matrix
echo "Building term-document matrix..."
{
    echo "# ZOS Term-Document Matrix"
    echo ""
    echo "Term | Count | Documents"
    echo "-----|-------|----------"
    head -50 "$OUT/key-terms.txt" | while read count term; do
        docs=$(grep -l "$term" zos/*.md 2>/dev/null | xargs -n1 basename | tr '\n' ', ' | sed 's/,$//')
        echo "$term | $count | $docs"
    done
} > "$OUT/term-document-matrix.md"

echo ""
echo "✅ Word index complete:"
echo "  Total words: $(wc -l < $OUT/total.words)"
echo "  Key terms: $(wc -l < $OUT/key-terms.txt)"
echo "  Output: $OUT/"
