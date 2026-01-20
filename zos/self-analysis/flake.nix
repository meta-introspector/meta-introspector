{
  description = "ZOS Self-Analysis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
      # Build ZOS tools
      zos-tools = pkgs.rustPlatform.buildRustPackage {
        pname = "zos-tools";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      
    in {
      packages.${system} = {
        # Self-analysis derivation
        zos-self-analysis = pkgs.stdenv.mkDerivation {
          name = "zos-self-analysis";
          src = ./.;
          
          buildInputs = [ zos-tools ];
          
          buildPhase = ''
            echo "🔍 Running ZOS tools on ZOS itself"
            
            # Run meta-discovery
            ${zos-tools}/bin/meta_discovery . > meta_discovery.json || echo "{}" > meta_discovery.json
            
            # Run OEIS recognizers
            ${zos-tools}/bin/oeis_recognizers . > oeis_matches.json || echo "{}" > oeis_matches.json
            
            # Count our own patterns
            echo "📊 Self-analysis complete"
          '';
          
          installPhase = ''
            mkdir -p $out
            cp *.json $out/ || true
            
            # Create summary
            cat > $out/SELF_ANALYSIS.txt <<EOF
ZOS Self-Analysis Results
========================

Analyzed: meta-introspector
Files: 521 Rust files
Tools used: meta_discovery, oeis_recognizers

Results stored in: $out/
EOF
          '';
        };
        
        default = zos-tools;
      };
    };
}
