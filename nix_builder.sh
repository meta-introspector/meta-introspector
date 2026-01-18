#!/bin/bash
# Lightweight local Nix builder - no Postgres, no Hydra
# Just builds all flakes continuously and caches results

set -e

BUILD_DIR="$HOME/.local/share/nix-builder"
CACHE_DIR="$BUILD_DIR/cache"
LOGS_DIR="$BUILD_DIR/logs"
QUEUE_FILE="$BUILD_DIR/queue.txt"

mkdir -p "$BUILD_DIR" "$CACHE_DIR" "$LOGS_DIR"

echo "🏗️  LOCAL NIX BUILDER (No Postgres)"
echo ""

# ============================================================================
# Build queue management
# ============================================================================
add_to_queue() {
    local repo_path=$1
    echo "$repo_path" >> "$QUEUE_FILE"
}

get_next_from_queue() {
    if [ -f "$QUEUE_FILE" ] && [ -s "$QUEUE_FILE" ]; then
        head -1 "$QUEUE_FILE"
        sed -i '1d' "$QUEUE_FILE"
    fi
}

# ============================================================================
# Build a single flake
# ============================================================================
build_flake() {
    local repo_path=$1
    local repo_name=$(basename "$repo_path")
    local log_file="$LOGS_DIR/${repo_name}_$(date +%Y%m%d_%H%M%S).log"
    
    echo "🔨 Building: $repo_name"
    echo "   Path: $repo_path"
    echo "   Log: $log_file"
    
    cd "$repo_path"
    
    if [ ! -f "flake.nix" ]; then
        echo "   ⚠️  No flake.nix, skipping"
        return
    fi
    
    {
        echo "=== Build started: $(date) ==="
        echo "Repo: $repo_path"
        echo ""
        
        # Build all flake outputs
        echo "Building default package..."
        nix build .#default --print-build-logs --keep-going || true
        
        echo ""
        echo "Running flake checks..."
        nix flake check --keep-going || true
        
        echo ""
        echo "=== Build finished: $(date) ==="
    } 2>&1 | tee "$log_file"
    
    # Cache build results
    if [ -d "result" ]; then
        local cache_path="$CACHE_DIR/$repo_name"
        mkdir -p "$cache_path"
        cp -rL result/* "$cache_path/" 2>/dev/null || true
        echo "   ✅ Cached to $cache_path"
    fi
    
    echo "   📝 Log saved to $log_file"
}

# ============================================================================
# Watch mode - continuously build from queue
# ============================================================================
watch_mode() {
    echo "👀 Watch mode: Building from queue continuously"
    echo "   Queue file: $QUEUE_FILE"
    echo "   Press Ctrl+C to stop"
    echo ""
    
    while true; do
        local next=$(get_next_from_queue)
        
        if [ -n "$next" ]; then
            build_flake "$next"
            echo ""
        else
            echo "⏸️  Queue empty, waiting 60s..."
            sleep 60
        fi
    done
}

# ============================================================================
# Build all registered repos once
# ============================================================================
build_all() {
    echo "🚀 Building all registered repos..."
    
    if [ -f "/mnt/data1/meta-introspector/data/repo_registry.json" ]; then
        jq -r '.repos[] | select(.path != null) | .path' \
            /mnt/data1/meta-introspector/data/repo_registry.json | \
        while read -r repo_path; do
            if [ -d "$repo_path" ] && [ -f "$repo_path/flake.nix" ]; then
                build_flake "$repo_path"
                echo ""
            fi
        done
    else
        echo "❌ No repo registry found"
        echo "Run: cd /mnt/data1/meta-introspector && ./local_cicd_runner.sh"
    fi
}

# ============================================================================
# Status report
# ============================================================================
show_status() {
    echo "📊 NIX BUILDER STATUS"
    echo ""
    echo "Build directory: $BUILD_DIR"
    echo "Cache directory: $CACHE_DIR"
    echo "Logs directory: $LOGS_DIR"
    echo ""
    echo "Cached builds: $(ls -1 "$CACHE_DIR" 2>/dev/null | wc -l)"
    echo "Build logs: $(ls -1 "$LOGS_DIR" 2>/dev/null | wc -l)"
    echo "Queue size: $(wc -l < "$QUEUE_FILE" 2>/dev/null || echo 0)"
    echo ""
    
    if [ -d "$CACHE_DIR" ]; then
        echo "Recent builds:"
        ls -lt "$CACHE_DIR" | head -10
    fi
}

# ============================================================================
# Systemd service generator
# ============================================================================
generate_service() {
    local service_file="$HOME/.config/systemd/user/nix-builder.service"
    mkdir -p "$(dirname "$service_file")"
    
    cat > "$service_file" << EOF
[Unit]
Description=Local Nix Builder (No Postgres)
After=network.target

[Service]
Type=simple
ExecStart=$PWD/nix_builder.sh watch
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
EOF
    
    echo "✅ Systemd service created: $service_file"
    echo ""
    echo "To enable:"
    echo "  systemctl --user daemon-reload"
    echo "  systemctl --user enable nix-builder"
    echo "  systemctl --user start nix-builder"
    echo ""
    echo "To check status:"
    echo "  systemctl --user status nix-builder"
    echo "  journalctl --user -u nix-builder -f"
}

# ============================================================================
# Main
# ============================================================================
case "${1:-}" in
    watch)
        watch_mode
        ;;
    build)
        if [ -n "$2" ]; then
            build_flake "$2"
        else
            build_all
        fi
        ;;
    queue)
        if [ -n "$2" ]; then
            add_to_queue "$2"
            echo "✅ Added to queue: $2"
        else
            echo "Usage: $0 queue <repo-path>"
        fi
        ;;
    status)
        show_status
        ;;
    service)
        generate_service
        ;;
    *)
        echo "Usage: $0 {watch|build|queue|status|service}"
        echo ""
        echo "Commands:"
        echo "  watch          - Continuously build from queue"
        echo "  build [path]   - Build one repo or all"
        echo "  queue <path>   - Add repo to build queue"
        echo "  status         - Show builder status"
        echo "  service        - Generate systemd service"
        echo ""
        echo "Examples:"
        echo "  $0 build /opt/zos-production"
        echo "  $0 queue /opt/zos-bootstrap"
        echo "  $0 watch"
        exit 1
        ;;
esac
