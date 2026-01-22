#!/bin/bash
# Categorize root markdown files by reading their content

cd /mnt/data1/meta-introspector

# Create category directories
mkdir -p docs/{71,theory,build,policy,audit,security,blockchain,transformer,monster}

echo "📚 Categorizing markdown files..."
echo ""

# Function to categorize a file
categorize() {
    local file="$1"
    local content=$(head -30 "$file" 2>/dev/null)
    
    # 71 and harmonics
    if echo "$file" | grep -qiE "^71_|harmonic|galois"; then
        echo "docs/71/$file"
    
    # Nix and build
    elif echo "$file" | grep -qiE "^nix_|^build_|bootstrap"; then
        echo "docs/build/$file"
    
    # Monster theory
    elif echo "$file" | grep -qiE "monster|muses|conformal"; then
        echo "docs/theory/$file"
    
    # Blockchain
    elif echo "$file" | grep -qiE "blockchain|solana|ethereum"; then
        echo "docs/blockchain/$file"
    
    # Transformer/ML
    elif echo "$file" | grep -qiE "transformer|training|model"; then
        echo "docs/transformer/$file"
    
    # Policy
    elif echo "$file" | grep -qiE "policy|anti_"; then
        echo "docs/policy/$file"
    
    # Audit/Report
    elif echo "$file" | grep -qiE "audit|report|clippy"; then
        echo "docs/audit/$file"
    
    # Security
    elif echo "$file" | grep -qiE "security"; then
        echo "docs/security/$file"
    
    # Theory (math)
    elif echo "$file" | grep -qiE "homotopy|orbit|periodicity|automorphism"; then
        echo "docs/theory/$file"
    
    # Check content for keywords
    elif echo "$content" | grep -qiE "71.*proof|71.*constant"; then
        echo "docs/71/$file"
    
    elif echo "$content" | grep -qiE "nix.*build|derivation|flake"; then
        echo "docs/build/$file"
    
    elif echo "$content" | grep -qiE "monster|galois.*group"; then
        echo "docs/theory/$file"
    
    else
        echo "docs/misc/$file"
    fi
}

# Process each markdown file
for file in *.md; do
    [ -f "$file" ] || continue
    
    # Skip README.md
    [ "$file" = "README.md" ] && continue
    
    target=$(categorize "$file")
    echo "  $file → $target"
done

echo ""
echo "Run './organize_docs.sh apply' to move files"
