{
  description = "Meta-Perf: perf recording perf analyzing perf - convergent labeling model";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "meta-perf-convergence";
          version = "0.1.0";
          
          src = ./.;
          
          nativeBuildInputs = with pkgs; [ linuxPackages.perf ];
          
          buildPhase = ''
            mkdir -p $out/meta-perf $out/convergence
            
            echo "🔬 Meta-Perf: Self-Referential Code Labeling"
            echo "============================================="
            echo ""
            
            # Level 0: Record some initial code
            echo "📊 Level 0: Recording initial program..."
            perf record -o $out/meta-perf/level0.perf.data \
              -F 997 -g -- sleep 0.1
            
            # Level 1: Record perf analyzing level 0
            echo "📊 Level 1: Recording perf script analyzing level 0..."
            perf record -o $out/meta-perf/level1.perf.data \
              -F 997 -g -- \
              perf script -i $out/meta-perf/level0.perf.data > /dev/null
            
            # Level 2: Record perf analyzing level 1 (analyzing level 0)
            echo "📊 Level 2: Recording perf script analyzing level 1..."
            perf record -o $out/meta-perf/level2.perf.data \
              -F 997 -g -- \
              perf script -i $out/meta-perf/level1.perf.data > /dev/null
            
            # Level 3: Record perf analyzing level 2 (analyzing level 1 analyzing level 0)
            echo "📊 Level 3: Recording perf script analyzing level 2..."
            perf record -o $out/meta-perf/level3.perf.data \
              -F 997 -g -- \
              perf script -i $out/meta-perf/level2.perf.data > /dev/null
            
            echo ""
            echo "📈 Analyzing convergence..."
            
            # Extract unique IPs from each level
            for level in 0 1 2 3; do
              perf script -i $out/meta-perf/level$level.perf.data -F ip \
                | grep -v '^#' | sort -u > $out/convergence/level$level.ips
              
              IP_COUNT=$(wc -l < $out/convergence/level$level.ips)
              SIZE=$(du -h $out/meta-perf/level$level.perf.data | cut -f1)
              
              echo "  Level $level: $IP_COUNT unique IPs, $SIZE perf data"
            done
            
            echo ""
            echo "🎯 Functional Analysis of Labeling System:"
            echo "=========================================="
            
            # Extract instruction frequencies from perf analyzing perf
            # This shows HOW the labeling system works
            for level in 1 2 3; do
              echo ""
              echo "Level $level: Labeling system instruction profile"
              
              # Get top instructions used by perf script
              perf report -i $out/meta-perf/level$level.perf.data \
                --stdio --no-children -n --percent-limit 1 \
                | head -30 > $out/convergence/level$level.profile
              
              # Extract symbol frequencies (what functions perf uses to label)
              perf script -i $out/meta-perf/level$level.perf.data -F sym \
                | grep -v '^#' | sort | uniq -c | sort -rn \
                > $out/convergence/level$level.symbols
              
              TOP_SYMBOL=$(head -1 $out/convergence/level$level.symbols | awk '{print $2}')
              TOP_COUNT=$(head -1 $out/convergence/level$level.symbols | awk '{print $1}')
              
              echo "  Top symbol: $TOP_SYMBOL ($TOP_COUNT samples)"
              echo "  → This is the core labeling function"
            done
            
            echo ""
            echo "📊 Labeling System Decomposition:"
            echo "  Level 1 symbols = how perf labels basic code"
            echo "  Level 2 symbols = how perf labels perf labeling"
            echo "  Level 3 symbols = meta-labeling convergence"
            echo ""
            echo "  Convergence → universal labeling vocabulary"
            
            # Compare IP sets between levels
            for level in 1 2 3; do
              prev=$((level - 1))
              
              # IPs in current level
              curr_count=$(wc -l < $out/convergence/level$level.ips)
              
              # IPs shared with previous level
              shared=$(comm -12 \
                $out/convergence/level$prev.ips \
                $out/convergence/level$level.ips | wc -l)
              
              # New IPs in current level
              new=$(comm -13 \
                $out/convergence/level$prev.ips \
                $out/convergence/level$level.ips | wc -l)
              
              convergence=$(echo "scale=2; $shared * 100 / $curr_count" | bc)
              
              echo "  Level $prev → $level:"
              echo "    Shared IPs: $shared / $curr_count ($convergence%)"
              echo "    New IPs: $new"
            done
            
            # Create convergence metadata
            cat > $out/convergence/meta.json << EOF
            {
              "concept": "meta-perf-functional-decomposition",
              "description": "perf recording perf script reveals the functional vocabulary of code labeling",
              "insight": "Instruction frequencies in perf script = how the labeling system works",
              "levels": 4,
              "outputs": {
                "profiles": "level*.profile - instruction frequencies of labeling",
                "symbols": "level*.symbols - functional vocabulary of analyzer",
                "ips": "level*.ips - unique instruction pointers"
              },
              "hypothesis": "Converged symbol frequencies = universal code labeling vocabulary",
              "derivation": "$out",
              "timestamp": "$(date -Iseconds)"
            }
            EOF
            
            echo ""
            echo "✅ Meta-Perf convergence data ready"
            echo "   Perf data: $out/meta-perf/level*.perf.data"
            echo "   IP sets: $out/convergence/level*.ips"
            echo "   Metadata: $out/convergence/meta.json"
          '';
          
          installPhase = ''
            echo ""
            echo "🧙 Meta-Perf: The wizard observes itself observing"
            echo "   Each level learns to label the previous level"
            echo "   Convergence = universal code labeling model"
          '';
        };
      }
    );
}
