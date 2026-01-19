#!/bin/bash
# Smart clone: check local first, normalize to HTTPS, skip duplicates

MIRROR_DIR="/mnt/data1/git"
QUEUE_FILE="data/queue_all.txt"
FAILED_LOG="data/failed_clones.txt"

normalize_url() {
  local url="$1"
  
  # Convert git@ to https://
  url=$(echo "$url" | sed 's|^git@github.com:|https://github.com/|')
  url=$(echo "$url" | sed 's|^git@gitlab.com:|https://gitlab.com/|')
  url=$(echo "$url" | sed 's|^git@|https://|' | sed 's|:|/|')
  
  # Convert git:// to https://
  url=$(echo "$url" | sed 's|^git://|https://|')
  
  # Skip relative paths and local paths
  [[ "$url" =~ ^\.\./ ]] && return 1
  [[ "$url" =~ ^/ ]] && return 1
  
  # Ensure https://
  [[ ! "$url" =~ ^https:// ]] && return 1
  
  echo "$url"
}

url_to_path() {
  echo "$1" | sed 's|https://||' | sed 's|\.git$||'
}

count=0
failed=0
skipped=0
local_found=0

while IFS= read -r url; do
  normalized=$(normalize_url "$url")
  [ $? -ne 0 ] && { ((skipped++)); continue; }
  
  canonical_path=$(url_to_path "$normalized")
  target="$MIRROR_DIR/$canonical_path"
  
  # Check if already cloned
  if [ -d "$target/.git" ] || [ -d "$target/refs" ]; then
    ((local_found++))
    [ $((local_found % 100)) -eq 0 ] && echo "  ✓ Found $local_found local repos..."
    continue
  fi
  
  echo "  Cloning: $normalized"
  mkdir -p "$(dirname "$target")"
  
  if timeout 300 git clone --mirror "$normalized" "$target" 2>&1; then
    ((count++))
  else
    ((failed++))
    echo "$normalized" >> "$FAILED_LOG"
  fi
  
  [ $((count % 10)) -eq 0 ] && [ $count -gt 0 ] && {
    echo "  ✅ $count new | ✓ $local_found local | ⏭️ $skipped skipped | ❌ $failed failed"
    sleep 2
  }
done < "$QUEUE_FILE"

echo ""
echo "✅ Cloned: $count new repos"
echo "✓  Found: $local_found already local"
echo "⏭️  Skipped: $skipped (invalid URLs)"
echo "❌ Failed: $failed"
