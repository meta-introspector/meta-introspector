{
  description = "Meta-Introspector Cross-Compilation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          crossSystem = null;
        };

        # Cross-compilation targets
        targets = {
          windows = {
            target = "x86_64-pc-windows-gnu";
            crossPkgs = import nixpkgs {
              inherit system overlays;
              crossSystem = {
                config = "x86_64-w64-mingw32";
              };
            };
          };
          macos-x86 = {
            target = "x86_64-apple-darwin";
            crossPkgs = import nixpkgs {
              inherit system overlays;
              crossSystem = {
                config = "x86_64-apple-darwin";
              };
            };
          };
          macos-arm = {
            target = "aarch64-apple-darwin";
            crossPkgs = import nixpkgs {
              inherit system overlays;
              crossSystem = {
                config = "aarch64-apple-darwin";
              };
            };
          };
        };

        # Rust toolchain with cross-compilation targets
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "x86_64-pc-windows-gnu"
            "x86_64-apple-darwin"
            "aarch64-apple-darwin"
          ];
        };

        # Build function for cross-compilation
        buildCross = targetName: targetInfo:
          let
            crossPkgs = targetInfo.crossPkgs;
          in
          crossPkgs.stdenv.mkDerivation {
            name = "meta-introspector-${targetName}";
            src = ./.;

            nativeBuildInputs = [
              rustToolchain
              pkgs.pkg-config
            ];

            buildInputs = with crossPkgs; [
              openssl
            ] ++ pkgs.lib.optionals (targetName == "windows") [
              crossPkgs.windows.pthreads
            ];

            buildPhase = ''
              export CARGO_TARGET_DIR=$TMPDIR/target
              cargo build --release --target ${targetInfo.target} --bin minimal-build-server
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp $CARGO_TARGET_DIR/${targetInfo.target}/release/minimal-build-server* $out/bin/
            '';
          };

      in
      {
        packages = {
          # Native build
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "meta-introspector";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          # Cross-compiled builds
          windows = buildCross "windows" targets.windows;
          macos-x86 = buildCross "macos-x86" targets.macos-x86;
          macos-arm = buildCross "macos-arm" targets.macos-arm;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.openssl
            
            # Cross-compilation tools
            pkgs.mingw_w64
            pkgs.wine64
            
            # macOS cross-compilation (if on Linux)
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];

          shellHook = ''
            echo "🔧 Cross-Compilation Environment Ready!"
            echo ""
            echo "Available targets:"
            echo "  • x86_64-pc-windows-gnu (Windows)"
            echo "  • x86_64-apple-darwin (macOS Intel)"
            echo "  • aarch64-apple-darwin (macOS ARM)"
            echo ""
            echo "Build commands:"
            echo "  nix build .#windows"
            echo "  nix build .#macos-x86"
            echo "  nix build .#macos-arm"
            echo ""
            echo "Or use cargo directly:"
            echo "  cargo build --release --target x86_64-pc-windows-gnu"
            echo "  cargo build --release --target x86_64-apple-darwin"
            echo "  cargo build --release --target aarch64-apple-darwin"
          '';
        };
      }
    );
}
