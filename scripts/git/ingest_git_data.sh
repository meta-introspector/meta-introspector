#!/bin/bash

# Ingest git configs and gitmodules into master repository list

echo "# Git Repository Discovery Report"
echo "Generated: $(date)"
echo ""

# Process git config files
echo "## Git Repositories from Config Files"
echo ""

while read -r config_path; do
    if [ -f "$config_path" ]; then
        repo_dir=$(dirname "$config_path" | sed 's|/.git$||')
        
        # Extract repository info
        if [ -d "$repo_dir/.git" ]; then
            cd "$repo_dir"
            
            # Get remote URLs
            remotes=$(git remote -v 2>/dev/null | grep fetch | awk '{print $2}' | sort -u)
            
            if [ -n "$remotes" ]; then
                echo "### $repo_dir"
                echo "$remotes" | while read -r remote; do
                    echo "- Remote: $remote"
                done
                
                # Get latest commit
                latest=$(git log -1 --format="%ci [%h] %s" 2>/dev/null || echo "No commits")
                echo "- Latest: $latest"
                
                # Get status
                status=$(git status --porcelain 2>/dev/null | wc -l)
                if [ "$status" -eq 0 ]; then
                    echo "- Status: Clean"
                else
                    echo "- Status: $status modified files"
                fi
                echo ""
            fi
            
            cd - > /dev/null
        fi
    fi
done < /home/mdupont/zos-server/git_config_list.txt

echo "## Repositories with Submodules"
echo ""

# Process gitmodules files
while read -r gitmodules_path; do
    if [ -f "$gitmodules_path" ]; then
        repo_dir=$(dirname "$gitmodules_path")
        echo "### $repo_dir"
        
        # Parse submodules
        grep -E '^\[submodule' "$gitmodules_path" | sed 's/\[submodule "/- Submodule: /' | sed 's/"\]//'
        echo ""
    fi
done < /home/mdupont/zos-server/gitmodules_list.txt
