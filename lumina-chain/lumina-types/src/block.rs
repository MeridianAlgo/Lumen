use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub prev_hash: [u8; 32],
    pub height: u64,
    pub timestamp: u64,
    pub tx_merkle_root: [u8; 32],
    pub state_root: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}
