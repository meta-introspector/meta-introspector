#!/bin/bash

# Generic system to find all mdupont changes from past N days

DAYS=${1:-3}
USER=${2:-mdupont}

echo "🔍 FINDING ALL $USER CHANGES FROM PAST $DAYS DAYS"
echo "=============================================="

cutoff_date=$(date -d "$DAYS days ago" +%s)
results_dir="/mnt/data1/meta-introspector/data/processed/repo_results"

echo "📅 Cutoff: $(date -d "$DAYS days ago")"
echo ""

total_repos=0
user_repos=0
recent_changes=0

for result_file in "$results_dir"/*.json; do
    if [ -f "$result_file" ]; then
        total_repos=$((total_repos + 1))
        repo_name=$(basename "$result_file" .json)
        
        # Check if repo path contains user
        repo_path=$(jq -r '.path // ""' "$result_file" 2>/dev/null)
        
        # Check if this is a user repository (path contains user or remotes point to user)
        is_user_repo=false
        if echo "$repo_path" | grep -q "$USER"; then
            is_user_repo=true
        else
            # Check remotes for user repositories
            user_remotes=$(jq -r '.remotes[]?.url // empty' "$result_file" 2>/dev/null | grep -c "$USER" || echo 0)
            if [ "$user_remotes" -gt 0 ]; then
                is_user_repo=true
            fi
        fi
        
        if [ "$is_user_repo" = true ]; then
            user_repos=$((user_repos + 1))
            
            # Check for recent commits
            commit_date=$(jq -r '.last_commit.date // ""' "$result_file" 2>/dev/null)
            if [ -n "$commit_date" ] && [ "$commit_date" != "null" ]; then
                commit_timestamp=$(date -d "$commit_date" +%s 2>/dev/null)
                
                if [ $? -eq 0 ] && [ "$commit_timestamp" -ge "$cutoff_date" ]; then
                    recent_changes=$((recent_changes + 1))
                    
                    # Extract details
                    commit_hash=$(jq -r '.last_commit.hash // ""' "$result_file" 2>/dev/null)
                    commit_msg=$(jq -r '.last_commit.message // ""' "$result_file" 2>/dev/null)
                    branch=$(jq -r '.branch // ""' "$result_file" 2>/dev/null)
                    modified_files=$(jq -r '.status.modified_files // 0' "$result_file" 2>/dev/null)
                    is_clean=$(jq -r '.status.is_clean // true' "$result_file" 2>/dev/null)
                    
                    echo "## $repo_name"
                    echo "**Path:** $repo_path"
                    echo "**Commit:** $commit_date [$commit_hash]"
                    echo "**Message:** $commit_msg"
                    echo "**Branch:** $branch"
                    if [ "$is_clean" = "true" ]; then
                        echo "**Status:** Clean"
                    else
                        echo "**Status:** $modified_files modified files"
                    fi
                    echo ""
                fi
            fi
            
            # Also show repos with uncommitted changes
            if [ "$is_clean" = "false" ] && [ "$modified_files" -gt 0 ]; then
                echo "## $repo_name (UNCOMMITTED CHANGES)"
                echo "**Path:** $repo_path"
                echo "**Status:** $modified_files modified files"
                echo "**Branch:** $branch"
                echo ""
            fi
        fi
    fi
done

echo "📊 **Summary:**"
echo "- Total repositories analyzed: $total_repos"
echo "- Your repositories: $user_repos"
echo "- Recent changes (past $DAYS days): $recent_changes"
