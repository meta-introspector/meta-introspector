#!/usr/bin/env bash
# Complete system bootstrap via central build system
# Runs all analysis on our codebase

set -euo pipefail

echo "🚀 Meta-Introspector Bootstrap"
echo "==============================="
echo ""
echo "Analysis Jobs:"
echo "  1. 001_keywords - Extract terms, emoji labels"
echo "  2. 002_primes - Prime arithmetization, Gödel numbers"
echo "  3. 003_harmonic_filter - Name/impl complexity harmony"
echo "  4. 004_markov_model - Markov chain harmonic prediction"
echo ""

cd "$(dirname "$0")/../.."

# Run all analysis jobs
echo "📊 Running analysis on codebase..."
echo ""

# Job 1: Keywords
echo "1️⃣  Keywords Analysis..."
nix build ./analysis/001_keywords --no-link 2>&1 | grep -E "Extracted|suspicious" || true
KEYWORDS_RESULT=$(nix-store -qR $(nix-store -qd ./analysis/001_keywords) | grep "001_keywords" | head -1)
echo "   Result: $KEYWORDS_RESULT"
echo ""

# Job 2: Primes
echo "2️⃣  Prime Arithmetization..."
nix build ./analysis/002_primes --no-link 2>&1 | grep -E "Assigned|primes" || true
PRIMES_RESULT=$(nix-store -qR $(nix-store -qd ./analysis/002_primes) | grep "002_primes" | head -1)
echo "   Result: $PRIMES_RESULT"
echo ""

# Job 3: Harmonic Filter
echo "3️⃣  Harmonic Filter..."
nix build ./analysis/003_harmonic_filter --no-link 2>&1 | grep -E "Analyzed|Mismatches" || true
HARMONIC_RESULT=$(nix-store -qR $(nix-store -qd ./analysis/003_harmonic_filter) | grep "003_harmonic" | head -1)
echo "   Result: $HARMONIC_RESULT"
echo ""

# Job 4: Markov Model
echo "4️⃣  Markov Model..."
nix build ./analysis/004_markov_model --no-link 2>&1 | grep -E "Collected|accuracy" || true
MARKOV_RESULT=$(nix-store -qR $(nix-store -qd ./analysis/004_markov_model) | grep "004_markov" | head -1)
echo "   Result: $MARKOV_RESULT"
echo ""

# Build central system
echo "🏗️  Building central system..."
nix build ./nix --print-build-logs

echo ""
echo "✅ Bootstrap complete!"
echo ""
echo "📊 Analysis Results:"
echo "  Keywords:        $KEYWORDS_RESULT"
echo "  Primes:          $PRIMES_RESULT"
echo "  Harmonic Filter: $HARMONIC_RESULT"
echo "  Markov Model:    $MARKOV_RESULT"
echo "  Central System:  result/"
echo ""
echo "Query results:"
echo "  cat $KEYWORDS_RESULT/analysis/all-terms.txt"
echo "  cat $PRIMES_RESULT/primes/term-to-prime.json"
echo "  cat $HARMONIC_RESULT/analysis/mismatches.json"
echo "  cat $MARKOV_RESULT/model/markov-transitions.json"
