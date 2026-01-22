{
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
    lib.wrapWithPerf = { pkgs, drv, name ? "${drv.name}-with-perf" }:
      pkgs.stdenv.mkDerivation {
        inherit name;
        src = drv.src or ./.;
        
        buildInputs = (drv.buildInputs or []) ++ [ pkgs.linuxPackages.perf ];
        
        buildPhase = ''
          # Record perf during original build
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        '';
        
        installPhase = ''
          # Install original outputs
          ${drv.installPhase or ""}
          
          # Add perf data
          mkdir -p $out/perf
          cp build.perf.data $out/perf/
          
          # Store metadata
          cat > $out/perf/metadata.json <<EOF
          {
            "timestamp": "$(date -Iseconds)",
            "original_drv": "${drv.name}",
            "perf_samples": $(perf report -i build.perf.data --stdio | grep -c "^#" || echo 0)
          }
          EOF
        '';
      };
    
    # Overlay that adds perf to all packages
    overlays.default = final: prev: {
      withPerf = name: 
        self.lib.wrapWithPerf {
          pkgs = final;
          drv = prev.${name};
        };
    };
  };
}
