#!/usr/bin/env bash
# Create data bucket submodules for independent versioning and HuggingFace publishing

set -euo pipefail

echo "🪣 Creating Data Bucket Submodules"
echo "===================================="

# Define buckets
BUCKETS=(
  "data-markov-analysis:Markov resonance analysis results"
  "data-eigenvectors:Eigenvector computations and similarity matrices"
  "data-moonshine:ELF Moonshine and codec detection"
  "data-blockchain:Blockchain contracts, blocks, and economic weights"
  "data-telemetry:Build telemetry and strace captures"
  "data-const71:Cross-language const x=71 equivalence proofs"
)

# Create bucket directories
for bucket_info in "${BUCKETS[@]}"; do
  bucket=$(echo "$bucket_info" | cut -d: -f1)
  desc=$(echo "$bucket_info" | cut -d: -f2)
  
  echo ""
  echo "📦 Creating $bucket"
  echo "   $desc"
  
  # Create directory
  mkdir -p "$bucket"
  
  # Initialize git repo
  cd "$bucket"
  git init
  
  # Create README
  cat > README.md << EOF
# $bucket

$desc

## Structure

This is a data bucket for the meta-introspector project.

- **Parent Project**: https://github.com/meta-introspector/meta-introspector
- **HuggingFace**: https://huggingface.co/datasets/meta-introspector/${bucket}

## Usage

\`\`\`bash
# Clone as submodule
git submodule add https://github.com/meta-introspector/${bucket}.git

# Or standalone
git clone https://github.com/meta-introspector/${bucket}.git
\`\`\`

## Data Format

See parent project documentation for data schemas and formats.

## License

Same as parent project.
EOF
  
  # Create .gitignore
  cat > .gitignore << EOF
# Large binary files (>100MB)
*.bin

# Temporary files
*.tmp
*.log

# Keep structure
!README.md
!.gitignore
EOF
  
  # Initial commit
  git add README.md .gitignore
  git commit -m "Initial commit: $bucket

$desc

Created as data bucket for meta-introspector project.
Will be published to HuggingFace datasets."
  
  cd ..
  
  echo "   ✅ Initialized $bucket/"
done

echo ""
echo "===================================="
echo "✅ Created ${#BUCKETS[@]} data buckets"
echo ""
echo "Next steps:"
echo "1. Create GitHub repos for each bucket"
echo "2. Add as submodules: git submodule add <url> <bucket>"
echo "3. Migrate data: bash migrate_data_to_buckets.sh"
echo "4. Push to HuggingFace datasets"
