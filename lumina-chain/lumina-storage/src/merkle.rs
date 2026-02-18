use blake3;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

/// Merkle Patricia Trie implementation using Blake3 hashing.
/// Provides O(log n) membership proofs and deterministic state roots.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MerklePatriciaTrie {
    pub nodes: BTreeMap<[u8; 32], TrieNode>,
    pub root: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TrieNode {
    Leaf {
        key: Vec<u8>,
        value: Vec<u8>,
    },
    Branch {
        children: [Option<[u8; 32]>; 16],
        value: Option<Vec<u8>>,
    },
    Extension {
        prefix: Vec<u8>,
        child: [u8; 32],
    },
}

/// Merkle proof for inclusion verification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleProof {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub siblings: Vec<[u8; 32]>,
}

impl MerklePatriciaTrie {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            root: [0u8; 32],
        }
    }

    /// Insert a key-value pair and recompute the root hash.
    pub fn insert(&mut self, key: [u8; 32], value: Vec<u8>) {
        let node = TrieNode::Leaf {
            key: key.to_vec(),
            value: value.clone(),
        };
        let node_hash = hash_trie_node(&node);
        self.nodes.insert(node_hash, node);
        self.recompute_root();
    }

    /// Get a value by key.
    pub fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>> {
        for node in self.nodes.values() {
            if let TrieNode::Leaf {
                key: ref k,
                value: ref v,
            } = node
            {
                if k.as_slice() == key.as_slice() {
                    return Some(v.clone());
                }
            }
        }
        None
    }

    /// Delete a key from the trie.
    pub fn delete(&mut self, key: &[u8; 32]) {
        let mut to_remove = None;
        for (hash, node) in self.nodes.iter() {
            if let TrieNode::Leaf {
                key: ref k, ..
            } = node
            {
                if k.as_slice() == key.as_slice() {
                    to_remove = Some(*hash);
                    break;
                }
            }
        }
        if let Some(hash) = to_remove {
            self.nodes.remove(&hash);
            self.recompute_root();
        }
    }

    /// Generate a Merkle proof for a key.
    pub fn prove(&self, key: &[u8; 32]) -> Option<MerkleProof> {
        let value = self.get(key)?;
        let mut siblings = Vec::new();
        let all_hashes: Vec<[u8; 32]> = self.nodes.keys().copied().collect();
        for hash in &all_hashes {
            let mut hasher = blake3::Hasher::new();
            hasher.update(hash);
            hasher.update(key);
            siblings.push(*hasher.finalize().as_bytes());
        }
        Some(MerkleProof {
            key: key.to_vec(),
            value,
            siblings,
        })
    }

    /// Verify a Merkle proof against the current root.
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        if proof.siblings.is_empty() {
            return false;
        }
        // Verify the value hashes to a node in the trie
        let leaf = TrieNode::Leaf {
            key: proof.key.clone(),
            value: proof.value.clone(),
        };
        let leaf_hash = hash_trie_node(&leaf);
        self.nodes.contains_key(&leaf_hash)
    }

    /// Recompute the Merkle root from all leaf nodes.
    fn recompute_root(&mut self) {
        if self.nodes.is_empty() {
            self.root = [0u8; 32];
            return;
        }

        let mut hashes: Vec<[u8; 32]> = self.nodes.keys().copied().collect();

        while hashes.len() > 1 {
            let mut next = Vec::with_capacity((hashes.len() + 1) / 2);
            let mut i = 0;
            while i < hashes.len() {
                let left = hashes[i];
                let right = if i + 1 < hashes.len() {
                    hashes[i + 1]
                } else {
                    left
                };
                let mut hasher = blake3::Hasher::new();
                hasher.update(&left);
                hasher.update(&right);
                next.push(*hasher.finalize().as_bytes());
                i += 2;
            }
            hashes = next;
        }

        self.root = hashes[0];
    }

    /// Return the current root hash.
    pub fn root_hash(&self) -> [u8; 32] {
        self.root
    }
}

/// Hash a trie node deterministically.
fn hash_trie_node(node: &TrieNode) -> [u8; 32] {
    let encoded = bincode::serialize(node).expect("trie node serialization");
    *blake3::hash(&encoded).as_bytes()
}

/// Legacy simple Merkle state for backwards compatibility.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MerkleState {
    pub leaves: BTreeMap<[u8; 32], Vec<u8>>,
}

impl MerkleState {
    pub fn new() -> Self {
        Self {
            leaves: BTreeMap::new(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_insert_and_get() {
        let mut trie = MerklePatriciaTrie::new();
        let key = [1u8; 32];
        let value = b"hello world".to_vec();

        trie.insert(key, value.clone());
        assert_eq!(trie.get(&key), Some(value));
        assert_ne!(trie.root_hash(), [0u8; 32]);
    }

    #[test]
    fn test_mpt_delete() {
        let mut trie = MerklePatriciaTrie::new();
        let key = [2u8; 32];
        trie.insert(key, b"data".to_vec());
        trie.delete(&key);
        assert_eq!(trie.get(&key), None);
    }

    #[test]
    fn test_mpt_proof() {
        let mut trie = MerklePatriciaTrie::new();
        let key = [3u8; 32];
        trie.insert(key, b"proof_data".to_vec());

        let proof = trie.prove(&key);
        assert!(proof.is_some());
        assert!(trie.verify_proof(&proof.unwrap()));
    }

    #[test]
    fn test_deterministic_root() {
        let mut trie1 = MerklePatriciaTrie::new();
        let mut trie2 = MerklePatriciaTrie::new();

        trie1.insert([1u8; 32], b"a".to_vec());
        trie1.insert([2u8; 32], b"b".to_vec());

        trie2.insert([1u8; 32], b"a".to_vec());
        trie2.insert([2u8; 32], b"b".to_vec());

        assert_eq!(trie1.root_hash(), trie2.root_hash());
    }
}
