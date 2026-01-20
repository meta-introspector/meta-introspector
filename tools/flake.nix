{
  description = "SOLFUNMEME P2P Block Collector - DAO Governed";

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
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        packages = {
          # WASM Client Plugin
          solana-rpc-wasm = pkgs.rustPlatform.buildRustPackage {
            pname = "solana-rpc-wasm";
            version = "0.1.0";
            src = ./tools/wasm-plugins/solana-rpc;
            cargoLock.lockFile = ./tools/wasm-plugins/solana-rpc/Cargo.lock;
            
            nativeBuildInputs = [ pkgs.wasm-pack ];
            
            buildPhase = ''
              wasm-pack build --target web --release
            '';
            
            installPhase = ''
              mkdir -p $out
              cp -r pkg/* $out/
            '';
          };
          
          # Server .so Plugin
          block-collector-plugin = pkgs.rustPlatform.buildRustPackage {
            pname = "block-collector-plugin";
            version = "0.1.0";
            src = ./tools/so-plugins/block-collector;
            cargoLock.lockFile = ./tools/so-plugins/block-collector/Cargo.lock;
            
            buildPhase = ''
              cargo build --release
            '';
            
            installPhase = ''
              mkdir -p $out/lib
              cp target/release/libblock_collector_plugin.so $out/lib/
            '';
          };
          
          default = self.packages.${system}.block-collector-plugin;
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wasm-pack
            cargo
            rustc
          ];
        };
      }
    );
}
