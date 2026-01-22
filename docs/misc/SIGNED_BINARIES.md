# Signed Binary Architecture

## Overview

Zero-trust architecture with GPG-signed binaries and isolated SSL dependencies.

## Components

### Core (No SSL)
- **minimal-build-server** (1.6MB)
  - Zero SSL dependencies
  - Fast compile, small attack surface
  - Loads capabilities dynamically

### Nix Wrapper
- **libnix.so** (4.6MB) 
  - Finds libraries in nix store
  - Loads system dependencies on demand
  - C API: `libnix_load(libs, count)`

### SSL Libraries (Signed)
- **libhttp.so** (4.4MB)
  - reqwest + rustls-tls
  - C API: `http_get(url, out)`
  - Built with: `nix develop -c cargo build --release`

- **libgit.so** (3.1MB)
  - gix + libgit2
  - C API: `git_clone(url, path)`
  - Built with: `nix develop -c cargo build --release`

## Security Model

### Signatures
All binaries are GPG signed:
```bash
./sign_binaries.sh      # Sign all binaries
./verify_binaries.sh    # Verify signatures
```

Each `.so` has a `.asc` signature file:
- `minimal-build-server.asc`
- `liblibnix.so.asc`
- `liblibhttp.so.asc`
- `liblibgit.so.asc`

### Verification
```bash
gpg --verify lib.so.asc lib.so
```

### Trust Model
1. **Core** - Minimal, auditable, no SSL
2. **Wrapper** - Loads from nix store (content-addressed)
3. **Libraries** - GPG signed, verified before load

## Build Process

### 1. Build Core
```bash
cargo build --bin minimal-build-server
```

### 2. Build SSL Libraries
```bash
cd libhttp && nix develop -c cargo build --release
cd libgit && nix develop -c cargo build --release
```

### 3. Sign All
```bash
./sign_binaries.sh
```

### 4. Verify
```bash
./verify_binaries.sh
```

## Runtime Loading

```
minimal-build-server
    ↓ bootstrap_libs()
libnix.so (verified)
    ↓ load_via_nix()
/nix/store/.../libssl.so (content-addressed)
    ↓ on demand
libhttp.so (verified)
libgit.so (verified)
```

## Nix Store Paths

Libraries loaded from nix store are content-addressed:
```
/nix/store/rfm5m2l26lqkskcvxn5bm5xqh6c8wqr5-openssl-3.6.0/lib/libssl.so
```

Hash = f(source code + dependencies + build process)

## Distribution

### Minimal Deployment
Ship only:
- `minimal-build-server` (1.6MB)
- `minimal-build-server.asc`

Pull dependencies on first run via nix.

### Full Deployment
Ship all signed binaries:
- Core + signatures
- libnix.so + signature
- libhttp.so + signature
- libgit.so + signature

### Nix Flakes
Each library has its own flake:
```
libhttp/flake.nix  # HTTP with SSL deps
libgit/flake.nix   # Git with SSL deps
```

## Security Properties

1. **Minimal Attack Surface** - Core has no SSL code
2. **Signed Binaries** - GPG verification before load
3. **Content-Addressed** - Nix store guarantees reproducibility
4. **Isolated Dependencies** - SSL in separate .so files
5. **Auditable** - Small core, clear boundaries

## Future: ZK Proofs

Extend to zero-knowledge proofs:
```
/nix/store/public/    - Pure functions (no proof needed)
/nix/store/verified/  - ZK proofs of execution
/nix/store/trusted/   - GPG signed (current)
/nix/store/private/   - Encrypted
```
