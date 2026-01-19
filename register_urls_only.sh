#!/bin/bash
# Register URLs only - don't clone yet

echo "📋 Registering all discovered URLs (no cloning)"

# Scan existing repos for URLs
find /mnt/data1/git -type d \( -name "*.git" -o -path "*/github.com/*" \) | while read repo; do
  for file in flake.nix default.nix Cargo.toml README.md .gitmodules; do
    [ -f "$repo/$file" ] && grep -Eoh 'https?://[^"'\'' <>]+' "$repo/$file" 2>/dev/null
  done
done | grep -E 'github\.com|gitlab\.com|bitbucket\.org' | sort -u > data/all_discovered_urls.txt

echo "✅ Found $(wc -l < data/all_discovered_urls.txt) unique URLs"

# Register in git-sources (no clone)
count=0
while read url; do
  repo_name=$(basename "$url" .git)
  canonical_path=$(echo "$url" | sed 's|https://||' | sed 's|http://||' | sed 's|git@||' | sed 's|:|/|' | sed 's|\.git$||')
  
  # Register with URL, will clone on-demand later
  echo "$url|$canonical_path" >> data/url_registry.txt
  
  ((count++))
  if [ $((count % 100)) -eq 0 ]; then
    echo "  Registered $count URLs..."
  fi
done < data/all_discovered_urls.txt

echo "✅ Registered $count URLs in data/url_registry.txt"
echo "💡 Clone on-demand when needed, not all at once"
