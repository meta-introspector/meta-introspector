#!/bin/bash

# Cross-reference local git repos with GitHub meta-introspector repos

echo "# Meta-Introspector Repository Cross-Reference"
echo "Generated: $(date)"
echo ""

# Extract GitHub repo names from JSON
github_repos=$(jq -r '.[].name' /home/mdupont/nix/index/github_meta-introspector_repos.json)

echo "## Local Git Repositories Matching Meta-Introspector GitHub Repos"
echo ""

# Check recent repos against GitHub list
while read -r line; do
    timestamp=$(echo "$line" | cut -d' ' -f1)
    date_time=$(echo "$line" | cut -d' ' -f2-3)
    repo_path=$(echo "$line" | cut -d' ' -f4-)
    
    # Extract repo name from path
    repo_name=$(basename "$repo_path")
    
    # Check if this repo name exists in GitHub list
    if echo "$github_repos" | grep -q "^${repo_name}$"; then
        echo "### $repo_name"
        echo "- **Local Path:** $repo_path"
        echo "- **Last Modified:** $date_time"
        echo "- **GitHub:** https://github.com/meta-introspector/$repo_name"
        
        # Get git info if it's a git repo
        if [ -d "$repo_path/.git" ]; then
            cd "$repo_path"
            branch=$(git branch --show-current 2>/dev/null || echo "No branch")
            status=$(git status --porcelain 2>/dev/null | wc -l)
            if [ "$status" -eq 0 ]; then
                status_text="Clean"
            else
                status_text="$status modified files"
            fi
            echo "- **Branch:** $branch"
            echo "- **Status:** $status_text"
            cd - > /dev/null
        fi
        echo ""
    fi
done < /mnt/data1/meta-introspector/data/raw/recent_repos_3months.txt

echo "## Summary"
echo "- Total GitHub meta-introspector repos: $(echo "$github_repos" | wc -l)"
echo "- Recent local repos (3 months): $(wc -l < /mnt/data1/meta-introspector/data/raw/recent_repos_3months.txt)"
