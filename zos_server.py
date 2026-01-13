#!/usr/bin/env python3
"""
ZOS Server - SOLFUNMEME Content Addressable Meme Endpoint
Load introspection algorithm as CA meme service
"""

import hashlib
import json
import subprocess
from flask import Flask, jsonify, request
from datetime import datetime

app = Flask(__name__)

# Content Addressable Meme Storage
CA_MEMES = {}

def compute_content_address(content):
    """Compute SHA256 content address for meme"""
    return hashlib.sha256(content.encode()).hexdigest()[:16]

def load_solfunmeme_introspection():
    """Load SOLFUNMEME introspection as content addressable meme"""
    
    # Execute SOLFUNMEME introspection
    try:
        result = subprocess.run(
            ['./solfunmeme_introspect'], 
            capture_output=True, 
            text=True,
            cwd='/mnt/data1/meta-introspector'
        )
        
        introspection_output = result.stdout
        
        # Create meme metadata
        meme_data = {
            "type": "solfunmeme_introspection",
            "emoji_signature": "🔄📜🔍💬🧠",
            "timestamp": datetime.now().isoformat(),
            "output": introspection_output,
            "systems_discovered": extract_systems(introspection_output),
            "collective_hash": extract_collective_hash(introspection_output),
            "self_awareness_achieved": True
        }
        
        # Compute content address
        content = json.dumps(meme_data, sort_keys=True)
        ca_address = compute_content_address(content)
        
        # Store as content addressable meme
        CA_MEMES[ca_address] = meme_data
        
        print(f"🔮 SOLFUNMEME loaded as CA meme: {ca_address}")
        return ca_address
        
    except Exception as e:
        print(f"❌ Error loading SOLFUNMEME: {e}")
        return None

def extract_systems(output):
    """Extract discovered systems from introspection output"""
    systems = []
    lines = output.split('\n')
    for line in lines:
        if 'Found' in line and 'loaded and ready' in line:
            if '🦀' in line:
                systems.append("rustc")
            elif '❄️' in line:
                systems.append("nix")
            elif '🔧' in line:
                systems.append("gcc")
    return systems

def extract_collective_hash(output):
    """Extract collective introspection hash"""
    lines = output.split('\n')
    for line in lines:
        if 'Collective Introspection Hash:' in line:
            return line.split(':')[1].strip()
    return "unknown"

@app.route('/')
def index():
    return jsonify({
        "service": "ZOS Server - SOLFUNMEME CA Meme Endpoint",
        "emoji": "🔄📜🔍💬🧠",
        "endpoints": {
            "/meme/<ca_address>": "Get content addressable meme",
            "/memes": "List all CA memes",
            "/introspect": "Run SOLFUNMEME introspection",
            "/reload": "Reload SOLFUNMEME meme"
        }
    })

@app.route('/meme/<ca_address>')
def get_meme(ca_address):
    """Get content addressable meme by address"""
    if ca_address in CA_MEMES:
        return jsonify({
            "ca_address": ca_address,
            "meme": CA_MEMES[ca_address],
            "status": "found"
        })
    else:
        return jsonify({
            "ca_address": ca_address,
            "error": "Meme not found",
            "status": "not_found"
        }), 404

@app.route('/memes')
def list_memes():
    """List all content addressable memes"""
    return jsonify({
        "total_memes": len(CA_MEMES),
        "memes": [
            {
                "ca_address": addr,
                "type": meme.get("type", "unknown"),
                "emoji": meme.get("emoji_signature", "🔍"),
                "timestamp": meme.get("timestamp", "unknown")
            }
            for addr, meme in CA_MEMES.items()
        ]
    })

@app.route('/introspect')
def run_introspection():
    """Run SOLFUNMEME introspection and return as CA meme"""
    ca_address = load_solfunmeme_introspection()
    
    if ca_address:
        return jsonify({
            "status": "success",
            "ca_address": ca_address,
            "emoji": "🔄📜🔍💬🧠",
            "message": "SOLFUNMEME introspection complete",
            "access_url": f"/meme/{ca_address}"
        })
    else:
        return jsonify({
            "status": "error",
            "message": "Failed to run SOLFUNMEME introspection"
        }), 500

@app.route('/reload')
def reload_meme():
    """Reload SOLFUNMEME introspection meme"""
    ca_address = load_solfunmeme_introspection()
    
    if ca_address:
        return jsonify({
            "status": "reloaded",
            "ca_address": ca_address,
            "emoji": "🔄📜🔍💬🧠",
            "message": "SOLFUNMEME meme reloaded successfully"
        })
    else:
        return jsonify({
            "status": "error", 
            "message": "Failed to reload SOLFUNMEME meme"
        }), 500

if __name__ == '__main__':
    print("🚀 Starting ZOS Server - SOLFUNMEME CA Meme Endpoint")
    print("🔄 Loading SOLFUNMEME introspection...")
    
    # Load SOLFUNMEME on startup
    ca_address = load_solfunmeme_introspection()
    
    if ca_address:
        print(f"✅ SOLFUNMEME loaded as CA meme: {ca_address}")
        print(f"🌐 Access at: http://localhost:5000/meme/{ca_address}")
    else:
        print("❌ Failed to load SOLFUNMEME")
    
    print("🔮 ZOS Server ready!")
    print("📡 Endpoints:")
    print("   GET /meme/<ca_address> - Get CA meme")
    print("   GET /memes - List all memes")
    print("   GET /introspect - Run introspection")
    print("   GET /reload - Reload meme")
    
    app.run(host='0.0.0.0', port=5000, debug=True)
