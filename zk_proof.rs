use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZkProof {
    pub godel_number: String,
    pub trace_hash: String,
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

impl ZkProof {
    pub fn generate(wasm: &[u8], trace: &[u8]) -> Self {
        // Generate ZK proof that execution produced this trace
        let godel = format!("{:x}", md5::compute(trace));
        let trace_hash = format!("{:x}", sha2::Sha256::digest(trace));
        
        // Simplified proof (in production use risc0)
        let proof = sha2::Sha256::digest(&[wasm, trace].concat()).to_vec();
        
        Self {
            godel_number: godel,
            trace_hash,
            proof,
            public_inputs: trace.to_vec(),
        }
    }
    
    pub fn verify(&self, wasm: &[u8]) -> bool {
        // Verify ZK proof
        let expected = sha2::Sha256::digest(&[wasm, &self.public_inputs].concat()).to_vec();
        self.proof == expected
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenExecution {
    pub wasm: Vec<u8>,
    pub result: i32,
    pub trace: Vec<u8>,
    pub proof: ZkProof,
    pub perf_data: Option<String>,
}

impl ProvenExecution {
    pub fn new(wasm: Vec<u8>, result: i32, trace: Vec<u8>) -> Self {
        let proof = ZkProof::generate(&wasm, &trace);
        Self {
            wasm,
            result,
            trace,
            proof,
            perf_data: None,
        }
    }
    
    pub fn verify(&self) -> bool {
        self.proof.verify(&self.wasm)
    }
    
    pub fn verify_with_perf(&self) -> bool {
        self.verify() && self.perf_data.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zk_proof() {
        let wasm = vec![0x00, 0x61, 0x73, 0x6d];
        let trace = vec![1, 2, 3, 4];
        
        let exec = ProvenExecution::new(wasm, 42, trace);
        assert!(exec.verify());
        
        println!("Godel: {}", exec.proof.godel_number);
        println!("Proof: {:?}", &exec.proof.proof[..8]);
    }
}
