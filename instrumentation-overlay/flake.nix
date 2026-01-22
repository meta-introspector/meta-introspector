{
  description = "Instrumentation Overlay - Full transparency for any build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    overlays.default = final: prev: {
      
      # Instrumented GCC with all dumps enabled
      gcc-instrumented = prev.gcc.overrideAttrs (old: {
        configureFlags = (old.configureFlags or []) ++ [
          "--enable-checking=release"
          "--enable-languages=c,c++"
        ];
        
        # Wrap to enable all dumps
        postInstall = (old.postInstall or "") + ''
          for bin in $out/bin/gcc $out/bin/g++; do
            mv $bin $bin.real
            cat > $bin <<'EOF'
#!/bin/sh
exec $bin.real \
  -fdump-tree-all \
  -fdump-rtl-all \
  -fdump-ipa-all \
  -ftime-report \
  -fmem-report \
  -save-temps=obj \
  "$@"
EOF
            chmod +x $bin
          done
        '';
      });
      
      # Instrumented LLVM with all dumps enabled
      llvm-instrumented = prev.llvm.overrideAttrs (old: {
        cmakeFlags = (old.cmakeFlags or []) ++ [
          "-DLLVM_ENABLE_DUMP=ON"
          "-DLLVM_ENABLE_ASSERTIONS=ON"
        ];
        
        postInstall = (old.postInstall or "") + ''
          for bin in $out/bin/clang $out/bin/clang++; do
            mv $bin $bin.real
            cat > $bin <<'EOF'
#!/bin/sh
exec $bin.real \
  -mllvm -print-after-all \
  -mllvm -print-before-all \
  -mllvm -time-passes \
  -ftime-trace \
  -save-temps=obj \
  "$@"
EOF
            chmod +x $bin
          done
        '';
      });
      
      # Instrumented stdenv that uses instrumented compilers
      stdenv-instrumented = prev.stdenvAdapters.overrideCC 
        prev.stdenv 
        final.gcc-instrumented;
      
      # QEMU with full tracing
      qemu-traced = prev.qemu.overrideAttrs (old: {
        configureFlags = (old.configureFlags or []) ++ [
          "--enable-trace-backends=log"
          "--enable-debug"
        ];
      });
      
      # Wrapper function for any derivation
      withFullInstrumentation = drv: drv.overrideAttrs (old: {
        name = "${old.name or drv.name}-instrumented";
        
        # Use instrumented stdenv
        stdenv = final.stdenv-instrumented;
        
        # Add instrumentation tools
        nativeBuildInputs = (old.nativeBuildInputs or []) ++ [
          final.perf-tools
          final.strace
          final.ltrace
          final.gdb
          final.valgrind
        ];
        
        # Wrap build phase with full tracing
        preBuild = (old.preBuild or "") + ''
          export NIX_DEBUG=1
          export VERBOSE=1
          
          # Create output directory for traces
          mkdir -p $out/traces
          
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
          PERF_PID=$!
          
          # Trap to stop perf on exit
          trap "kill $PERF_PID 2>/dev/null || true" EXIT
        '';
        
        postBuild = (old.postBuild or "") + ''
          # Collect all compiler dumps
          find . -name "*.dump" -o -name "*.i" -o -name "*.s" -o -name "*.bc" \
            | tar czf $out/traces/compiler-dumps.tar.gz -T -
          
          # Collect time traces
          find . -name "*.json" -path "*/time-trace/*" \
            | tar czf $out/traces/time-traces.tar.gz -T -
        '';
        
        # Add metadata
        postInstall = (old.postInstall or "") + ''
          cat > $out/traces/metadata.json <<EOF
          {
            "timestamp": "$(date -Iseconds)",
            "original_drv": "${drv.name}",
            "gcc_version": "$(${final.gcc-instrumented}/bin/gcc --version | head -1)",
            "llvm_version": "$(${final.llvm-instrumented}/bin/clang --version | head -1)",
            "instrumentation": {
              "gcc_dumps": true,
              "llvm_dumps": true,
              "perf": true,
              "time_trace": true
            }
          }
          EOF
        '';
      });
      
      # QEMU wrapper for full memory tracing
      runInQemu = drv: final.stdenv.mkDerivation {
        name = "${drv.name}-qemu-traced";
        
        buildInputs = [ final.qemu-traced ];
        
        buildPhase = ''
          # Run under QEMU with full tracing
          qemu-x86_64 \
            -d in_asm,out_asm,op,int,exec,cpu,fpu,mmu \
            -D $out/qemu-trace.log \
            ${drv}/bin/* || true
        '';
        
        installPhase = ''
          mkdir -p $out/traces
          mv qemu-trace.log $out/traces/
          
          # Parse and analyze
          cat > $out/traces/analysis.txt <<EOF
          QEMU Trace Analysis
          ===================
          Instructions executed: $(grep -c "IN:" $out/traces/qemu-trace.log || echo 0)
          Memory accesses: $(grep -c "0x" $out/traces/qemu-trace.log || echo 0)
          EOF
        '';
      };
    };
    
    # Helper function for llama.cpp specifically
    lib.instrumentLlamaCpp = { pkgs, src }:
      let
        instrumented = pkgs.stdenv.mkDerivation {
          name = "llama.cpp-fully-instrumented";
          inherit src;
          
          nativeBuildInputs = with pkgs; [
            cmake
            gcc-instrumented
            llvm-instrumented
            perf-tools
          ];
          
          cmakeFlags = [
            "-DCMAKE_BUILD_TYPE=RelWithDebInfo"
            "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON"
          ];
          
          preBuild = ''
            # Record everything
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
            PERF_PID=$!
            trap "kill $PERF_PID" EXIT
          '';
          
          postBuild = ''
            # Collect all artifacts
            mkdir -p $out/traces
            find . -name "*.dump" -o -name "*.i" -o -name "*.s" | \
              tar czf $out/traces/compiler-artifacts.tar.gz -T -
          '';
        };
      in instrumented;
  };
}
