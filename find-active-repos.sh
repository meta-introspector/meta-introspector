#!/bin/bash
# Find all repos with activity in 2025 and check if they're in git-sources registry

YEAR="2025"
SINCE="${YEAR}-01-01"
UNTIL="${YEAR}-12-31"

echo "🔍 Finding all repos with activity in $YEAR"
echo ""

# Collect all repos with activity
ACTIVE_REPOS=$(mktemp)
REGISTRY_REPOS=$(mktemp)

# Get repos from git-sources registry
./target/release/git-sources list | grep "Path:" | awk '{print $2}' > "$REGISTRY_REPOS"

# Find all git repos and check for activity
echo "Scanning for active repositories..."
locate -r "\.git$" | grep -v "/\.git/" | while read -r git_dir; do
    repo_path=$(dirname "$git_dir")
    
    # Check if repo has commits in 2025
    commit_count=$(git -C "$repo_path" log --since="$SINCE" --until="$UNTIL" --oneline 2>/dev/null | wc -l)
    
    if [ "$commit_count" -gt 0 ]; then
        repo_name=$(basename "$repo_path")
        echo "$repo_path|$repo_name|$commit_count" >> "$ACTIVE_REPOS"
    fi
done

echo ""
echo "📊 Activity Report for $YEAR"
echo "=============================="
echo ""

# Compare with registry
echo "Repos with activity: $(wc -l < "$ACTIVE_REPOS")"
echo "Repos in registry: $(wc -l < "$REGISTRY_REPOS")"
echo ""

echo "✅ REGISTERED (in git-sources):"
echo "--------------------------------"
while IFS='|' read -r path name commits; do
    if grep -q "^$path$" "$REGISTRY_REPOS"; then
        echo "  ✓ $name ($commits commits)"
    fi
done < "$ACTIVE_REPOS"

echo ""
echo "❌ MISSING (need to add):"
echo "-------------------------"
while IFS='|' read -r path name commits; do
    if ! grep -q "^$path$" "$REGISTRY_REPOS"; then
        echo "  ✗ $name ($commits commits) - $path"
    fi
done < "$ACTIVE_REPOS"

# Save detailed report
cat > data/activity-vs-registry.json << EOF
{
  "year": $YEAR,
  "active_repos": $(wc -l < "$ACTIVE_REPOS"),
  "registered_repos": $(wc -l < "$REGISTRY_REPOS"),
  "missing": [
EOF

FIRST=true
while IFS='|' read -r path name commits; do
    if ! grep -q "^$path$" "$REGISTRY_REPOS"; then
        if [ "$FIRST" = false ]; then
            echo "," >> data/activity-vs-registry.json
        fi
        echo -n "    {\"name\": \"$name\", \"path\": \"$path\", \"commits\": $commits}" >> data/activity-vs-registry.json
        FIRST=false
    fi
done < "$ACTIVE_REPOS"

cat >> data/activity-vs-registry.json << EOF

  ]
}
EOF

rm "$ACTIVE_REPOS" "$REGISTRY_REPOS"

echo ""
echo "📄 Detailed report: data/activity-vs-registry.json"
