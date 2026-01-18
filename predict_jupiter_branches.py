#!/usr/bin/env python3
"""
Jupiter Branch Prediction from Blockchain Traces
Learns branch behavior from real transaction execution logs
"""

import json
from collections import defaultdict, Counter
from pathlib import Path

CFG_FILE = "/mnt/data1/meta-introspector/data/jupiter_cfg/jupiter_cfg.json"
TRACES_DIR = "/mnt/data1/meta-introspector/data/jupiter_traces"

class BranchPredictor:
    def __init__(self):
        self.branch_history = defaultdict(Counter)  # addr -> {taken: count, not_taken: count}
        self.branch_patterns = defaultdict(list)  # addr -> [taken, not_taken, taken, ...]
        self.cfg = None
        
    def load_cfg(self, cfg_file):
        """Load control flow graph"""
        with open(cfg_file) as f:
            self.cfg = json.load(f)
        print(f"Loaded CFG: {len(self.cfg['blocks'])} blocks, {len(self.cfg['edges'])} edges")
    
    def learn_from_trace(self, trace_file):
        """Learn branch behavior from execution trace"""
        with open(trace_file) as f:
            trace = json.load(f)
        
        # Trace format: list of executed basic block addresses
        executed_blocks = trace.get('executed_blocks', [])
        
        # Identify branches taken
        for i in range(len(executed_blocks) - 1):
            current = executed_blocks[i]
            next_block = executed_blocks[i + 1]
            
            # Find this edge in CFG
            block = self._find_block(current)
            if block and block.get('branch_type') == 'conditional':
                # Determine if branch was taken
                successors = self._get_successors(current)
                if len(successors) == 2:
                    # Conditional branch has 2 successors
                    taken = (next_block == successors[0])
                    self.branch_history[current]['taken' if taken else 'not_taken'] += 1
                    self.branch_patterns[current].append(taken)
    
    def _find_block(self, addr):
        """Find block by address"""
        for block in self.cfg['blocks']:
            if block['addr'] == addr:
                return block
        return None
    
    def _get_successors(self, addr):
        """Get successor blocks"""
        return [edge['to'] for edge in self.cfg['edges'] if edge['from'] == addr]
    
    def predict_branch(self, addr):
        """Predict if branch will be taken"""
        if addr not in self.branch_history:
            return None, 0.5  # Unknown, 50/50
        
        counts = self.branch_history[addr]
        total = counts['taken'] + counts['not_taken']
        if total == 0:
            return None, 0.5
        
        taken_prob = counts['taken'] / total
        prediction = taken_prob > 0.5
        confidence = max(taken_prob, 1 - taken_prob)
        
        return prediction, confidence
    
    def get_statistics(self):
        """Get branch prediction statistics"""
        stats = {
            'total_branches': len(self.branch_history),
            'branches': []
        }
        
        for addr, counts in self.branch_history.items():
            total = counts['taken'] + counts['not_taken']
            taken_prob = counts['taken'] / total if total > 0 else 0
            
            stats['branches'].append({
                'addr': addr,
                'taken': counts['taken'],
                'not_taken': counts['not_taken'],
                'taken_probability': taken_prob,
                'total_executions': total
            })
        
        # Sort by most executed
        stats['branches'].sort(key=lambda x: x['total_executions'], reverse=True)
        return stats
    
    def export_predictions(self, output_file):
        """Export branch predictions"""
        stats = self.get_statistics()
        with open(output_file, 'w') as f:
            json.dump(stats, f, indent=2)

def simulate_trace_collection():
    """Simulate collecting traces from blockchain"""
    print("=== Simulating Trace Collection ===\n")
    
    # In reality, this would:
    # 1. Fetch Jupiter transactions from Solana
    # 2. Parse execution logs
    # 3. Map log messages to basic block addresses
    # 4. Build execution trace
    
    print("To collect real traces:")
    print("1. Get Jupiter transaction signatures from Solscan")
    print("2. Fetch with: solana transaction <sig> --output json")
    print("3. Parse logMessages to identify executed code paths")
    print("4. Map compute unit consumption to basic blocks")
    print("5. Build execution trace: [BB1, BB2, BB3, ...]")
    print()
    
    # Example trace format
    example_trace = {
        'transaction': '5xYz...',
        'executed_blocks': [
            0x120,  # Entry point
            0x150,  # Validation
            0x180,  # Branch taken
            0x200,  # Swap logic
            0x250,  # CPI call
            0x280,  # Return
        ],
        'compute_units': 50000,
        'success': True
    }
    
    Path(TRACES_DIR).mkdir(parents=True, exist_ok=True)
    with open(f"{TRACES_DIR}/example_trace.json", 'w') as f:
        json.dump(example_trace, f, indent=2)
    
    print(f"Example trace saved to: {TRACES_DIR}/example_trace.json")

def main():
    print("=== Jupiter Branch Predictor ===\n")
    
    # Check if CFG exists
    if not Path(CFG_FILE).exists():
        print(f"CFG not found. Run build_jupiter_cfg.py first.")
        return
    
    # Initialize predictor
    predictor = BranchPredictor()
    predictor.load_cfg(CFG_FILE)
    print()
    
    # Simulate trace collection
    simulate_trace_collection()
    print()
    
    # Learn from traces (if any exist)
    traces = list(Path(TRACES_DIR).glob("*.json"))
    if traces:
        print(f"Learning from {len(traces)} trace(s)...")
        for trace_file in traces:
            try:
                predictor.learn_from_trace(trace_file)
            except Exception as e:
                print(f"  Error processing {trace_file.name}: {e}")
        
        # Export predictions
        output_file = f"{TRACES_DIR}/branch_predictions.json"
        predictor.export_predictions(output_file)
        print(f"Predictions saved to: {output_file}")
        
        # Show statistics
        stats = predictor.get_statistics()
        print(f"\n=== Branch Statistics ===")
        print(f"Total branches tracked: {stats['total_branches']}")
        print(f"\nTop 10 most executed branches:")
        for i, branch in enumerate(stats['branches'][:10]):
            print(f"  {i+1}. BB@{branch['addr']:x}: "
                  f"{branch['taken_probability']:.1%} taken "
                  f"({branch['total_executions']} executions)")
    else:
        print("No traces found. Collect traces from blockchain first.")
        print("\nNext steps:")
        print("1. Find Jupiter transactions on Solscan")
        print("2. Use trace_jupiter.py to analyze them")
        print("3. Build execution traces")
        print("4. Run this script again to learn branch patterns")

if __name__ == "__main__":
    main()
