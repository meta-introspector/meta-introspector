#!/bin/bash

# Repository Analysis Script - Ordered by Latest Commit
# Orders repositories by latest commit date and shows git status

cd /mnt/data1/meta-introspector/data/repos

# Create temporary file to store repo data
temp_file=$(mktemp)

for repo in *; do
    if [ -L "$repo" ]; then
        target=$(readlink "$repo")
        
        if [ -d "$target" ]; then
            cd "$target"
            
            # Check if it's a git repo
            if [ -d ".git" ]; then
                # Get latest commit timestamp for sorting
                commit_timestamp=$(git log -1 --format="%ct" 2>/dev/null || echo "0")
                commit_date=$(git log -1 --format="%ci" 2>/dev/null || echo "No commits")
                commit_hash=$(git log -1 --format="%h" 2>/dev/null || echo "")
                commit_msg=$(git log -1 --format="%s" 2>/dev/null || echo "")
                
                # Get git status
                status=$(git status --porcelain 2>/dev/null | wc -l)
                if [ "$status" -eq 0 ]; then
                    status_text="Clean"
                else
                    status_text="$status modified files"
                fi
                
                # Get branch info
                branch=$(git branch --show-current 2>/dev/null || echo "No branch")
                
                echo "$commit_timestamp|$repo|$commit_date|$commit_hash|$commit_msg|$status_text|$branch" >> "$temp_file"
            fi
            
            cd - > /dev/null
        fi
    fi
done

# Sort by timestamp (descending) and format output
echo "# Repository Analysis - Ordered by Latest Commit"
echo "Generated: $(date)"
echo ""

sort -t'|' -k1,1nr "$temp_file" | while IFS='|' read -r timestamp repo commit_date commit_hash commit_msg status branch; do
    echo "## $repo"
    echo "**Latest commit:** $commit_date [$commit_hash]"
    echo "**Message:** $commit_msg"
    echo "**Status:** $status"
    echo "**Branch:** $branch"
    echo ""
done

# Clean up
rm "$temp_file"
