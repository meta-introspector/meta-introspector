#!/usr/bin/env bash
set -e

LOG_FILE="/tmp/windows-build-$(date +%s).log"

echo "Starting Windows cross-build in background..."
echo "Log file: $LOG_FILE"

NIX_PATH=nixpkgs=channel:nixos-unstable nix build --impure --expr '
with import <nixpkgs> { crossSystem = { config = "x86_64-w64-mingw32"; }; };
rustPlatform.buildRustPackage {
  pname = "minimal-build-server";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
' > "$LOG_FILE" 2>&1 &

BUILD_PID=$!
echo "Build PID: $BUILD_PID"
echo "Monitor with: tail -f $LOG_FILE"
echo "$BUILD_PID" > /tmp/windows-build.pid
