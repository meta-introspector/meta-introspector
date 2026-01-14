{
  description = "Nix derivation: const x=71";
  outputs = { self }: {
    packages.x86_64-linux.default = derivation {
      name = "const-71";
      system = "x86_64-linux";
      builder = "/bin/sh";
      args = [ "-c" "echo 71 > $out" ];
    };
  };
}
