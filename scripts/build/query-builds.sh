#!/usr/bin/env bash
# Query all meta-introspector builds in nix store by metadata

set -euo pipefail

echo "🔍 Querying meta-introspector builds in nix store..."
echo ""

# Find all our builds by metadata marker
BUILDS=$(find /nix/store -type d -name ".meta-introspector" 2>/dev/null | sed 's|/.meta-introspector||')

if [ -z "$BUILDS" ]; then
    echo "No builds found with metadata"
    exit 0
fi

COUNT=$(echo "$BUILDS" | wc -l)
echo "Found $COUNT builds with metadata:"
echo ""

# Show each build with metadata
echo "$BUILDS" | while read -r build_path; do
    if [ -f "$build_path/.meta-introspector/metadata.json" ]; then
        echo "📦 $build_path"
        
        # Extract key metadata
        TIMESTAMP=$(jq -r '.["meta-introspector"].timestamp' "$build_path/.meta-introspector/metadata.json" 2>/dev/null || echo "unknown")
        COMMIT=$(jq -r '.["meta-introspector"].commit' "$build_path/.meta-introspector/metadata.json" 2>/dev/null || echo "unknown")
        BUILDER=$(jq -r '.["meta-introspector"].builder' "$build_path/.meta-introspector/metadata.json" 2>/dev/null || echo "unknown")
        
        echo "   Timestamp: $TIMESTAMP"
        echo "   Commit: ${COMMIT:0:8}"
        echo "   Builder: $BUILDER"
        
        # Check for perf data
        if [ -d "$build_path/perf" ]; then
            PERF_FILES=$(find "$build_path/perf" -name "*.perf.data" | wc -l)
            echo "   Perf traces: $PERF_FILES"
        fi
        
        echo ""
    fi
done

# Summary
echo "Summary:"
echo "  Total builds: $COUNT"
echo "  With perf data: $(find /nix/store -path "*/.meta-introspector/../perf/*.perf.data" 2>/dev/null | wc -l)"
echo ""
echo "To query specific build:"
echo "  jq . /nix/store/xxx/.meta-introspector/metadata.json"
