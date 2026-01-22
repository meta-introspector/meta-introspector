# Universal Semantic Labeler: Decompression ↔ Compilation Mapping

## Vision

Map instruction pointers between decompressor and compiler to create universal semantic labels.

## Core Insight

```
XZ decompression instruction → Compiler instruction
    ↓                              ↓
Byte in archive              → AST node
    ↓                              ↓
Semantic meaning emerges from the correlation
```

## Architecture

```
Nix Store XZ Archive
    ↓
Incremental decompression (layer by layer)
Instruction pointer trace
    ↓
    ↔ (correlation)
    ↓
Compiler instruction pointer trace
Compilation stages
    ↓
Semantic labels
```

## The Mapping

### Decompressor Side
```
IP: 0x7f1234 in xz_decompress()
    ↓ decodes
Bytes: [0x1f, 0x8b, 0x08, ...]
    ↓ produces
Text: "fn main() {"
```

### Compiler Side
```
IP: 0x7f5678 in rustc::parse::parser::parse_fn()
    ↓ processes
Text: "fn main() {"
    ↓ produces
AST: FnDecl { name: "main", ... }
```

### Correlation
```
xz_decompress @ 0x7f1234 ←→ parse_fn @ 0x7f5678
    ↓
Semantic label: "function_declaration"
```

## Implementation

### Phase 1: Dual Perf Recording

```rust
// Record decompression
fn perf_record_decompress(xz_file: &str) -> PerfTrace {
    let mut cmd = Command::new("perf");
    cmd.args([
        "record",
        "-e", "cycles,instructions",
        "-g", "--call-graph", "dwarf",
        "-o", "/tmp/decompress.perf",
        "xz", "-d", "-c", xz_file
    ]);
    
    cmd.output().expect("Failed to record");
    parse_perf_data("/tmp/decompress.perf")
}

// Record compilation
fn perf_record_compile(source: &str) -> PerfTrace {
    let mut cmd = Command::new("perf");
    cmd.args([
        "record",
        "-e", "cycles,instructions",
        "-g", "--call-graph", "dwarf",
        "-o", "/tmp/compile.perf",
        "rustc", source
    ]);
    
    cmd.output().expect("Failed to record");
    parse_perf_data("/tmp/compile.perf")
}
```

### Phase 2: Instruction Pointer Correlation

```rust
struct IPMapping {
    decompress_ip: u64,
    compiler_ip: u64,
    correlation: f64,
    semantic_label: String,
}

fn correlate_traces(
    decompress: &PerfTrace,
    compile: &PerfTrace
) -> Vec<IPMapping> {
    let mut mappings = Vec::new();
    
    // Time-align the traces
    let aligned = align_by_timestamp(decompress, compile);
    
    // Find correlated instruction pointers
    for window in aligned.windows(100) {
        let decomp_ips = window.iter()
            .filter(|e| e.trace == TraceType::Decompress)
            .map(|e| e.ip)
            .collect::<Vec<_>>();
        
        let compile_ips = window.iter()
            .filter(|e| e.trace == TraceType::Compile)
            .map(|e| e.ip)
            .collect::<Vec<_>>();
        
        // Compute correlation
        let corr = compute_correlation(&decomp_ips, &compile_ips);
        
        if corr > 0.7 {
            mappings.push(IPMapping {
                decompress_ip: decomp_ips[0],
                compiler_ip: compile_ips[0],
                correlation: corr,
                semantic_label: infer_semantic(&decomp_ips, &compile_ips),
            });
        }
    }
    
    mappings
}
```

### Phase 3: Incremental Decompression with Labeling

```rust
struct LayeredDecompression {
    layers: Vec<Layer>,
    current_offset: usize,
}

struct Layer {
    compressed_bytes: Vec<u8>,
    decompressed_bytes: Vec<u8>,
    instruction_pointers: Vec<u64>,
    semantic_labels: Vec<String>,
}

impl LayeredDecompression {
    fn decompress_layer(&mut self, size: usize) -> Layer {
        let perf_start = start_perf_recording();
        
        // Decompress next chunk
        let compressed = &self.data[self.current_offset..self.current_offset + size];
        let decompressed = xz_decompress_chunk(compressed);
        
        // Stop perf and get IPs
        let ips = stop_perf_recording(perf_start);
        
        // Map to semantic labels
        let labels = self.map_to_semantics(&ips);
        
        Layer {
            compressed_bytes: compressed.to_vec(),
            decompressed_bytes: decompressed,
            instruction_pointers: ips,
            semantic_labels: labels,
        }
    }
    
    fn map_to_semantics(&self, ips: &[u64]) -> Vec<String> {
        ips.iter()
            .map(|ip| self.ip_to_semantic.get(ip).cloned().unwrap_or_default())
            .collect()
    }
}
```

### Phase 4: Mathematical Transform Documentation

```rust
struct Transform {
    input: Vec<u8>,
    output: Vec<u8>,
    instruction_sequence: Vec<Instruction>,
    mathematical_operation: MathOp,
}

enum MathOp {
    LZ77Decode { distance: usize, length: usize },
    HuffmanDecode { tree: HuffmanTree },
    DeltaFilter { delta: i32 },
    RangeDecoder { range: (u64, u64) },
}

fn document_transform(layer: &Layer) -> Transform {
    // Analyze instruction sequence
    let ops = analyze_instructions(&layer.instruction_pointers);
    
    // Identify mathematical operation
    let math_op = classify_operation(&ops);
    
    Transform {
        input: layer.compressed_bytes.clone(),
        output: layer.decompressed_bytes.clone(),
        instruction_sequence: ops,
        mathematical_operation: math_op,
    }
}
```

### Phase 5: Compiler Stage Mapping

```rust
enum CompilerStage {
    Lexing,
    Parsing,
    MacroExpansion,
    NameResolution,
    TypeChecking,
    BorrowChecking,
    MIRGeneration,
    Optimization,
    CodeGen,
}

fn map_compiler_stage(ip: u64, symbols: &SymbolTable) -> CompilerStage {
    let func_name = symbols.lookup(ip);
    
    match func_name.as_str() {
        s if s.contains("lex") => CompilerStage::Lexing,
        s if s.contains("parse") => CompilerStage::Parsing,
        s if s.contains("expand") => CompilerStage::MacroExpansion,
        s if s.contains("resolve") => CompilerStage::NameResolution,
        s if s.contains("typeck") => CompilerStage::TypeChecking,
        s if s.contains("borrow") => CompilerStage::BorrowChecking,
        s if s.contains("mir") => CompilerStage::MIRGeneration,
        s if s.contains("opt") => CompilerStage::Optimization,
        s if s.contains("codegen") => CompilerStage::CodeGen,
        _ => CompilerStage::Lexing,
    }
}
```

### Phase 6: Universal Semantic Labeler

```rust
struct UniversalLabeler {
    decompress_to_compiler: HashMap<u64, u64>,
    ip_to_semantic: HashMap<u64, String>,
    byte_to_semantic: HashMap<Vec<u8>, String>,
}

impl UniversalLabeler {
    fn label_bytes(&self, bytes: &[u8]) -> Vec<String> {
        let mut labels = Vec::new();
        
        // Find which decompressor IPs process these bytes
        let decomp_ips = self.find_decompress_ips(bytes);
        
        // Map to compiler IPs
        let compiler_ips: Vec<_> = decomp_ips.iter()
            .filter_map(|ip| self.decompress_to_compiler.get(ip))
            .collect();
        
        // Get semantic labels
        for ip in compiler_ips {
            if let Some(label) = self.ip_to_semantic.get(ip) {
                labels.push(label.clone());
            }
        }
        
        labels
    }
    
    fn train(&mut self, xz_file: &str, source_file: &str) {
        // Record both traces
        let decomp_trace = perf_record_decompress(xz_file);
        let compile_trace = perf_record_compile(source_file);
        
        // Correlate
        let mappings = correlate_traces(&decomp_trace, &compile_trace);
        
        // Build lookup tables
        for mapping in mappings {
            self.decompress_to_compiler.insert(
                mapping.decompress_ip,
                mapping.compiler_ip
            );
            
            self.ip_to_semantic.insert(
                mapping.compiler_ip,
                mapping.semantic_label
            );
        }
    }
}
```

## Example: Labeling Process

```rust
// 1. XZ archive contains compressed Rust source
let xz_bytes = read_file("/nix/store/.../rustc-1.75.0.tar.xz");

// 2. Decompress incrementally with perf
let mut decompressor = LayeredDecompression::new(xz_bytes);
let layer1 = decompressor.decompress_layer(4096);

// Layer 1 IPs: [0x7f1234, 0x7f1240, 0x7f1250, ...]
// Decompressed: "use std::collections::HashMap;\n\nfn main() {"

// 3. Compile with perf
let compile_trace = perf_record_compile("main.rs");

// Compiler IPs: [0x7f5678, 0x7f5690, 0x7f56a0, ...]
// Stage: Lexing → Parsing → TypeChecking

// 4. Correlate
// 0x7f1234 (xz_decode_lz77) ←→ 0x7f5678 (rustc::parse::token)
// 0x7f1240 (xz_decode_range) ←→ 0x7f5690 (rustc::parse::parse_fn)

// 5. Label
let labeler = UniversalLabeler::trained();
let labels = labeler.label_bytes(&layer1.compressed_bytes);
// ["keyword_use", "identifier", "keyword_fn", "function_declaration"]
```

## Correlation Strength

```rust
fn compute_correlation(decomp_ips: &[u64], compile_ips: &[u64]) -> f64 {
    // Temporal correlation
    let temporal = temporal_correlation(decomp_ips, compile_ips);
    
    // Frequency correlation
    let frequency = frequency_correlation(decomp_ips, compile_ips);
    
    // Causality (decompress must happen before compile)
    let causality = check_causality(decomp_ips, compile_ips);
    
    (temporal + frequency) * causality
}
```

## Mathematical Documentation

```rust
struct MathematicalLayer {
    // Input space
    compressed_domain: Vec<u8>,
    
    // Transform
    operation: MathOp,
    
    // Output space
    decompressed_domain: Vec<u8>,
    
    // Instruction trace
    instruction_path: Vec<u64>,
    
    // Semantic interpretation
    meaning: String,
}

fn document_mathematically(layer: &Layer) -> MathematicalLayer {
    MathematicalLayer {
        compressed_domain: layer.compressed_bytes.clone(),
        operation: identify_math_operation(&layer.instruction_pointers),
        decompressed_domain: layer.decompressed_bytes.clone(),
        instruction_path: layer.instruction_pointers.clone(),
        meaning: infer_semantic_meaning(&layer),
    }
}
```

## Integration with Test Driver

```rust
// Add to test_driver.rs
fn analyze_nix_archive(archive: &str) -> UniversalLabeler {
    let mut labeler = UniversalLabeler::new();
    
    // Extract source from archive
    let source = extract_source(archive);
    
    // Train labeler
    labeler.train(archive, &source);
    
    // Now can label any bytes from this archive
    labeler
}
```

## The Goal

**Every byte in a compressed archive gets a semantic label derived from how the compiler processes it.**

This creates a universal mapping:
```
Compressed byte → Decompressor IP → Compiler IP → Semantic label
```

The correlation strength tells us how strongly certain decompression operations map to certain compilation stages.

## Next Steps

2. [ ] Time-align traces
3. [ ] Correlate instruction pointers
4. [ ] Build IP → semantic mapping
5. [ ] Implement incremental decompression
6. [ ] Document mathematical transforms
7. [ ] Train universal labeler
8. [ ] Label compressed bytes directly
