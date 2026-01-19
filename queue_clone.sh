#!/bin/bash
# Queue-based cloning: local first, remote slowly

echo "📦 Queueing 13,126 URLs for cloning..."

local_count=0
remote_count=0

cat data/master_url_list.txt | while read url; do
  canonical_path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ -d "$target" ]; then
    continue
  fi
  
  # Check if local
  if [[ "$url" == file://* ]] || [[ "$url" == /mnt/* ]]; then
    echo "$url" >> data/queue_local.txt
    ((local_count++))
  else
    echo "$url" >> data/queue_remote.txt
    ((remote_count++))
  fi
done

echo "✅ Queued:"
echo "  Local: $local_count"
echo "  Remote: $remote_count"
