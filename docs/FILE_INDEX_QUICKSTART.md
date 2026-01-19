# File Index Service - Quick Start

## 🚀 Get Started in 5 Minutes

### Step 1: Build the Service

```bash
# Build the server and CLI
cargo build --release --bin file-index-server
cargo build --release --bin file-index

# Or build both at once
cargo build --release --bins
```

### Step 2: Start the Server

```bash
# Start in foreground (for testing)
./target/release/file-index-server

# Or start in background
./target/release/file-index-server &

# Check it's running
curl http://localhost:3030/health
```

You should see:
```
🚀 Starting File Index Server...
📁 Scanning filesystem...
  Scanning: "."
    Indexed 10000 files...
    Indexed 20000 files...
✅ Indexed 25432 files in 2.34s
🔮 Pre-fetching predicted queries:
✅ Server listening on http://127.0.0.1:3030
```

### Step 3: Test the CLI

```bash
# Find all Rust files
./target/release/file-index query ext rs | head -10

# Find Cargo.toml files
./target/release/file-index query name Cargo.toml

# Find files matching pattern
./target/release/file-index query pattern "src/main"

# Get statistics
./target/release/file-index stats
```

### Step 4: Use in Shell Scripts

```bash
# Source the utilities
source lib/search_utils_v2.sh

# Check service status
service_status

# Use the functions
find_rust_files . | head -10
find_flakes .
find_workspaces .

# Get intelligent results
find_priority 20        # Top 20 priority files
predict_queries         # See predictions
index_stats            # View statistics
```

## 📊 Compare Performance

### Traditional find
```bash
time find . -name "*.rs" | wc -l
# real    0m2.456s
```

### File Index Service
```bash
time ./target/release/file-index query ext rs | wc -l
# real    0m0.012s  (200x faster!)
```

## 🎯 Common Use Cases

### 1. Find Files by Extension

```bash
# Shell
find_rust_files .
find_by_ext . toml

# CLI
file-index query ext rs
file-index query ext toml

# HTTP
curl http://localhost:3030/query/ext/rs | jq '.data[].path'
```

### 2. Find Specific Files

```bash
# Shell
find_flakes .
find_workspaces .

# CLI
file-index query name flake.nix
file-index query name Cargo.toml

# HTTP
curl http://localhost:3030/query/name/flake.nix
```

### 3. Pattern Matching

```bash
# Shell
find_grep . "tokio::main" rs

# CLI
file-index query pattern "src/main"

# HTTP
curl "http://localhost:3030/query/pattern?q=src/main"
```

### 4. Intelligent Queries

```bash
# Get top priority files (most likely to be accessed)
file-index priority --limit 100

# See predicted queries
file-index predict

# Get statistics
file-index stats
```

## 🔄 Update Your Scripts

### Before
```bash
#!/bin/bash
for file in $(find . -name "*.rs"); do
    echo "Processing: $file"
done
```

### After
```bash
#!/bin/bash
source lib/search_utils_v2.sh

for file in $(find_rust_files .); do
    echo "Processing: $file"
done
```

**Benefits**:
- 200x faster
- Automatic fallback if service is down
- No code changes needed
- Intelligent caching

## 🛠️ Troubleshooting

### Service won't start

```bash
# Check if port is in use
netstat -tulpn | grep 3030

# Try different port
FILE_INDEX_PORT=3031 ./target/release/file-index-server
```

### CLI can't connect

```bash
# Check service is running
curl http://localhost:3030/health

# Check server URL
file-index --server http://localhost:3030 stats
```

### Slow initial scan

```bash
# Normal for first run (scanning filesystem)
# Subsequent starts are fast (loads from cache)

# To speed up, reduce scan roots in file_index_server.rs:
let roots = vec![
    std::path::PathBuf::from("."),  // Only scan current dir
];
```

### Cache is stale

```bash
# Refresh the index
file-index refresh

# Or restart server (auto-refreshes)
pkill file-index-server
./target/release/file-index-server &
```

## 📈 Monitor Performance

### Check Statistics

```bash
file-index stats
```

Output:
```
📊 Index Statistics:
  Total files:    25432
  Total size:     1234567890 bytes
  Total queries:  156
  Unique queries: 12
  Cache hit rate: 95.50%
```

### Watch Predictions

```bash
file-index predict
```

Output:
```
🔮 Predicted queries:
  1. ext:rs
  2. name:Cargo.toml
  3. ext:toml
  4. name:flake.nix
  5. pattern:src/main
```

### Monitor Logs

```bash
# Server logs show all queries
tail -f /tmp/file-index-server.log
```

## 🎓 Advanced Usage

### Custom Server URL

```bash
# Use different server
export FILE_INDEX_SERVER=http://localhost:3031
file-index stats
```

### Output Formats

```bash
# Just paths (default)
file-index query ext rs

# JSON format
file-index query ext rs --format json

# Count only
file-index query ext rs --format count

# Detailed (with size and priority)
file-index query ext rs --format detailed
```

### Batch Queries

```bash
# Query multiple extensions
for ext in rs toml nix; do
    echo "=== $ext files ==="
    file-index query ext $ext | wc -l
done
```

## 🚀 Production Deployment

### Install System-wide

```bash
# Copy binaries
sudo cp target/release/file-index-server /usr/local/bin/
sudo cp target/release/file-index /usr/local/bin/

# Copy shell utilities
sudo cp lib/search_utils_v2.sh /usr/local/lib/

# Add to bashrc
echo 'source /usr/local/lib/search_utils_v2.sh' >> ~/.bashrc
```

### Systemd Service

Create `/etc/systemd/system/file-index-server.service`:

```ini
[Unit]
Description=File Index Service
After=network.target

[Service]
Type=simple
User=youruser
ExecStart=/usr/local/bin/file-index-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable file-index-server
sudo systemctl start file-index-server
sudo systemctl status file-index-server
```

## 📚 Next Steps

1. **Read the docs**: `docs/FILE_INDEX_SERVICE.md`
2. **Migrate scripts**: Update your shell scripts to use v2
3. **Monitor performance**: Track query latency and cache hit rate
4. **Tune parameters**: Adjust cache size, scan roots, etc.
5. **Add monitoring**: Integrate with your monitoring system

## 🎉 Success!

You now have:
- ✅ Centralized file indexing
- ✅ 200x faster queries
- ✅ Intelligent caching
- ✅ Predictive pre-fetching
- ✅ Backward compatible shell scripts

Enjoy your blazing-fast file queries! 🚀
