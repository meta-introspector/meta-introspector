{
  description = "Solana program: Orca_Whirlpool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Orca_Whirlpool";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc program.so
          echo "Program: Orca_Whirlpool" > info.txt
          echo "Address: whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" > $out/address.txt
          echo "Orca_Whirlpool" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
