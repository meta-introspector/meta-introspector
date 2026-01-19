#!/usr/bin/env bash
# Test search utilities

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/search_utils.sh"

echo "🧪 Testing Search Utilities"
echo "=============================="
echo ""

# Test 1: Find Rust files
echo "1️⃣ Testing find_rust_files..."
count=$(find_rust_files . | wc -l)
echo "   Found $count Rust files"
echo ""

# Test 2: Find flakes
echo "2️⃣ Testing find_flakes..."
count=$(find_flakes . | wc -l)
echo "   Found $count flake.nix files"
echo ""

# Test 3: Find workspaces
echo "3️⃣ Testing find_workspaces..."
workspaces=$(find_workspaces . | head -3)
if [ -n "$workspaces" ]; then
    echo "   Found workspaces:"
    echo "$workspaces" | sed 's/^/   - /'
else
    echo "   No workspaces found"
fi
echo ""

# Test 4: Grep operations
echo "4️⃣ Testing grep_count..."
if [ -f "Cargo.toml" ]; then
    count=$(grep_count "name" "Cargo.toml")
    echo "   'name' appears $count times in Cargo.toml"
fi
echo ""

# Test 5: Find by extension
echo "5️⃣ Testing find_by_ext..."
count=$(find_by_ext . "sh" | wc -l)
echo "   Found $count shell scripts"
echo ""

# Test 6: Find git repos
echo "6️⃣ Testing find_git_repos..."
count=$(find_git_repos . | wc -l)
echo "   Found $count git repositories"
echo ""

# Test 7: Find multi ext
echo "7️⃣ Testing find_multi_ext..."
count=$(find_multi_ext . rs toml | wc -l)
echo "   Found $count Rust and TOML files"
echo ""

# Test 8: Grep errors (if build log exists)
echo "8️⃣ Testing grep_errors..."
if [ -f "build.log" ]; then
    errors=$(grep_errors "build.log" | head -3)
    if [ -n "$errors" ]; then
        echo "   Top errors:"
        echo "$errors" | sed 's/^/   /'
    else
        echo "   No errors in build.log"
    fi
else
    echo "   No build.log found (skipped)"
fi
echo ""

echo "✅ All tests completed!"
echo ""
echo "To use in your scripts:"
echo "  source lib/search_utils.sh"
echo ""
echo "See docs/SEARCH_UTILS_MIGRATION.md for full documentation"
