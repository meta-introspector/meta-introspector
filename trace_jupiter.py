#!/usr/bin/env python3
"""
Jupiter eBPF Execution Tracer
Analyzes Jupiter transactions and maps to eBPF instructions
"""

import json
import subprocess
import sys
from pathlib import Path

JUPITER_PROGRAM = "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"
DISASM_FILE = "/mnt/data1/meta-introspector/data/solana_decompiled/Jupiter/disasm.s"

def get_transaction(sig):
    """Fetch transaction from Solana"""
    result = subprocess.run(
        ["solana", "transaction", sig, "--output", "json"],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print(f"Error fetching transaction: {result.stderr}")
        return None
    return json.loads(result.stdout)

def extract_jupiter_instructions(tx):
    """Extract Jupiter instructions from transaction"""
    instructions = []
    for ix in tx.get("transaction", {}).get("message", {}).get("instructions", []):
        if ix.get("programId") == JUPITER_PROGRAM:
            instructions.append(ix)
    return instructions

def parse_discriminator(data):
    """Parse 8-byte instruction discriminator"""
    # data is base64 encoded
    import base64
    decoded = base64.b64decode(data)
    discriminator = decoded[:8].hex()
    return discriminator

def map_to_ebpf(discriminator):
    """Map discriminator to eBPF function"""
    # Read disassembly and find entry point
    with open(DISASM_FILE) as f:
        lines = f.readlines()
    
    # Look for function that handles this discriminator
    # This requires pattern matching in the eBPF code
    return f"Function handling discriminator: {discriminator}"

def trace_execution(tx_sig):
    """Trace Jupiter execution"""
    print(f"=== Tracing Jupiter Transaction ===")
    print(f"Signature: {tx_sig}\n")
    
    # 1. Get transaction
    print("1. Fetching transaction...")
    tx = get_transaction(tx_sig)
    if not tx:
        return
    
    # 2. Extract Jupiter instructions
    print("2. Extracting Jupiter instructions...")
    jupiter_ixs = extract_jupiter_instructions(tx)
    print(f"   Found {len(jupiter_ixs)} Jupiter instruction(s)\n")
    
    # 3. Analyze each instruction
    for i, ix in enumerate(jupiter_ixs):
        print(f"=== Instruction {i+1} ===")
        
        # Discriminator
        discriminator = parse_discriminator(ix["data"])
        print(f"Discriminator: {discriminator}")
        
        # Accounts
        print(f"Accounts: {len(ix['accounts'])}")
        for j, acc_idx in enumerate(ix["accounts"]):
            acc = tx["transaction"]["message"]["accountKeys"][acc_idx]
            print(f"  [{j}] {acc}")
        
        # Map to eBPF
        print(f"\neBPF mapping:")
        print(map_to_ebpf(discriminator))
        print()
    
    # 4. Execution logs
    print("=== Execution Logs ===")
    logs = tx.get("meta", {}).get("logMessages", [])
    in_jupiter = False
    for log in logs:
        if "Program JUP4Fb2" in log:
            in_jupiter = True
        if in_jupiter:
            print(log)
            if "success" in log.lower() or "failed" in log.lower():
                in_jupiter = False
    
    # 5. State changes
    print("\n=== State Changes ===")
    pre_balances = tx.get("meta", {}).get("preTokenBalances", [])
    post_balances = tx.get("meta", {}).get("postTokenBalances", [])
    
    for pre, post in zip(pre_balances, post_balances):
        if pre["uiTokenAmount"]["uiAmount"] != post["uiTokenAmount"]["uiAmount"]:
            delta = post["uiTokenAmount"]["uiAmount"] - pre["uiTokenAmount"]["uiAmount"]
            print(f"Account {pre['accountIndex']}: {delta:+.6f} {pre['mint'][:8]}...")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 trace_jupiter.py <transaction_signature>")
        print("\nExample signatures from Solscan:")
        print("- Recent Jupiter swaps: https://solscan.io/account/JUP4Fb2...")
        sys.exit(1)
    
    trace_execution(sys.argv[1])
