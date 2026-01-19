#!/bin/bash
# Populate /mnt/data1/git with symlinks to existing repos

MIRROR_DIR="/mnt/data1/git"
LINKS_DIR="$MIRROR_DIR/.links"

mkdir -p "$LINKS_DIR"

count=0
skipped=0

while read gitdir; do
  repo_dir=$(dirname "$gitdir")
  
  # Get remote URL
  remote=$(git -C "$repo_dir" remote get-url origin 2>/dev/null || echo "")
  
  [ -z "$remote" ] && { ((skipped++)); continue; }
  
  # Normalize to path
  path=$(echo "$remote" | sed 's|https://||' | sed 's|http://||' | sed 's|git://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="$MIRROR_DIR/$path"
  
  # Skip if already exists (real dir or symlink)
  [ -e "$target" ] && continue
  
  mkdir -p "$(dirname "$target")"
  ln -s "$repo_dir" "$target"
  
  ((count++))
  [ $((count % 100)) -eq 0 ] && echo "  ✅ Linked $count repos..."
done < gitdirs.txt

echo "✅ Created $count symlinks"
echo "⏭️  Skipped $skipped (no remote)"
