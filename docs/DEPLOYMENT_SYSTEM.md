# ZOS Multi-Environment Deployment System

Canonical structure for managing ZOS deployments across multiple platforms, environments, and users.

## Quick Start

```bash
# 1. Setup canonical structure
cd ~/meta-introspector
./setup-canonical.sh

# 2. Deploy to Oracle Cloud (prod)
./tools/deploy.sh deploy oracle node1 prod

# 3. Deploy locally (dev)
./tools/deploy.sh deploy local laptop dev

# 4. List all deployments
./tools/deploy.sh list

# 5. Check status
./tools/deploy.sh status oracle-node1

# 6. View logs
./tools/deploy.sh logs oracle-node1
```

## Structure

```
~/meta-introspector/                   # Root (canonical for all users)
├── repos/                             # Git repos (symlinked)
├── envs/                              # Environment configs (dev/qa/prod)
├── deployments/                       # All deployment targets
│   ├── oracle/                        # Oracle Cloud nodes
│   ├── huggingface/                   # HF Spaces
│   ├── vercel/                        # Vercel deployments
│   ├── cloudflare/                    # CF Workers
│   ├── local/                         # Local nodes (laptop/phone)
│   ├── users/                         # Other users' deployments
│   └── registry.json                  # Central node registry
├── telemetry/                         # Centralized telemetry
│   ├── logs/                          # Per-node logs
│   ├── metrics/                       # Metrics storage
│   └── traces/                        # Distributed tracing
├── credentials/                       # Credentials (gitignored)
└── tools/                             # Management tools
    └── deploy.sh                      # Universal deployer
```

## Deployment Targets

### Cloud Platforms
- **Oracle Cloud** - Always Free tier (4 ARM OCPUs, 24 GB RAM)
- **Hugging Face Spaces** - Free GPU/CPU spaces
- **Vercel** - Serverless functions
- **Cloudflare Workers** - Edge computing

### Local Nodes
- **Laptop** - Development node
- **Phone** - Mobile node (Termux)
- **Desktop** - Local server

### Multi-User
- Each user has own namespace under `deployments/users/{username}/`
- Separate credentials, telemetry, and node registry

## Environments

### Dev
- Local development
- Debug logging
- No SSH auth required
- Sample rate: 100%

### QA
- Testing environment
- Info logging
- SSH auth required
- Sample rate: 50%

### Prod
- Production deployments
- Warn logging
- SSH auth required
- Sample rate: 10%

## Telemetry

All nodes send telemetry to centralized location:

```
telemetry/
├── logs/{platform}-{node}/           # Application logs
├── metrics/{platform}-{node}/        # Prometheus metrics
└── traces/{platform}-{node}/         # Distributed traces
```

### OpenTelemetry Integration
- OTLP receiver on port 4317 (gRPC) and 4318 (HTTP)
- Automatic log/metric/trace collection
- Exporters to file, Prometheus, Jaeger

## Node Registry

Central registry tracks all nodes:

```json
{
  "nodes": [
    {
      "id": "oracle-node1-20260115",
      "platform": "oracle",
      "node_name": "node1",
      "environment": "prod",
      "owner": "mdupont",
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
  ]
}
```

## Commands

### Deploy
```bash
# Deploy to Oracle Cloud (prod)
./tools/deploy.sh deploy oracle node1 prod

# Deploy to Hugging Face (qa)
./tools/deploy.sh deploy huggingface space1 qa

# Deploy locally (dev)
./tools/deploy.sh deploy local laptop dev
```

### Manage
```bash
# List all nodes
./tools/deploy.sh list

# List Oracle nodes only
./tools/deploy.sh list oracle

# Check node status
./tools/deploy.sh status oracle-node1

# View logs
./tools/deploy.sh logs oracle-node1

# Update node
./tools/deploy.sh update oracle-node1

# Destroy node
./tools/deploy.sh destroy oracle-node1
```

### Register External Node
```bash
# Register a node deployed elsewhere
./tools/deploy.sh register my-node-id metadata.json
```

## Adding New Platforms

1. Create platform directory: `deployments/{platform}/`
2. Add deployment function in `tools/deploy.sh`
3. Create platform-specific templates
4. Update documentation

## Adding New Users

1. Create user directory: `deployments/users/{username}/`
2. User deploys with: `./tools/deploy.sh deploy oracle {username}-node1 prod`
3. Telemetry automatically namespaced
4. Separate credentials in `deployments/users/{username}/credentials/`

## Search & Discovery

```bash
# Quick find tool
./quick-find.sh oci          # Find OCI code
./quick-find.sh deploy       # Find deployment scripts
./quick-find.sh creds        # Check credentials

# Project index
cat PROJECT_INDEX.md         # Human-readable
cat .project_map             # Machine-readable
```

## Files

- `CANONICAL_STRUCTURE.md` - Detailed structure documentation
- `PROJECT_INDEX.md` - Project index and search guide
- `setup-canonical.sh` - Setup script
- `tools/deploy.sh` - Universal deployment manager
- `quick-find.sh` - Quick search helper
- `.project_map` - Machine-readable index

## Next Steps

1. ✅ Setup canonical structure
2. ✅ Document everything
3. 🔄 Deploy to Oracle Cloud
4. ⏳ Deploy to Hugging Face
5. ⏳ Deploy to Vercel
6. ⏳ Deploy to Cloudflare
7. ⏳ Setup telemetry aggregation
8. ⏳ Add monitoring dashboard
9. ⏳ Multi-user support
10. ⏳ CI/CD pipeline
