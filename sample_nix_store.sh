#!/bin/bash
# Sample random binaries from /nix/store for name→instruction mapping

echo "🔬 Sampling /nix/store binaries"

# Find random .so files
find /nix/store -maxdepth 3 -type f -name "*.so*" 2>/dev/null | \
    shuf | head -10 > /tmp/nix_samples.txt

echo "📦 Found $(wc -l < /tmp/nix_samples.txt) libraries"

# Create JSON for our mapper
echo '{"libraries": [' > /tmp/nix_samples.json
cat /tmp/nix_samples.txt | while read path; do
    echo "  \"$path\","
done | sed '$ s/,$//' >> /tmp/nix_samples.json
echo ']}' >> /tmp/nix_samples.json

echo "✅ Created /tmp/nix_samples.json"
cat /tmp/nix_samples.json
