# overlay.nix
final: prev: {
  # Custom Rust nightly toolchain with profiling
  rustNightlyProfiling = prev.rustc.override {
    # Use standard rustc with custom flags
  };

  # Build environment with profiling tools  
  rustProfilingEnv = prev.mkShell {
    buildInputs = with final; [
      rustc
      cargo
      gcc
      gdb
    ];

    shellHook = ''
      export RUSTFLAGS="-C force-frame-pointers=yes"
      export CARGO_PROFILE_RELEASE_DEBUG=true
      echo "Rust with profiling enabled"
      rustc --version
    '';
  };
}
