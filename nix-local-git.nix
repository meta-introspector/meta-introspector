{ pkgs ? import <nixpkgs> {} }:

let
  # Configure git to use local mirror
  gitConfigFile = pkgs.writeText "gitconfig" ''
    [url "file:///mnt/data1/git/github.com/"]
      insteadOf = https://github.com/
      insteadOf = git@github.com:
    [url "file:///mnt/data1/git/gitlab.com/"]
      insteadOf = https://gitlab.com/
      insteadOf = git@gitlab.com:
    [url "file:///mnt/data1/git/huggingface.co/"]
      insteadOf = https://huggingface.co/
  '';

  # Wrapper that sets GIT_CONFIG_GLOBAL
  buildWithLocalGit = drv: drv.overrideAttrs (old: {
    GIT_CONFIG_GLOBAL = gitConfigFile;
    nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ pkgs.git ];
  });

in {
  # Export the wrapper function
  inherit buildWithLocalGit;
  
  # Example: Build a package using local git
  examplePackage = buildWithLocalGit (pkgs.rustPlatform.buildRustPackage {
    pname = "example";
    version = "0.1.0";
    
    src = pkgs.fetchgit {
      url = "https://github.com/meta-introspector/meta-introspector";
      rev = "main";
      sha256 = ""; # Will use local mirror
    };
    
    cargoLock.lockFile = ./Cargo.lock;
  });
}
