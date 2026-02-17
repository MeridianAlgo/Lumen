use serde::{Serialize, Deserialize};
use lumina_crypto::signatures::Sig;
use crate::instruction::StablecoinInstruction;
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub instruction: StablecoinInstruction,
    pub sender: [u8; 32], // Public key bytes
    pub nonce: u64,
    pub signature: Option<Sig>,
}

impl Transaction {
    pub fn signable_bytes(&self) -> Vec<u8> {
        let mut tx = self.clone();
        tx.signature = None;
        bincode::serialize(&tx).expect("Failed to serialize transaction for signing")
    }
}
