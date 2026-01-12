#!/bin/bash

# Collect last month's commits from repos where mdupont made the last commit

echo "🔍 COLLECTING LAST MONTH'S COMMITS FROM YOUR REPOS"
echo "================================================="

one_month_ago=$(date -d "1 month ago" +%s)
results_dir="/mnt/data1/meta-introspector/data/processed/repo_results"

echo "📅 Looking since: $(date -d "1 month ago")"
echo ""

your_repos=0
total_commits=0

for result_file in "$results_dir"/*.json; do
    if [ -f "$result_file" ]; then
        repo_name=$(basename "$result_file" .json)
        repo_path=$(jq -r '.path // ""' "$result_file" 2>/dev/null)
        
        # Skip if no valid path
        if [ -z "$repo_path" ] || [ "$repo_path" = "null" ]; then
            continue
        fi
        
        # Check if repo exists and is git repo
        if [ -d "$repo_path/.git" ]; then
            cd "$repo_path"
            
            # Get last commit author
            last_author=$(git log -1 --format="%an" 2>/dev/null)
            
            # If you made the last commit, collect your commits from past month
            if echo "$last_author" | grep -qi "mdupont"; then
                your_repos=$((your_repos + 1))
                
                echo "## $repo_name"
                echo "**Path:** $repo_path"
                echo "**Last commit by:** $last_author"
                echo ""
                
                # Get your commits from past month
                commits=$(git log --author="mdupont" --since="1 month ago" --oneline 2>/dev/null)
                
                if [ -n "$commits" ]; then
                    commit_count=$(echo "$commits" | wc -l)
                    total_commits=$((total_commits + commit_count))
                    
                    echo "**Your commits (past month): $commit_count**"
                    echo '```'
                    echo "$commits"
                    echo '```'
                else
                    echo "**No commits from you in past month**"
                fi
                echo ""
            fi
            
            cd - > /dev/null
        fi
    fi
done

echo "📊 **Summary:**"
echo "- Repositories where you made last commit: $your_repos"
echo "- Your total commits in past month: $total_commits"
