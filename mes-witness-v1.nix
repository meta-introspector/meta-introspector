{ pkgs ? import <nixpkgs> {} }:

let
  # GNU Mes with full telemetry
  mes-witness = pkgs.stdenv.mkDerivation {
    pname = "mes-witness-v1";
    version = "0.27.1";
    
    src = /mnt/data1/nix/time/2024/05/30/mes;
    
    nativeBuildInputs = with pkgs; [
      strace
      linuxPackages.perf
      gdb
    ];
    
    # Capture everything
    preBuild = ''
      mkdir -p $out/telemetry
      export TELEMETRY_DIR=$out/telemetry
      
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        -e cycles,instructions,cache-misses,branch-misses \
        -g --call-graph dwarf &
      PERF_PID=$!
      
      echo "📊 Perf recording started (PID: $PERF_PID)"
    '';
    
    buildPhase = ''
      echo "🔨 Building Mes with full telemetry..."
      
      # Wrap build with strace
      strace -f -o $TELEMETRY_DIR/mes_bootstrap.strace \
        -e trace=all \
        -s 1024 \
        -tt -T \
        bash ./build.sh 2>&1 | tee $TELEMETRY_DIR/mes_build.log
    '';
    
    postBuild = ''
      # Stop perf
      kill $PERF_PID 2>/dev/null || true
      wait $PERF_PID 2>/dev/null || true
      
      # Generate perf report
      perf report -i $TELEMETRY_DIR/mes_bootstrap.perf.data \
        --stdio > $TELEMETRY_DIR/mes_perf_report.txt
      
      # Extract syscall summary
      grep -E "^[0-9]+" $TELEMETRY_DIR/mes_bootstrap.strace | \
        awk '{print $NF}' | sort | uniq -c | sort -rn \
        > $TELEMETRY_DIR/syscall_summary.txt
      
      echo "✅ Telemetry captured"
    '';
    
    installPhase = ''
      mkdir -p $out/bin $out/lib $out/share
      
      # Install Mes
      cp -r bin/* $out/bin/ || true
      cp -r lib/* $out/lib/ || true
      cp -r share/* $out/share/ || true
      
      # Install telemetry
      echo "📊 Telemetry saved to: $out/telemetry"
      ls -lh $out/telemetry/
    '';
    
    meta = {
      description = "GNU Mes bootstrap with full telemetry (witness v1)";
      license = pkgs.lib.licenses.gpl3Plus;
    };
  };

in mes-witness
