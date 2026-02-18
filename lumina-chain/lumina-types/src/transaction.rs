use serde::{Serialize, Deserialize};
use crate::instruction::StablecoinInstruction;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub sender: [u8; 32],
    pub nonce: u64,
    pub instruction: StablecoinInstruction,
    pub signature: Vec<u8>,
    pub gas_limit: u64,
    pub gas_price: u64,
}

impl Transaction {
    pub fn id(&self) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&bincode::serialize(self).unwrap());
        *hasher.finalize().as_bytes()
    }
}
