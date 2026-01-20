#!/usr/bin/env bash
set -euo pipefail

URL="$1"
SEARCH_PATHS="${2:-/mnt/data1/git:/mnt/data1/meta-introspector}"

# Normalize URL to repo name
REPO_NAME=$(basename "$URL" .git)

# Search for checkout
IFS=':' read -ra PATHS <<< "$SEARCH_PATHS"
for base in "${PATHS[@]}"; do
    if [ -d "$base/$REPO_NAME/.git" ]; then
        echo "$base/$REPO_NAME"
        exit 0
    fi
    
    # Also check subdirectories
    find "$base" -maxdepth 3 -type d -name "$REPO_NAME" 2>/dev/null | while read -r path; do
        if [ -d "$path/.git" ]; then
            echo "$path"
            exit 0
        fi
    done
done

exit 1
