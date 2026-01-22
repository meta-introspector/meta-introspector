{
  description = "PHP const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "php-71";
      src = pkgs.writeText "const71.php" ''
        <?php
        $x = 71;
        echo "x = $x\n";
      '';
      nativeBuildInputs = [ pkgs.php ];
      dontUnpack = true;
      buildPhase = ''
        php $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.php
        cat > $out/bin/php-71 << 'EOF'
#!/bin/sh
${pkgs.php}/bin/php $(dirname $0)/const71.php
EOF
        chmod +x $out/bin/php-71
      '';
    };
  };
}
