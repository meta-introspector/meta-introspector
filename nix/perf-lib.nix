{
  description = "Standard perf recording library - canonical implementation";
  
  outputs = { self, nixpkgs }: {
    # Standard perf recording function
    lib.perfRecord = { pkgs, name, command, output ? "$out/perf" }: ''
      mkdir -p ${output}
      ${pkgs.linuxPackages.perf}/bin/perf record \
        -o ${output}/${name}.perf.data \
        -F 99 \
        -g \
        --call-graph dwarf \
        -- ${command}
    '';
    
    # Standard perf build wrapper
    lib.perfBuild = { pkgs, name, buildCommand }: pkgs.stdenv.mkDerivation {
      inherit name;
      buildPhase = self.lib.perfRecord {
        inherit pkgs name;
        command = buildCommand;
      };
      installPhase = ''
        # Perf data already in $out/perf from buildPhase
        echo "Perf data: $out/perf/${name}.perf.data"
      '';
    };
    
    # Overlay for adding perf recording to any derivation
    overlays.perf = final: prev: {
      withPerf = drv: drv.overrideAttrs (old: {
        buildPhase = ''
          ${self.lib.perfRecord {
            pkgs = final;
            name = drv.name;
            command = old.buildPhase or "make";
          }}
          ${old.buildPhase or ""}
        '';
      });
    };
    
    # Standard scripts
    apps = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        # Interactive perf recording
        perf-build = {
          type = "app";
          program = "${pkgs.writeShellScript "perf-build" ''
            if [ $# -lt 1 ]; then
              echo "Usage: perf-build <flake-ref>"
              exit 1
            fi
            
            timestamp=$(date +%Y%m%d_%H%M%S)
            output="perf_$timestamp.data"
            
            ${pkgs.linuxPackages.perf}/bin/perf record \
              -o "$output" \
              -F 99 \
              -g \
              --call-graph dwarf \
              -- ${pkgs.nix}/bin/nix build "$@" --print-build-logs
            
            echo "Perf data: $output"
          ''}";
        };
      }
    );
  };
}
