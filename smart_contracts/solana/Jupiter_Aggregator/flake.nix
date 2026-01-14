{
  description = "Solana program: Jupiter_Aggregator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Jupiter_Aggregator";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 program.so
          echo "Program: Jupiter_Aggregator" > info.txt
          echo "Address: JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4" > $out/address.txt
          echo "Jupiter_Aggregator" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
