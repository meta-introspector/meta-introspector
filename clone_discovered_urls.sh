#!/bin/bash
# Clone the 269 newly discovered URLs

echo "📦 Cloning 269 newly discovered URLs..."

count=0
failed=0

cat data/scanned_urls.txt | while read url; do
  canonical_path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ ! -d "$target" ]; then
    echo "  Cloning: $url"
    mkdir -p "$(dirname "$target")"
    
    if git clone --mirror "$url" "$target" 2>&1 | grep -q "done"; then
      ((count++))
    else
      ((failed++))
      echo "$url" >> data/failed_clones.txt
    fi
    
    if [ $((count % 10)) -eq 0 ] && [ $count -gt 0 ]; then
      echo "  ✅ Cloned $count repos ($failed failed)..."
    fi
  fi
done

echo "✅ Cloned $count new repos"
echo "❌ Failed: $failed (saved to data/failed_clones.txt)"
