#!/bin/bash
# Universal ZOS Deployment Manager
set -euo pipefail

ROOT="$HOME/meta-introspector"
REGISTRY="$ROOT/deployments/registry.json"

usage() {
    cat <<EOF
ZOS Universal Deployment Manager

Usage: $0 <command> [options]

Commands:
  deploy <platform> <name> <env>  - Deploy new node
  list [platform]                 - List deployments
  status <node_id>                - Check node status
  logs <node_id>                  - View node logs
  update <node_id>                - Update node
  destroy <node_id>               - Destroy node
  register <node_id> <metadata>   - Register external node
  telemetry <node_id>             - View telemetry

Platforms:
  oracle, huggingface, vercel, cloudflare, local

Environments:
  dev, qa, prod

Examples:
  $0 deploy oracle node1 prod
  $0 list oracle
  $0 status oracle-node1
  $0 logs oracle-node1
EOF
}

deploy_node() {
    local platform=$1
    local name=$2
    local env=$3
    local node_id="${platform}-${name}"
    local deploy_dir="$ROOT/deployments/${platform}/${name}"
    
    echo "🚀 Deploying $node_id to $env environment"
    
    # Create deployment directory
    mkdir -p "$deploy_dir"/{config,logs,state,scripts}
    
    # Create metadata
    cat > "$deploy_dir/metadata.json" <<EOF
{
  "deployment_id": "${node_id}-$(date +%Y%m%d%H%M%S)",
  "platform": "$platform",
  "node_name": "$name",
  "environment": "$env",
  "owner": "$USER",
  "created_at": "$(date -Iseconds)",
  "status": "deploying",
  "endpoints": {},
  "telemetry": {
    "logs": "telemetry/logs/${node_id}/",
    "metrics_endpoint": ""
  }
}
EOF
    
    # Platform-specific deployment
    case $platform in
        oracle)
            deploy_oracle "$name" "$env" "$deploy_dir"
            ;;
        huggingface)
            deploy_huggingface "$name" "$env" "$deploy_dir"
            ;;
        vercel)
            deploy_vercel "$name" "$env" "$deploy_dir"
            ;;
        cloudflare)
            deploy_cloudflare "$name" "$env" "$deploy_dir"
            ;;
        local)
            deploy_local "$name" "$env" "$deploy_dir"
            ;;
        *)
            echo "❌ Unknown platform: $platform"
            exit 1
            ;;
    esac
    
    # Register node
    register_node "$node_id" "$deploy_dir/metadata.json"
    
    echo "✅ Deployment complete: $node_id"
}

deploy_oracle() {
    local name=$1
    local env=$2
    local deploy_dir=$3
    
    echo "📦 Deploying to Oracle Cloud..."
    cd "$ROOT/repos/zos-qa"
    ./deploy-oracle-rust.sh
    
    # Update metadata with outputs
    # TODO: Parse terraform outputs
}

deploy_huggingface() {
    local name=$1
    local env=$2
    local deploy_dir=$3
    
    echo "📦 Deploying to Hugging Face Spaces..."
    # TODO: Implement HF deployment
}

deploy_vercel() {
    local name=$1
    local env=$2
    local deploy_dir=$3
    
    echo "📦 Deploying to Vercel..."
    # TODO: Implement Vercel deployment
}

deploy_cloudflare() {
    local name=$1
    local env=$2
    local deploy_dir=$3
    
    echo "📦 Deploying to Cloudflare Workers..."
    # TODO: Implement CF deployment
}

deploy_local() {
    local name=$1
    local env=$2
    local deploy_dir=$3
    
    echo "📦 Deploying locally..."
    
    # Create systemd service
    cat > "$deploy_dir/scripts/zos-${name}.service" <<EOF
[Unit]
Description=ZOS Server - $name
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$ROOT/repos/zos-qa
Environment="ZOS_ENV=$env"
Environment="ZOS_NODE_ID=local-$name"
ExecStart=$ROOT/repos/zos-qa/target/release/zos_server
Restart=always

[Install]
WantedBy=multi-user.target
EOF
    
    echo "  Created systemd service"
    echo "  Install: sudo cp $deploy_dir/scripts/zos-${name}.service /etc/systemd/system/"
    echo "  Enable: sudo systemctl enable zos-${name}"
    echo "  Start: sudo systemctl start zos-${name}"
}

list_nodes() {
    local platform=${1:-}
    
    if [ ! -f "$REGISTRY" ]; then
        echo "No nodes registered"
        return
    fi
    
    echo "📋 Registered Nodes:"
    echo ""
    
    if [ -z "$platform" ]; then
        jq -r '.nodes[] | "\(.id)\t\(.platform)\t\(.environment)\t\(.status)"' "$REGISTRY" | column -t
    else
        jq -r ".nodes[] | select(.platform==\"$platform\") | \"\(.id)\t\(.platform)\t\(.environment)\t\(.status)\"" "$REGISTRY" | column -t
    fi
}

node_status() {
    local node_id=$1
    local metadata=$(jq -r ".nodes[] | select(.id==\"$node_id\")" "$REGISTRY")
    
    if [ -z "$metadata" ]; then
        echo "❌ Node not found: $node_id"
        exit 1
    fi
    
    echo "📊 Status: $node_id"
    echo "$metadata" | jq .
}

node_logs() {
    local node_id=$1
    local log_dir="$ROOT/telemetry/logs/$node_id"
    
    if [ ! -d "$log_dir" ]; then
        echo "❌ No logs found for: $node_id"
        exit 1
    fi
    
    echo "📜 Logs: $node_id"
    tail -f "$log_dir"/*.log
}

register_node() {
    local node_id=$1
    local metadata_file=$2
    
    if [ ! -f "$REGISTRY" ]; then
        echo '{"version":"1.0","nodes":[]}' > "$REGISTRY"
    fi
    
    local metadata=$(cat "$metadata_file")
    
    # Add to registry
    jq ".nodes += [$(echo "$metadata" | jq '{id: .deployment_id, platform, node_name, environment, owner, status, endpoints, telemetry}')]" "$REGISTRY" > "$REGISTRY.tmp"
    mv "$REGISTRY.tmp" "$REGISTRY"
    
    echo "✅ Registered: $node_id"
}

# Main
case "${1:-}" in
    deploy)
        [ $# -lt 4 ] && usage && exit 1
        deploy_node "$2" "$3" "$4"
        ;;
    list)
        list_nodes "${2:-}"
        ;;
    status)
        [ $# -lt 2 ] && usage && exit 1
        node_status "$2"
        ;;
    logs)
        [ $# -lt 2 ] && usage && exit 1
        node_logs "$2"
        ;;
    register)
        [ $# -lt 3 ] && usage && exit 1
        register_node "$2" "$3"
        ;;
    *)
        usage
        exit 1
        ;;
esac
