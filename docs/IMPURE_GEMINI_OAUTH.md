# Impure Gemini OAuth Integration

## Overview

Impure nix builds for Gemini CLI with OAuth credential handling and telemetry capture.

## Location

Source: `/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules/`

## Key Features

### 1. Impure Derivation (`__impure = true`)

Allows access to environment variables and external credentials during build:

```nix
__impure = true;
GEMINI_API_KEY = builtins.getEnv "GEMINI_API_KEY";
```

### 2. OAuth Credential Handling

Copies OAuth credentials from source directory to Gemini config:

```bash
GEMINI_CONFIG_DIR="/tmp/.gemini"
cp creds/oauth_creds.json "$GEMINI_CONFIG_DIR/"
cp creds/google_accounts.json "$GEMINI_CONFIG_DIR/"
cp creds/settings.json "$GEMINI_CONFIG_DIR/"
```

### 3. Telemetry Capture

Captures all Gemini CLI interactions with timestamps and metadata:

```json
{
  "timestamp": "2025-10-08T12:00:00Z",
  "test_type": "consolidated_impure_derivation",
  "gemini_cli_source": "github:meta-introspector/gemini-cli",
  "status": "completed"
}
```

## Usage

### Build Impure Gemini Package

```bash
cd /mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules/

# Set API key
export GEMINI_API_KEY="your-api-key"

# Build with impure flag
nix build --impure

# Run telemetry capture
nix run --impure
```

### Credential Setup

```bash
# Create creds directory
mkdir -p creds/

# Add OAuth credentials
cat > creds/oauth_creds.json << 'JSON'
{
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "refresh_token": "your-refresh-token"
}
JSON

# Add Google accounts
cat > creds/google_accounts.json << 'JSON'
{
  "accounts": [
    {
      "email": "your-email@gmail.com",
      "tokens": {...}
    }
  ]
}
JSON

# Add settings
cat > creds/settings.json << 'JSON'
{
  "model": "gemini-2.5-flash",
  "temperature": 0.7
}
JSON
```

## Integration with Meta-Introspector

### Add to Our Flake

```nix
inputs = {
  # ... existing inputs
  impure-gemini.url = "path:/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules";
};

outputs = { self, impure-gemini, ... }:
  {
    packages.impure-gemini-telemetry = impure-gemini.packages.${system}.default;
    
    apps.gemini-with-oauth = {
      type = "app";
      program = "${impure-gemini.packages.${system}.default}/bin/gemini";
    };
  };
```

### Use with Mining Demos

```bash
# Run branch mining with OAuth-enabled Gemini
export GEMINI_API_KEY="your-key"

# Capture telemetry
nix run --impure .#gemini-with-oauth -- \
  "Analyze branch predictions from demo_branch_mining"

# Check telemetry logs
cat result/logs/impure-telemetry.log
cat result/telemetry/summary.json
```

## Telemetry Output Structure

```
result/
├── logs/
│   └── impure-telemetry.log    # Full execution log
└── telemetry/
    └── summary.json             # Structured telemetry data
```

## Security Considerations

1. **Never commit credentials** - Use `.gitignore` for `creds/`
2. **Use environment variables** - `GEMINI_API_KEY` from env
3. **Impure builds** - Required for credential access
4. **Temporary storage** - Credentials copied to `/tmp/.gemini`
5. **Cleanup** - Remove credentials after use

## Advanced Usage

### Batch Telemetry Capture

```bash
# Capture telemetry for all mining demos
for demo in demo_*.rs; do
  echo "Analyzing $demo..."
  nix run --impure .#gemini-with-oauth -- \
    "Analyze $demo and suggest optimizations" \
    > "telemetry/${demo%.rs}.log"
done
```

### Automated Documentation with OAuth

```bash
# Generate docs with authenticated Gemini
nix run --impure .#gemini-with-oauth -- \
  "Generate comprehensive documentation for all mining systems" \
  > docs/AUTO_GENERATED.md
```

### Mining Analysis Pipeline

```bash
# 1. Run mining
cargo run --release --bin demo_branch_mining > results.txt

# 2. Analyze with OAuth Gemini
nix run --impure .#gemini-with-oauth -- \
  "Analyze these results: $(cat results.txt)" \
  > analysis.txt

# 3. Export telemetry
cp result/telemetry/summary.json telemetry/branch_mining_$(date +%Y%m%d).json
```

## Mycology Context Integration

The impure build supports mycology framework context:

```nix
lib.runTelemetry = { mycologyContext }: 
  impureGeminiTelemetry { inherit mycologyContext; };
```

Use for:
- Distributed mining coordination
- Multi-agent analysis
- Shared telemetry aggregation

## Troubleshooting

### Credentials Not Found

```bash
# Check creds directory
ls -la creds/

# Verify GEMINI_CONFIG_DIR
echo $GEMINI_CONFIG_DIR
ls -la /tmp/.gemini/
```

### API Key Issues

```bash
# Check API key is set
echo ${#GEMINI_API_KEY}  # Should show length

# Test with simple prompt
nix run --impure .#gemini-with-oauth -- "Hello"
```

### Impure Build Fails

```bash
# Ensure --impure flag
nix build --impure

# Check for __impure = true in derivation
nix show-derivation .#impure-gemini-telemetry
```

## Next Steps

1. Integrate into meta-introspector flake
2. Set up OAuth credentials
3. Test with mining demos
4. Capture telemetry for analysis
5. Export to HuggingFace datasets

## References

- Original flake: `streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telemetry/`
- Modularized version: `streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules/`
- Gemini CLI: `github:meta-introspector/gemini-cli`
