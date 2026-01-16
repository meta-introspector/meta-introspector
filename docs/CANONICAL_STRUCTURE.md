# ZOS Canonical Directory Structure

## Root Structure
```
~/meta-introspector/                    # Root monorepo
├── .project_map                        # Machine-readable index
├── PROJECT_INDEX.md                    # Human-readable index
├── quick-find.sh                       # Search helper
│
├── repos/                              # Git submodules/repos
│   ├── zos-qa/                        # Main deployment workspace
│   ├── zombie_driver2/                # Analysis tools
│   ├── zos-server/                    # Server collection
│   └── rust-build/                    # Rust compiler submodule
│
├── envs/                              # Environment configs
│   ├── dev/                           # Development
│   │   ├── config.toml
│   │   ├── .env
│   │   └── docker-compose.yml
│   ├── qa/                            # QA/Testing
│   │   ├── config.toml
│   │   └── .env
│   ├── prod/                          # Production
│   │   ├── config.toml
│   │   └── .env
│   └── local/                         # Local testing
│       ├── systemd/                   # Systemd services
│       ├── docker/                    # Docker configs
│       └── nix/                       # Nix configs
│
├── deployments/                       # Deployment targets
│   ├── oracle/                        # Oracle Cloud
│   │   ├── node1/
│   │   ├── node2/
│   │   └── terraform/
│   ├── huggingface/                   # HF Spaces
│   │   ├── space1/
│   │   └── space2/
│   ├── vercel/                        # Vercel
│   │   └── api/
│   ├── cloudflare/                    # Cloudflare Workers
│   │   └── workers/
│   ├── local/                         # Local nodes
│   │   ├── laptop/
│   │   ├── phone/
│   │   └── desktop/
│   └── users/                         # Other users
│       ├── user1/
│       └── user2/
│
├── telemetry/                         # Centralized telemetry
│   ├── logs/                          # Log aggregation
│   │   ├── oracle-node1/
│   │   ├── hf-space1/
│   │   ├── laptop/
│   │   └── phone/
│   ├── metrics/                       # Metrics storage
│   │   ├── prometheus/
│   │   └── grafana/
│   ├── traces/                        # Distributed tracing
│   │   └── jaeger/
│   └── events/                        # Event stream
│       └── kafka/
│
├── data/                              # Data storage
│   ├── build_analysis/                # Build analysis results
│   ├── 71_flakes_perf/               # Perf data
│   ├── compressed_declarations/       # Compressed decls
│   └── shared/                        # Shared datasets
│
├── credentials/                       # Credentials (gitignored)
│   ├── oracle/
│   │   ├── oci_config
│   │   └── oci_private_key.pem
│   ├── huggingface/
│   │   └── hf_token
│   ├── vercel/
│   │   └── vercel_token
│   ├── cloudflare/
│   │   └── cf_token
│   └── ssh/
│       ├── id_rsa
│       └── id_rsa.pub
│
└── tools/                             # Management tools
    ├── deploy.sh                      # Universal deployer
    ├── telemetry-collector.rs         # Telemetry aggregator
    ├── log-aggregator.rs              # Log collector
    └── health-checker.rs              # Health monitoring
```

## Environment Structure

Each environment follows this pattern:

```
envs/{env_name}/
├── config.toml                        # Environment config
├── .env                               # Environment variables
├── nodes.json                         # Node registry
└── deployments.json                   # Active deployments
```

### config.toml
```toml
[environment]
name = "dev"
tier = "development"
log_level = "debug"

[telemetry]
endpoint = "http://localhost:4317"
sample_rate = 1.0

[auth]
require_ssh_key = false
allowed_keys = []
```

## Deployment Structure

Each deployment follows this pattern:

```
deployments/{platform}/{node_name}/
├── metadata.json                      # Deployment metadata
├── config/                            # Node-specific config
│   ├── zos-config.toml
│   └── .env
├── logs/                              # Local log cache
│   └── {date}.log
├── state/                             # Deployment state
│   ├── terraform.tfstate
│   └── deployment.json
└── scripts/                           # Deployment scripts
    ├── deploy.sh
    ├── update.sh
    └── destroy.sh
```

### metadata.json
```json
{
  "deployment_id": "oracle-node1-20260115",
  "platform": "oracle",
  "node_name": "node1",
  "environment": "prod",
  "owner": "mdupont",
  "created_at": "2026-01-15T10:00:00Z",
  "status": "active",
  "endpoints": {
    "http": "http://node1.example.com:8080",
    "libp2p": "/ip4/1.2.3.4/tcp/4001"
  },
  "telemetry": {
    "logs": "telemetry/logs/oracle-node1/",
    "metrics_endpoint": "http://node1.example.com:9090"
  }
}
```

## Node Registry

Central registry of all nodes:

```
deployments/registry.json
```

```json
{
  "nodes": [
    {
      "id": "oracle-node1",
      "platform": "oracle",
      "environment": "prod",
      "owner": "mdupont",
      "status": "active",
      "endpoints": {...},
      "telemetry": {...}
    },
    {
      "id": "laptop-local",
      "platform": "local",
      "environment": "dev",
      "owner": "mdupont",
      "status": "active",
      "endpoints": {...},
      "telemetry": {...}
    }
  ]
}
```

## Telemetry Structure

```
telemetry/
├── config/
│   ├── otel-collector.yaml            # OpenTelemetry config
│   └── prometheus.yml                 # Prometheus config
├── logs/
│   ├── {platform}-{node}/
│   │   └── {date}/
│   │       ├── application.log
│   │       ├── system.log
│   │       └── audit.log
├── metrics/
│   └── {platform}-{node}/
│       └── {date}.prom
└── traces/
    └── {platform}-{node}/
        └── {date}.json
```

## User Structure

For multi-user deployments:

```
deployments/users/{username}/
├── profile.json                       # User profile
├── nodes/                             # User's nodes
│   ├── oracle-user1-node1/
│   └── hf-user1-space1/
├── credentials/                       # User credentials
│   └── {platform}/
└── telemetry/                         # User telemetry
    └── logs/
```

## Symlink Strategy

```bash
# In ~/meta-introspector/repos/
ln -s ~/zos-qa zos-qa
ln -s ~/zombie_driver2 zombie_driver2
ln -s ~/zos-server zos-server

# Credentials
ln -s ~/.solfunmeme-keys credentials/oracle
ln -s ~/.ssh credentials/ssh
```

## Migration Commands

```bash
# Create structure
mkdir -p ~/meta-introspector/{repos,envs/{dev,qa,prod,local},deployments/{oracle,huggingface,vercel,cloudflare,local,users},telemetry/{logs,metrics,traces,events},data,credentials,tools}

# Move existing repos
cd ~/meta-introspector/repos
ln -s ~/zos-qa zos-qa
ln -s ~/zombie_driver2 zombie_driver2
ln -s ~/zos-server zos-server

# Link credentials
cd ~/meta-introspector/credentials
ln -s ~/.solfunmeme-keys oracle
ln -s ~/.ssh ssh
```

## Benefits

1. **Canonical Structure** - Same layout for all users
2. **Multi-Environment** - dev/qa/prod separation
3. **Multi-Platform** - Oracle, HF, Vercel, Cloudflare, local
4. **Multi-User** - Each user has own namespace
5. **Centralized Telemetry** - All logs/metrics in one place
6. **Easy Discovery** - Standard paths for everything
7. **Scalable** - Add new platforms/users easily
