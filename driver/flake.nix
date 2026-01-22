{
  description = "Unified driver binary - replaces all tools with proven gateway";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system} = {
        # The unified driver binary
        default = pkgs.rustPlatform.buildRustPackage {
          name = "driver";
          src = ../..;
          cargoLock.lockFile = ../../Cargo.lock;
          
          buildPhase = ''
            cargo build --release --bin driver
          '';
          
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/driver $out/bin/
            
            # Create symlinks for all commands
            for cmd in nix cargo git jq bash ssh curl perf; do
              ln -s driver $out/bin/$cmd
            done
          '';
        };
        
        # Overlay to replace system tools
        overlay = final: prev: {
          nix = self.packages.${system}.default;
          cargo = self.packages.${system}.default;
          git = self.packages.${system}.default;
          jq = self.packages.${system}.default;
          curl = self.packages.${system}.default;
        };
      };
      
      # Shell environment with driver
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [ self.packages.${system}.default ];
        
        shellHook = ''
          echo "🚀 Unified Driver Environment"
          echo "=============================="
          echo ""
          echo "All commands go through proven gateway:"
          echo "  nix, cargo, git, jq, bash, ssh, curl, perf"
          echo ""
          echo "Every syscall generates a ZK proof."
          echo "The kernel is just a proof generator."
          echo ""
        '';
      };
    };
}
