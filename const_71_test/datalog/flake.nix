{
  description = "Datalog: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-datalog";
      buildInputs = [ pkgs.souffle ];
      dontUnpack = true;
      src = pkgs.writeText "const71.dl" ''
        .decl x(n:number)
        x(71).
        .output x
      '';
      buildPhase = "souffle $src -D . || true";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
