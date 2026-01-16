// Distributed P2P network: Nodes share findings via libp2p and HuggingFace

use std::collections::HashMap;

// P2P Node that shares findings
pub struct P2PNode {
    pub node_id: String,
    pub local_findings: HashMap<String, Finding>,
    pub peer_findings: HashMap<String, Finding>,
}

#[derive(Clone)]
pub struct Finding {
    pub hash: String,
    pub data_type: String,  // "rustc_ip", "git_oid", "snippet"
    pub compression_ratio: f64,
    pub source: String,
    pub timestamp: u64,
}

impl P2PNode {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            local_findings: HashMap::new(),
            peer_findings: HashMap::new(),
        }
    }
    
    pub fn add_finding(&mut self, finding: Finding) {
        self.local_findings.insert(finding.hash.clone(), finding);
    }
    
    pub fn export_to_huggingface(&self) -> String {
        // Export findings as JSON for HuggingFace dataset
        let mut json = String::from("[\n");
        
        for finding in self.local_findings.values() {
            json.push_str(&format!(
                "  {{\"hash\":\"{}\",\"type\":\"{}\",\"ratio\":{:.3},\"source\":\"{}\"}},\n",
                finding.hash, finding.data_type, finding.compression_ratio, finding.source
            ));
        }
        
        json.push_str("]\n");
        json
    }
    
    pub fn share_with_peer(&mut self, peer_findings: Vec<Finding>) {
        for finding in peer_findings {
            if !self.local_findings.contains_key(&finding.hash) {
                self.peer_findings.insert(finding.hash.clone(), finding);
            }
        }
    }
    
    pub fn report(&self) {
        println!("\n📡 P2P Node: {}", self.node_id);
        println!("  Local findings: {}", self.local_findings.len());
        println!("  Peer findings: {}", self.peer_findings.len());
        println!("  Total knowledge: {}", self.local_findings.len() + self.peer_findings.len());
    }
}

pub struct P2PNetwork {
    pub nodes: Vec<P2PNode>,
}

impl P2PNetwork {
    pub fn new(num_nodes: usize) -> Self {
        let nodes = (0..num_nodes)
            .map(|i| P2PNode::new(format!("node_{}", i)))
            .collect();
        
        Self { nodes }
    }
    
    pub fn broadcast_findings(&mut self) {
        // Each node shares with all others
        for i in 0..self.nodes.len() {
            let findings: Vec<Finding> = self.nodes[i].local_findings.values().cloned().collect();
            
            for j in 0..self.nodes.len() {
                if i != j {
                    self.nodes[j].share_with_peer(findings.clone());
                }
            }
        }
    }
    
    pub fn export_all_to_huggingface(&self, path: &str) {
        for (i, node) in self.nodes.iter().enumerate() {
            let json = node.export_to_huggingface();
            let file_path = format!("{}/node_{}.json", path, i);
            std::fs::write(&file_path, json).ok();
        }
    }
    
    pub fn report(&self) {
        println!("\n🌐 P2P Network Report");
        println!("  Total nodes: {}", self.nodes.len());
        
        let total_local: usize = self.nodes.iter().map(|n| n.local_findings.len()).sum();
        let total_peer: usize = self.nodes.iter().map(|n| n.peer_findings.len()).sum();
        
        println!("  Total local findings: {}", total_local);
        println!("  Total peer findings: {}", total_peer);
        println!("  Network knowledge: {}", total_local + total_peer);
    }
}
