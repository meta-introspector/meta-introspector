#!/bin/bash
# Register all submodules from ~/nix/.gitmodules into git-sources
# Registers even uninitialized submodules (just records metadata)

GITMODULES="$HOME/nix/.gitmodules"
BASE_DIR="$HOME/nix"

echo "🔍 Extracting submodules from $GITMODULES"

# Parse .gitmodules and register each submodule
awk '
/^\[submodule/ { 
    if (path != "" && url != "") {
        print path "|" url
    }
    path = ""
    url = ""
}
/path = / { 
    sub(/.*path = /, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    path = $0
}
/url = / {
    sub(/.*url = /, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    url = $0
}
END {
    if (path != "" && url != "") {
        print path "|" url
    }
}
' "$GITMODULES" | while IFS='|' read -r path url; do
    full_path="$BASE_DIR/$path"
    name=$(basename "$path")
    
    # Register even if not initialized
    if [ -d "$full_path" ] || [ ! -e "$full_path" ]; then
        echo "Registering: $name ($url)"
        ./target/release/git-sources register "$name" "$full_path" 2>&1 | grep -E "Registered|Error" || true
    fi
done

echo ""
echo "✅ Registration complete"
