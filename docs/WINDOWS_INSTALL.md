# Windows Installation Guide

## Download

Download the latest installer from [GitHub Releases](https://github.com/meta-introspector/meta-introspector/releases):

- `meta-introspector-setup-0.1.0.exe` (Windows Installer)

## Installation

### Option 1: GUI Installer (Recommended)

1. **Download** `meta-introspector-setup-0.1.0.exe`
2. **Run** the installer (double-click)
3. **Follow** the installation wizard
4. **Optional**: Check "Add to PATH" to use from command line
5. **Launch** from Start Menu or Desktop

### Option 2: Portable Binary

1. **Download** `minimal-build-server-x86_64-pc-windows-msvc.zip`
2. **Extract** to any folder
3. **Run** `minimal-build-server.exe`

## What Gets Installed

```
C:\Program Files\Meta-Introspector\
├── minimal-build-server.exe    (Main server)
├── demos\                       (Demo binaries)
│   ├── demo_function_loader.exe
│   ├── demo_meta_mcp.exe
│   └── demo_*.exe
├── libs\                        (Shared libraries)
│   ├── content_address.dll
│   ├── mcp.dll
│   └── git_ops.dll
└── docs\                        (Documentation)
    ├── QUICKSTART.md
    ├── DEVOPS_GUIDE.md
    └── ...
```

## Usage

### Start Server

**From Start Menu:**
- Click "Meta-Introspector" in Start Menu

**From Command Line:**
```cmd
minimal-build-server
```

**From PowerShell:**
```powershell
& "C:\Program Files\Meta-Introspector\minimal-build-server.exe"
```

Server starts on `http://127.0.0.1:3000`

### Test Installation

```powershell
# Check version
minimal-build-server --version

# Test API
Invoke-WebRequest -Uri http://127.0.0.1:3000/help | Select-Object -Expand Content
```

### Using from PowerShell

```powershell
# List available binaries
Invoke-RestMethod -Uri http://127.0.0.1:3000/binaries

# Build a target
$body = @{
    target = "demo_hello"
} | ConvertTo-Json

Invoke-RestMethod -Uri http://127.0.0.1:3000/compile `
    -Method Post `
    -ContentType "application/json" `
    -Body $body
```

## Firewall Configuration

Windows Firewall may prompt for network access. Choose:
- **Private networks** for local development
- **Public networks** if you need external access

Or configure manually:
```powershell
# Allow inbound on port 3000
New-NetFirewallRule -DisplayName "Meta-Introspector" `
    -Direction Inbound `
    -LocalPort 3000 `
    -Protocol TCP `
    -Action Allow
```

## Add to PATH

If you didn't check "Add to PATH" during installation:

```powershell
# Add to user PATH
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$newPath = "$path;C:\Program Files\Meta-Introspector"
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")
```

## Uninstallation

### Via Control Panel
1. Open "Add or Remove Programs"
2. Find "Meta-Introspector"
3. Click "Uninstall"

### Via Command Line
```cmd
"C:\Program Files\Meta-Introspector\unins000.exe" /SILENT
```

## Troubleshooting

### Port Already in Use

```powershell
# Find process using port 3000
Get-NetTCPConnection -LocalPort 3000 | Select-Object -Property OwningProcess

# Kill process
Stop-Process -Id <PID>
```

### Permission Denied

Run as Administrator:
```powershell
Start-Process minimal-build-server.exe -Verb RunAs
```

### Missing DLLs

Install Visual C++ Redistributable:
- Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe

### Antivirus Blocking

Add exception for:
- `C:\Program Files\Meta-Introspector\`

## System Requirements

- **OS**: Windows 10 or later (64-bit)
- **RAM**: 2GB minimum, 4GB recommended
- **Disk**: 500MB free space
- **Network**: Internet connection for downloads

## Building from Source (Windows)

```powershell
# Install Rust
winget install Rustlang.Rustup

# Clone repository
git clone https://github.com/meta-introspector/meta-introspector
cd meta-introspector

# Build
cargo build --release --bins

# Run
.\target\release\minimal-build-server.exe
```

## Creating Installer from Source

```powershell
# Install Inno Setup
choco install innosetup -y

# Build binaries
cargo build --release --bins

# Create installer
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss

# Installer created in: installer\meta-introspector-setup-0.1.0.exe
```

## Support

- **Documentation**: https://meta-introspector.github.io/
- **Issues**: https://github.com/meta-introspector/meta-introspector/issues
- **Discussions**: https://github.com/meta-introspector/meta-introspector/discussions

---

**Version**: 0.1.0  
**Last Updated**: 2026-01-17
