#!/usr/bin/env bash
# Cross-compile for Windows and macOS using Nix

set -e

echo "🔧 Cross-Compilation with Nix"
echo ""

# Build for Windows
echo "🪟 Building for Windows (x86_64-pc-windows-gnu)..."
nix build --impure --expr '
  let
    pkgs = import <nixpkgs> {
      crossSystem = { config = "x86_64-w64-mingw32"; };
    };
  in
  pkgs.rustPlatform.buildRustPackage {
    pname = "minimal-build-server-windows";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    doCheck = false;
  }
' -o result-windows

echo "✅ Windows build complete: result-windows/bin/"

# Build for macOS x86_64
echo ""
echo "🍎 Building for macOS Intel (x86_64-apple-darwin)..."
nix build --impure --expr '
  let
    pkgs = import <nixpkgs> {
      crossSystem = { config = "x86_64-apple-darwin"; };
    };
  in
  pkgs.rustPlatform.buildRustPackage {
    pname = "minimal-build-server-macos-x86";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    doCheck = false;
  }
' -o result-macos-x86 || echo "⚠️  macOS cross-compilation requires Darwin SDK"

# Build for macOS ARM
echo ""
echo "🍎 Building for macOS ARM (aarch64-apple-darwin)..."
nix build --impure --expr '
  let
    pkgs = import <nixpkgs> {
      crossSystem = { config = "aarch64-apple-darwin"; };
    };
  in
  pkgs.rustPlatform.buildRustPackage {
    pname = "minimal-build-server-macos-arm";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    doCheck = false;
  }
' -o result-macos-arm || echo "⚠️  macOS cross-compilation requires Darwin SDK"

echo ""
echo "✅ Cross-compilation complete!"
echo ""
echo "Results:"
ls -lh result-*/bin/ 2>/dev/null || true
