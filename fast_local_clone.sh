#!/bin/bash
# Fast local clone from existing /mnt/data1/git repos

MIRROR_DIR="/mnt/data1/git"
TARGET_DIR="/mnt/data1/meta-introspector/local_repos"

mkdir -p "$TARGET_DIR"

echo "🚀 Fast cloning from local mirror..."

count=0
skipped=0

while IFS= read -r url; do
  # Skip relative/local paths only
  [[ "$url" =~ ^\.\./ ]] && { ((skipped++)); continue; }
  [[ "$url" =~ ^/ ]] && { ((skipped++)); continue; }
  
  # Normalize URL to path
  path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git://||' | sed 's|\.git$||')
  source="$MIRROR_DIR/$path"
  
  if [ -d "$source" ]; then
    repo_name=$(basename "$path")
    target="$TARGET_DIR/$repo_name"
    
    if [ ! -d "$target" ]; then
      git clone --local "$source" "$target" 2>/dev/null && ((count++))
    fi
  fi
  
  [ $((count % 100)) -eq 0 ] && [ $count -gt 0 ] && echo "  ✅ Cloned $count repos..."
done < data/queue_all.txt

echo "⏭️  Skipped $skipped invalid URLs"

echo "✅ Cloned $count repos from local mirror"
