#!/bin/bash

# Quick stat and sort git repos by date

temp_file=$(mktemp)

while read -r config_path; do
    if [ -f "$config_path" ]; then
        repo_dir=$(dirname "$config_path" | sed 's|/.git$||')
        
        if [ -d "$repo_dir/.git" ]; then
            cd "$repo_dir"
            
            # Get timestamp for sorting
            timestamp=$(git log -1 --format="%ct" 2>/dev/null || echo "0")
            date=$(git log -1 --format="%ci" 2>/dev/null || echo "No commits")
            
            echo "$timestamp|$repo_dir|$date" >> "$temp_file"
            cd - > /dev/null
        fi
    fi
done < /home/mdupont/zos-server/git_config_list.txt

# Sort by timestamp and show results
sort -t'|' -k1,1nr "$temp_file" | head -20 | while IFS='|' read -r timestamp repo_dir date; do
    echo "$date - $repo_dir"
done

rm "$temp_file"
