#!/usr/bin/env python3
"""
🌟 ZOS SERVER: Zero-Knowledge Operating System with SOLFUNMEME + Rust-as-a-Service Integration
Loads SOLFUNMEME introspection AND rustc_driver.so as paid compilation endpoints
"""

import asyncio
import json
import subprocess
import hashlib
import requests
from pathlib import Path
from typing import Dict, Any, Optional
from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse
from pydantic import BaseModel
import uvicorn

app = FastAPI(title="ZOS Server", description="Zero-Knowledge Operating System with SOLFUNMEME + Rust-as-a-Service")

class CompileRequest(BaseModel):
    source_code: str
    target: Optional[str] = None
    optimization: Optional[str] = None
    features: list = []
    payment_lamports: int

class ZOSServer:
    def __init__(self):
        self.loaded_services = {}
        self.meme_cache = {}
        self.rust_service_url = "http://localhost:8080"
        self.zombie_driver_path = "/home/mdupont/zombie_driver2"
        
    async def load_solfunmeme_service(self) -> Dict[str, Any]:
        """Load SOLFUNMEME introspection as content addressable service"""
        try:
            # Run SOLFUNMEME introspection
            result = subprocess.run(
                ["./solfunmeme_introspect"],
                cwd="/mnt/data1/meta-introspector",
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode == 0:
                # Create content hash for addressability
                content_hash = hashlib.sha256(result.stdout.encode()).hexdigest()[:16]
                
                service_info = {
                    "service_id": f"solfunmeme_{content_hash}",
                    "type": "introspection_meme",
                    "output": result.stdout,
                    "hash": content_hash,
                    "status": "loaded",
                    "endpoints": [
                        f"/meme/{content_hash}",
                        f"/introspect/{content_hash}",
                        f"/solfunmeme/{content_hash}"
                    ]
                }
                
                self.loaded_services[content_hash] = service_info
                return service_info
            else:
                raise Exception(f"SOLFUNMEME failed: {result.stderr}")
                
        except Exception as e:
            return {"error": str(e), "status": "failed"}

    async def load_rust_service(self) -> Dict[str, Any]:
        """Load Rust-as-a-Service using zombie_driver2"""
        try:
            # Start Rust compilation service in background
            rust_process = subprocess.Popen(
                ["cargo", "run", "--bin", "rust_as_a_service"],
                cwd="/mnt/data1/meta-introspector",
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE
            )
            
            # Wait a moment for service to start
            await asyncio.sleep(2)
            
            # Test if service is running
            try:
                response = requests.get(f"{self.rust_service_url}/metrics", timeout=5)
                if response.status_code == 200:
                    service_hash = hashlib.sha256(f"rust_service_{self.zombie_driver_path}".encode()).hexdigest()[:16]
                    
                    service_info = {
                        "service_id": f"rustc_{service_hash}",
                        "type": "rust_compilation",
                        "zombie_driver": self.zombie_driver_path,
                        "hash": service_hash,
                        "status": "loaded",
                        "process_id": rust_process.pid,
                        "endpoints": [
                            f"/compile/{service_hash}",
                            f"/rust/{service_hash}",
                            f"/devnet/compile/{service_hash}"
                        ],
                        "pricing": {
                            "base_cost": 1000,
                            "per_line_cost": 10,
                            "optimization_multiplier": 1.5,
                            "feature_cost": 100
                        }
                    }
                    
                    self.loaded_services[service_hash] = service_info
                    return service_info
                else:
                    raise Exception("Rust service not responding")
            except requests.RequestException as e:
                raise Exception(f"Failed to connect to Rust service: {e}")
                
        except Exception as e:
            return {"error": str(e), "status": "failed"}

    async def compile_rust_code(self, request: CompileRequest) -> Dict[str, Any]:
        """Compile Rust code using loaded rustc service"""
        try:
            # Forward to Rust-as-a-Service
            response = requests.post(
                f"{self.rust_service_url}/compile",
                json={
                    "source_code": request.source_code,
                    "target": request.target,
                    "optimization": request.optimization,
                    "features": request.features
                },
                timeout=60
            )
            
            if response.status_code == 200:
                result = response.json()
                
                # Verify payment covers cost
                if request.payment_lamports >= result.get("cost_lamports", 0):
                    return {
                        "compilation": result,
                        "payment_status": "accepted",
                        "change_lamports": request.payment_lamports - result.get("cost_lamports", 0)
                    }
                else:
                    return {
                        "error": "Insufficient payment",
                        "required_lamports": result.get("cost_lamports", 0),
                        "provided_lamports": request.payment_lamports
                    }
            else:
                return {"error": "Compilation service error", "status_code": response.status_code}
                
        except Exception as e:
            return {"error": str(e), "status": "failed"}

zos = ZOSServer()

@app.get("/")
async def root():
    return {
        "message": "🌟 ZOS Server: Zero-Knowledge Operating System + Rust-as-a-Service", 
        "version": "2.0.0",
        "services": ["solfunmeme", "rustc_driver", "zombie_driver2"]
    }

@app.post("/load/solfunmeme")
async def load_solfunmeme():
    """Load SOLFUNMEME introspection service"""
    result = await zos.load_solfunmeme_service()
    return JSONResponse(content=result)

@app.post("/load/rust")
async def load_rust():
    """Load Rust-as-a-Service using zombie_driver2"""
    result = await zos.load_rust_service()
    return JSONResponse(content=result)

@app.post("/devnet/compile")
async def devnet_compile(request: CompileRequest):
    """Compile Rust code on devnet (pay-per-compilation)"""
    result = await zos.compile_rust_code(request)
    return JSONResponse(content=result)

@app.get("/services")
async def list_services():
    """List all loaded services"""
    return {"services": zos.loaded_services}

@app.get("/pricing/rust")
async def rust_pricing():
    """Get Rust compilation pricing"""
    try:
        response = requests.get(f"{zos.rust_service_url}/pricing", timeout=5)
        if response.status_code == 200:
            return response.json()
        else:
            return {"error": "Pricing service unavailable"}
    except:
        return {
            "base_cost": 1000,
            "per_line_cost": 10,
            "optimization_multiplier": 1.5,
            "feature_cost": 100,
            "currency": "lamports"
        }

@app.get("/meme/{content_hash}")
async def get_meme(content_hash: str):
    """Get meme by content hash"""
    if content_hash in zos.loaded_services:
        return zos.loaded_services[content_hash]
    raise HTTPException(status_code=404, detail="Meme not found")

@app.get("/compile/{service_hash}")
async def get_compile_service(service_hash: str):
    """Get compilation service info by hash"""
    if service_hash in zos.loaded_services:
        service = zos.loaded_services[service_hash]
        if service.get("type") == "rust_compilation":
            return service
    raise HTTPException(status_code=404, detail="Compilation service not found")

@app.get("/introspect/{content_hash}")
async def introspect_service(content_hash: str):
    """Introspect service by content hash"""
    if content_hash in zos.loaded_services:
        service = zos.loaded_services[content_hash]
        return {
            "introspection": service.get("output", ""),
            "metadata": {
                "hash": service.get("hash"),
                "type": service.get("type"),
                "status": service.get("status")
            }
        }
    raise HTTPException(status_code=404, detail="Service not found")

if __name__ == "__main__":
    print("🚀 Starting ZOS Server with Rust-as-a-Service...")
    print("🦀 Loading zombie_driver2 rustc capabilities...")
    print("💰 Pay-per-compilation devnet service enabled")
    uvicorn.run(app, host="0.0.0.0", port=8000)
