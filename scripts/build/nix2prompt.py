#!/usr/bin/env python3
"""
nix2prompt: Convert Nix build failures into Gemini prompts
"""

import subprocess
import json
import sys
from pathlib import Path

def extract_build_error(build_output):
    """Extract key error information from nix build output"""
    lines = build_output.split('\n')
    
    error_info = {
        "error_type": "unknown",
        "error_message": "",
        "file": "",
        "line": 0,
        "context": [],
        "derivation": ""
    }
    
    for i, line in enumerate(lines):
        # Detect error type
        if "error:" in line.lower():
            error_info["error_message"] = line.strip()
            
            # Get context (5 lines before and after)
            start = max(0, i - 5)
            end = min(len(lines), i + 6)
            error_info["context"] = lines[start:end]
            
        # Extract file and line
        if " at " in line and ".nix:" in line:
            parts = line.split(" at ")[-1].split(":")
            if len(parts) >= 2:
                error_info["file"] = parts[0].strip()
                try:
                    error_info["line"] = int(parts[1])
                except:
                    pass
        
        # Extract derivation
        if "building '" in line or "error: builder for '" in line:
            error_info["derivation"] = line.split("'")[1] if "'" in line else ""
    
    # Classify error type
    error_msg = error_info["error_message"].lower()
    if "undefined variable" in error_msg:
        error_info["error_type"] = "undefined_variable"
    elif "syntax error" in error_msg:
        error_info["error_type"] = "syntax_error"
    elif "attribute" in error_msg and "missing" in error_msg:
        error_info["error_type"] = "missing_attribute"
    elif "cannot coerce" in error_msg:
        error_info["error_type"] = "type_error"
    elif "builder for" in error_msg and "failed" in error_msg:
        error_info["error_type"] = "build_failure"
    
    return error_info

def create_gemini_prompt(error_info, flake_path):
    """Create a structured prompt for Gemini"""
    
    # Read the flake file if available
    flake_content = ""
    if error_info["file"]:
        try:
            with open(error_info["file"]) as f:
                flake_content = f.read()
        except:
            pass
    
    prompt = f"""Analyze and fix this Nix build error:

## Error Information
- **Type**: {error_info['error_type']}
- **Message**: {error_info['error_message']}
- **File**: {error_info['file']}
- **Line**: {error_info['line']}
- **Derivation**: {error_info['derivation']}

## Error Context
```
{chr(10).join(error_info['context'])}
```

## Flake Content (around error)
```nix
{flake_content[max(0, error_info['line']-10):error_info['line']+10] if flake_content else 'N/A'}
```

## Task
Provide a JSON response with:
{{
  "fix_type": "string (e.g., 'add_input', 'fix_syntax', 'add_attribute')",
  "description": "string (brief explanation)",
  "commands": ["array of shell commands to run"],
  "files": [
    {{
      "path": "relative/path/to/file",
      "content": "complete fixed file content"
    }}
  ],
  "explanation": "detailed explanation of the fix"
}}

Focus on:
1. Identifying the root cause
2. Providing a complete, working fix
3. Explaining why the fix works
"""
    
    return prompt

def nix2prompt(flake_path, output_file=None):
    """Convert nix build failure to Gemini prompt"""
    
    print(f"🔨 Building {flake_path}...")
    
    # Try to build
    result = subprocess.run(
        ["nix", "build", flake_path, "--show-trace"],
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        print("✅ Build succeeded - no errors to fix!")
        return None
    
    print(f"❌ Build failed - analyzing error...")
    
    # Extract error
    error_info = extract_build_error(result.stderr)
    
    # Create prompt
    prompt = create_gemini_prompt(error_info, flake_path)
    
    # Create output structure
    output = {
        "flake_path": flake_path,
        "error_info": error_info,
        "prompt": prompt,
        "build_output": {
            "stdout": result.stdout[-1000:],  # Last 1000 chars
            "stderr": result.stderr[-1000:]
        }
    }
    
    # Save to file
    if output_file:
        with open(output_file, 'w') as f:
            json.dump(output, f, indent=2)
        print(f"📝 Prompt saved to: {output_file}")
    
    return output

def call_gemini_with_prompt(prompt_file):
    """Call Gemini with the generated prompt"""
    
    with open(prompt_file) as f:
        data = json.load(f)
    
    prompt = data["prompt"]
    
    print(f"🤖 Calling Gemini to fix error...")
    
    # Call Gemini via impure derivation
    result = subprocess.run([
        "nix", "run",
        "./nix/gemini-evolution-impure.nix#gemini-call",
        "--impure",
        "--",
        "-p", prompt,
        "--output-format", "json",
        "--model", "gemini-2.5-flash"
    ], capture_output=True, text=True, timeout=60)
    
    if result.returncode == 0:
        print("✅ Gemini response received")
        return result.stdout
    else:
        print(f"❌ Gemini call failed: {result.stderr[:200]}")
        return None

def apply_fix(fix_json):
    """Apply the fix suggested by Gemini"""
    
    fix = json.loads(fix_json)
    
    print(f"🔧 Applying fix: {fix['fix_type']}")
    print(f"   {fix['description']}")
    
    # Run commands
    if fix.get('commands'):
        for cmd in fix['commands']:
            print(f"   Running: {cmd}")
            subprocess.run(cmd, shell=True)
    
    # Write files
    if fix.get('files'):
        for file_info in fix['files']:
            path = file_info['path']
            content = file_info['content']
            print(f"   Writing: {path}")
            Path(path).parent.mkdir(parents=True, exist_ok=True)
            with open(path, 'w') as f:
                f.write(content)
    
    print("✅ Fix applied")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: nix2prompt <flake-path> [--fix]")
        print("")
        print("Examples:")
        print("  nix2prompt .#default")
        print("  nix2prompt .#default --fix")
        sys.exit(1)
    
    flake_path = sys.argv[1]
    auto_fix = "--fix" in sys.argv
    
    # Generate prompt
    output = nix2prompt(flake_path, "nix_error_prompt.json")
    
    if output is None:
        sys.exit(0)
    
    print("")
    print("📋 Prompt generated:")
    print(output["prompt"][:500] + "...")
    
    if auto_fix:
        print("")
        print("🤖 Auto-fix enabled - calling Gemini...")
        
        fix_response = call_gemini_with_prompt("nix_error_prompt.json")
        
        if fix_response:
            apply_fix(fix_response)
            
            print("")
            print("🔨 Retrying build...")
            result = subprocess.run(["nix", "build", flake_path], capture_output=True)
            
            if result.returncode == 0:
                print("🎉 Build succeeded after fix!")
            else:
                print("❌ Build still failing - may need manual intervention")
