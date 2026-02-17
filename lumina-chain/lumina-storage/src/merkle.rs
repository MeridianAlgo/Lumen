use blake3;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MerkleState {
    // We use a BTreeMap to ensure deterministic ordering for the root hash
    pub leaves: BTreeMap<[u8; 32], Vec<u8>>,
}

impl MerkleState {
    pub fn new() -> Self {
        Self { leaves: BTreeMap::new() }
    }

    pub fn insert(&mut self, key: [u8; 32], value: Vec<u8>) {
        self.leaves.insert(key, value);
    }

    pub fn root_hash(&self) -> [u8; 32] {
        if self.leaves.is_empty() {
            return [0u8; 32];
        }
        
        let mut hasher = blake3::Hasher::new();
        for (key, value) in &self.leaves {
            hasher.update(key);
            hasher.update(value);
        }
        *hasher.finalize().as_bytes()
    }
}
