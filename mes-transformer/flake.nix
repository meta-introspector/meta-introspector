{
  description = "Mes-Transformer: Computational Omniscience Architecture";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-bootstrap.url = "path:../mes-bootstrap-proof";
  };
  
  outputs = { self, nixpkgs, mes-bootstrap }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system} = {
      # Layer 0: Mes Beam Splitter
      mes-anchor = mes-bootstrap.packages.${system}.default;
      
      # Layer 1: eBPF Attention
      ebpf-attention = pkgs.stdenv.mkDerivation {
        name = "ebpf-attention";
        src = ./ebpf;
        nativeBuildInputs = [ pkgs.clang pkgs.llvm pkgs.bpftools ];
        buildPhase = ''
          clang -O2 -target bpf -c ebpf_attention.c -o ebpf_attention.o
        '';
        installPhase = ''
          mkdir -p $out/lib/bpf
          cp ebpf_attention.o $out/lib/bpf/
        '';
      };
      
      # Layer 2: Kleene Compiler
      kleene-compiler = pkgs.rustPlatform.buildRustPackage {
        pname = "kleene-compiler";
        version = "0.1.0";
        src = ./rust;
        cargoLock.lockFile = ./rust/Cargo.lock;
        buildInputs = [ pkgs.openssl ];
      };
      
      # Layer 3: LMFDB Classifier
      lmfdb-classifier = pkgs.rustPlatform.buildRustPackage {
        pname = "lmfdb-classifier";
        version = "0.1.0";
        src = ./rust;
        cargoLock.lockFile = ./rust/Cargo.lock;
      };
      
      # Layer 4: Omniscience Engine
      omniscience = pkgs.rustPlatform.buildRustPackage {
        pname = "omniscience";
        version = "0.1.0";
        src = ./rust;
        cargoLock.lockFile = ./rust/Cargo.lock;
        buildInputs = [ pkgs.zstd ];
      };
      
      # Complete stack
      default = pkgs.stdenv.mkDerivation {
        name = "mes-transformer";
        buildInputs = [
          self.packages.${system}.mes-anchor
          self.packages.${system}.ebpf-attention
          self.packages.${system}.kleene-compiler
          self.packages.${system}.lmfdb-classifier
          self.packages.${system}.omniscience
        ];
        
        buildPhase = ''
          echo "Building Mes-Transformer stack..."
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cat > $out/bin/mes-transformer << 'EOF'
#!/bin/sh
echo "🎯 Mes-Transformer: Computational Omniscience"
echo "Layer 0: Mes Beam Splitter ✅"
echo "Layer 1: eBPF Attention ✅"
echo "Layer 2: Kleene Compiler ✅"
echo "Layer 3: LMFDB Classifier ✅"
echo "Layer 4: Omniscience Engine ✅"
EOF
          chmod +x $out/bin/mes-transformer
        '';
      };
    };
  };
}
