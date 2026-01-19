#!/bin/bash
# Run cargo2nix on unique dependency sets only

GROUPS_FILE="cargo_deps_groups.json"
OUTPUT_DIR="cargo2nix_cache"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Running cargo2nix on unique dependency sets..."

# Get unique hashes
hashes=$(jq -r 'keys[]' "$GROUPS_FILE")
total=$(echo "$hashes" | wc -l)
count=0

for hash in $hashes; do
  ((count++))
  
  # Skip empty deps
  [[ "$hash" == "e3b0c44298fc1c14" ]] && continue
  
  # Get first repo with this hash
  repo=$(jq -r ".\"$hash\"[0]" "$GROUPS_FILE")
  
  # Skip if already cached
  [[ -f "$OUTPUT_DIR/$hash.nix" ]] && continue
  
  echo "[$count/$total] Processing $hash ($repo)"
  
  # Run cargo2nix
  cd "$repo" 2>/dev/null || continue
  
  if nix run github:cargo2nix/cargo2nix -- -f -o "/tmp/cargo2nix_$hash.nix" 2>/dev/null; then
    mv "/tmp/cargo2nix_$hash.nix" "$OUTPUT_DIR/$hash.nix"
    echo "  ✅ Cached"
  else
    echo "  ❌ Failed"
  fi
  
  cd - >/dev/null
done

echo "✅ Processed $count unique dependency sets"
echo "📊 Cache: $OUTPUT_DIR/"
