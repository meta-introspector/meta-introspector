{
  description = "GNU Mes bootstrap baseline - GF(2^19)";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-perf.url = "git+https://github.com/meta-introspector/meta-introspector?ref=singularity-clean&dir=mes-perf-recorder";
  };
  
  outputs = { self, nixpkgs, mes-perf }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "mes-const71" ''
      # Use the full Mes bootstrap perf data as reference
      # This is our GF(2^19) baseline
      echo "71"
      echo "Galois Baseline: GF(2^19) = 524,288 states"
      echo "Perf data: ${mes-perf.packages.${system}.default}/mes-bootstrap.perf.data"
    '';
    
    # Reference to actual bootstrap
    apps.${system}.bootstrap = {
      type = "app";
      program = "${mes-perf.packages.${system}.default}/bin/record-mes";
    };
  };
}
