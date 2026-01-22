# Deployment Strategy

## Architecture

```
Linux Dev Server (this machine)
  ↓ builds
minimal-build-server (Linux)
  ↓ deploys to
Windows Laptop (prod)
  ↓ uses
libwin.so (Windows resolver)
```

## Security Model

### Dev Server (Linux)
- Development mode
- Hot reload enabled
- Full access to code
- `/compile`, `/restart`, `/upgrade`

### Prod Server (Windows Laptop)
- Production mode
- Secured endpoints
- No hot reload
- Only `/compile` with auth token
- TLS required

## Deployment Options

### Option 1: Cross-compile on Linux
```bash
# Build Windows binary on Linux
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release

# Deploy to Windows
scp target/x86_64-pc-windows-gnu/release/minimal-build-server.exe windows-laptop:
```

### Option 2: GitHub Actions
```yaml
# .github/workflows/windows-build.yml
on: push
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo build --release
      - uses: actions/upload-artifact@v4
        with:
          name: windows-binary
          path: target/release/minimal-build-server.exe
```

### Option 3: Remote Windows Builder
```bash
# Linux server pushes to Windows builder
curl -X POST https://windows-builder/compile \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"target":"minimal-build-server","platform":"windows"}'
```

## Trust Model

### Code Signing
```rust
// Sign Windows builds
fn sign_windows_binary(path: &str) -> Result<(), String> {
    Command::new("signtool")
        .args(["sign", "/f", "cert.pfx", path])
        .output()?;
    Ok(())
}
```

### Verification
```rust
// Verify signature before deploy
fn verify_signature(path: &str) -> Result<bool, String> {
    let output = Command::new("signtool")
        .args(["verify", "/pa", path])
        .output()?;
    Ok(output.status.success())
}
```

## Windows GUI & Installer

### Option A: Build on Linux (cross-compile)
```bash
# Use cargo-wix for MSI installer
cargo install cargo-wix
cargo wix --target x86_64-pc-windows-gnu
```

### Option B: Build on GitHub Actions
```yaml
- name: Build Windows GUI
  run: cargo build --release --features gui
  
- name: Create Installer
  run: |
    cargo install cargo-wix
    cargo wix
```

### Option C: Build on Windows Laptop
```bash
# Windows laptop builds its own GUI
./minimal-build-server.exe compile --target gui --use-libwin
```

## Recommended: Hybrid Approach

1. **Linux Dev Server**: Develop and test
2. **GitHub Actions**: Build Windows binaries (trusted CI)
3. **Windows Laptop**: Download signed binaries, run prod server
4. **Cross-compile**: For quick iterations

## Security Checklist

- [ ] TLS certificates for prod server
- [ ] Auth tokens for API endpoints
- [ ] Code signing for Windows binaries
- [ ] Disable hot reload in prod
- [ ] Firewall rules on Windows laptop
- [ ] Audit logs for all builds
- [ ] Verify signatures before execution

## Next Steps

1. Set up cross-compiler on Linux
2. Configure GitHub Actions for Windows builds
3. Deploy prod server to Windows laptop
4. Build Windows GUI with libwin
5. Create MSI installer
