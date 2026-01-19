#!/bin/bash
# Scan all repos in /mnt/data1/git/ for URLs, then clone those too

echo "🔍 Scanning all repos in /mnt/data1/git/ for URLs..."

# Scan all repos
find /mnt/data1/git -type d -name "*.git" -o -type d -path "*/github.com/*" | while read repo; do
  # Check common files
  for file in flake.nix default.nix Cargo.toml README.md .gitmodules; do
    if [ -f "$repo/$file" ]; then
      grep -Eoh 'https?://[^"'\'' <>]+' "$repo/$file" 2>/dev/null || true
    fi
  done
done | grep -E 'github\.com|gitlab\.com|bitbucket\.org' | sort -u > data/all_discovered_urls.txt

echo "✅ Found $(wc -l < data/all_discovered_urls.txt) unique URLs"

# Clone new ones
echo "📦 Cloning newly discovered repos..."

count=0
while read url; do
  canonical_path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  target="/mnt/data1/git/$canonical_path"
  
  if [ ! -d "$target" ]; then
    echo "  Cloning: $url"
    mkdir -p "$(dirname "$target")"
    git clone --mirror "$url" "$target" 2>&1 | grep -q "done" && ((count++))
  fi
  
  if [ $((count % 10)) -eq 0 ] && [ $count -gt 0 ]; then
    echo "  Cloned $count new repos..."
  fi
done < data/all_discovered_urls.txt

echo "✅ Cloned $count new repos"
echo "🔄 Run again to find more URLs (recursive discovery)"
