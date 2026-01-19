#!/bin/bash
# Step 1: Copy local repos to /mnt/data1/git/ as canonical versions
# Then add remotes to track upstream

echo "📦 Creating canonical repo copies in /mnt/data1/git/"

mkdir -p /mnt/data1/git

count=0
dirty=0

while read gitdir; do
  repo_dir=$(dirname "$gitdir")
  
  # Get remote URL
  remote=$(git -C "$repo_dir" remote get-url origin 2>/dev/null || echo "")
  
  if [ -z "$remote" ]; then
    continue
  fi
  
  # Normalize to path: github.com/org/repo
  canonical_path=$(echo "$remote" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ ! -d "$target" ]; then
    echo "  Copying: $repo_dir → $target"
    mkdir -p "$(dirname "$target")"
    
    # Clone as bare repo (clean)
    git clone --mirror "$repo_dir" "$target" 2>&1 | grep -E "Cloning|done" || true
    
    # Check if source was dirty
    if git -C "$repo_dir" status --porcelain 2>/dev/null | grep -q .; then
      echo "    ⚠️  Source was dirty: $repo_dir"
      echo "$repo_dir" >> /mnt/data1/git/dirty_sources.txt
      ((dirty++))
    fi
    
    ((count++))
    
    if [ $((count % 100)) -eq 0 ]; then
      echo "  Copied $count repos ($dirty dirty)..."
    fi
  fi
done < gitdirs.txt

echo "✅ Copied $count repos to /mnt/data1/git/"
echo "⚠️  Found $dirty dirty sources (saved to /mnt/data1/git/dirty_sources.txt)"
echo ""
echo "Next: Run add_remotes.sh to add upstream remotes"
