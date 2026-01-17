# 🚀 Quick Start Guide for New Users

## Step 1: Download the Minimal Server

```bash
# Download pre-built binary (easiest)
curl -L https://github.com/meta-introspector/meta-introspector/releases/latest/download/minimal-build-server-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Or build from source
git clone https://github.com/meta-introspector/meta-introspector
cd meta-introspector
cargo build --release --bin minimal-build-server
```

## Step 2: Start the Dev Server

```bash
# Run the server
./minimal-build-server

# Server starts on http://localhost:3000
```

## Step 3: Use Dev Server to Build QA

The dev server can build everything else for you!

```bash
# Ask dev server to build QA server
curl -X POST http://localhost:3000/build \
  -H "Content-Type: application/json" \
  -d '{
    "target": "qa-server",
    "action": "build"
  }'

# Or download pre-built QA binaries
curl -X POST http://localhost:3000/build \
  -H "Content-Type: application/json" \
  -d '{
    "target": "qa-server",
    "action": "download"
  }'
```

## Step 4: Deploy QA Server

```bash
# Dev server deploys QA for you
curl -X POST http://localhost:3000/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "target": "qa-server",
    "port": 3001
  }'

# QA server now running on http://localhost:3001
```

## Step 5: Build All Other Binaries (Optional)

```bash
# Use QA server to build all 220 binaries
curl -X POST http://localhost:3001/build \
  -H "Content-Type: application/json" \
  -d '{
    "target": "all-binaries",
    "action": "build"
  }'
```

## That's It! 🎉

You now have:
- ✅ Dev server (port 3000) - Your control panel
- ✅ QA server (port 3001) - Builds and tests
- ✅ All binaries available

## Web UI (Coming Soon)

Visit http://localhost:3000 in your browser for a simple UI to:
- Download binaries
- Start/stop servers
- View build status
- Deploy to cloud

## Need Help?

```bash
# Check server status
curl http://localhost:3000/health

# List available binaries
curl http://localhost:3000/binaries

# Get help
curl http://localhost:3000/help
```
