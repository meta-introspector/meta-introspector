# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Use perf probes to instrument ALL functions from LMFDB catalog
# No compilation needed - kernel does the work!

set -e

echo "🔬 Setting up perf probes from LMFDB catalog"

# Load top functions from Parquet catalog
CATALOG="data/nix_lmfdb_analysis/functions_all.parquet"

if [ ! -f "$CATALOG" ]; then
    echo "❌ Catalog not found: $CATALOG"
    exit 1
fi

echo "📊 Loading LMFDB catalog..."

# Extract top 100 functions by conductor using Python
python3 << 'EOF'
import pyarrow.parquet as pq
import sys

table = pq.read_table('data/nix_lmfdb_analysis/functions_all.parquet')
df = table.to_pandas()

# Sort by conductor, take top 100
top = df.nlargest(100, 'conductor')

# Output as: binary:function:conductor
for _, row in top.iterrows():
    print(f"{row['binary']}:{row['function_name']}:{row['conductor']}")
EOF

# Add perf probes for each function
echo ""
echo "🎯 Adding perf probes..."

python3 << 'EOF' | while IFS=: read binary func conductor; do
import pyarrow.parquet as pq

table = pq.read_table('data/nix_lmfdb_analysis/functions_all.parquet')
df = table.to_pandas()
top = df.nlargest(100, 'conductor')

for _, row in top.iterrows():
    print(f"{row['binary']}:{row['function_name']}:{row['conductor']}")
EOF

while IFS=: read binary func conductor; do
    # Add probe
    sudo perf probe -x "/nix/store/*/$binary" "$func" 2>/dev/null && \
        echo "  ✅ $func (conductor: $conductor)" || \
        echo "  ⚠️  $func (failed)"
done

echo ""
echo "📋 Active probes:"
sudo perf probe -l | head -20

echo ""
echo "🚀 Ready to record! Run:"
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
echo "  sudo perf script > nix_build_trace.txt"
