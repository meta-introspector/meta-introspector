#!/bin/bash
# Quick project search helper

# Load centralized search utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/search_utils.sh"

PROJECTS=(
    "/home/mdupont/zos-qa"
    "/home/mdupont/zombie_driver2"
    "/home/mdupont/zos-server"
    "/home/mdupont/meta-introspector"
)

case "$1" in
    "oci"|"oracle")
        echo "🔍 Searching for Oracle/OCI code..."
        for proj in "${PROJECTS[@]}"; do
            echo "📁 $proj:"
            find_grep "$proj" "OciClient\|oracle\|oci_core" rs | head -3
        done
        ;;
    "axum"|"server")
        echo "🔍 Searching for Axum servers..."
        for proj in "${PROJECTS[@]}"; do
            echo "📁 $proj:"
            find_grep "$proj" "axum::Router\|#\[tokio::main\]" rs | head -3
        done
        ;;
    "deploy")
        echo "🔍 Searching for deployment scripts..."
        for proj in "${PROJECTS[@]}"; do
            echo "📁 $proj:"
            find_multi_ext "$proj" "deploy.sh" "deploy.rs" | head -3
        done
        ;;
    "terraform"|"tf")
        echo "🔍 Searching for Terraform configs..."
        for proj in "${PROJECTS[@]}"; do
            echo "📁 $proj:"
            find_multi_ext "$proj" tf tfvars | head -3
        done
        ;;
    "creds"|"keys")
        echo "🔍 Checking credentials..."
        echo "📁 ~/.solfunmeme-keys/:"
        ls -1 ~/.solfunmeme-keys/ 2>/dev/null
        echo ""
        echo "📁 ~/.ssh/:"
        ls -1 ~/.ssh/*.pub 2>/dev/null
        echo ""
        echo "📁 ~/.oci/:"
        ls -1 ~/.oci/ 2>/dev/null
        ;;
    "workspace")
        echo "🔍 Finding Cargo workspaces..."
        for proj in "${PROJECTS[@]}"; do
            if [ -f "$proj/Cargo.toml" ]; then
                if grep -q "\[workspace\]" "$proj/Cargo.toml" 2>/dev/null; then
                    echo "✅ $proj (workspace)"
                    grep_context "members = " "$proj/Cargo.toml" 20 | grep "\"" | head -10
                fi
            fi
        done
        ;;
    "index")
        cat ~/PROJECT_INDEX.md
        ;;
    *)
        echo "Quick Find - Project Search Helper"
        echo ""
        echo "Usage: $0 <command>"
        echo ""
        echo "Commands:"
        echo "  oci|oracle    - Find Oracle/OCI code"
        echo "  axum|server   - Find Axum servers"
        echo "  deploy        - Find deployment scripts"
        echo "  terraform|tf  - Find Terraform configs"
        echo "  creds|keys    - Check credentials"
        echo "  workspace     - Find Cargo workspaces"
        echo "  index         - Show project index"
        echo ""
        echo "Examples:"
        echo "  $0 oci"
        echo "  $0 deploy"
        echo "  $0 creds"
        ;;
esac
