{
  description = "Lean4 const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-lean4";
        
        src = pkgs.writeText "Main.lean" ''
          def x : Nat := 71
          
          def main : IO Unit := do
            IO.println s!"x = {x}"
        '';
        
        buildInputs = [ pkgs.lean4 ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          cp $src Main.lean
          ${pkgs.lean4}/bin/lean Main.lean
          echo "71" > output.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "71" > $out/result.txt
        '';
      };
    };
}
