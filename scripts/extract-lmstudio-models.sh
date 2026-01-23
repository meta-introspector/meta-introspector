#!/usr/bin/env bash
# Extract LM Studio model catalog for offline reference

set -euo pipefail

OUTPUT_DIR="/mnt/data1/meta-introspector/data/lmstudio-models"
mkdir -p "$OUTPUT_DIR"

echo "=== Extracting LM Studio Model Catalog ==="
echo "Output: $OUTPUT_DIR"
echo ""

# Fetch the models page
echo "Fetching models list from lmstudio.ai..."
curl -s "https://lmstudio.ai/models?sort=size&dir=asc" > "$OUTPUT_DIR/models-page.html"

# Extract model names (basic parsing)
echo "Extracting model names..."
grep -oP '(?<=data-model-id=")[^"]+' "$OUTPUT_DIR/models-page.html" > "$OUTPUT_DIR/model-ids.txt" 2>/dev/null || true

# Count models
MODEL_COUNT=$(wc -l < "$OUTPUT_DIR/model-ids.txt" 2>/dev/null || echo "0")
echo "Found $MODEL_COUNT models"

# Save metadata
cat > "$OUTPUT_DIR/metadata.json" << EOF
{
  "source": "https://lmstudio.ai/models",
  "extracted": "$(date -Iseconds)",
  "model_count": $MODEL_COUNT,
  "sort": "size",
  "direction": "asc"
}
EOF

echo ""
echo "✅ Model catalog extracted"
echo "Files:"
echo "  - $OUTPUT_DIR/models-page.html"
echo "  - $OUTPUT_DIR/model-ids.txt"
echo "  - $OUTPUT_DIR/metadata.json"
