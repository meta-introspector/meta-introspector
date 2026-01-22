#!/bin/bash
# Analyze repo ownership: source URL, org/owner, fork status

LOGS_DIR="$HOME/.local/share/nix-builder/logs"
REPORT="REPO_OWNERSHIP_REPORT.md"

cd /mnt/data1/meta-introspector

echo "🔍 Analyzing repo ownership..."

cat > "$REPORT" << 'EOF'
# Repository Ownership Report

**Generated**: $(date)

## Summary

EOF

# Get unique repo paths from logs
repos=$(ls -1 "$LOGS_DIR"/*.log | sed 's/_[0-9]*\.log$//' | sort -u)

total=0
ours=0
forks=0
external=0

cat >> "$REPORT" << 'EOF'
| Repository | Path | Remote URL | Owner | Fork Status |
|------------|------|------------|-------|-------------|
EOF

for log in "$LOGS_DIR"/*.log; do
    # Extract repo path from log
    repo_path=$(grep "^Repo: " "$log" | head -1 | cut -d' ' -f2)
    
    if [ -z "$repo_path" ] || [ ! -d "$repo_path" ]; then
        continue
    fi
    
    ((total++))
    
    # Get short name
    name=$(basename "$repo_path" | cut -c1-30)
    
    # Get git remote
    if [ -d "$repo_path/.git" ]; then
        cd "$repo_path"
        remote=$(git remote get-url origin 2>/dev/null || echo "no-remote")
        
        # Determine owner
        if echo "$remote" | grep -q "meta-introspector"; then
            owner="meta-introspector"
            ((ours++))
        elif echo "$remote" | grep -q "github.com"; then
            owner=$(echo "$remote" | sed 's|.*github.com[:/]||; s|/.*||')
            ((external++))
        else
            owner="local"
        fi
        
        # Check if fork
        upstream=$(git remote get-url upstream 2>/dev/null)
        if [ -n "$upstream" ]; then
            fork_status="fork"
            ((forks++))
        else
            fork_status="source"
        fi
        
        echo "| $name | $repo_path | $remote | $owner | $fork_status |" >> "$REPORT"
    else
        echo "| $name | $repo_path | no-git | local | source |" >> "$REPORT"
    fi
done

echo "" >> "$REPORT"
echo "## Statistics" >> "$REPORT"
echo "" >> "$REPORT"
echo "- **Total repos**: $total" >> "$REPORT"
echo "- **meta-introspector owned**: $ours" >> "$REPORT"
echo "- **External repos**: $external" >> "$REPORT"
echo "- **Forks**: $forks" >> "$REPORT"
echo "- **Original sources**: $((total - forks))" >> "$REPORT"

echo "" >> "$REPORT"
echo "## By Owner" >> "$REPORT"
echo "" >> "$REPORT"

# Group by owner
for log in "$LOGS_DIR"/*.log; do
    repo_path=$(grep "^Repo: " "$log" | head -1 | cut -d' ' -f2)
    [ -z "$repo_path" ] || [ ! -d "$repo_path/.git" ] && continue
    
    cd "$repo_path"
    remote=$(git remote get-url origin 2>/dev/null || echo "no-remote")
    
    if echo "$remote" | grep -q "github.com"; then
        owner=$(echo "$remote" | sed 's|.*github.com[:/]||; s|/.*||')
        echo "$owner"
    fi
done | sort | uniq -c | sort -rn >> "$REPORT"

echo "✅ Report saved to $REPORT"
cat "$REPORT" | head -80
