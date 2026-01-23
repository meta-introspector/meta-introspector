#!/usr/bin/env bash
# Query LM Studio models from HuggingFace and cache locally

set -euo pipefail

OUTPUT_DIR="/mnt/data1/meta-introspector/vendor/lmstudio-models-cache"
mkdir -p "$OUTPUT_DIR"

echo "=== Fetching LM Studio Models from HuggingFace ==="
echo "Output: $OUTPUT_DIR"
echo ""

# Fetch the organization's models
echo "Querying lmstudio-community organization..."
curl -s "https://huggingface.co/api/organizations/lmstudio-community/models" > "$OUTPUT_DIR/models.json"

# Count models
MODEL_COUNT=$(jq '. | length' "$OUTPUT_DIR/models.json" 2>/dev/null || echo "0")
echo "Found $MODEL_COUNT models"

# Extract model names and metadata
jq -r '.[] | "\(.id),\(.downloads),\(.likes)"' "$OUTPUT_DIR/models.json" > "$OUTPUT_DIR/models.csv" 2>/dev/null || true

# Save metadata
cat > "$OUTPUT_DIR/metadata.json" << EOF
{
  "source": "https://huggingface.co/lmstudio-community",
  "fetched": "$(date -Iseconds)",
  "model_count": $MODEL_COUNT,
  "api_endpoint": "https://huggingface.co/api/organizations/lmstudio-community/models"
}
EOF

echo ""
echo "✅ Model catalog cached"
echo "Files:"
echo "  - $OUTPUT_DIR/models.json (full data)"
echo "  - $OUTPUT_DIR/models.csv (id,downloads,likes)"
echo "  - $OUTPUT_DIR/metadata.json"
echo ""
echo "Top 10 models by downloads:"
jq -r '.[] | "\(.downloads)\t\(.id)"' "$OUTPUT_DIR/models.json" 2>/dev/null | sort -rn | head -10 || true
