#!/bin/bash

# Check parent directories for git repos
cd /mnt/data1/meta-introspector/data/repos

for repo in *; do
    if [ -L "$repo" ]; then
        target=$(readlink "$repo")
        
        if [ -d "$target" ]; then
            # Check if target itself is git repo
            if [ -d "$target/.git" ]; then
                echo "GIT: $repo -> $target"
                continue
            fi
            
            # Check parent directories for .git
            current_dir="$target"
            while [ "$current_dir" != "/" ] && [ "$current_dir" != "." ]; do
                if [ -d "$current_dir/.git" ]; then
                    cd "$current_dir"
                    commit_date=$(git log -1 --format="%ci" 2>/dev/null || echo "No commits")
                    commit_hash=$(git log -1 --format="%h" 2>/dev/null || echo "")
                    branch=$(git branch --show-current 2>/dev/null || echo "No branch")
                    status=$(git status --porcelain 2>/dev/null | wc -l)
                    if [ "$status" -eq 0 ]; then
                        status_text="Clean"
                    else
                        status_text="$status modified files"
                    fi
                    echo "PARENT-GIT: $repo -> $current_dir | $commit_date [$commit_hash] | $status_text | $branch"
                    cd - > /dev/null
                    break
                fi
                current_dir=$(dirname "$current_dir")
            done
        fi
    fi
done | sort -k3
