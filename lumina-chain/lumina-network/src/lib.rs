use libp2p::{
    gossipsub, mdns, swarm::NetworkBehaviour,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
pub struct LuminaBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub async fn start_p2p() -> Result<(), Box<dyn Error>> {
    println!("Starting P2P Network...");
    // 1. Generate keys
    // 2. Create transport
    // 3. Create behaviour
    // 4. Create swarm
    // 5. Listen and serve
    Ok(())
}
