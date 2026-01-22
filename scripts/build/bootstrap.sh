#!/usr/bin/env bash
# Complete system bootstrap via central build system
# Runs all analysis on our codebase

set -euo pipefail

# Absolute paths for nix commands (works with sudo -E)
NIX="${NIX:-$(command -v nix)}"
NIX_STORE="${NIX_STORE:-$(command -v nix-store)}"
NIX_BUILD="${NIX_BUILD:-$(command -v nix-build)}"

echo "🚀 Meta-Introspector Bootstrap"
echo "==============================="
echo ""

# Setup zos user if needed (requires sudo with preserved PATH)
if ! command -v nix-as-zos &> /dev/null; then
    echo "⚙️  Setting up zos user (requires sudo)..."
    
    if [ "$EUID" -ne 0 ]; then
        echo "❌ Please run with: sudo -E ./bootstrap"
        echo "   (The -E preserves PATH for nix)"
        exit 1
    fi
    
    # Verify nix is available
    if ! command -v nix &> /dev/null; then
        echo "❌ nix not found in PATH"
        echo "   Run: sudo -E ./bootstrap"
        exit 1
    fi
    
    # Run setup script
    ./scripts/build/setup-zos-user.sh
    
    # Configure git for zos user
    ./scripts/build/configure-zos-git.sh
    
    echo "✅ ZOS user configured"
    echo ""
fi

# Ensure zos git is configured (even if nix-as-zos exists)
if [ "$EUID" -eq 0 ]; then
    zos_git_count=$(sudo -u zos git config --global --get-regexp 'url\.' 2>/dev/null | wc -l)
    if [ "$zos_git_count" -lt 10 ]; then
        echo "⚙️  Configuring zos git cache..."
        ./scripts/build/configure-zos-git.sh
        echo ""
    fi
fi

echo "Analysis Jobs:"
echo "  1. 001_keywords - Extract terms, emoji labels"
echo "  2. 002_primes - Prime arithmetization, Gödel numbers"
echo "  3. 003_harmonic_filter - Name/impl complexity harmony"
echo "  4. 004_markov_model - Markov chain harmonic prediction"
echo "  5. 005_meta_analysis - Apply 4 tools to 236 executables"
echo ""

cd "$(dirname "$0")/../.."

# Run all analysis jobs
echo "📊 Running analysis on codebase..."
echo ""

# Job 1: Keywords
echo "1️⃣  Keywords Analysis..."
$NIX build ./analysis/001_keywords --no-link 2>&1 | grep -E "Extracted|suspicious" || true
KEYWORDS_RESULT=$($NIX_STORE -qR $($NIX_STORE -qd ./analysis/001_keywords) | grep "001_keywords" | head -1)
echo "   Result: $KEYWORDS_RESULT"
echo ""

# Job 2: Primes
echo "2️⃣  Prime Arithmetization..."
$NIX build ./analysis/002_primes --no-link 2>&1 | grep -E "Assigned|primes" || true
PRIMES_RESULT=$($NIX_STORE -qR $($NIX_STORE -qd ./analysis/002_primes) | grep "002_primes" | head -1)
echo "   Result: $PRIMES_RESULT"
echo ""

# Job 3: Harmonic Filter
echo "3️⃣  Harmonic Filter..."
$NIX build ./analysis/003_harmonic_filter --no-link 2>&1 | grep -E "Analyzed|Mismatches" || true
HARMONIC_RESULT=$($NIX_STORE -qR $($NIX_STORE -qd ./analysis/003_harmonic_filter) | grep "003_harmonic" | head -1)
echo "   Result: $HARMONIC_RESULT"
echo ""

# Job 4: Markov Model
echo "4️⃣  Markov Model..."
$NIX build ./analysis/004_markov_model --no-link 2>&1 | grep -E "Collected|accuracy" || true
MARKOV_RESULT=$($NIX_STORE -qR $($NIX_STORE -qd ./analysis/004_markov_model) | grep "004_markov" | head -1)
echo "   Result: $MARKOV_RESULT"
echo ""

# Job 5: Meta-Analysis (apply 4 tools to 236 executables)
echo "5️⃣  Meta-Analysis (236 executables)..."
$NIX build ./analysis/005_meta_analysis --no-link 2>&1 | grep -E "Found|Analyzed|generated" || true
META_RESULT=$($NIX_STORE -qR $($NIX_STORE -qd ./analysis/005_meta_analysis) | grep "005_meta" | head -1)
echo "   Result: $META_RESULT"
echo ""

# Build central system
echo "🏗️  Building central system..."
$NIX build ./nix --print-build-logs

echo ""
echo "✅ Bootstrap complete!"
echo ""
echo "📊 Analysis Results:"
echo "  Keywords:        $KEYWORDS_RESULT"
echo "  Primes:          $PRIMES_RESULT"
echo "  Harmonic Filter: $HARMONIC_RESULT"
echo "  Markov Model:    $MARKOV_RESULT"
echo "  Meta-Analysis:   $META_RESULT"
echo "  Central System:  result/"
echo ""
echo "Query results:"
echo "  cat $KEYWORDS_RESULT/analysis/all-terms.txt"
echo "  cat $PRIMES_RESULT/primes/term-to-prime.json"
echo "  cat $HARMONIC_RESULT/analysis/mismatches.json"
echo "  cat $MARKOV_RESULT/model/markov-transitions.json"
echo "  cat $META_RESULT/reports/conversion-plan.txt"
