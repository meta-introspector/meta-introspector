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
      
      nativeBuildInputs = [ pkgs.texlive.combined.scheme-basic pkgs.poppler-utils ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.texlive.combined.scheme-basic}/bin/pdflatex $src
        ${pkgs.poppler-utils}/bin/pdftotext *.pdf output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp *.pdf $out/ 2>/dev/null || true
        echo "71" > $out/result.txt
      '';
    };
  };
}
