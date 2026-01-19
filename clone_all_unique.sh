#!/bin/bash
# Clone all 15K unique repos to /mnt/data1/git/ by root URL

echo "📦 Cloning 15,244 unique repos to /mnt/data1/git/"

count=0
skipped=0
declare -A seen_urls

while read gitdir; do
  repo_dir=$(dirname "$gitdir")
  
  # Get remote URL
  remote=$(git -C "$repo_dir" remote get-url origin 2>/dev/null || echo "")
  
  if [ -z "$remote" ]; then
    ((skipped++))
    continue
  fi
  
  # Skip duplicates
  if [ -n "${seen_urls[$remote]}" ]; then
    ((skipped++))
    continue
  fi
  seen_urls[$remote]=1
  
  # Normalize to path
  canonical_path=$(echo "$remote" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ ! -d "$target" ]; then
    mkdir -p "$(dirname "$target")"
    git clone --mirror "$repo_dir" "$target" 2>&1 | grep -q "done" && ((count++))
  fi
  
  if [ $((count % 100)) -eq 0 ]; then
    echo "  Cloned $count repos (skipped $skipped)..."
  fi
done < gitdirs.txt

echo "✅ Cloned $count unique repos"
echo "⏭️  Skipped $skipped (no remote or duplicate)"
echo ""
echo "Next: Run scan_all_for_urls.sh to find more URLs"
