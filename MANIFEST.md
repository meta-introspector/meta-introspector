# Binary Manifest

## Core
- **minimal-build-server** (1.6MB)
  - Zero SSL dependencies
  - Signature: minimal-build-server.asc
  - Built: 2026-01-15

## Wrappers
- **liblibnix.so** (4.6MB)
  - Nix store wrapper
  - Signature: liblibnix.so.asc
  - Built: 2026-01-15

## SSL Libraries
- **liblibhttp.so** (4.4MB)
  - HTTP client with rustls
  - Signature: liblibhttp.so.asc
  - Built: 2026-01-15
  - Location: libhttp/target/release/

- **liblibgit.so** (3.1MB)
  - Git operations with libgit2
  - Signature: liblibgit.so.asc
  - Built: 2026-01-15
  - Location: libgit/target/release/

## Verification

```bash
# Verify all signatures
./verify_binaries.sh

# Verify individual binary
gpg --verify target/debug/minimal-build-server.asc target/debug/minimal-build-server
```

## Checksums

```bash
# Generate checksums
sha256sum target/debug/minimal-build-server > checksums.txt
sha256sum target/debug/liblibnix.so >> checksums.txt
sha256sum libhttp/target/release/liblibhttp.so >> checksums.txt
sha256sum libgit/target/release/liblibgit.so >> checksums.txt

# Verify checksums
sha256sum -c checksums.txt
```

## Nix Store Dependencies

Runtime dependencies loaded from nix store:
- openssl-3.6.0
- libgit2
- curl

All content-addressed and reproducible.
