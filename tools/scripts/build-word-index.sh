#!/usr/bin/env bash
set -euo pipefail

echo "📚 Building ZOS Word Index"
echo "=========================="

OUT="zos-results/word-index"
mkdir -p "$OUT"

# Index all documentation
echo "Indexing documentation..."
find zos/ -name "*.md" -type f | while read file; do
    # Extract words, lowercase, sort, count
    tr '[:upper:]' '[:lower:]' < "$file" | \
    grep -oE '\w+' | \
    sort | uniq -c | sort -rn > "$OUT/$(basename $file .md).words"
done

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
