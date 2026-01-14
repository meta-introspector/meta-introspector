#!/usr/bin/env bash
# Migrate scattered data files to canonical data/ structure

set -euo pipefail

echo "🗂️  Migrating to Canonical Data Structure"
echo "=========================================="

# Create directory structure
mkdir -p data/{markov_analysis/results,eigenvectors,moonshine,const_71_analysis/{build_logs,binaries,perf_traces},blockchain/{contracts,blocks},telemetry/{sessions,build_analysis,strace_logs},similarity}

# Markov analysis
echo "📊 Markov analysis..."
[ -f markov_similarity_matrix.bin ] && mv markov_similarity_matrix.bin data/markov_analysis/
[ -f markov_similarity_matrix_meta.json ] && mv markov_similarity_matrix_meta.json data/markov_analysis/
[ -f markov_global_matrix.json ] && mv markov_global_matrix.json data/markov_analysis/
[ -f markov_file_index_mapping.json ] && mv markov_file_index_mapping.json data/markov_analysis/
mv markov_symbol_scores*.json data/markov_analysis/ 2>/dev/null || true
[ -d markov_results ] && mv markov_results/* data/markov_analysis/results/ 2>/dev/null || true

# Eigenvectors
echo "🔢 Eigenvectors..."
[ -f markov_dominant_eigenvector.txt ] && mv markov_dominant_eigenvector.txt data/eigenvectors/
[ -f symbol_eigenvector_results.txt ] && mv symbol_eigenvector_results.txt data/eigenvectors/
[ -f term_eigenvectors.txt ] && mv term_eigenvectors.txt data/eigenvectors/
[ -f eigenvector_label_mapping.txt ] && mv eigenvector_label_mapping.txt data/eigenvectors/

# Moonshine
echo "🌙 Moonshine..."
[ -f elf_moonshine_map.txt ] && mv elf_moonshine_map.txt data/moonshine/
[ -f codec_binary_extraction.txt ] && mv codec_binary_extraction.txt data/moonshine/
[ -f binary_fingerprint_decoder.txt ] && mv binary_fingerprint_decoder.txt data/moonshine/
[ -f automorphic_orbit_lmfdb.txt ] && mv automorphic_orbit_lmfdb.txt data/moonshine/

# Const 71
echo "🔢 Const 71 analysis..."
[ -d const_71_analysis ] && cp -r const_71_analysis/* data/const_71_analysis/ 2>/dev/null || true
[ -d const_equivalence_nix ] && cp -r const_equivalence_nix/* data/const_71_analysis/ 2>/dev/null || true

# Blockchain
echo "⛓️  Blockchain data..."
[ -d top_contracts ] && cp top_contracts/*.json data/blockchain/contracts/ 2>/dev/null || true
[ -d blockchain_blocks ] && cp blockchain_blocks/*.json data/blockchain/blocks/ 2>/dev/null || true

# Telemetry
echo "📡 Telemetry..."
[ -d data/build_analysis ] && mv data/build_analysis/* data/telemetry/build_analysis/ 2>/dev/null || true

# Similarity
echo "🔍 Similarity..."
[ -f symbol_similarity_results.txt ] && mv symbol_similarity_results.txt data/similarity/

# Cleanup
echo "🧹 Cleanup..."
[ -f chunk_000_timing.txt ] && mv chunk_000_timing.txt data/markov_analysis/
[ -f test_chunk.txt ] && mv test_chunk.txt data/markov_analysis/
[ -f failed_files_exclude.txt ] && mv failed_files_exclude.txt data/markov_analysis/
[ -f markov_name_path_analysis.txt ] && mv markov_name_path_analysis.txt data/markov_analysis/

echo ""
echo "✅ Migration complete!"
echo ""
echo "Data structure:"
du -sh data/*/ 2>/dev/null || true
