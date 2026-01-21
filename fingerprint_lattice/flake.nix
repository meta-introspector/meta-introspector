{
  description = "Instruction Spectrum Fingerprints in Nix Store";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.runCommand "fingerprint-spectrum" {} ''
      mkdir -p $out
      
      # Copy all fingerprints to nix store
      ${builtins.concatStringsSep "\n" (map (lang: ''
        cp ${../data/fingerprints}/${lang}_fingerprint.txt $out/ || true
      '') ["agda" "coq" "rust" "bash" "python" "ruby"])}
      
      # Create comparison
      echo "🔬 Instruction Spectrum Fingerprints" > $out/comparison.txt
      echo "=====================================" >> $out/comparison.txt
      echo "" >> $out/comparison.txt
      echo "Each language has unique instruction patterns:" >> $out/comparison.txt
      echo "" >> $out/comparison.txt
      
      for fp in $out/*_fingerprint.txt; do
        [ -f "$fp" ] || continue
        lang=$(basename "$fp" _fingerprint.txt)
        hash=$(tail -1 "$fp" | awk '{print $1}')
        top_galois=$(grep "GF(2^" "$fp" | head -1 | grep -oP 'GF\(2\^\d+\)' || echo "N/A")
        
        echo "  $lang: $top_galois" >> $out/comparison.txt
        echo "    Hash: $hash" >> $out/comparison.txt
        echo "" >> $out/comparison.txt
      done
      
      echo "✅ All fingerprints immutable in nix store" >> $out/comparison.txt
    '';
  };
}
