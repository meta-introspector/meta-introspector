#!/bin/bash
# Setup canonical ZOS structure
set -euo pipefail

ROOT="$HOME/meta-introspector"

echo "🏗️  Setting up canonical ZOS structure"
echo "Root: $ROOT"
echo ""

# Create directory structure
echo "📁 Creating directories..."
mkdir -p "$ROOT"/{repos,envs/{dev,qa,prod,local/{systemd,docker,nix}},deployments/{oracle,huggingface,vercel,cloudflare,local/{laptop,phone,desktop},users},telemetry/{logs,metrics,traces,events},data,credentials,tools}

# Create symlinks for repos
echo "🔗 Linking repositories..."
cd "$ROOT/repos"
[ ! -e zos-qa ] && ln -s ~/zos-qa zos-qa && echo "  ✅ zos-qa"
[ ! -e zombie_driver2 ] && ln -s ~/zombie_driver2 zombie_driver2 && echo "  ✅ zombie_driver2"
[ ! -e zos-server ] && ln -s ~/zos-server zos-server && echo "  ✅ zos-server"

# Link credentials
echo "🔑 Linking credentials..."
cd "$ROOT/credentials"
[ ! -e oracle ] && ln -s ~/.solfunmeme-keys oracle && echo "  ✅ oracle"
[ ! -e ssh ] && ln -s ~/.ssh ssh && echo "  ✅ ssh"

# Create environment configs
echo "⚙️  Creating environment configs..."

# Dev environment
cat > "$ROOT/envs/dev/config.toml" <<EOF
[environment]
name = "dev"
tier = "development"
log_level = "debug"

[telemetry]
endpoint = "http://localhost:4317"
sample_rate = 1.0

[auth]
require_ssh_key = false
EOF

# QA environment
cat > "$ROOT/envs/qa/config.toml" <<EOF
[environment]
name = "qa"
tier = "testing"
log_level = "info"

[telemetry]
endpoint = "http://localhost:4317"
sample_rate = 0.5

[auth]
require_ssh_key = true
EOF

# Prod environment
cat > "$ROOT/envs/prod/config.toml" <<EOF
[environment]
name = "prod"
tier = "production"
log_level = "warn"

[telemetry]
endpoint = "http://telemetry.example.com:4317"
sample_rate = 0.1

[auth]
require_ssh_key = true
EOF

# Create node registry
echo "📋 Creating node registry..."
cat > "$ROOT/deployments/registry.json" <<EOF
{
  "version": "1.0",
  "updated_at": "$(date -Iseconds)",
  "nodes": []
}
EOF

# Create telemetry config
echo "📊 Creating telemetry config..."
cat > "$ROOT/telemetry/config/otel-collector.yaml" <<EOF
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318

processors:
  batch:
    timeout: 10s

exporters:
  logging:
    loglevel: debug
  file:
    path: $ROOT/telemetry/logs/otel.json

service:
  pipelines:
    traces:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging, file]
    metrics:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging, file]
    logs:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging, file]
EOF

# Create .gitignore
echo "🚫 Creating .gitignore..."
cat > "$ROOT/.gitignore" <<EOF
# Credentials
credentials/
*.pem
*.key
*.token
.env

# Telemetry data
telemetry/logs/
telemetry/metrics/
telemetry/traces/

# Build artifacts
target/
*.so
*.dylib

# State files
*.tfstate
*.tfstate.backup
deployment.json

# Logs
*.log
EOF

# Update project map
echo "🗺️  Updating project map..."
cat > "$ROOT/.project_map" <<EOF
# Project Map - Canonical Structure
# Root: ~/meta-introspector
# Format: category|repo_name|relative_path|description

# Repositories
repo_zos_qa|repos/zos-qa||Main deployment workspace
repo_zombie|repos/zombie_driver2||Analysis tools
repo_zos_server|repos/zos-server||Server collection

# Environments
env_dev|envs/dev|config.toml|Development environment
env_qa|envs/qa|config.toml|QA environment
env_prod|envs/prod|config.toml|Production environment

# Deployments
deploy_oracle|deployments/oracle||Oracle Cloud deployments
deploy_hf|deployments/huggingface||Hugging Face Spaces
deploy_vercel|deployments/vercel||Vercel deployments
deploy_cf|deployments/cloudflare||Cloudflare Workers
deploy_local|deployments/local||Local nodes

# Telemetry
telemetry_logs|telemetry/logs||Centralized logs
telemetry_metrics|telemetry/metrics||Metrics storage
telemetry_traces|telemetry/traces||Distributed traces
telemetry_config|telemetry/config|otel-collector.yaml|OpenTelemetry config

# Registry
node_registry|deployments|registry.json|Central node registry

# Credentials
creds_oracle|credentials/oracle||Oracle credentials
creds_ssh|credentials/ssh||SSH keys
EOF

echo ""
echo "✅ Canonical structure created!"
echo ""
echo "📁 Structure:"
echo "  $ROOT/repos/              - Git repositories"
echo "  $ROOT/envs/               - Environment configs (dev/qa/prod)"
echo "  $ROOT/deployments/        - Deployment targets"
echo "  $ROOT/telemetry/          - Centralized telemetry"
echo "  $ROOT/credentials/        - Credentials (gitignored)"
echo "  $ROOT/tools/              - Management tools"
echo ""
echo "📋 Next steps:"
echo "  1. Review: cat $ROOT/CANONICAL_STRUCTURE.md"
echo "  2. Deploy: cd $ROOT/repos/zos-qa && ./deploy-oracle-rust.sh"
echo "  3. Monitor: $ROOT/tools/health-checker.rs"
