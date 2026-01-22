{ pkgs ? import <nixpkgs> {} }:

let
  mingw = pkgs.pkgsCross.mingwW64;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust from nixpkgs
    cargo
    rustc
    
    # Cross-compilation tools
    mingw.stdenv.cc
    mingw.windows.pthreads
    wine64
    
    # Build tools
    pkg-config
    openssl
  ];

  # Set environment for Windows cross-compilation
  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "wine64";
  
  # Fix linking issues
  NIX_LDFLAGS = "-L${mingw.windows.pthreads}/lib";

  shellHook = ''
    echo "🔧 Nix Cross-Compilation Environment"
    echo ""
    echo "Available commands:"
    echo "  ./cross-compile.sh          - Build all targets"
    echo "  cargo build --target x86_64-pc-windows-gnu"
    echo ""
    echo "Targets configured:"
    echo "  • x86_64-pc-windows-gnu"
    echo "  • x86_64-apple-darwin (requires macOS SDK)"
    echo "  • aarch64-apple-darwin (requires macOS SDK)"
    echo ""
    echo "Linker: ${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
  '';
}
