#!/usr/bin/env bash
# Migrate scattered data files to bucket submodules

set -euo pipefail

echo "🗂️  Migrating Data to Buckets"
echo "=============================="

# Markov Analysis Bucket
echo "📊 data-markov-analysis..."
mkdir -p data-markov-analysis/{matrices,scores,results}
[ -f markov_similarity_matrix.bin ] && mv markov_similarity_matrix.bin data-markov-analysis/matrices/
[ -f markov_similarity_matrix_meta.json ] && mv markov_similarity_matrix_meta.json data-markov-analysis/matrices/
[ -f markov_global_matrix.json ] && mv markov_global_matrix.json data-markov-analysis/matrices/
[ -f markov_file_index_mapping.json ] && mv markov_file_index_mapping.json data-markov-analysis/
mv markov_symbol_scores*.json data-markov-analysis/scores/ 2>/dev/null || true
[ -d markov_results ] && mv markov_results/* data-markov-analysis/results/ 2>/dev/null || true
[ -f markov_name_path_analysis.txt ] && mv markov_name_path_analysis.txt data-markov-analysis/
[ -f chunk_000_timing.txt ] && mv chunk_000_timing.txt data-markov-analysis/
[ -f failed_files_exclude.txt ] && mv failed_files_exclude.txt data-markov-analysis/

# Eigenvectors Bucket
echo "🔢 data-eigenvectors..."
mkdir -p data-eigenvectors/{dominant,symbol,term}
[ -f markov_dominant_eigenvector.txt ] && mv markov_dominant_eigenvector.txt data-eigenvectors/dominant/
[ -f symbol_eigenvector_results.txt ] && mv symbol_eigenvector_results.txt data-eigenvectors/symbol/
[ -f term_eigenvectors.txt ] && mv term_eigenvectors.txt data-eigenvectors/term/
[ -f eigenvector_label_mapping.txt ] && mv eigenvector_label_mapping.txt data-eigenvectors/
[ -f symbol_similarity_results.txt ] && mv symbol_similarity_results.txt data-eigenvectors/

# Moonshine Bucket
echo "🌙 data-moonshine..."
mkdir -p data-moonshine/{maps,codecs,fingerprints,orbits}
[ -f elf_moonshine_map.txt ] && mv elf_moonshine_map.txt data-moonshine/maps/
[ -f codec_binary_extraction.txt ] && mv codec_binary_extraction.txt data-moonshine/codecs/
[ -f binary_fingerprint_decoder.txt ] && mv binary_fingerprint_decoder.txt data-moonshine/fingerprints/
[ -f automorphic_orbit_lmfdb.txt ] && mv automorphic_orbit_lmfdb.txt data-moonshine/orbits/

# Blockchain Bucket
echo "⛓️  data-blockchain..."
mkdir -p data-blockchain/{contracts,blocks,nodes}
[ -d top_contracts ] && cp -r top_contracts/* data-blockchain/contracts/ 2>/dev/null || true
[ -d blockchain_blocks ] && cp -r blockchain_blocks/* data-blockchain/blocks/ 2>/dev/null || true

# Telemetry Bucket
echo "📡 data-telemetry..."
mkdir -p data-telemetry/{sessions,build_analysis,strace}
[ -d data/telemetry ] && cp -r data/telemetry/* data-telemetry/ 2>/dev/null || true
[ -d data/build_analysis ] && cp -r data/build_analysis/* data-telemetry/build_analysis/ 2>/dev/null || true

# Const71 Bucket
echo "🔢 data-const71..."
mkdir -p data-const71/{builds,binaries,perf,analysis}
[ -d const_71_analysis ] && cp -r const_71_analysis/* data-const71/analysis/ 2>/dev/null || true
[ -d const_equivalence_nix ] && cp -r const_equivalence_nix/* data-const71/analysis/ 2>/dev/null || true

echo ""
echo "✅ Migration complete!"
echo ""
echo "Bucket sizes:"
du -sh data-*/ 2>/dev/null || echo "No buckets created yet"
echo ""
echo "Next: Commit each bucket and push to GitHub/HuggingFace"
