{
  description = "Perl const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "perl-71";
      src = pkgs.writeText "const71.pl" ''
        my $x = 71;
        print "x = $x\n";
      '';
      nativeBuildInputs = [ pkgs.perl ];
      buildPhase = ''
        perl $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.pl
        cat > $out/bin/perl-71 << 'EOF'
#!/bin/sh
${pkgs.perl}/bin/perl $(dirname $0)/const71.pl
EOF
        chmod +x $out/bin/perl-71
      '';
    };
  };
}
