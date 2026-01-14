{
  description = "Solana program: Serum_DEX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Serum_DEX";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin program.so
          echo "Program: Serum_DEX" > info.txt
          echo "Address: 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" > $out/address.txt
          echo "Serum_DEX" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
