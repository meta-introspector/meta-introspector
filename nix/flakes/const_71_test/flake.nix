{
  description = "Meta-introspector complete system build";
  
  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Import all 71 language tests
      const71 = import ./const_71_test { inherit pkgs; };
      
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "meta-introspector-complete";
        
        buildInputs = [ pkgs.jq ];
        
        buildPhase = ''
          mkdir -p $out/{bin,perf,logs,.meta-introspector}
          
          # Collect all 71 language outputs
          ${pkgs.lib.concatMapStringsSep "\n" (lang: ''
            if [ -d "${const71.${lang}}" ]; then
              cp -r ${const71.${lang}}/* $out/ 2>/dev/null || true
            fi
          '') (builtins.attrNames const71)}
          
          # Collect all perf data
          find ${const71} -name "*.perf.data" -exec cp {} $out/perf/ \; 2>/dev/null || true
          
          # Generate metadata
          cat > $out/.meta-introspector/metadata.json <<EOF
          {
            "version": "1.0",
            "timestamp": "$(date -Iseconds)",
            "commit": "${self.rev or "dirty"}",
            "languages": ${builtins.length (builtins.attrNames const71)},
            "perf_traces": $(find $out/perf -name "*.perf.data" | wc -l),
            "store_path": "$out"
          }
          EOF
          
          # Create build log
          cat > $out/logs/build.log <<EOF
          Meta-introspector complete build
          =================================
          Timestamp: $(date -Iseconds)
          Commit: ${self.rev or "dirty"}
          Languages: ${builtins.length (builtins.attrNames const71)}
          Perf traces: $(find $out/perf -name "*.perf.data" | wc -l)
          EOF
        '';
        
        installPhase = ''
          echo "Complete system in: $out"
        '';
      };
    };
}
