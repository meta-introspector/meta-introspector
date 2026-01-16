use libp2p::{gossipsub, kad, mdns, noise, swarm::SwarmEvent, tcp, yamux, Swarm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Meme {
    hash: String,
    data: Vec<u8>,
    meme_type: MemeType,
    godel_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum MemeType {
    GitObject,
    WasmBlock,
    Recipe,
}

pub struct MemeSwarm {
    swarm: Swarm<MemeNode>,
    memes: HashMap<String, Meme>,
}

impl MemeSwarm {
    pub async fn new() -> Self {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = local_key.public().to_peer_id();
        
        println!("🌐 P2P Peer ID: {}", local_peer_id);
        
        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();
        
        let behaviour = MemeNode::new(local_peer_id);
        let swarm = Swarm::new(transport, behaviour, local_peer_id, Default::default());
        
        Self {
            swarm,
            memes: HashMap::new(),
        }
    }
    
    pub fn publish_meme(&mut self, meme: Meme) {
        let hash = meme.hash.clone();
        let data = serde_json::to_vec(&meme).unwrap();
        
        self.swarm.behaviour_mut().gossipsub
            .publish(gossipsub::IdentTopic::new("memes"), data)
            .ok();
        
        self.memes.insert(hash, meme);
        println!("📤 Published meme");
    }
    
    pub fn get_meme(&self, hash: &str) -> Option<&Meme> {
        self.memes.get(hash)
    }
}

struct MemeNode {
    gossipsub: gossipsub::Behaviour,
    kad: kad::Behaviour<kad::store::MemoryStore>,
    mdns: mdns::tokio::Behaviour,
}

impl MemeNode {
    fn new(peer_id: libp2p::PeerId) -> Self {
        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(libp2p::identity::Keypair::generate_ed25519()),
            gossipsub::Config::default(),
        ).unwrap();
        
        let kad = kad::Behaviour::new(
            peer_id,
            kad::store::MemoryStore::new(peer_id),
        );
        
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            peer_id,
        ).unwrap();
        
        Self { gossipsub, kad, mdns }
    }
}

impl libp2p::swarm::NetworkBehaviour for MemeNode {
    type ConnectionHandler = libp2p::swarm::dummy::ConnectionHandler;
    type ToSwarm = ();
    
    fn handle_established_inbound_connection(
        &mut self,
        _: libp2p::swarm::ConnectionId,
        _: libp2p::PeerId,
        _: &libp2p::Multiaddr,
        _: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }
    
    fn handle_established_outbound_connection(
        &mut self,
        _: libp2p::swarm::ConnectionId,
        _: libp2p::PeerId,
        _: &libp2p::Multiaddr,
        _: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }
    
    fn on_swarm_event(&mut self, _: libp2p::swarm::FromSwarm) {}
    
    fn on_connection_handler_event(
        &mut self,
        _: libp2p::PeerId,
        _: libp2p::swarm::ConnectionId,
        _: libp2p::swarm::THandlerOutEvent<Self>,
    ) {}
    
    fn poll(&mut self, _: &mut std::task::Context) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::ToSwarm, libp2p::swarm::THandlerInEvent<Self>>> {
        std::task::Poll::Pending
    }
}
