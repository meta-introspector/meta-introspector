#!/bin/bash
# CLI driver to test builds

BIN=${1:-"all_commits_collector"}

echo "🔨 Building $BIN..."
echo ""

cargo build --bin "$BIN" 2>&1 | tee /tmp/build_output.txt

echo ""
echo "📊 Parsing errors..."

# Extract errors
grep "error\[E" /tmp/build_output.txt | while read -r line; do
    ERROR_CODE=$(echo "$line" | grep -o "error\[E[0-9]*\]" | head -1)
    echo "  $ERROR_CODE"
done

echo ""
echo "💡 Suggestions:"

# Check for common errors
if grep -q "use of unresolved module or unlinked crate \`gix\`" /tmp/build_output.txt; then
    echo "  ❌ gix dependency - Move to libgit.so"
fi

if grep -q "use of unresolved module or unlinked crate \`reqwest\`" /tmp/build_output.txt; then
    echo "  ❌ reqwest dependency - Move to libhttp.so"
fi

if grep -q "\`main\` function not found" /tmp/build_output.txt; then
    echo "  ❌ Missing main() - Add fn main() or remove [[bin]] entry"
fi

echo ""
