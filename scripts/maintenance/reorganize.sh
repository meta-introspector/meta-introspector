#!/bin/bash

# Meta-Introspector Reorganization Script
# Reorganizes the current structure into a cleaner hierarchy

set -e

BASE_DIR="/mnt/data1/meta-introspector"
cd "$BASE_DIR"

echo "Creating new directory structure..."

# Create main directories
mkdir -p data/{raw,processed,domains}
mkdir -p analysis/{reports,statistics,ecosystems,special}
mkdir -p logs

echo "Moving raw data files..."
mv master_canonical_index.json data/raw/ 2>/dev/null || true
mv complete_index.json data/raw/ 2>/dev/null || true
mv file_manifest.txt data/raw/ 2>/dev/null || true
mv repos.txt data/raw/ 2>/dev/null || true

echo "Moving processed data..."
mv canonical_structure.json data/processed/ 2>/dev/null || true
mv canonical_tld_structure.json data/processed/ 2>/dev/null || true
mv canonical/ data/processed/ 2>/dev/null || true
mv canonical-tld/ data/processed/ 2>/dev/null || true
mv canonical-forms/ data/processed/ 2>/dev/null || true

echo "Moving domain directories..."
for tld in com org co io edu fr de cz us net me ht dev; do
    if [ -d "$tld" ]; then
        mv "$tld" data/domains/ 2>/dev/null || true
    fi
done

echo "Moving analysis results..."
mv tld-stats/ analysis/statistics/ 2>/dev/null || true
mv rust-ecosystem/ analysis/ecosystems/ 2>/dev/null || true
mv split-decls/ analysis/special/ 2>/dev/null || true
mv value-lattice/ analysis/special/ 2>/dev/null || true

# Keep existing analysis directory content
if [ -d "analysis" ] && [ "$(ls -A analysis 2>/dev/null)" ]; then
    mv analysis/* analysis/reports/ 2>/dev/null || true
    rmdir analysis 2>/dev/null || true
fi

echo "Moving logs..."
mv service-logs/ logs/ 2>/dev/null || true

echo "Moving repositories data..."
mv repos/ data/ 2>/dev/null || true

echo "Reorganization complete!"
echo "New structure:"
echo "├── data/"
echo "│   ├── raw/            # Original indexes and manifests"
echo "│   ├── processed/      # Canonical forms and structures"  
echo "│   ├── domains/        # Domain-based organization"
echo "│   └── repos/          # Repository data"
echo "├── analysis/"
echo "│   ├── reports/        # Analysis outputs"
echo "│   ├── statistics/     # TLD stats, counts"
echo "│   ├── ecosystems/     # Language-specific analysis"
echo "│   └── special/        # Split-decls, value-lattice"
echo "├── tools/              # Scripts and utilities"
echo "├── docs/               # Documentation"
echo "└── logs/               # Service logs"
