#!/bin/bash

# Repository Analysis Script
# Orders repositories by latest commit date and shows git status

echo "# Repository Analysis Report"
echo "Generated: $(date)"
echo ""

cd /mnt/data1/meta-introspector/data/repos

for repo in *; do
    if [ -L "$repo" ]; then
        target=$(readlink "$repo")
        echo "## $repo"
        echo "Target: $target"
        
        if [ -d "$target" ]; then
            cd "$target"
            
            # Check if it's a git repo
            if [ -d ".git" ]; then
                # Get latest commit info
                latest_commit=$(git log -1 --format="%ci %h %s" 2>/dev/null || echo "No commits")
                echo "Latest commit: $latest_commit"
                
                # Get git status
                status=$(git status --porcelain 2>/dev/null | wc -l)
                if [ "$status" -eq 0 ]; then
                    echo "Status: Clean"
                else
                    echo "Status: $status modified files"
                fi
                
                # Get branch info
                branch=$(git branch --show-current 2>/dev/null || echo "No branch")
                echo "Branch: $branch"
                
            else
                echo "Status: Not a git repository"
            fi
            
            cd - > /dev/null
        else
            echo "Status: Target does not exist"
        fi
        
        echo ""
    fi
done | tee repo_analysis.md
