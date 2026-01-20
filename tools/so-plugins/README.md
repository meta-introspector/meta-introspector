# ZOS Server Plugins

All ZOS infrastructure as SO plugins loaded by zos_server.

## Plugins

### DNS Server (`dns-server`)
- Port: 5353
- Logs: `zos_dns.log`
- MITM-ready DNS with query logging

### File Proxy (`file-proxy`)
- Port: 8080
- Serves: `/mnt/data1/meta-introspector`
- Logs: `zos_proxy.log`

### GitHub Proxy (`github-proxy`)
- Port: 9418
- Proxies to local git mirrors
- Logs: `zos_proxy.log`

### Nix Proxy (`nix-proxy`)
- Port: 5000
- Proxies to nix-serve
- Logs: `zos_proxy.log`

### LLM Proxy (`llm-proxy`)
- Port: 11435
- Proxies to Ollama (11434)
- Logs: `zos_llm.log`
- Audits all LLM requests/responses

### Block Collector (`block-collector`)
- Existing: Solana block collection

## Build All Plugins

```bash
cd tools/so-plugins
for plugin in */; do
    cd "$plugin"
    cargo build --release
    cd ..
done
```

## Load into ZOS Server

```bash
zos_server \
    --plugin target/release/libzos_dns_server.so \
    --plugin target/release/libzos_file_proxy.so \
    --plugin target/release/libzos_github_proxy.so \
    --plugin target/release/libzos_nix_proxy.so
```

## Architecture

```
zos_server
  ├── dns-server (5353)      - DNS + MITM logging
  ├── file-proxy (8080)      - File serving
  ├── github-proxy (9418)    - Git mirror
  ├── nix-proxy (5000)       - Nix binary cache
  └── llm-proxy (11435)      - Ollama LLM auditing
```

All logged to `zos_*.log` for audit trail.
