#!/usr/bin/env python3
"""
Complete Python→Rust lifting pipeline with perf traces
script2test → test2perf → perf2prompt → rewrite
"""

import subprocess
import json
from pathlib import Path
from datetime import datetime

PROJECT_ROOT = Path("/mnt/data1/meta-introspector")

def script2test(py_file):
    """
    Generate tests for Python script
    Shows how script is used
    """
    print(f"📝 script2test: {py_file}")
    
    with open(py_file) as f:
        py_content = f.read()
    
    # Analyze imports and usage
    imports = [line for line in py_content.split('\n') if line.startswith('import') or line.startswith('from')]
    functions = [line for line in py_content.split('\n') if line.strip().startswith('def ')]
    
    # Check if script is executable
    is_executable = py_content.startswith('#!/')
    has_main = '__main__' in py_content
    
    # Generate test cases
    test_cases = []
    
    if is_executable and has_main:
        # CLI script - test with different args
        test_cases.append({
            "type": "cli",
            "command": f"python3 {py_file} --help",
            "expected": "help output"
        })
        test_cases.append({
            "type": "cli",
            "command": f"python3 {py_file}",
            "expected": "default behavior"
        })
    
    if functions:
        # Library - test functions
        for func_line in functions[:3]:  # First 3 functions
            func_name = func_line.split('def ')[1].split('(')[0]
            test_cases.append({
                "type": "function",
                "function": func_name,
                "test": f"test_{func_name}()"
            })
    
    result = {
        "py_file": str(py_file),
        "imports": imports,
        "functions": [f.split('def ')[1].split('(')[0] for f in functions],
        "is_executable": is_executable,
        "has_main": has_main,
        "test_cases": test_cases,
        "timestamp": datetime.now().isoformat()
    }
    
    # Save test spec
    test_file = PROJECT_ROOT / "data/tests" / f"{Path(py_file).stem}_tests.json"
    test_file.parent.mkdir(parents=True, exist_ok=True)
    with open(test_file, 'w') as f:
        json.dump(result, f, indent=2)
    
    print(f"   ✅ Generated {len(test_cases)} test cases")
    return result

def test2perf(test_spec):
    """
    Run tests with perf recording
    Captures execution trace
    """
    print(f"🔍 test2perf: {test_spec['py_file']}")
    
    py_file = test_spec['py_file']
    perf_dir = PROJECT_ROOT / "data/perf_traces"
    perf_dir.mkdir(parents=True, exist_ok=True)
    
    traces = []
    
    for i, test_case in enumerate(test_spec['test_cases']):
        perf_file = perf_dir / f"{Path(py_file).stem}_test{i}.perf.data"
        
        if test_case['type'] == 'cli':
            cmd = test_case['command']
            
            # Record with perf
            print(f"   Recording: {cmd}")
            result = subprocess.run(
                f"perf record -o {perf_file} -e 'syscalls:*' -e 'sched:*' --call-graph dwarf {cmd}",
                shell=True,
                capture_output=True,
                timeout=30
            )
            
            if perf_file.exists():
                # Parse trace
                trace_txt = perf_dir / f"{Path(py_file).stem}_test{i}.trace.txt"
                subprocess.run(
                    f"perf script -i {perf_file} > {trace_txt}",
                    shell=True
                )
                
                # Extract metrics
                with open(trace_txt) as f:
                    trace_content = f.read()
                
                syscalls = trace_content.count('syscalls:')
                
                traces.append({
                    "test_case": test_case,
                    "perf_file": str(perf_file),
                    "trace_file": str(trace_txt),
                    "syscalls": syscalls,
                    "success": result.returncode == 0
                })
                
                print(f"   ✅ Recorded {syscalls} syscalls")
    
    # Save perf results
    perf_result_file = perf_dir / f"{Path(py_file).stem}_perf.json"
    perf_result = {
        "py_file": py_file,
        "traces": traces,
        "total_syscalls": sum(t['syscalls'] for t in traces),
        "timestamp": datetime.now().isoformat()
    }
    
    with open(perf_result_file, 'w') as f:
        json.dump(perf_result, f, indent=2)
    
    print(f"   ✅ Total syscalls: {perf_result['total_syscalls']}")
    return perf_result

def perf2prompt(perf_result):
    """
    Convert perf traces to Gemini prompt
    Mathematical curve lifting: Python → Proof
    """
    print(f"📊 perf2prompt: {perf_result['py_file']}")
    
    py_file = perf_result['py_file']
    
    with open(py_file) as f:
        py_content = f.read()
    
    # Analyze perf traces
    trace_analysis = []
    for trace in perf_result['traces']:
        if Path(trace['trace_file']).exists():
            with open(trace['trace_file']) as f:
                trace_content = f.read()
            
            # Extract key syscalls
            syscall_types = {}
            for line in trace_content.split('\n'):
                if 'syscalls:sys_enter_' in line:
                    syscall = line.split('syscalls:sys_enter_')[1].split()[0]
                    syscall_types[syscall] = syscall_types.get(syscall, 0) + 1
            
            trace_analysis.append({
                "test": trace['test_case']['command'] if 'command' in trace['test_case'] else trace['test_case']['function'],
                "syscalls": trace['syscalls'],
                "syscall_types": syscall_types
            })
    
    # Create mathematical lifting prompt
    prompt = f"""Lift this Python script to Rust with mathematical proof of equivalence.

## Original Python Script: {py_file}

```python
{py_content}
```

## Execution Profile (Perf Traces)

Total syscalls: {perf_result['total_syscalls']}

### Trace Analysis
{json.dumps(trace_analysis, indent=2)}

## Mathematical Lifting Task

Prove equivalence: Python_behavior ≈ Rust_behavior

1. **Behavioral Equivalence**
   - Same syscalls in same order
   - Same I/O patterns
   - Same computational results

2. **Performance Curve**
   - Map Python syscall curve to Rust
   - Preserve or improve performance
   - Document complexity: O(n) → O(n)

3. **Type Lifting**
   - Python dynamic types → Rust static types
   - Prove type safety
   - No runtime type errors

## Output Format

Provide JSON with:
{{
  "rust_code": "complete Rust implementation",
  "cargo_toml": {{"dependencies": {{}}}},
  "equivalence_proof": {{
    "syscall_mapping": {{"python_syscall": "rust_syscall"}},
    "complexity_preserved": "O(n) analysis",
    "type_safety": "proof that types are correct"
  }},
  "test_cases": [
    {{
      "input": "test input",
      "python_output": "expected output",
      "rust_output": "same output",
      "syscalls_match": true
    }}
  ],
  "performance_analysis": {{
    "python_syscalls": {perf_result['total_syscalls']},
    "rust_syscalls": "predicted count",
    "improvement": "percentage"
  }}
}}

Focus on:
1. Exact behavioral equivalence
2. Mathematical proof of correctness
3. Syscall-level compatibility
4. Performance preservation or improvement
"""
    
    # Save prompt
    prompt_file = PROJECT_ROOT / "data/prompts" / f"{Path(py_file).stem}_lift.json"
    prompt_file.parent.mkdir(parents=True, exist_ok=True)
    
    prompt_data = {
        "py_file": py_file,
        "perf_result": perf_result,
        "trace_analysis": trace_analysis,
        "prompt": prompt,
        "timestamp": datetime.now().isoformat()
    }
    
    with open(prompt_file, 'w') as f:
        json.dump(prompt_data, f, indent=2)
    
    print(f"   ✅ Created lifting prompt with {perf_result['total_syscalls']} syscalls")
    return prompt_data

def lift_script(py_file):
    """
    Complete pipeline: script2test → test2perf → perf2prompt
    """
    print(f"\n{'='*60}")
    print(f"Lifting: {py_file}")
    print(f"{'='*60}\n")
    
    # Step 1: Generate tests
    test_spec = script2test(py_file)
    
    # Step 2: Record perf traces
    perf_result = test2perf(test_spec)
    
    # Step 3: Create lifting prompt
    prompt_data = perf2prompt(perf_result)
    
    print(f"\n✅ Lifting complete!")
    print(f"   Tests: {len(test_spec['test_cases'])}")
    print(f"   Traces: {len(perf_result['traces'])}")
    print(f"   Syscalls: {perf_result['total_syscalls']}")
    print(f"   Prompt: {prompt_data['prompt'][:100]}...")
    
    return prompt_data

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) < 2:
        print("Usage: lift_python.py <python-file>")
        print("")
        print("Pipeline: script2test → test2perf → perf2prompt")
        print("")
        print("Example:")
        print("  python3 lift_python.py scripts/build/nix2prompt.py")
        sys.exit(1)
    
    py_file = sys.argv[1]
    lift_script(py_file)
