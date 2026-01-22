#!/usr/bin/env python3
"""
Evolution Server: Runs bootstrap 10k times and collaborates with AI to fix errors
"""

import subprocess
import json
import time
import os
from pathlib import Path
from datetime import datetime

class EvolutionServer:
    def __init__(self):
        self.project_root = Path(__file__).parent.parent.parent
        self.iteration = 0
        self.max_iterations = 10000
        self.errors = []
        self.evolutions = []
        
    def run_bootstrap(self):
        """Run single bootstrap iteration"""
        print(f"\n{'='*60}")
        print(f"Iteration {self.iteration} / {self.max_iterations}")
        print(f"{'='*60}\n")
        
        result = subprocess.run(
            ["./scripts/build/bootstrap.sh"],
            cwd=self.project_root,
            capture_output=True,
            text=True
        )
        
        return {
            "iteration": self.iteration,
            "returncode": result.returncode,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "timestamp": datetime.now().isoformat()
        }
    
    def analyze_error(self, result):
        """Analyze error and prepare for AI collaboration"""
        error_data = {
            "iteration": result["iteration"],
            "error_type": self.classify_error(result["stderr"]),
            "stderr": result["stderr"][-1000:],  # Last 1000 chars
            "stdout": result["stdout"][-1000:],
            "timestamp": result["timestamp"]
        }
        
        # Save error for AI review
        error_file = self.project_root / f"data/errors/iter_{self.iteration}_error.json"
        error_file.parent.mkdir(parents=True, exist_ok=True)
        with open(error_file, 'w') as f:
            json.dump(error_data, f, indent=2)
        
        return error_data
    
    def classify_error(self, stderr):
        """Classify error type"""
        if "cannot connect to socket" in stderr:
            return "nix_daemon"
        elif "duplicates" in stderr.lower():
            return "duplicates_found"
        elif "build failed" in stderr.lower():
            return "build_failure"
        elif "permission denied" in stderr.lower():
            return "permission"
        else:
            return "unknown"
    
    def request_ai_fix(self, error_data):
        """Create request for AI to fix error"""
        request = {
            "type": "fix_request",
            "iteration": error_data["iteration"],
            "error_type": error_data["error_type"],
            "context": {
                "stderr": error_data["stderr"],
                "stdout": error_data["stdout"]
            },
            "request": f"Please analyze this error and suggest a fix for iteration {error_data['iteration']}",
            "timestamp": datetime.now().isoformat()
        }
        
        # Save request
        request_file = self.project_root / f"data/ai_requests/iter_{self.iteration}_request.json"
        request_file.parent.mkdir(parents=True, exist_ok=True)
        with open(request_file, 'w') as f:
            json.dump(request, f, indent=2)
        
        print(f"\n🤖 AI Fix Request Created:")
        print(f"   File: {request_file}")
        print(f"   Error Type: {error_data['error_type']}")
        print(f"   Waiting for AI response...")
        
        return request_file
    
    def check_for_ai_response(self, request_file):
        """Check if AI has provided a fix"""
        response_file = request_file.parent / f"{request_file.stem}_response.json"
        
        if response_file.exists():
            with open(response_file, 'r') as f:
                return json.load(f)
        
        return None
    
    def apply_ai_fix(self, fix_data):
        """Apply fix suggested by AI"""
        print(f"\n✨ Applying AI fix:")
        print(f"   Type: {fix_data.get('fix_type')}")
        print(f"   Description: {fix_data.get('description')}")
        
        if fix_data.get('commands'):
            for cmd in fix_data['commands']:
                print(f"   Running: {cmd}")
                subprocess.run(cmd, shell=True, cwd=self.project_root)
        
        if fix_data.get('files'):
            for file_change in fix_data['files']:
                file_path = self.project_root / file_change['path']
                print(f"   Updating: {file_path}")
                with open(file_path, 'w') as f:
                    f.write(file_change['content'])
        
        return True
    
    def check_convergence(self):
        """Check if system has converged"""
        if self.iteration < 10:
            return False
        
        # Check last 10 orbits
        orbit_file = self.project_root / "data/last_orbit.txt"
        if not orbit_file.exists():
            return False
        
        # Simple convergence: same orbit for 10 iterations
        # (Real implementation would check actual orbit history)
        return False
    
    def run(self):
        """Main evolution loop"""
        print("🚀 Evolution Server Starting")
        print(f"   Max iterations: {self.max_iterations}")
        print(f"   Project: {self.project_root}")
        print(f"   Mode: AI-collaborative evolution")
        print()
        
        while self.iteration < self.max_iterations:
            self.iteration += 1
            
            # Run bootstrap
            result = self.run_bootstrap()
            
            if result["returncode"] == 0:
                print(f"✅ Iteration {self.iteration}: Success")
                
                # Check for evolution
                orbit_file = self.project_root / "data/last_orbit.txt"
                if orbit_file.exists():
                    orbit = orbit_file.read_text().strip()
                    print(f"   Orbit: {orbit}")
                
                # Check convergence
                if self.check_convergence():
                    print("\n🎯 CONVERGENCE DETECTED!")
                    print("   System has reached stable form")
                    break
                
            else:
                print(f"❌ Iteration {self.iteration}: Failed")
                
                # Analyze error
                error_data = self.analyze_error(result)
                print(f"   Error type: {error_data['error_type']}")
                
                # Request AI fix
                request_file = self.request_ai_fix(error_data)
                
                # Wait for AI response (with timeout)
                timeout = 300  # 5 minutes
                start_time = time.time()
                
                while time.time() - start_time < timeout:
                    response = self.check_for_ai_response(request_file)
                    
                    if response:
                        print(f"\n✅ AI Response Received!")
                        
                        # Apply fix
                        if self.apply_ai_fix(response):
                            print(f"   Fix applied, retrying iteration {self.iteration}")
                            self.iteration -= 1  # Retry same iteration
                            break
                    
                    time.sleep(5)  # Check every 5 seconds
                else:
                    print(f"   ⏱️  Timeout waiting for AI response")
                    print(f"   Continuing to next iteration...")
            
            # Brief pause between iterations
            time.sleep(1)
        
        print(f"\n🏁 Evolution Complete!")
        print(f"   Total iterations: {self.iteration}")
        print(f"   Errors encountered: {len(self.errors)}")
        print(f"   Evolutions detected: {len(self.evolutions)}")

if __name__ == "__main__":
    server = EvolutionServer()
    server.run()
