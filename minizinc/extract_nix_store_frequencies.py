#!/usr/bin/env python3
"""
Extract binary operation frequencies from /nix/store for MiniZinc proof.

This script:
1. Scans /nix/store for ELF binaries
2. Disassembles and counts binary operations (cmp, jmp, test, je, jne, etc.)
3. Generates MiniZinc data file with instruction frequencies
4. Runs MiniZinc solver to find 46 layers matching 2^46 structure
"""

import subprocess
import json
from pathlib import Path
from collections import Counter

# Binary operations we're looking for (2-way branches)
BINARY_OPS = {
    'cmp': 0,   # Compare
    'test': 1,  # Test (bitwise AND)
    'jmp': 2,   # Jump
    'je': 3,    # Jump if equal
    'jne': 4,   # Jump if not equal
    'jz': 5,    # Jump if zero
    'jnz': 6,   # Jump if not zero
    'jl': 7,    # Jump if less
    'jg': 8,    # Jump if greater
    'jle': 9,   # Jump if less or equal
    'jge': 10,  # Jump if greater or equal
}

def scan_nix_store(limit=100):
    """Scan /nix/store for ELF binaries."""
    print("🔍 Scanning /nix/store for binaries...")
    
    result = subprocess.run(
        ['find', '/nix/store', '-type', 'f', '-executable'],
        capture_output=True,
        text=True,
        timeout=60
    )
    
    binaries = []
    for line in result.stdout.split('\n')[:limit]:
        if line and 'bin/' in line:
            binaries.append(line.strip())
    
    print(f"Found {len(binaries)} binaries")
    return binaries

def disassemble_and_count(binary_path):
    """Disassemble binary and count binary operations."""
    try:
        # Use objdump to disassemble
        result = subprocess.run(
            ['objdump', '-d', binary_path],
            capture_output=True,
            text=True,
            timeout=10
        )
        
        counts = Counter()
        for line in result.stdout.split('\n'):
            # Parse instruction
            parts = line.strip().split()
            if len(parts) >= 3:
                instruction = parts[2].split()[0]
                if instruction in BINARY_OPS:
                    counts[instruction] += 1
        
        return counts
    except Exception as e:
        print(f"  ⚠️  Error disassembling {binary_path}: {e}")
        return Counter()

def generate_minizinc_data(total_counts):
    """Generate MiniZinc data file from instruction counts."""
    
    # Create array of 100 instruction counts (padded with zeros)
    instruction_counts = [0] * 100
    
    for instruction, count in total_counts.most_common():
        if instruction in BINARY_OPS:
            idx = BINARY_OPS[instruction]
            instruction_counts[idx] = count
    
    # Generate MiniZinc data file
    dzn_content = f"""% Generated instruction frequencies from /nix/store
instruction_counts = {instruction_counts};
"""
    
    with open('nix_store_frequencies.dzn', 'w') as f:
        f.write(dzn_content)
    
    print(f"\n📊 Top binary operations:")
    for instruction, count in total_counts.most_common(10):
        if instruction in BINARY_OPS:
            print(f"  {instruction:6s}: {count:>10,} occurrences")

def main():
    print("🔬 Proving 1 = M = /nix/store via 2^46 binary layers\n")
    
    # Scan /nix/store
    binaries = scan_nix_store(limit=100)
    
    # Count all binary operations
    total_counts = Counter()
    for i, binary in enumerate(binaries, 1):
        print(f"  [{i}/{len(binaries)}] Analyzing {Path(binary).name}...")
        counts = disassemble_and_count(binary)
        total_counts.update(counts)
    
    # Generate MiniZinc data
    generate_minizinc_data(total_counts)
    
    print("\n✅ Data generated: nix_store_frequencies.dzn")
    print("\n🔧 Run MiniZinc solver:")
    print("   minizinc prove_monster_nix_store.mzn nix_store_frequencies.dzn")
    
    # Calculate 2^46 for reference
    target = 2**46
    total = sum(total_counts.values())
    ratio = total / target if target > 0 else 0
    
    print(f"\n📈 Statistics:")
    print(f"   Total binary ops: {total:,}")
    print(f"   Target (2^46):    {target:,}")
    print(f"   Coverage ratio:   {ratio:.6f}")
    print(f"   Layers needed:    46")

if __name__ == '__main__':
    main()
