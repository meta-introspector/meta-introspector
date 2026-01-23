#!/usr/bin/env bash
# Find highest ranked models that fit in 12GB GPU

set -euo pipefail

GPU_VRAM_GB=12
OUTPUT_FILE="/mnt/data1/meta-introspector/data/models-12gb.txt"

echo "=== Finding Best Models for 12GB GPU ==="
echo "GPU VRAM: ${GPU_VRAM_GB}GB"
echo ""

# Rough VRAM estimates (GB per billion parameters):
# - FP16: ~2GB per 1B params
# - 8-bit: ~1GB per 1B params  
# - 4-bit: ~0.5GB per 1B params

# For 12GB GPU with 4-bit quantization: ~20B params max
# For 12GB GPU with 8-bit quantization: ~10B params max

echo "Recommended models for 12GB GPU:"
echo ""

cat > "$OUTPUT_FILE" << 'EOF'
# Best Models for 12GB GPU (sorted by capability)

## 4-bit Quantization (~20B params max)

### Top Tier (Reasoning + Tools)
- gpt-oss-20b (4-bit) - OpenAI's first open model, tool use, reasoning
- Magistral-24B (4-bit) - Mistral reasoning model, 128K context
- Devstral-24B (4-bit) - Mistral coding model, multi-file editing
- Codestral-22B (4-bit) - Mistral coding, 80+ languages

### High Performance (General)
- Qwen3-14B (4-bit) - Tool use + reasoning, multilingual
- Phi-4-14B (4-bit) - Microsoft, strong reasoning
- Mistral-Nemo-12B (4-bit) - Multilingual, NVIDIA collab

### Vision Models
- Qwen3-VL-8B (4-bit) - Vision + language, spatial reasoning
- Gemma-3-12B (4-bit) - Google, vision + text

## 8-bit Quantization (~10B params max)

### Best Overall
- Qwen3-8B (8-bit) - Tool use + reasoning
- DeepSeek-R1-8B (8-bit) - Chain-of-thought reasoning
- Mistral-7B (8-bit) - Popular, well-balanced

### Specialized
- Olmo-3-7B (8-bit) - Tool use + reasoning
- Granite-4.0-7B (8-bit) - IBM, multilingual, RAG, tools

## Smallest (FP16 possible)

### Ultra-Efficient
- Gemma-3-4B (FP16) - Google, vision capable
- Qwen3-4B (FP16) - Tool use + reasoning
- Phi-4-3B (FP16) - Microsoft, strong for size
- Ministral-3B (FP16) - Mistral, cost-effective

## Recommended Setup

1. **Primary**: gpt-oss-20b (4-bit) or Magistral-24B (4-bit)
2. **Coding**: Devstral-24B (4-bit) or Codestral-22B (4-bit)
3. **Vision**: Qwen3-VL-8B (4-bit)
4. **Fast/Local**: Qwen3-4B (FP16) or Phi-4-3B (FP16)

## Download Commands (LM Studio)

```bash
# Install LMS CLI
npx lmstudio install-cli

# Download models
lms download lmstudio-community/gpt-oss-20b-GGUF
lms download lmstudio-community/Magistral-24B-GGUF
lms download lmstudio-community/Qwen3-8B-GGUF
lms download lmstudio-community/Phi-4-3B-GGUF
```

## VRAM Usage Estimates

| Model | Quantization | VRAM | Speed |
|-------|-------------|------|-------|
| gpt-oss-20b | 4-bit | ~10GB | Medium |
| Magistral-24B | 4-bit | ~12GB | Medium |
| Qwen3-14B | 4-bit | ~7GB | Fast |
| Qwen3-8B | 8-bit | ~8GB | Fast |
| Phi-4-3B | FP16 | ~6GB | Very Fast |

EOF

cat "$OUTPUT_FILE"

echo ""
echo "✅ Model recommendations saved to: $OUTPUT_FILE"
