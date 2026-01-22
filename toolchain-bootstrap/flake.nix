{
  description = "Level 1: Toolchain Bootstrap Recording";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    # Record building bash from source
    bash-toolchain = pkgs.bash.overrideAttrs (old: {
      nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ pkgs.perf ];
      
      preBuild = ''
    # TODO: Migrate to use perf-lib.lib.perfBuild
    # See: docs/nix/PERF_FLAKE_TEMPLATE.md
        # Start # Use: perf-lib.lib.perfBuild (see docs/nix/PERF_FLAKE_TEMPLATE.md)
    # TODO: Migrate to use perf-lib.lib.perfBuild
    # See: docs/nix/PERF_FLAKE_TEMPLATE.md
        # Use: perf-lib.lib.perfBuild (see docs/nix/PERF_FLAKE_TEMPLATE.md)
        PERF_PID=$!
        echo $PERF_PID > /tmp/perf.pid
      '' + (old.preBuild or "");
      
      postBuild = ''
        # Stop perf
        kill -INT $(cat /tmp/perf.pid) || true
        wait $(cat /tmp/perf.pid) || true
      '' + (old.postBuild or "");
    });
    
  in {
    packages.${system} = {
      inherit bash-toolchain;
      default = bash-toolchain;
    };
  };
}
