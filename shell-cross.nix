{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust from nixpkgs
    cargo
    rustc
    
    # Cross-compilation tools
    pkgsCross.mingwW64.stdenv.cc
    pkgsCross.mingwW64.windows.pthreads
    wine64
    
    # Build tools
    pkg-config
    openssl
  ];

  shellHook = ''
    echo "🔧 Nix Cross-Compilation Environment"
    echo ""
    echo "Available commands:"
    echo "  ./cross-compile.sh          - Build all targets"
    echo "  nix build .#windows         - Build Windows only"
    echo "  nix build .#macos-x86       - Build macOS Intel"
    echo "  nix build .#macos-arm       - Build macOS ARM"
    echo ""
    echo "Targets configured:"
    echo "  • x86_64-pc-windows-gnu"
    echo "  • x86_64-apple-darwin"
    echo "  • aarch64-apple-darwin"
  '';
}
