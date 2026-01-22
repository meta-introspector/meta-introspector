#!/usr/bin/env python3
"""
Evolution Monitor API Server
Exposes evolution server errors and status via HTTP API
Based on zos-minimal-server pattern
"""

from flask import Flask, jsonify, request
from pathlib import Path
import json
from datetime import datetime

app = Flask(__name__)

PROJECT_ROOT = Path(__file__).parent.parent.parent
AI_REQUESTS_DIR = PROJECT_ROOT / "data/ai_requests"
ERRORS_DIR = PROJECT_ROOT / "data/errors"
ITERATIONS_DIR = PROJECT_ROOT / "data/iterations"

@app.route('/health', methods=['GET'])
def health():
    """Health check endpoint"""
    return jsonify({
        "status": "ok",
        "timestamp": datetime.now().isoformat(),
        "service": "evolution-monitor"
    })

@app.route('/api/v1/errors', methods=['GET'])
def get_errors():
    """Get all errors"""
    errors = []
    
    if ERRORS_DIR.exists():
        for error_file in sorted(ERRORS_DIR.glob("iter_*_error.json")):
            try:
                with open(error_file) as f:
                    errors.append(json.load(f))
            except:
                pass
    
    return jsonify({
        "count": len(errors),
        "errors": errors
    })

@app.route('/api/v1/errors/latest', methods=['GET'])
def get_latest_error():
    """Get latest error"""
    if not ERRORS_DIR.exists():
        return jsonify({"error": "No errors found"}), 404
    
    error_files = sorted(ERRORS_DIR.glob("iter_*_error.json"))
    if not error_files:
        return jsonify({"error": "No errors found"}), 404
    
    with open(error_files[-1]) as f:
        return jsonify(json.load(f))

@app.route('/api/v1/requests', methods=['GET'])
def get_ai_requests():
    """Get all AI fix requests"""
    requests = []
    
    if AI_REQUESTS_DIR.exists():
        for req_file in sorted(AI_REQUESTS_DIR.glob("iter_*_request.json")):
            try:
                with open(req_file) as f:
                    req_data = json.load(f)
                    
                    # Check for response
                    resp_file = req_file.parent / f"{req_file.stem}_response.json"
                    if resp_file.exists():
                        with open(resp_file) as rf:
                            req_data['response'] = json.load(rf)
                    
                    requests.append(req_data)
            except:
                pass
    
    return jsonify({
        "count": len(requests),
        "requests": requests
    })

@app.route('/api/v1/requests/<int:iteration>', methods=['GET'])
def get_request(iteration):
    """Get specific iteration request"""
    req_file = AI_REQUESTS_DIR / f"iter_{iteration}_request.json"
    
    if not req_file.exists():
        return jsonify({"error": f"Request for iteration {iteration} not found"}), 404
    
    with open(req_file) as f:
        req_data = json.load(f)
    
    # Check for response
    resp_file = AI_REQUESTS_DIR / f"iter_{iteration}_request_response.json"
    if resp_file.exists():
        with open(resp_file) as f:
            req_data['response'] = json.load(f)
    
    return jsonify(req_data)

@app.route('/api/v1/status', methods=['GET'])
def get_status():
    """Get evolution server status"""
    status = {
        "timestamp": datetime.now().isoformat(),
        "errors_count": len(list(ERRORS_DIR.glob("iter_*_error.json"))) if ERRORS_DIR.exists() else 0,
        "requests_count": len(list(AI_REQUESTS_DIR.glob("iter_*_request.json"))) if AI_REQUESTS_DIR.exists() else 0,
        "iterations_count": len(list(ITERATIONS_DIR.glob("iter_*.log"))) if ITERATIONS_DIR.exists() else 0
    }
    
    # Get latest orbit
    orbit_file = PROJECT_ROOT / "data/last_orbit.txt"
    if orbit_file.exists():
        status['latest_orbit'] = orbit_file.read_text().strip()
    
    # Get latest proof
    proof_file = PROJECT_ROOT / "data/last_proof.txt"
    if proof_file.exists():
        status['latest_proof'] = proof_file.read_text().strip()
    
    return jsonify(status)

@app.route('/api/v1/iterations', methods=['GET'])
def get_iterations():
    """Get iteration history"""
    iterations = []
    
    if ITERATIONS_DIR.exists():
        for iter_file in sorted(ITERATIONS_DIR.glob("iter_*_orbit.txt")):
            iteration = int(iter_file.stem.split('_')[1])
            orbit = iter_file.read_text().strip()
            
            proof_file = ITERATIONS_DIR / f"iter_{iteration}_proof.txt"
            proof = proof_file.read_text().strip() if proof_file.exists() else None
            
            iterations.append({
                "iteration": iteration,
                "orbit": orbit,
                "proof": proof
            })
    
    return jsonify({
        "count": len(iterations),
        "iterations": iterations
    })

if __name__ == '__main__':
    print("🚀 Evolution Monitor API Server")
    print("================================")
    print("")
    print("Endpoints:")
    print("  GET /health")
    print("  GET /api/v1/errors")
    print("  GET /api/v1/errors/latest")
    print("  GET /api/v1/requests")
    print("  GET /api/v1/requests/<iteration>")
    print("  GET /api/v1/status")
    print("  GET /api/v1/iterations")
    print("")
    print("Starting on http://0.0.0.0:8080")
    print("")
    
    app.run(host='0.0.0.0', port=8080, debug=False)
