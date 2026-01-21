{
  description = "Scheme const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "scheme-71";
      src = pkgs.writeText "const71.scm" ''
        (define x 71)
        (display "x = ")
        (display x)
        (newline)
      '';
      nativeBuildInputs = [ pkgs.guile ];
      dontUnpack = true;
      buildPhase = ''
        guile $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.scm
        cat > $out/bin/scheme-71 << 'EOF'
#!/bin/sh
${pkgs.guile}/bin/guile $(dirname $0)/const71.scm
EOF
        chmod +x $out/bin/scheme-71
      '';
    };
  };
}
