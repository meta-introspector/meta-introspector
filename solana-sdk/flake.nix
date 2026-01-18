{
  description = "Solana SDK and tools in Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        
        # Solana CLI tools
        solana-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "solana-cli";
          version = "1.18.26";
          
          src = pkgs.fetchFromGitHub {
            owner = "solana-labs";
            repo = "solana";
            rev = "v${version}";
            hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Update with real hash
          };
          
          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };
          
          buildInputs = with pkgs; [
            openssl
            pkg-config
            udev
            protobuf
          ];
          
          # Build only CLI tools
          cargoBuildFlags = [ "-p" "solana-cli" ];
        };
        
        # Anchor framework for Solana
        anchor = pkgs.rustPlatform.buildRustPackage rec {
          pname = "anchor-cli";
          version = "0.30.1";
          
          src = pkgs.fetchFromGitHub {
            owner = "coral-xyz";
            repo = "anchor";
            rev = "v${version}";
            hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Update
          };
          
          cargoLock.lockFile = "${src}/Cargo.lock";
          
          buildInputs = with pkgs; [ openssl pkg-config ];
        };
        
      in {
        packages = {
          default = pkgs.buildEnv {
            name = "solana-sdk";
            paths = [
              rustToolchain
              # solana-cli  # Uncomment when hash is fixed
              # anchor      # Uncomment when hash is fixed
            ];
          };
          
          # Individual packages
          rust = rustToolchain;
          # solana = solana-cli;
          # anchor-cli = anchor;
          
          # Solana program template
          solana-program-template = pkgs.stdenv.mkDerivation {
            name = "solana-program-template";
            src = ./.;
            
            installPhase = ''
              mkdir -p $out/template
              cat > $out/template/lib.rs << 'EOF'
              use solana_program::{
                  account_info::AccountInfo,
                  entrypoint,
                  entrypoint::ProgramResult,
                  pubkey::Pubkey,
                  msg,
              };

              entrypoint!(process_instruction);

              pub fn process_instruction(
                  program_id: &Pubkey,
                  accounts: &[AccountInfo],
                  instruction_data: &[u8],
              ) -> ProgramResult {
                  msg!("Hello Solana!");
                  Ok(())
              }
              EOF
              
              cat > $out/template/Cargo.toml << 'EOF'
              [package]
              name = "solana-program"
              version = "0.1.0"
              edition = "2021"

              [dependencies]
              solana-program = "1.18"

              [lib]
              crate-type = ["cdylib", "lib"]
              EOF
            '';
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            
            # Build tools
            pkg-config
            openssl
            
            # Solana development (when available)
            # solana-cli
            # anchor
          ];
          
          shellHook = ''
            echo "🌞 Solana SDK Development Environment"
            echo ""
            echo "Rust toolchain: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build-bpf    # Build Solana program"
            echo "  solana-test-validator  # Run local validator"
            echo "  anchor init <name>     # Create Anchor project"
            echo ""
            echo "Template: nix build .#solana-program-template"
          '';
          
          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          SOLANA_METRICS_CONFIG = "";
        };
        
        # Apps for running Solana tools
        apps = {
          template = {
            type = "app";
            program = toString (pkgs.writeShellScript "create-solana-program" ''
              set -e
              NAME=''${1:-my-solana-program}
              echo "Creating Solana program: $NAME"
              mkdir -p $NAME/src
              cp ${self.packages.${system}.solana-program-template}/template/* $NAME/
              mv $NAME/lib.rs $NAME/src/
              echo "✅ Created $NAME/"
              echo "   cd $NAME && cargo build-bpf"
            '');
          };
        };
      }
    );
}
