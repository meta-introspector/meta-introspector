#!/bin/bash
# Check all registered repos for GitHub activity in the last year
# Compare with git-sources registry to find missing repos

YEAR="2025"
OUTPUT="data/github-activity-check.json"

echo "🔍 Checking GitHub activity for year $YEAR"
echo "Registry: $(./target/release/git-sources list | grep -c 'Path:') repos"
echo ""

# Get all registered repos
echo "{"
echo "  \"year\": $YEAR,"
echo "  \"repos_in_registry\": $(./target/release/git-sources list | grep -c 'Path:'),"
echo "  \"repos_with_activity\": ["

FIRST=true
./target/release/git-sources foreach "git log --since='$YEAR-01-01' --until='$YEAR-12-31' --oneline 2>/dev/null | wc -l" 2>&1 | \
while IFS= read -r line; do
    if [[ "$line" =~ ^===\ (.+)\ ===$ ]]; then
        REPO_NAME="${BASH_REMATCH[1]}"
    elif [[ "$line" =~ ^[0-9]+$ ]]; then
        COMMIT_COUNT="$line"
        if [ "$COMMIT_COUNT" -gt 0 ]; then
            if [ "$FIRST" = false ]; then
                echo ","
            fi
            echo -n "    {\"repo\": \"$REPO_NAME\", \"commits\": $COMMIT_COUNT}"
            FIRST=false
        fi
    fi
done

echo ""
echo "  ]"
echo "}"

echo ""
echo "✅ Activity check complete"
echo "Output: $OUTPUT"
