{
  description = "Solana program: Raydium_AMM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Raydium_AMM";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 program.so
          echo "Program: Raydium_AMM" > info.txt
          echo "Address: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" > $out/address.txt
          echo "Raydium_AMM" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
