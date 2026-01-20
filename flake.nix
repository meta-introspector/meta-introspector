{
  description = "SOLFUNMEME Meta-Introspector WASM Binaries";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        wasmBuildInputs = with pkgs; [
          rustToolchain
          wasm-pack
          wasm-bindgen-cli
          binaryen
          nodejs
        ];
        
        # Orbit extraction tool
        extract-orbits = pkgs.rustPlatform.buildRustPackage {
          pname = "extract-orbits";
          version = "0.1.0";
          src = pkgs.writeTextDir "extract_orbits.rs" (builtins.readFile ./extract_orbits.rs);
          cargoLock = null;
          buildPhase = ''
            rustc extract_orbits.rs -O -o extract_orbits
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp extract_orbits $out/bin/
          '';
        };

        buildWasmPackage = { name, src, cargoToml ? "${src}/Cargo.toml" }:
          pkgs.stdenv.mkDerivation {
            inherit name src;
            
            buildInputs = wasmBuildInputs ++ [ pkgs.linuxPackages.perf ];
            
            buildPhase = ''
              export HOME=$TMPDIR
              cd ${src}
              
              # Build with perf recording
              perf record -g -o $TMPDIR/build.perf.data -- \
                wasm-pack build --target web --release --out-dir $out/pkg || true
              
              # Optimize with binaryen
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm || true
              
              # Generate hash
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256 || true
              
              # Store perf data in output
              mkdir -p $out/perf
              cp $TMPDIR/build.perf.data $out/perf/ || true
              perf report -i $TMPDIR/build.perf.data --stdio > $out/perf/report.txt || true
            '';
            
            installPhase = ''
              echo "WASM package built: $out/pkg"
              echo "Perf data stored: $out/perf"
            '';
          };

      in
      {
        packages = {
          # Senator Plugin WASM
          senator-plugin = buildWasmPackage {
            name = "senator-plugin-wasm";
            src = ./senator_plugin;
          };

          # Safe Wallet WASM
          safe-wallet = pkgs.stdenv.mkDerivation {
            name = "safe-wallet-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              # Create minimal safe wallet package
              mkdir -p safe_wallet/src
              cat > safe_wallet/Cargo.toml <<EOF
              [package]
              name = "safe_wallet"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              js-sys = "0.3"
              web-sys = { version = "0.3", features = ["Window"] }
              EOF

              cat > safe_wallet/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;

              #[wasm_bindgen]
              pub struct SafeWallet {
                  chains: Vec<String>,
              }

              #[wasm_bindgen]
              impl SafeWallet {
                  #[wasm_bindgen(constructor)]
                  pub fn new() -> SafeWallet {
                      SafeWallet { chains: vec![] }
                  }
                  
                  #[wasm_bindgen]
                  pub fn add_chain(&mut self, chain: String) {
                      self.chains.push(chain);
                  }
              }
              EOF

              cd safe_wallet
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "Safe wallet WASM built: $out/pkg"
            '';
          };

          # Living Meme WASM
          living-meme = pkgs.stdenv.mkDerivation {
            name = "living-meme-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              mkdir -p living_meme/src
              cat > living_meme/Cargo.toml <<EOF
              [package]
              name = "living_meme"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              js-sys = "0.3"
              EOF

              cat > living_meme/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;
              use serde::{Deserialize, Serialize};

              #[derive(Serialize, Deserialize)]
              #[wasm_bindgen]
              pub struct LivingMeme {
                  rank: u32,
                  generation: u32,
                  fitness: f64,
              }

              #[wasm_bindgen]
              impl LivingMeme {
                  #[wasm_bindgen(constructor)]
                  pub fn birth(rank: u32) -> LivingMeme {
                      LivingMeme { rank, generation: 1, fitness: 0.0 }
                  }
                  
                  #[wasm_bindgen]
                  pub fn fitness(&self) -> f64 {
                      self.fitness
                  }
              }
              EOF

              cd living_meme
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "Living meme WASM built: $out/pkg"
            '';
          };

          # Threshold Reconstruction WASM
          threshold = pkgs.stdenv.mkDerivation {
            name = "threshold-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              mkdir -p threshold/src
              cat > threshold/Cargo.toml <<EOF
              [package]
              name = "threshold"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              EOF

              cat > threshold/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;

              #[wasm_bindgen]
              pub struct ThresholdSystem {
                  threshold: usize,
                  collected: usize,
              }

              #[wasm_bindgen]
              impl ThresholdSystem {
                  #[wasm_bindgen(constructor)]
                  pub fn new() -> ThresholdSystem {
                      ThresholdSystem { threshold: 71, collected: 0 }
                  }
                  
                  #[wasm_bindgen]
                  pub fn collect(&mut self) {
                      self.collected += 1;
                  }
                  
                  #[wasm_bindgen]
                  pub fn can_reconstruct(&self) -> bool {
                      self.collected >= self.threshold
                  }
              }
              EOF

              cd threshold
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "Threshold WASM built: $out/pkg"
            '';
          };

          # Discovery Network WASM
          discovery = pkgs.stdenv.mkDerivation {
            name = "discovery-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              mkdir -p discovery/src
              cat > discovery/Cargo.toml <<EOF
              [package]
              name = "discovery"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              EOF

              cat > discovery/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;

              #[wasm_bindgen]
              pub struct DiscoveryNetwork {
                  discoveries: u32,
              }

              #[wasm_bindgen]
              impl DiscoveryNetwork {
                  #[wasm_bindgen(constructor)]
                  pub fn new() -> DiscoveryNetwork {
                      DiscoveryNetwork { discoveries: 0 }
                  }
                  
                  #[wasm_bindgen]
                  pub fn discover(&mut self) {
                      self.discoveries += 1;
                  }
              }
              EOF

              cd discovery
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "Discovery WASM built: $out/pkg"
            '';
          };

          # Identity Node WASM
          identity-node = pkgs.stdenv.mkDerivation {
            name = "identity-node-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              mkdir -p identity_node/src
              cat > identity_node/Cargo.toml <<EOF
              [package]
              name = "identity_node"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              EOF

              cat > identity_node/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;

              #[wasm_bindgen]
              pub struct IdentityNode {
                  owner: String,
                  earnings: u64,
              }

              #[wasm_bindgen]
              impl IdentityNode {
                  #[wasm_bindgen(constructor)]
                  pub fn new(owner: String) -> IdentityNode {
                      IdentityNode { owner, earnings: 0 }
                  }
                  
                  #[wasm_bindgen]
                  pub fn earnings(&self) -> u64 {
                      self.earnings
                  }
              }
              EOF

              cd identity_node
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "Identity node WASM built: $out/pkg"
            '';
          };

          # LLM Batching WASM
          llm-batching = pkgs.stdenv.mkDerivation {
            name = "llm-batching-wasm";
            src = ./src;
            
            buildInputs = wasmBuildInputs;
            
            buildPhase = ''
              export HOME=$TMPDIR
              
              mkdir -p llm_batching/src
              cat > llm_batching/Cargo.toml <<EOF
              [package]
              name = "llm_batching"
              version = "0.1.0"
              edition = "2021"

              [lib]
              crate-type = ["cdylib"]

              [dependencies]
              wasm-bindgen = "0.2"
              serde = { version = "1.0", features = ["derive"] }
              serde-wasm-bindgen = "0.6"
              EOF

              cat > llm_batching/src/lib.rs <<EOF
              use wasm_bindgen::prelude::*;

              #[wasm_bindgen]
              pub struct LLMBatcher {
                  batch_size: usize,
                  queued: usize,
              }

              #[wasm_bindgen]
              impl LLMBatcher {
                  #[wasm_bindgen(constructor)]
                  pub fn new(batch_size: usize) -> LLMBatcher {
                      LLMBatcher { batch_size, queued: 0 }
                  }
                  
                  #[wasm_bindgen]
                  pub fn add_request(&mut self) -> bool {
                      self.queued += 1;
                      self.queued >= self.batch_size
                  }
              }
              EOF

              cd llm_batching
              wasm-pack build --target web --release --out-dir $out/pkg
              wasm-opt -Oz -o $out/pkg/optimized.wasm $out/pkg/*_bg.wasm
              sha256sum $out/pkg/optimized.wasm > $out/pkg/wasm.sha256
            '';
            
            installPhase = ''
              echo "LLM batching WASM built: $out/pkg"
            '';
          };

          # Build all WASM packages
          all-wasm = pkgs.symlinkJoin {
            name = "all-wasm-packages";
            paths = [
              self.packages.${system}.senator-plugin
              self.packages.${system}.safe-wallet
              self.packages.${system}.living-meme
              self.packages.${system}.threshold
              self.packages.${system}.discovery
              self.packages.${system}.identity-node
              self.packages.${system}.llm-batching
            ];
          };
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = wasmBuildInputs ++ [
            pkgs.cargo-watch
            pkgs.cargo-edit
          ];
          
          shellHook = ''
            echo "🚀 SOLFUNMEME WASM Development Environment"
            echo "Available commands:"
            echo "  nix build .#senator-plugin"
            echo "  nix build .#safe-wallet"
            echo "  nix build .#living-meme"
            echo "  nix build .#threshold"
            echo "  nix build .#discovery"
            echo "  nix build .#identity-node"
            echo "  nix build .#llm-batching"
            echo "  nix build .#all-wasm"
          '';
        };

        # Default package
        defaultPackage = self.packages.${system}.all-wasm;
        
        # Orbit analysis package
        packages.extract-orbits = extract-orbits;
      }
    );
}
