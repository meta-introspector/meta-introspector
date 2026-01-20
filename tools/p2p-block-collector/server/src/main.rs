use p2p_shared::*;
use std::collections::HashMap;

struct Server {
    clients: HashMap<String, ClientState>,
}

struct ClientState {
    peer_id: String,
    blocks_submitted: u64,
    earned: f64,
}

impl Server {
    fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    
    async fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Register { peer_id } => {
                println!("✅ Client registered: {}", peer_id);
                self.clients.insert(peer_id.clone(), ClientState {
                    peer_id,
                    blocks_submitted: 0,
                    earned: 0.0,
                });
            }
            Message::BlockData { data } => {
                println!("📦 Block received: slot {}", data.block.slot);
                if let Some(client) = self.clients.get_mut(&data.client_id) {
                    client.blocks_submitted += 1;
                    client.earned += 0.001;
                    println!("💰 Paid {} SOL (total: {})", 0.001, client.earned);
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🚀 P2P Block Collector Server");
    println!("Contract: {}", CONTRACT_ADDRESS);
    
    let mut server = Server::new();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("⏳ Listening on 0.0.0.0:9000");
    
    tokio::signal::ctrl_c().await.unwrap();
    println!("👋 Shutting down");
}
