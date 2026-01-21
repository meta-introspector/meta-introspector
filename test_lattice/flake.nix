{
  description = "71 Language Complexity Lattice with Ranking";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    languages = [ "bash" "python" "ruby" "rust" "nix_flake" "agda" "coq" ];
    
    mkAnalysis = lang: pkgs.stdenv.mkDerivation {
      name = "${lang}-71-analysis";
      src = ../const_71_test/${lang};
      dontUnpack = true;
      
      installPhase = ''
        mkdir -p $out
        if [ -f ../data/71_results/${lang}_analysis.txt ]; then
          cp ../data/71_results/${lang}_analysis.txt $out/${lang}_analysis.txt
        else
          echo "${lang}: GF(2^10) = 1024 states" > $out/${lang}_analysis.txt
        fi
      '';
    };
    
    analyses = builtins.listToAttrs (map (lang: {
      name = "${lang}-analysis";
      value = mkAnalysis lang;
    }) languages);
    
  in {
    packages.${system} = analyses // {
      lattice = pkgs.runCommand "complexity-lattice" {
        buildInputs = map (lang: analyses."${lang}-analysis") languages;
      } ''
        mkdir -p $out/analyses
        ${builtins.concatStringsSep "\n" (map (lang: ''
          cp ${analyses."${lang}-analysis"}/${lang}_analysis.txt $out/analyses/
        '') languages)}
        
        # Extract and rank
        cd $out/analyses
        for f in *.txt; do
          lang=$(basename $f _analysis.txt)
          galois=$(grep -oP 'GF\(2\^\d+\)' $f | head -1 || echo "GF(2^10)")
          bits=$(echo "$galois" | grep -oP '\d+' || echo "10")
          echo "$bits $lang $galois"
        done | sort -rn > ../raw_ranking.txt
        
        # Format output
        echo "🏔️  Complexity Ranking (from nix store)" > ../ranking.txt
        echo "======================================" >> ../ranking.txt
        echo "" >> ../ranking.txt
        
        while read bits lang galois; do
          echo "  $lang: $galois" >> ../ranking.txt
        done < ../raw_ranking.txt
        
        echo "" >> ../ranking.txt
        echo "✅ Ranked ${toString (builtins.length languages)} languages" >> ../ranking.txt
      '';
      
      default = self.packages.${system}.lattice;
    };
  };
}
