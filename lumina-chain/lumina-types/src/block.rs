use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockHeader {
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub transactions_root: [u8; 32],
    pub state_root: [u8; 32],
    pub timestamp: u64,
    pub proposer: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub votes: Vec<Vote>, // Commit signatures from validators
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vote {
    pub validator: [u8; 32],
    pub signature: Vec<u8>,
}

impl Block {
    pub fn hash(&self) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&bincode::serialize(&self.header).unwrap());
        *hasher.finalize().as_bytes()
    }
}
