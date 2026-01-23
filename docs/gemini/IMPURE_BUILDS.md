# Gemini Impure Builds

## Overview

Impure Nix builds allow AI-driven development by enabling network access during build time to call Gemini API for code generation, analysis, and telemetry capture.

## Key Concept

Traditional Nix builds are **pure** - no network, deterministic outputs. Impure builds use `__impure = true` to bypass this for AI integration.

## Existing Implementations

### 1. ✅ Working: Consolidated Impure Gemini Telemetry Test (September 2025)
**Location**: `streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telemetry/`

**Status**: WORKING - Complete test with credential handling

Features:
- Full `__impure` derivation with telemetry capture
- Credential management (oauth, google accounts, settings)
- Timeout-protected Gemini CLI calls
- JSON telemetry output
- Apps for direct CLI access

Key components:
```nix
__impure = true;
GEMINI_API_KEY = builtins.getEnv "GEMINI_API_KEY";
buildInputs = [ nodejs_22 jq curl gemini-cli ];
```

### 2. Working: Gemini CLI Build (September 2025)
**Location**: `streamofrandom/2025/09/25/rungemini/`

**Status**: Successfully built gemini-cli with npm deps

Features:
- Override `npmDepsHash` for reproducible builds
- Dev shell with gemini-cli available
- Documented npm dependency challenges

### 3. Modular Structure (October 2025)
**Location**: `streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules/`

Modular structure with:
- `impure-gemini-telemetry-derivation.nix` - Core `__impure` derivation
- `lib-run-telemetry.nix` - Library wrapper
- `apps-default.nix` - Telemetry viewer
- `inputs.nix` - Flake inputs (gemini-cli fork)

### 4. Gemini Integration (September 2025)
**Location**: `streamofrandom/2025/09/27/7-concepts/2-gemini-integration/`

Consolidated flake with:
- CLI runner from derivation
- Python API consumer
- Dev shell with home directory access
- **Note**: Has syntax errors (duplicate `buildInputs`)

## Implementation

### Core Derivation Pattern

```nix
pkgs.stdenv.mkDerivation {
  pname = "gemini-impure-build";
  version = "1.0";
  
  __impure = true;  # Enable network access
  
  buildInputs = [
    pkgs.nodejs_22
    pkgs.jq
    pkgs.curl
    pkgs.cacert
    gemini-cli.packages.${system}.default
  ];
  
  NIX_BUILD_TELEMETRY = "true";
  FLAKE_NIX_CONTENT = flakeNixContent;
  
  buildPhase = ''
    # Call Gemini API during build
    gemini "Generate code for X"
    # Capture telemetry
  '';
}
```

### Required Nix Flags

```bash
nix build --impure --extra-experimental-features "nix-command flakes impure-derivations"
```

## Use Cases

1. **AI-Driven Code Generation** - Generate Rust from Python during build
2. **Build-Time Analysis** - Ask Gemini to analyze build errors
3. **Telemetry Capture** - Record AI interactions for meta-analysis
4. **Self-Evolving Builds** - Builds that improve themselves via AI

## Integration with Meta-Introspector

### Evolution Pipeline

```
Build Error → Impure Derivation → Gemini API → Fix → Rebuild
```

### Telemetry Flow

```
Nix Build → __impure → Gemini CLI → Telemetry → Parquet → Analysis
```

## Example: Working Impure Telemetry Test

From `streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telemetry/`:

```nix
{
  description = "Consolidated Impure Gemini CLI telemetry capture";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    gemini-cli.url = "github:meta-introspector/gemini-cli?ref=feature/CRQ-016-nixify-2025-10-06";
    credsSourceDir.url = "path:./default-creds-source";
  };

  outputs = { self, nixpkgs, gemini-cli, credsSourceDir, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      
      impureGeminiTelemetry = pkgs.stdenv.mkDerivation {
        pname = "consolidated-impure-gemini-telemetry";
        version = "1.0";
        
        __impure = true;
        
        buildInputs = [
          pkgs.nodejs_22
          pkgs.jq
          pkgs.curl
          gemini-cli.packages.x86_64-linux.default
        ];
        
        GEMINI_API_KEY = builtins.getEnv "GEMINI_API_KEY";
        
        buildPhase = ''
          mkdir -p $out/{logs,telemetry}
          
          # Run Gemini with timeout protection
          timeout 60 ${gemini-cli.packages.x86_64-linux.default}/bin/gemini \
            "test prompt" 2>&1 | tee $out/logs/impure-telemetry.log
          
          # Create telemetry summary
          cat > $out/telemetry/summary.json << EOF
          {
            "timestamp": "$(date -Iseconds)",
            "test_type": "impure_derivation",
            "status": "completed"
          }
          EOF
        '';
      };
    in
    {
      packages.x86_64-linux.default = impureGeminiTelemetry;
      
      apps.x86_64-linux.gemini = {
        type = "app";
        program = "${gemini-cli.packages.x86_64-linux.default}/bin/gemini";
      };
    };
}
```

### Running the Test

```bash
cd streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telemetry

# Build with impure flag
nix build --impure --extra-experimental-features "impure-derivations"

# Run gemini directly
nix run .#gemini -- "test prompt"

# Check telemetry
cat result/telemetry/summary.json
cat result/logs/impure-telemetry.log
```

## Known Issues

1. **Nix Daemon Connection** - Impure builds may fail with "Connection reset by peer"
2. **Syntax Errors** - Some existing flakes have duplicate attribute definitions
3. **API Keys** - Require `GEMINI_API_KEY` environment variable
4. **Reproducibility** - Impure builds are not reproducible by design

## References

- **Consolidated Implementation**: `time-2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules/`
- **Gemini CLI Fork**: `github:meta-introspector/gemini-cli?ref=feature/CRQ-016-nixify-2025-10-06`
- **Tutorial**: `time-2025/09/27/7-concepts/4-documentation/TUTORIAL_GEMINI_CLI_NIX_INTEGRATION.md`
- **Test Scripts**: `time-2025/09/25/run_gemini_prompt_derivation.sh`

## Security Considerations

- Impure builds are **not reproducible**
- Network access required (API keys)
- Use only in development/evolution pipelines
- Final artifacts should be pure builds

## Next Steps

1. Fix syntax errors in existing flakes
2. Integrate with `evolution_server.py`
3. Use for Python → Rust lifting pipeline
4. Capture telemetry in proven builds
5. Enable self-rewriting via AI feedback
