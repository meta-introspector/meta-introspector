# Project Index - Quick Reference

**Last Updated**: 2026-01-15

## 🗂️ Repository Locations

### Primary Projects

| Project | Path | Purpose | Key Files |
|---------|------|---------|-----------|
| **meta-introspector** | `/home/mdupont/meta-introspector` | Main analysis repo | 57K domains, 33.9M files analyzed |
| **zombie_driver2** | `/home/mdupont/zombie_driver2` | Rust analysis tools | `unified_p2p_server.rs`, `Cargo.toml` (153 binaries) |
| **zos-server** | `/home/mdupont/zos-server` | Analysis tools collection | 72 .rs files, axum deps |
| **zos-qa** | `/home/mdupont/zos-qa` | **MAIN DEPLOYMENT PROJECT** | Complete workspace with all deployment code |

## 🎯 ZOS-QA Project Structure (MOST IMPORTANT)

**Location**: `/home/mdupont/zos-qa/`

### Workspace Crates
```
zos-qa/
├── zos-oci/              # OCI API client (reqwest-based)
│   └── src/lib.rs        # InstanceConfiguration, OciClient
├── zos-oracle/           # Oracle plugin + Resource Manager
│   ├── src/lib.rs        # C-compatible plugin interface
│   ├── src/stack_deployer.rs  # NEW: ORM API client
│   └── src/bin/deploy.rs      # NEW: Deployment CLI
├── zos-macros/           # Zero-dep macros
├── zos-traits/           # Core traits
├── zos-types/            # Type definitions
├── zos-deploy/           # Deployment utilities
├── zos-libp2p/           # LibP2P networking
├── zos-minimal-server/   # Minimal server impl
└── unified_p2p_server.rs # Main P2P server (40K lines)
```

### Key Files Found

| File | Purpose | Status |
|------|---------|--------|
| `unified_p2p_server.rs` | LibP2P server with P2PVerb system | ✅ Working |
| `zos-oci/src/lib.rs` | OCI API client (reqwest) | ✅ Working |
| `zos-oracle/src/lib.rs` | Oracle plugin (uses OCI CLI) | ✅ Working |
| `templates/oracle-zos-instance/main.tf` | Terraform config | ✅ Complete |
| `templates/oracle-zos-instance/terraform.tfvars` | TF vars | ✅ Has values |
| `deploy-zos-oracle.sh` | Original deploy script | ✅ Working |
| `Cargo.toml` | Workspace config | ✅ 15 members |

## 🔑 Credentials & Config

| Item | Location | Status |
|------|----------|--------|
| OCI Config | `~/.solfunmeme-keys/oci_config` | ✅ Found |
| OCI Private Key | `~/.solfunmeme-keys/oci_private_key.pem` | ✅ Found |
| SSH Public Key | `~/.ssh/id_rsa.pub` | ✅ Found |
| SSH Private Key | `~/.ssh/id_rsa` | ✅ Found |

### OCI Config Contents
```ini
[DEFAULT]
user=ocid1.user.oc1..aaaaaaaas5losxb2h3z4gvjw7llmttn2a5pmjhleaz5bk4p54h7msvpd3o4q
fingerprint=be:e9:0b:eb:e6:ac:37:d5:9e:83:6f:28:32:58:db:ab
tenancy=ocid1.tenancy.oc1..aaaaaaaapxfkcjaczqslvnbekbqq2eefxgwx7kqbakvddhzaaiym62vmt5la
region=us-ashburn-1
key_file=/home/mdupont/.ssh/jmikedupont2@gmail.com-2026-01-08T23_46_45.686Z.pem
```

## 🚀 Deployment Scripts

| Script | Location | Method | Status |
|--------|----------|--------|--------|
| `deploy-zos-oracle.sh` | `/home/mdupont/zos-qa/` | OCI CLI + Rust client | ✅ Working |
| `deploy-oracle-rust.sh` | `/home/mdupont/zos-qa/` | **NEW: Pure Rust ORM API** | ✅ Created |
| `deploy-oracle.sh` | `/home/mdupont/zombie_driver2/` | Terraform direct | ⚠️ Old approach |

## 🔍 Search Patterns That Worked

### Finding OCI Implementation
```bash
# Found in zos-qa, not zombie_driver2 or zos-server
find /home/mdupont/zos-qa -name "*.rs" | xargs grep -l "OciClient"
cat /home/mdupont/zos-qa/zos-oci/src/lib.rs
```

### Finding Credentials
```bash
ls ~/.solfunmeme-keys/
cat ~/.solfunmeme-keys/oci_config
```

### Finding Terraform Templates
```bash
ls /home/mdupont/zos-qa/templates/oracle-zos-instance/
```

## 📦 Dependencies

### zos-qa Workspace
- **tokio** 1.0 (async runtime)
- **axum** 0.7 (HTTP server)
- **reqwest** 0.11 (HTTP client)
- **serde** 1.0 (serialization)
- **anyhow** 1.0 (error handling)

### Tools Installed
- ✅ Terraform v1.8.1
- ❌ OCI CLI (not installed, but not needed - using Rust API)

## 🎯 Quick Commands

### Deploy to Oracle
```bash
cd ~/zos-qa
./deploy-oracle-rust.sh
```

### Build Specific Crate
```bash
cd ~/zos-qa/zos-oracle
cargo build --release --features full --bin deploy
```

### Test OCI Connection
```bash
cd ~/zos-qa/zos-oci
cargo run
```

### Check Workspace
```bash
cd ~/zos-qa
cargo build --workspace
```

## 🔧 What We Built Today

1. **`src/unified_server.rs`** - Axum + LibP2P with SSH auth
2. **`zos-oracle/src/stack_deployer.rs`** - Oracle Resource Manager API client
3. **`zos-oracle/src/bin/deploy.rs`** - Deployment CLI
4. **`deploy-oracle-rust.sh`** - One-command deployment
5. **`DEPLOY_ORACLE_RUST.md`** - Complete documentation

## 💡 Lessons Learned

### Search Strategy
1. **Start with most recent/active project** - zos-qa was the answer
2. **Check for workspace structure** - `Cargo.toml` with `[workspace]`
3. **Look for credentials first** - `~/.solfunmeme-keys/` had everything
4. **Check templates/** - Terraform configs were ready
5. **grep for specific types** - `OciClient`, `InstanceConfiguration`

### Time Wasters
- ❌ Searching zombie_driver2 first (older project)
- ❌ Searching zos-server (different purpose)
- ❌ Looking for terraform CLI (not needed)
- ❌ Trying to find axum server (it wasn't built yet)

### What Worked
- ✅ User said "we have it in rust" - searched for Rust OCI code
- ✅ User pointed to `~/zos-qa/zos-oci/src/lib.rs` - found everything
- ✅ Checked credentials location - found complete config
- ✅ Found workspace structure - understood full architecture

## 🚀 Next Time: Fast Path

```bash
# 1. Check project index
cat ~/PROJECT_INDEX.md

# 2. Go to main project
cd ~/zos-qa

# 3. Check workspace structure
cat Cargo.toml | grep members

# 4. Find relevant crate
ls zos-*/

# 5. Check credentials
ls ~/.solfunmeme-keys/

# 6. Deploy
./deploy-oracle-rust.sh
```

## 📝 TODO

- [ ] Deploy to Oracle Cloud (test stack creation)
- [ ] Deploy to Hugging Face Spaces
- [ ] Deploy to Vercel
- [ ] Deploy to Cloudflare Workers
- [ ] Set up multi-cloud federation
- [ ] Add monitoring/telemetry
- [ ] Create CI/CD pipeline

## 🔗 Related Documentation

- `~/zos-qa/DEPLOY_ORACLE_RUST.md` - Oracle deployment guide
- `~/zos-qa/README.md` - Project overview
- `~/zombie_driver2/ORACLE_DEPLOYMENT.md` - Old deployment docs
- `~/meta-introspector/README.md` - Analysis system overview
