#!/bin/bash
# Bulk download Wikidata, OEIS, LMFDB data for numerical codebreaking

set -e

DATA_DIR="data/math-databases"
mkdir -p "$DATA_DIR"/{wikidata,oeis,lmfdb}

echo "📥 BULK MATH DATABASE DOWNLOADER"
echo ""

# ============================================================================
# WIKIDATA - RDF Dumps
# ============================================================================
download_wikidata() {
    echo "📊 Downloading Wikidata..."
    cd "$DATA_DIR/wikidata"
    
    # Get latest truthy dump (smaller, faster)
    DUMP_URL="https://dumps.wikimedia.org/wikidatawiki/entities/latest-truthy.nt.gz"
    
    if [ ! -f "latest-truthy.nt.gz" ]; then
        echo "  Downloading truthy RDF dump (~40GB compressed)..."
        wget -c "$DUMP_URL" || curl -C - -O "$DUMP_URL"
    else
        echo "  ✓ Already downloaded"
    fi
    
    # Extract mathematical entities (Q-numbers for math concepts)
    if [ ! -f "math-entities.nt" ]; then
        echo "  Extracting mathematical entities..."
        zcat latest-truthy.nt.gz | \
            grep -E "(Q167|Q82435|Q12916|Q395|Q11348|Q11352)" | \
            head -100000 > math-entities.nt
        echo "  ✓ Extracted $(wc -l < math-entities.nt) math triples"
    fi
    
    cd - > /dev/null
}

# ============================================================================
# OEIS - Full Database
# ============================================================================
download_oeis() {
    echo "📊 Downloading OEIS..."
    cd "$DATA_DIR/oeis"
    
    # Main sequence database
    if [ ! -f "stripped.gz" ]; then
        echo "  Downloading stripped database..."
        wget -c https://oeis.org/stripped.gz
    else
        echo "  ✓ Already downloaded"
    fi
    
    # Names database
    if [ ! -f "names.gz" ]; then
        echo "  Downloading names..."
        wget -c https://oeis.org/names.gz
    else
        echo "  ✓ Already downloaded"
    fi
    
    # Extract and index
    if [ ! -f "sequences.txt" ]; then
        echo "  Extracting sequences..."
        zcat stripped.gz > sequences.txt
        echo "  ✓ Extracted $(wc -l < sequences.txt) sequences"
    fi
    
    if [ ! -f "names.txt" ]; then
        echo "  Extracting names..."
        zcat names.gz > names.txt
        echo "  ✓ Extracted $(wc -l < names.txt) names"
    fi
    
    cd - > /dev/null
}

# ============================================================================
# LMFDB - Bulk Downloads
# ============================================================================
download_lmfdb() {
    echo "📊 Downloading LMFDB..."
    cd "$DATA_DIR/lmfdb"
    
    # Elliptic curves over Q (conductor <= 10000)
    if [ ! -f "elliptic_curves.json" ]; then
        echo "  Downloading elliptic curves..."
        curl -o elliptic_curves.json \
            'https://www.lmfdb.org/api/ec_curvedata/?conductor={$lte:10000}&_format=json'
    else
        echo "  ✓ Already downloaded"
    fi
    
    # Modular forms (level <= 100)
    if [ ! -f "modular_forms.json" ]; then
        echo "  Downloading modular forms..."
        curl -o modular_forms.json \
            'https://www.lmfdb.org/api/mf_newforms/?level={$lte:100}&_format=json'
    else
        echo "  ✓ Already downloaded"
    fi
    
    # L-functions (degree <= 4)
    if [ ! -f "lfunctions.json" ]; then
        echo "  Downloading L-functions..."
        curl -o lfunctions.json \
            'https://www.lmfdb.org/api/lfunc_instances/?degree={$lte:4}&_format=json'
    else
        echo "  ✓ Already downloaded"
    fi
    
    cd - > /dev/null
}

# ============================================================================
# Main execution
# ============================================================================

echo "Starting bulk downloads..."
echo ""

# Download in parallel
download_oeis &
OEIS_PID=$!

download_lmfdb &
LMFDB_PID=$!

# Wikidata is huge, do it separately
# download_wikidata &
# WIKIDATA_PID=$!

echo "⏳ Waiting for downloads to complete..."
wait $OEIS_PID
echo "✅ OEIS complete"

wait $LMFDB_PID
echo "✅ LMFDB complete"

# wait $WIKIDATA_PID
# echo "✅ Wikidata complete"

echo ""
echo "📊 DOWNLOAD SUMMARY"
echo ""
echo "OEIS:"
ls -lh "$DATA_DIR/oeis/"
echo ""
echo "LMFDB:"
ls -lh "$DATA_DIR/lmfdb/"
echo ""
# echo "Wikidata:"
# ls -lh "$DATA_DIR/wikidata/"

echo ""
echo "✅ All downloads complete!"
echo ""
echo "Next steps:"
echo "  1. Run: cargo run --release --bin index_math_databases"
echo "  2. This will index all data into local DB"
echo "  3. Then run numerical_codebreaker with full data"
