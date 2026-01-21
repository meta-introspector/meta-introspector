{
  description = "LaTeX const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "latex-71";
      src = pkgs.writeText "const71.tex" ''
        \documentclass{article}
        \begin{document}
        \newcommand{\constx}{71}
        The value is: \constx
        \end{document}
      '';
      
      nativeBuildInputs = [ pkgs.texlive.combined.scheme-basic ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.texlive.combined.scheme-basic}/bin/pdflatex $src
        ${pkgs.texlive.combined.scheme-basic}/bin/pdftotext const71.pdf output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp const71.pdf $out/
        echo "71" > $out/result.txt
      '';
    };
  };
}
