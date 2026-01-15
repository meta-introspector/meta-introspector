#!/bin/bash
# Multi-dimensional semiotic message capture
# Each binary is a functional utterance operating on nix store
# Captures: Markov(input/output/trace) + AST layers + perf + nix store effects

set -e

SESSION_ID="utterance_$(date +%s)"
OUTPUT_DIR="data/utterances/$SESSION_ID"
mkdir -p "$OUTPUT_DIR"/{markov,ast,perf,nix_store,traces}

echo "🎭 Multi-dimensional Message Capture: $SESSION_ID"
echo ""

capture_binary_message() {
    local bin=$1
    local bin_path=$2
    local msg_dir="$OUTPUT_DIR/$bin"
    
    mkdir -p "$msg_dir"/{markov,ast,perf,nix_store,trace}
    
    echo "📡 Capturing: $bin"
    
    # 1. Markov Models (input/output/trace)
    echo "  🔢 Markov models..."
    timeout 2s "$bin_path" 2>&1 | tee "$msg_dir/markov/output.txt" | \
        cargo run --bin markov_analyzer -- --stdin > "$msg_dir/markov/output_model.json" 2>/dev/null || true
    
    # 2. AST Layers (syn/HIR/MIR/LLVM)
    echo "  🌳 AST layers..."
    # Source -> syn AST
    if [ -f "${bin}.rs" ]; then
        cargo run --bin syn_compressor -- "${bin}.rs" > "$msg_dir/ast/syn.json" 2>/dev/null || true
    fi
    
    # Binary -> HIR/MIR via rustc
    rustc --emit=mir -Z unpretty=hir "${bin}.rs" > "$msg_dir/ast/hir.txt" 2>/dev/null || true
    rustc --emit=mir "${bin}.rs" -o "$msg_dir/ast/mir.txt" 2>/dev/null || true
    
    # Binary -> LLVM IR
    rustc --emit=llvm-ir "${bin}.rs" -o "$msg_dir/ast/llvm.ll" 2>/dev/null || true
    
    # Binary -> objdump
    objdump -d "$bin_path" > "$msg_dir/ast/objdump.asm" 2>/dev/null || true
    
    # Binary -> goblin
    cargo run --bin binary_symbol_study -- "$bin_path" > "$msg_dir/ast/goblin.json" 2>/dev/null || true
    
    # 3. Perf Recording
    echo "  📊 Perf recording..."
    perf record -o "$msg_dir/perf/perf.data" -g -- timeout 1s "$bin_path" 2>/dev/null || true
    perf script -i "$msg_dir/perf/perf.data" > "$msg_dir/perf/perf.script" 2>/dev/null || true
    
    # 4. Nix Store Effects
    echo "  📦 Nix store effects..."
    # Capture nix store paths before
    nix-store -q --references "$bin_path" > "$msg_dir/nix_store/references.txt" 2>/dev/null || true
    nix-store -q --requisites "$bin_path" > "$msg_dir/nix_store/requisites.txt" 2>/dev/null || true
    
    # 5. Strace (system call trace)
    echo "  🔍 System trace..."
    strace -o "$msg_dir/trace/strace.log" -f -tt -T timeout 1s "$bin_path" 2>/dev/null || true
    
    # 6. Message Metadata
    cat > "$msg_dir/message.json" <<EOF
{
  "binary": "$bin",
  "session": "$SESSION_ID",
  "timestamp": "$(date -Iseconds)",
  "dimensions": {
    "markov": "$(ls -lh $msg_dir/markov/*.json 2>/dev/null | wc -l) models",
    "ast_layers": "$(ls $msg_dir/ast/ 2>/dev/null | wc -l) layers",
    "perf": "$([ -f $msg_dir/perf/perf.data ] && echo 'captured' || echo 'none')",
    "nix_store": "$(wc -l < $msg_dir/nix_store/references.txt 2>/dev/null || echo 0) refs",
    "trace": "$(wc -l < $msg_dir/trace/strace.log 2>/dev/null || echo 0) syscalls"
  }
}
EOF
    
    echo "  ✅ Message captured"
}

# Build and capture messages
BINARIES=$(grep 'name = ' Cargo.toml | grep -A1 '\[\[bin\]\]' | grep 'name = ' | cut -d'"' -f2 | head -10)

for bin in $BINARIES; do
    if cargo build --bin "$bin" 2>/dev/null; then
        capture_binary_message "$bin" "target/debug/$bin"
    fi
done

echo ""
echo "📊 Multi-dimensional messages captured: $OUTPUT_DIR/"
echo ""
echo "Each message contains:"
echo "  - Markov models (input/output/trace)"
echo "  - AST layers (syn/HIR/MIR/LLVM/objdump/goblin)"
echo "  - Perf recordings"
echo "  - Nix store effects"
echo "  - System call traces"
