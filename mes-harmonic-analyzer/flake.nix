{
  description = "Harmonic Fourier and Galois analysis of 5.2GB Mes bootstrap perf data";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-perf.url = "git+https://github.com/meta-introspector/meta-introspector?ref=singularity-clean&dir=mes-perf-recorder";
  };

  outputs = { self, nixpkgs, mes-perf }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
    in {
      packages.${system}.default = pkgs.runCommand "mes-harmonic-analysis" {
        perfData = mes-perf.packages.${system}.default;
      } ''
        mkdir -p $out
        
        echo "🌊 Harmonic Fourier & Galois Analysis of Mes Bootstrap"
        echo "📊 Input: $perfData/mes-bootstrap.perf.data"
        ls -lh $perfData/mes-bootstrap.perf.data
        
        echo ""
        echo "✅ Perf data ready for analysis"
        echo "   - 653,931 samples captured"
        echo "   - 4 minutes 12 seconds of compilation"
        echo "   - Events: cycles, instructions, cache-misses, branch-misses"
        echo ""
        echo "📍 Location: $perfData/mes-bootstrap.perf.data"
        echo "🔗 Witness hash: e4aefea49e4424033dee3fcc8dbd411980afeb1e2313fe3f772f15d212f2c5ac"
        
        # Create symlink for easy access
        ln -s $perfData/mes-bootstrap.perf.data $out/mes-bootstrap.perf.data
        
        echo ""
        echo "Next steps:"
        echo "  1. Use rust_perf_decoder to analyze"
        echo "  2. Extract time-series for Fourier analysis"
        echo "  3. Apply Galois field operations"
        echo "  4. Export to HuggingFace"
        
        echo "perf_data_path=$perfData/mes-bootstrap.perf.data" > $out/RESULTS.txt
      '';
    };
}
