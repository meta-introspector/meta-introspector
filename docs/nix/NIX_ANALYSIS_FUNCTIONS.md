# Nix Analysis Functions

Composable analysis functions that can be applied to any Nix build.

## Usage

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    meta-introspector.url = "github:meta-introspector/meta-introspector";
  };

  outputs = { self, nixpkgs, meta-introspector }: {
    packages.x86_64-linux.my-project = 
      let
        pkgs = import nixpkgs { system = "x86_64-linux"; };
        analyzers = meta-introspector.lib.analyzers;
      in
      analyzers.withFullAnalysis pkgs.stdenv.mkDerivation {
        name = "my-project";
        src = ./.;
        buildPhase = "make";
      };
  };
}
```

## Analysis Functions

### Source Analysis

```nix
analyzers.analyzeSource = drv: pkgs.runCommand "${drv.name}-source-analysis" {
  buildInputs = [ meta-introspector.packages.x86_64-linux.markov_resonance_analyzer ];
} ''
  mkdir -p $out
  markov_resonance_analyzer ${drv.src} > $out/markov_symbol_scores.parquet
'';
```

### Build Process Analysis

```nix
analyzers.analyzeBuild = drv: pkgs.runCommand "${drv.name}-build-analysis" {
  buildInputs = [ meta-introspector.packages.x86_64-linux.build-logs-to-parquet ];
} ''
  mkdir -p $out
  # Capture build logs
  ${drv} 2>&1 | tee build.log
  build-logs-to-parquet build.log $out/nix_build_logs.parquet
'';
```

### Output Analysis

```nix
analyzers.analyzeOutput = drv: pkgs.runCommand "${drv.name}-output-analysis" {
  buildInputs = [ 
    meta-introspector.packages.x86_64-linux.byte_provenance_tracker
    meta-introspector.packages.x86_64-linux.elf_moonshine_detector
  ];
} ''
  mkdir -p $out
  byte_provenance_tracker ${drv} > $out/byte_provenance.parquet
  elf_moonshine_detector ${drv} > $out/moonshine.parquet
'';
```

### Grammar Extraction

```nix
analyzers.extractGrammar = drv: pkgs.runCommand "${drv.name}-grammar" {
  buildInputs = [ meta-introspector.packages.x86_64-linux.nix_store_grammar ];
} ''
  mkdir -p $out
  nix_store_grammar ${drv} > $out/nix_store_grammars.parquet
'';
```

### Git Temporal Analysis

```nix
analyzers.analyzeGitHistory = src: pkgs.runCommand "git-temporal-analysis" {
  buildInputs = [ meta-introspector.packages.x86_64-linux.git_temporal_morphisms ];
} ''
  mkdir -p $out
  git_temporal_morphisms ${src} > $out/git_temporal_morphisms.parquet
'';
```

### Full Analysis Pipeline

```nix
analyzers.withFullAnalysis = drv: pkgs.symlinkJoin {
  name = "${drv.name}-analyzed";
  paths = [
    drv
    (analyzers.analyzeSource drv)
    (analyzers.analyzeBuild drv)
    (analyzers.analyzeOutput drv)
    (analyzers.extractGrammar drv)
  ];
};
```

## Complete Example

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    meta-introspector.url = "github:meta-introspector/meta-introspector";
  };

  outputs = { self, nixpkgs, meta-introspector }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      analyzers = meta-introspector.lib.analyzers;
      
      # Your project
      myProject = pkgs.rustPlatform.buildRustPackage {
        pname = "my-rust-project";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      
    in {
      packages.${system} = {
        # Original build
        default = myProject;
        
        # With source analysis
        with-source-analysis = analyzers.analyzeSource myProject;
        
        # With build analysis
        with-build-analysis = analyzers.analyzeBuild myProject;
        
        # With output analysis
        with-output-analysis = analyzers.analyzeOutput myProject;
        
        # Full analysis
        fully-analyzed = analyzers.withFullAnalysis myProject;
      };
      
      # Analysis results as separate outputs
      analysis.${system} = {
        markov = "${self.packages.${system}.with-source-analysis}/markov_symbol_scores.parquet";
        build-logs = "${self.packages.${system}.with-build-analysis}/nix_build_logs.parquet";
        provenance = "${self.packages.${system}.with-output-analysis}/byte_provenance.parquet";
      };
    };
}
```

## Integration with Existing Projects

### Wrap Existing Flake

```bash
# Add analysis to any flake
nix flake init -t github:meta-introspector/meta-introspector#analyzed-project
```

### Analyze External Project

```nix
{
  inputs.target.url = "github:some-org/some-project";
  
  outputs = { self, nixpkgs, meta-introspector, target }:
    let
      analyzers = meta-introspector.lib.analyzers;
    in {
      packages.x86_64-linux.analyzed-target = 
        analyzers.withFullAnalysis target.packages.x86_64-linux.default;
    };
}
```

## Output Structure

```
/nix/store/xxx-my-project-analyzed/
├── bin/                          # Original binaries
├── markov_symbol_scores.parquet  # Source analysis
├── nix_build_logs.parquet        # Build analysis
├── byte_provenance.parquet       # Output analysis
├── nix_store_grammars.parquet    # Grammar extraction
└── git_temporal_morphisms.parquet # Git history
```

## Querying Results

```bash
# Query any analysis result
nix run github:meta-introspector/meta-introspector#query-parquet -- \
  /nix/store/xxx-my-project-analyzed/markov_symbol_scores.parquet \
  "SELECT * FROM markov_symbol_scores LIMIT 10"
```
