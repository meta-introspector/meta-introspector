#!/usr/bin/env bash
set -euo pipefail

REPO_PATH="${1:-.}"
cd "$REPO_PATH"

echo "📊 Collecting git metadata for: $REPO_PATH"

# Basic git info
REMOTE=$(git remote get-url origin 2>/dev/null || echo "none")
BRANCH=$(git branch --show-current 2>/dev/null || echo "detached")
COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "none")
COMMIT_COUNT=$(git rev-list --count HEAD 2>/dev/null || echo "0")

# File statistics
TOTAL_FILES=$(git ls-files | wc -l)
RUST_FILES=$(git ls-files "*.rs" | wc -l)
NIX_FILES=$(git ls-files "*.nix" | wc -l)
TOML_FILES=$(git ls-files "*.toml" | wc -l)

# Language detection
PRIMARY_LANG="unknown"
if [ "$RUST_FILES" -gt 0 ]; then PRIMARY_LANG="rust"; fi
if [ "$NIX_FILES" -gt "$RUST_FILES" ]; then PRIMARY_LANG="nix"; fi

# Contributors
CONTRIBUTORS=$(git shortlog -sn --all | wc -l)

# Dates
FIRST_COMMIT=$(git log --reverse --format="%ai" | head -1 || echo "unknown")
LAST_COMMIT=$(git log -1 --format="%ai" || echo "unknown")

# Output TOML
cat > zos.toml <<EOF
[repo]
remote = "$REMOTE"
branch = "$BRANCH"
commit = "$COMMIT"
commit_count = $COMMIT_COUNT

[stats]
total_files = $TOTAL_FILES
rust_files = $RUST_FILES
nix_files = $NIX_FILES
toml_files = $TOML_FILES
primary_language = "$PRIMARY_LANG"
contributors = $CONTRIBUTORS

[timeline]
first_commit = "$FIRST_COMMIT"
last_commit = "$LAST_COMMIT"

[classification]
# Mathematical description of repo
file_entropy = 0.0  # To be computed
symbol_count = 0    # To be computed
markov_signature = ""  # To be computed
EOF

echo "✅ Created zos.toml"
