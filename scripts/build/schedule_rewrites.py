#!/usr/bin/env python3
"""
Schedule Python to Rust rewrites via Gemini
"""

import json
import subprocess
from pathlib import Path
from datetime import datetime

PROJECT_ROOT = Path("/mnt/data1/meta-introspector")
REWRITE_QUEUE = PROJECT_ROOT / "data/rewrite_queue.json"
REWRITE_RESULTS = PROJECT_ROOT / "data/rewrite_results"

def find_python_scripts():
    """Find all Python scripts in project"""
    scripts = []
    for py_file in PROJECT_ROOT.rglob("*.py"):
        if ".git" not in str(py_file) and "venv" not in str(py_file):
            scripts.append(str(py_file.relative_to(PROJECT_ROOT)))
    return scripts

def create_rewrite_queue():
    """Create queue of Python scripts to rewrite"""
    scripts = find_python_scripts()
    
    queue = {
        "created": datetime.now().isoformat(),
        "total_scripts": len(scripts),
        "pending": scripts,
        "in_progress": [],
        "completed": [],
        "failed": []
    }
    
    REWRITE_QUEUE.parent.mkdir(parents=True, exist_ok=True)
    with open(REWRITE_QUEUE, 'w') as f:
        json.dump(queue, f, indent=2)
    
    print(f"✅ Created rewrite queue: {len(scripts)} Python scripts")
    return queue

def create_rewrite_prompt(py_file):
    """Create Gemini prompt for Python→Rust rewrite"""
    
    with open(PROJECT_ROOT / py_file) as f:
        py_content = f.read()
    
    prompt = f"""Rewrite this Python script to Rust:

## Original Python File: {py_file}

```python
{py_content}
```

## Requirements
1. Preserve all functionality
2. Use idiomatic Rust
3. Add proper error handling
4. Include tests
5. Use appropriate crates (clap, serde, tokio, etc.)

## Output Format
Provide a JSON response with:
{{
  "rust_file": "path/to/file.rs",
  "cargo_toml": {{
    "dependencies": {{}},
    "dev_dependencies": {{}}
  }},
  "rust_code": "complete Rust code",
  "tests": "test code",
  "explanation": "key differences and improvements"
}}
"""
    
    return prompt

def schedule_rewrite(py_file):
    """Schedule a Python→Rust rewrite with Gemini"""
    
    # Load queue
    with open(REWRITE_QUEUE) as f:
        queue = json.load(f)
    
    # Move to in_progress
    if py_file in queue["pending"]:
        queue["pending"].remove(py_file)
        queue["in_progress"].append(py_file)
        
        with open(REWRITE_QUEUE, 'w') as f:
            json.dump(queue, f, indent=2)
    
    print(f"🔄 Scheduling rewrite: {py_file}")
    
    # Create prompt
    prompt = create_rewrite_prompt(py_file)
    
    # Save prompt
    prompt_file = REWRITE_RESULTS / f"{Path(py_file).stem}_prompt.json"
    prompt_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(prompt_file, 'w') as f:
        json.dump({
            "py_file": py_file,
            "prompt": prompt,
            "timestamp": datetime.now().isoformat()
        }, f, indent=2)
    
    print(f"📝 Prompt saved: {prompt_file}")
    
    return prompt_file

def call_gemini_rewrite(prompt_file):
    """Call Gemini to perform rewrite"""
    
    with open(prompt_file) as f:
        data = json.load(f)
    
    print(f"🤖 Calling Gemini for rewrite...")
    
    # Call via impure derivation
    result = subprocess.run([
        "nix", "run",
        "./nix/gemini-evolution-impure.nix#gemini-call",
        "--impure",
        "--",
        "-p", data["prompt"],
        "--output-format", "json",
        "--model", "gemini-2.5-flash"
    ], capture_output=True, text=True, timeout=120)
    
    if result.returncode == 0:
        return json.loads(result.stdout)
    else:
        print(f"❌ Gemini call failed: {result.stderr[:200]}")
        return None

def apply_rewrite(py_file, rewrite_data):
    """Apply the Rust rewrite"""
    
    rust_file = PROJECT_ROOT / rewrite_data["rust_file"]
    rust_file.parent.mkdir(parents=True, exist_ok=True)
    
    # Write Rust code
    with open(rust_file, 'w') as f:
        f.write(rewrite_data["rust_code"])
    
    print(f"✅ Wrote Rust code: {rust_file}")
    
    # Update Cargo.toml if needed
    if rewrite_data.get("cargo_toml"):
        print(f"📦 Dependencies: {list(rewrite_data['cargo_toml']['dependencies'].keys())}")
    
    # Compile test
    print(f"🔨 Testing compilation...")
    result = subprocess.run(
        ["rustc", "--crate-type", "bin", str(rust_file), "-o", "/tmp/test_binary"],
        capture_output=True
    )
    
    if result.returncode == 0:
        print(f"✅ Compilation successful")
        return True
    else:
        print(f"❌ Compilation failed: {result.stderr.decode()[:200]}")
        return False

def process_next_rewrite():
    """Process next Python script in queue"""
    
    with open(REWRITE_QUEUE) as f:
        queue = json.load(f)
    
    if not queue["pending"]:
        print("✅ Queue empty - all scripts processed!")
        return None
    
    # Get next script
    py_file = queue["pending"][0]
    
    print(f"\n{'='*60}")
    print(f"Processing: {py_file}")
    print(f"Remaining: {len(queue['pending'])}")
    print(f"{'='*60}\n")
    
    # Schedule
    prompt_file = schedule_rewrite(py_file)
    
    # Call Gemini
    rewrite_data = call_gemini_rewrite(prompt_file)
    
    if rewrite_data:
        # Apply rewrite
        success = apply_rewrite(py_file, rewrite_data)
        
        # Update queue
        with open(REWRITE_QUEUE) as f:
            queue = json.load(f)
        
        queue["in_progress"].remove(py_file)
        
        if success:
            queue["completed"].append(py_file)
            print(f"✅ Rewrite completed: {py_file}")
        else:
            queue["failed"].append(py_file)
            print(f"❌ Rewrite failed: {py_file}")
        
        with open(REWRITE_QUEUE, 'w') as f:
            json.dump(queue, f, indent=2)
        
        return py_file
    else:
        # Failed - move back to pending
        with open(REWRITE_QUEUE) as f:
            queue = json.load(f)
        
        queue["in_progress"].remove(py_file)
        queue["failed"].append(py_file)
        
        with open(REWRITE_QUEUE, 'w') as f:
            json.dump(queue, f, indent=2)
        
        return None

def show_status():
    """Show rewrite queue status"""
    
    if not REWRITE_QUEUE.exists():
        print("❌ No queue found. Run: python3 schedule_rewrites.py init")
        return
    
    with open(REWRITE_QUEUE) as f:
        queue = json.load(f)
    
    print(f"\n📊 Python→Rust Rewrite Status")
    print(f"{'='*60}")
    print(f"Total scripts:    {queue['total_scripts']}")
    print(f"Pending:          {len(queue['pending'])}")
    print(f"In progress:      {len(queue['in_progress'])}")
    print(f"Completed:        {len(queue['completed'])}")
    print(f"Failed:           {len(queue['failed'])}")
    print(f"{'='*60}\n")
    
    if queue['completed']:
        print("✅ Recently completed:")
        for script in queue['completed'][-5:]:
            print(f"   - {script}")
    
    if queue['failed']:
        print("\n❌ Failed:")
        for script in queue['failed'][-5:]:
            print(f"   - {script}")

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) < 2:
        print("Usage: schedule_rewrites.py <command>")
        print("")
        print("Commands:")
        print("  init      - Create rewrite queue")
        print("  status    - Show queue status")
        print("  next      - Process next script")
        print("  run       - Process all scripts")
        sys.exit(1)
    
    cmd = sys.argv[1]
    
    if cmd == "init":
        create_rewrite_queue()
    
    elif cmd == "status":
        show_status()
    
    elif cmd == "next":
        process_next_rewrite()
    
    elif cmd == "run":
        print("🚀 Processing all Python scripts...")
        while True:
            result = process_next_rewrite()
            if result is None:
                break
        show_status()
