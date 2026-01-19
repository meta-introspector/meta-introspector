#!/bin/bash
# Slow clone 13,126 URLs with rate limiting

echo "📦 Starting slow clone of 13,126 URLs..."

count=0
failed=0
skipped=0

while read url; do
  canonical_path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ -d "$target" ]; then
    ((skipped++))
    continue
  fi
  
  echo "  Cloning: $url"
  mkdir -p "$(dirname "$target")"
  
  if timeout 300 git clone --mirror "$url" "$target" 2>&1 | grep -q "done"; then
    ((count++))
  else
    ((failed++))
    echo "$url" >> data/failed_clones.txt
  fi
  
  if [ $((count % 10)) -eq 0 ] && [ $count -gt 0 ]; then
    echo "  ✅ Cloned $count repos ($failed failed, $skipped skipped)..."
    sleep 2
  fi
done < data/queue_all.txt

echo "✅ Cloned $count new repos"
echo "⏭️  Skipped: $skipped"
echo "❌ Failed: $failed"
