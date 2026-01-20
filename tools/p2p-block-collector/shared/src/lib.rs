use serde::{Deserialize, Serialize};

pub const CONTRACT_ADDRESS: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub slot: u64,
    pub hash: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMessage {
    pub block: Block,
    pub client_id: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Register { peer_id: String },
    Registered { contract: String },
    FetchRequest { slot_start: u64, slot_end: u64 },
    BlockData { data: BlockMessage },
    Payment { amount: f64, tx_hash: String },
    Ban { reason: String, duration_secs: u64 },
}
