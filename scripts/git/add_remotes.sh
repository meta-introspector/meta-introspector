#!/bin/bash
# Step 2: Add remotes to canonical repos

echo "🔗 Adding remotes to canonical repos in /mnt/data1/git/"

count=0

find /mnt/data1/git -name "*.git" -type d | while read repo; do
  # Extract org/repo from path
  path=$(echo "$repo" | sed 's|/mnt/data1/git/||' | sed 's|\.git$||')
  
  cd "$repo"
  
  # Add upstream remote if it's github/gitlab
  if [[ "$path" == github.com/* ]]; then
    upstream="https://$path.git"
    git remote add upstream "$upstream" 2>/dev/null || true
  fi
  
  # Add our fork if it exists
  fork_path=$(echo "$path" | sed 's|github.com/[^/]*/|github.com/meta-introspector/|')
  if [ "$fork_path" != "$path" ]; then
    fork_url="https://$fork_path.git"
    git remote add fork "$fork_url" 2>/dev/null || true
  fi
  
  ((count++))
  
  if [ $((count % 100)) -eq 0 ]; then
    echo "  Added remotes to $count repos..."
  fi
done

echo "✅ Added remotes to $count repos"
