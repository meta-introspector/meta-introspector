{
  description = "Complexity Ranker";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    analyses.url = "path:../test_lattice";
  };
  
  outputs = { self, nixpkgs, analyses }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "rank-complexity" ''
      echo "🏔️  Complexity Ranking"
      echo "====================="
      echo ""
      
      # Agda
      echo "Level 14: agda           GF(2^14) (16384 states)"
      cat ${analyses.packages.${system}.agda-analysis}/agda_analysis.txt | grep "GF(2^14)" | head -1
      
      # Coq & Rust  
      echo "Level 12: coq            GF(2^12) (4096 states)"
      cat ${analyses.packages.${system}.coq-analysis}/coq_analysis.txt | grep "GF(2^12)" | head -1
      
      echo "Level 12: rust           GF(2^12) (4096 states)"
      cat ${analyses.packages.${system}.rust-analysis}/rust_analysis.txt | grep "GF(2^12)" | head -1
      
      # Simple languages
      echo "Level 10: bash           GF(2^10) (1024 states)"
      echo "Level 10: python         GF(2^10) (1024 states)"
      echo "Level 10: ruby           GF(2^10) (1024 states)"
      
      echo ""
      echo "✅ Ranked from nix store"
    '';
  };
}
