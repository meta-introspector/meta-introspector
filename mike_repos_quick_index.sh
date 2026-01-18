#!/bin/bash
echo "🔍 MIKE'S REPOS QUICK INDEX"
echo ""

REPOS=(
    "/opt/zos-production"
    "/opt/zos-bootstrap"
    "/home/mdupont/zos-qa"
)

for repo in "${REPOS[@]}"; do
    if [ -d "$repo" ]; then
        echo "## $repo"
        echo "Rust files: $(find "$repo" -name "*.rs" 2>/dev/null | wc -l)"
        echo "Cargo.toml: $(find "$repo" -name "Cargo.toml" 2>/dev/null | wc -l)"
        echo "Nix files: $(find "$repo" -name "*.nix" 2>/dev/null | wc -l)"
        echo "Lines of Rust: $(find "$repo" -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')"
        echo ""
    fi
done
