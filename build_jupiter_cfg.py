#!/usr/bin/env python3
"""
Jupiter eBPF Control Flow Graph (CFG) Builder
Extracts basic blocks, branches, and builds CFG from eBPF disassembly
"""

import re
from collections import defaultdict
from pathlib import Path

DISASM_FILE = "/mnt/data1/meta-introspector/data/solana_decompiled/Jupiter/disasm.s"
OUTPUT_DIR = "/mnt/data1/meta-introspector/data/jupiter_cfg"

class BasicBlock:
    def __init__(self, start_addr, instructions):
        self.start_addr = start_addr
        self.instructions = instructions
        self.successors = []  # Branch targets
        self.predecessors = []  # Incoming branches
        self.branch_type = None  # conditional, unconditional, call, return
        
    def __repr__(self):
        return f"BB@{self.start_addr:x} ({len(self.instructions)} insns)"

def parse_disassembly():
    """Parse eBPF disassembly into instructions"""
    instructions = []
    with open(DISASM_FILE) as f:
        for line in f:
            # Match: "      36:\tbf 48 00 00 00 00 00 00\tr8 = r4"
            match = re.match(r'\s+(\d+):\t([0-9a-f ]+)\t(.+)', line)
            if match:
                addr = int(match.group(1))
                bytecode = match.group(2).strip()
                asm = match.group(3).strip()
                instructions.append({
                    'addr': addr,
                    'bytecode': bytecode,
                    'asm': asm
                })
    return instructions

def is_branch(insn):
    """Check if instruction is a branch"""
    asm = insn['asm']
    # eBPF branch instructions
    if any(asm.startswith(x) for x in ['if', 'goto', 'call', 'exit']):
        return True
    # Conditional branches: jeq, jne, jgt, jge, jlt, jle, etc.
    if re.match(r'j[a-z]+\s+', asm):
        return True
    return False

def get_branch_target(insn):
    """Extract branch target address"""
    asm = insn['asm']
    # Match: "if r1 != 0x0 goto +0x1d <.text+0x118>"
    # Match: "call 0x228ad"
    # Match: "goto +0x35 <.text+0x268>"
    
    # Relative offset
    match = re.search(r'goto ([+-]0x[0-9a-f]+)', asm)
    if match:
        offset = int(match.group(1), 16)
        return insn['addr'] + offset
    
    # Absolute address
    match = re.search(r'<\.text\+(0x[0-9a-f]+)>', asm)
    if match:
        return int(match.group(1), 16)
    
    # Call target
    match = re.search(r'call (0x[0-9a-f]+)', asm)
    if match:
        return int(match.group(1), 16)
    
    return None

def build_basic_blocks(instructions):
    """Split instructions into basic blocks"""
    blocks = []
    leaders = set([instructions[0]['addr']])  # First instruction is a leader
    
    # Find all leaders (branch targets and instructions after branches)
    for i, insn in enumerate(instructions):
        if is_branch(insn):
            target = get_branch_target(insn)
            if target:
                leaders.add(target)
            # Instruction after branch is also a leader
            if i + 1 < len(instructions):
                leaders.add(instructions[i + 1]['addr'])
    
    # Build blocks
    current_block = []
    start_addr = instructions[0]['addr']
    
    for insn in instructions:
        if insn['addr'] in leaders and current_block:
            # Start new block
            blocks.append(BasicBlock(start_addr, current_block))
            current_block = []
            start_addr = insn['addr']
        current_block.append(insn)
    
    if current_block:
        blocks.append(BasicBlock(start_addr, current_block))
    
    return blocks

def build_cfg(blocks):
    """Build control flow graph by linking blocks"""
    addr_to_block = {bb.start_addr: bb for bb in blocks}
    
    for bb in blocks:
        last_insn = bb.instructions[-1]
        
        if is_branch(last_insn):
            target = get_branch_target(last_insn)
            if target and target in addr_to_block:
                target_bb = addr_to_block[target]
                bb.successors.append(target_bb)
                target_bb.predecessors.append(bb)
                
                # Determine branch type
                if 'if' in last_insn['asm']:
                    bb.branch_type = 'conditional'
                    # Fall-through edge
                    next_addr = bb.instructions[-1]['addr'] + 1
                    if next_addr in addr_to_block:
                        next_bb = addr_to_block[next_addr]
                        bb.successors.append(next_bb)
                        next_bb.predecessors.append(bb)
                elif 'call' in last_insn['asm']:
                    bb.branch_type = 'call'
                elif 'exit' in last_insn['asm']:
                    bb.branch_type = 'return'
                else:
                    bb.branch_type = 'unconditional'
        else:
            # Fall-through to next block
            next_addr = bb.instructions[-1]['addr'] + 1
            if next_addr in addr_to_block:
                next_bb = addr_to_block[next_addr]
                bb.successors.append(next_bb)
                next_bb.predecessors.append(bb)
    
    return blocks

def export_cfg_dot(blocks, output_file):
    """Export CFG to Graphviz DOT format"""
    with open(output_file, 'w') as f:
        f.write("digraph CFG {\n")
        f.write("  node [shape=box];\n")
        
        for bb in blocks:
            label = f"BB@{bb.start_addr:x}\\n"
            label += f"{len(bb.instructions)} instructions\\n"
            if bb.branch_type:
                label += f"[{bb.branch_type}]"
            
            f.write(f'  "bb_{bb.start_addr:x}" [label="{label}"];\n')
            
            for succ in bb.successors:
                f.write(f'  "bb_{bb.start_addr:x}" -> "bb_{succ.start_addr:x}";\n')
        
        f.write("}\n")

def export_cfg_json(blocks, output_file):
    """Export CFG to JSON for analysis"""
    import json
    
    cfg_data = {
        'blocks': [],
        'edges': []
    }
    
    for bb in blocks:
        cfg_data['blocks'].append({
            'addr': bb.start_addr,
            'size': len(bb.instructions),
            'branch_type': bb.branch_type,
            'instructions': [insn['asm'] for insn in bb.instructions]
        })
        
        for succ in bb.successors:
            cfg_data['edges'].append({
                'from': bb.start_addr,
                'to': succ.start_addr
            })
    
    with open(output_file, 'w') as f:
        json.dump(cfg_data, f, indent=2)

def main():
    Path(OUTPUT_DIR).mkdir(parents=True, exist_ok=True)
    
    print("=== Jupiter eBPF CFG Builder ===\n")
    
    print("1. Parsing disassembly...")
    instructions = parse_disassembly()
    print(f"   Found {len(instructions)} instructions\n")
    
    print("2. Building basic blocks...")
    blocks = build_basic_blocks(instructions)
    print(f"   Created {len(blocks)} basic blocks\n")
    
    print("3. Building control flow graph...")
    cfg = build_cfg(blocks)
    print(f"   Linked {sum(len(bb.successors) for bb in cfg)} edges\n")
    
    print("4. Analyzing CFG...")
    conditional_branches = sum(1 for bb in cfg if bb.branch_type == 'conditional')
    calls = sum(1 for bb in cfg if bb.branch_type == 'call')
    returns = sum(1 for bb in cfg if bb.branch_type == 'return')
    print(f"   Conditional branches: {conditional_branches}")
    print(f"   Function calls: {calls}")
    print(f"   Returns: {returns}\n")
    
    print("5. Exporting CFG...")
    export_cfg_dot(cfg, f"{OUTPUT_DIR}/jupiter_cfg.dot")
    export_cfg_json(cfg, f"{OUTPUT_DIR}/jupiter_cfg.json")
    print(f"   DOT: {OUTPUT_DIR}/jupiter_cfg.dot")
    print(f"   JSON: {OUTPUT_DIR}/jupiter_cfg.json\n")
    
    print("6. Top 10 largest basic blocks:")
    sorted_blocks = sorted(cfg, key=lambda bb: len(bb.instructions), reverse=True)
    for i, bb in enumerate(sorted_blocks[:10]):
        print(f"   {i+1}. BB@{bb.start_addr:x}: {len(bb.instructions)} instructions")
    
    print("\n=== CFG Build Complete ===")
    print(f"Visualize: dot -Tpng {OUTPUT_DIR}/jupiter_cfg.dot -o {OUTPUT_DIR}/jupiter_cfg.png")

if __name__ == "__main__":
    main()
