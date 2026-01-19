#!/bin/bash
# update_indexes.sh - Refresh file indexes (scheduled hourly/daily)

set -e

echo "🔄 Updating file indexes..."
date

INDEX_DIR="/mnt/data1/meta-introspector"
TEMP_DIR="/tmp/index_update_$$"
mkdir -p "$TEMP_DIR"

# 1. Update git-tracked files (fast)
echo "1. Git-tracked files..."
cd "$INDEX_DIR"
git ls-files > "$TEMP_DIR/git_tracked.txt"
echo "   Found: $(wc -l < "$TEMP_DIR/git_tracked.txt") files"

# 2. Update flake paths (fast - from git index)
echo "2. Flake paths..."
git ls-files | grep "flake.nix" | xargs dirname > "$TEMP_DIR/flake_paths.txt"
echo "   Found: $(wc -l < "$TEMP_DIR/flake_paths.txt") flakes"

# 3. Update /nix/store logs (fast - single level)
echo "3. Nix store logs..."
ls /nix/store/*-with-logs 2>/dev/null > "$TEMP_DIR/store_logs.txt" || touch "$TEMP_DIR/store_logs.txt"
echo "   Found: $(wc -l < "$TEMP_DIR/store_logs.txt") log derivations"

# 4. Update project list (from existing JSON)
echo "4. Project list..."
if [ -f "$INDEX_DIR/nix_build_packages.json" ]; then
  jq -r 'to_entries[] | .key' "$INDEX_DIR/nix_build_packages.json" > "$TEMP_DIR/project_names.txt"
  echo "   Found: $(wc -l < "$TEMP_DIR/project_names.txt") projects"
fi

# 5. Incremental update of FILE_GIT_MAPPING.csv (only new files)
echo "5. Incremental file mapping..."
if [ -f "$INDEX_DIR/FILE_GIT_MAPPING.csv" ]; then
  # Get files added since last update
  last_update=$(stat -c %Y "$INDEX_DIR/FILE_GIT_MAPPING.csv")
  new_files=$(git ls-files | while read f; do
    if [ -f "$f" ] && [ $(stat -c %Y "$f") -gt $last_update ]; then
      echo "$f"
    fi
  done)
  
  if [ -n "$new_files" ]; then
    echo "   New files: $(echo "$new_files" | wc -l)"
    # Append to existing CSV (would need proper CSV formatting)
  else
    echo "   No new files since last update"
  fi
fi

# 6. Move temp files to index directory
echo "6. Installing indexes..."
mv "$TEMP_DIR/git_tracked.txt" "$INDEX_DIR/indexes/git_tracked.txt"
mv "$TEMP_DIR/flake_paths.txt" "$INDEX_DIR/indexes/flake_paths.txt"
mv "$TEMP_DIR/store_logs.txt" "$INDEX_DIR/indexes/store_logs.txt"
[ -f "$TEMP_DIR/project_names.txt" ] && mv "$TEMP_DIR/project_names.txt" "$INDEX_DIR/indexes/project_names.txt"

# Cleanup
rm -rf "$TEMP_DIR"

echo "✅ Indexes updated"
echo "   Location: $INDEX_DIR/indexes/"
ls -lh "$INDEX_DIR/indexes/"
