#!/bin/bash
# Register all 15,244 repos from gitdirs.txt

echo "📋 Registering 15,244 repos..."

count=0
while read gitdir; do
  repo_dir=$(dirname "$gitdir")
  repo_name=$(basename "$repo_dir")
  
  ./target/release/git-sources register "$repo_name" "$repo_dir" 2>&1 | grep -q "Registered" && ((count++))
  
  if [ $((count % 100)) -eq 0 ]; then
    echo "  Registered $count repos..."
  fi
done < gitdirs.txt

echo "✅ Registered $count repos total"
echo "📊 Running URL scanner on all repos..."

./target/release/extract_urls_from_packs

echo "✅ Complete!"
