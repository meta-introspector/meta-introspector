{
  description = "Haskell const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-haskell";
        
        src = pkgs.writeText "Main.hs" ''
          main :: IO ()
          main = let x = 71 in putStrLn $ "x = " ++ show x
        '';
        
        buildInputs = [ pkgs.ghc ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          ghc -O0 -o const-71-haskell $src
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-haskell $out/bin/
        '';
      };
    };
}
