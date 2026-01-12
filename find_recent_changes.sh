#!/bin/bash

# Find all changes from past 3 days from repository results

echo "🔍 ANALYZING CHANGES FROM PAST 3 DAYS"
echo "====================================="

three_days_ago=$(date -d "3 days ago" +%s)
results_dir="/mnt/data1/meta-introspector/data/processed/repo_results"

echo "📅 Looking for changes since: $(date -d "3 days ago")"
echo ""

changes_found=0

for result_file in "$results_dir"/*.json; do
    if [ -f "$result_file" ]; then
        repo_name=$(basename "$result_file" .json)
        
        # Extract last commit date from JSON
        commit_date=$(jq -r '.last_commit.date // empty' "$result_file" 2>/dev/null)
        
        if [ -n "$commit_date" ] && [ "$commit_date" != "null" ]; then
            # Convert commit date to timestamp
            commit_timestamp=$(date -d "$commit_date" +%s 2>/dev/null)
            
            if [ $? -eq 0 ] && [ "$commit_timestamp" -ge "$three_days_ago" ]; then
                changes_found=$((changes_found + 1))
                
                # Extract additional info
                commit_hash=$(jq -r '.last_commit.hash // ""' "$result_file" 2>/dev/null)
                commit_msg=$(jq -r '.last_commit.message // ""' "$result_file" 2>/dev/null)
                branch=$(jq -r '.branch // ""' "$result_file" 2>/dev/null)
                status=$(jq -r '.status.is_clean // true' "$result_file" 2>/dev/null)
                modified_files=$(jq -r '.status.modified_files // 0' "$result_file" 2>/dev/null)
                
                echo "## $repo_name"
                echo "**Last commit:** $commit_date [$commit_hash]"
                echo "**Message:** $commit_msg"
                echo "**Branch:** $branch"
                if [ "$status" = "true" ]; then
                    echo "**Status:** Clean"
                else
                    echo "**Status:** $modified_files modified files"
                fi
                echo ""
            fi
        fi
    fi
done

echo "📊 **Summary:** Found $changes_found repositories with changes in the past 3 days"
