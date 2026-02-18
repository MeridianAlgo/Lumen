use blake3;
use lumina_types::state::{AccountState, GlobalState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MerklePatriciaTrie {
    entries: BTreeMap<[u8; 32], Vec<u8>>,
    root: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum TrieNode {
    Leaf {
        path: Vec<u8>,
        value: Vec<u8>,
    },
    Extension {
        path: Vec<u8>,
        child: [u8; 32],
    },
    Branch {
        children: [Option<[u8; 32]>; 16],
        value: Option<Vec<u8>>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProofNode {
    Leaf {
        path: Vec<u8>,
        value: Vec<u8>,
    },
    Extension {
        path: Vec<u8>,
        child: [u8; 32],
    },
    Branch {
        children: [Option<[u8; 32]>; 16],
        value: Option<Vec<u8>>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleProof {
    pub key: [u8; 32],
    pub value: Vec<u8>,
    pub nodes: Vec<ProofNode>,
}

impl MerklePatriciaTrie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_global_state(state: &GlobalState) -> Self {
        let mut trie = Self::new();
        for (account, account_state) in &state.accounts {
            trie.insert_account(*account, account_state);
        }
        trie
    }

    pub fn insert_account(&mut self, key: [u8; 32], account: &AccountState) {
        let value = bincode::serialize(account).expect("account serialization");
        self.insert(key, value);
    }

    pub fn insert(&mut self, key: [u8; 32], value: Vec<u8>) {
        self.entries.insert(key, value);
        self.recompute_root();
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>> {
        self.entries.get(key).cloned()
    }

    pub fn delete(&mut self, key: &[u8; 32]) {
        self.entries.remove(key);
        self.recompute_root();
    }

    pub fn root_hash(&self) -> [u8; 32] {
        self.root
    }

    pub fn prove(&self, key: &[u8; 32]) -> Option<MerkleProof> {
        let value = self.entries.get(key)?.clone();
        let nibble_key = bytes_to_nibbles(key);
        let trie = self.build_trie()?;
        let mut nodes = Vec::new();
        collect_proof_nodes(&trie, &nibble_key, &mut nodes)?;

        Some(MerkleProof {
            key: *key,
            value,
            nodes,
        })
    }

    pub fn verify_proof(root: [u8; 32], proof: &MerkleProof) -> bool {
        if proof.nodes.is_empty() {
            return false;
        }

        let mut remaining = bytes_to_nibbles(&proof.key);
        let mut expected_hash = hash_proof_node(&proof.nodes[0]);
        if expected_hash != root {
            return false;
        }

        for idx in 0..proof.nodes.len() {
            match &proof.nodes[idx] {
                ProofNode::Leaf { path, value } => {
                    return path == &remaining && value == &proof.value;
                }
                ProofNode::Extension { path, child } => {
                    if !remaining.starts_with(path) {
                        return false;
                    }
                    remaining.drain(0..path.len());
                    if idx + 1 >= proof.nodes.len() {
                        return false;
                    }
                    let child_hash = hash_proof_node(&proof.nodes[idx + 1]);
                    if &child_hash != child {
                        return false;
                    }
                }
                ProofNode::Branch { children, value } => {
                    if remaining.is_empty() {
                        return value.as_ref() == Some(&proof.value);
                    }

                    let nib = remaining[0] as usize;
                    remaining.remove(0);
                    let Some(child_hash) = children[nib] else {
                        return false;
                    };
                    if idx + 1 >= proof.nodes.len() {
                        return false;
                    }
                    let next_hash = hash_proof_node(&proof.nodes[idx + 1]);
                    if next_hash != child_hash {
                        return false;
                    }
                }
            }

            expected_hash = hash_proof_node(&proof.nodes[idx]);
            if idx == 0 && expected_hash != root {
                return false;
            }
        }

        false
    }

    fn recompute_root(&mut self) {
        self.root = self.build_trie().map(|node| node.hash).unwrap_or([0u8; 32]);
    }

    fn build_trie(&self) -> Option<HashedNode> {
        let data: Vec<(Vec<u8>, Vec<u8>)> = self
            .entries
            .iter()
            .map(|(k, v)| (bytes_to_nibbles(k), v.clone()))
            .collect();
        build_hashed_node(data, Vec::new())
    }
}

pub fn state_root_from_global_state(state: &GlobalState) -> [u8; 32] {
    MerklePatriciaTrie::from_global_state(state).root_hash()
}

#[derive(Clone)]
struct HashedNode {
    hash: [u8; 32],
    node: TrieNode,
    children: Vec<HashedNode>,
}

fn build_hashed_node(entries: Vec<(Vec<u8>, Vec<u8>)>, prefix: Vec<u8>) -> Option<HashedNode> {
    if entries.is_empty() {
        return None;
    }

    if entries.len() == 1 {
        let (key, value) = &entries[0];
        let path = key[prefix.len()..].to_vec();
        let node = TrieNode::Leaf {
            path,
            value: value.clone(),
        };
        return Some(HashedNode {
            hash: hash_trie_node(&node),
            node,
            children: Vec::new(),
        });
    }

    let has_exact = entries.iter().any(|(k, _)| k.len() == prefix.len());
    let common_extension = if has_exact {
        Vec::new()
    } else {
        longest_common_extension(&entries, prefix.len())
    };

    if !common_extension.is_empty() {
        let child_prefix = [prefix.clone(), common_extension.clone()].concat();
        let child = build_hashed_node(entries, child_prefix)?;
        let node = TrieNode::Extension {
            path: common_extension,
            child: child.hash,
        };
        return Some(HashedNode {
            hash: hash_trie_node(&node),
            node,
            children: vec![child],
        });
    }

    let mut children_hashes: [Option<[u8; 32]>; 16] = [None; 16];
    let mut children_nodes = Vec::new();
    let mut value_at_node = None;

    for (k, v) in &entries {
        if k.len() == prefix.len() {
            value_at_node = Some(v.clone());
            break;
        }
    }

    for nib in 0u8..=15 {
        let subset: Vec<(Vec<u8>, Vec<u8>)> = entries
            .iter()
            .filter(|(k, _)| k.len() > prefix.len() && k[prefix.len()] == nib)
            .cloned()
            .collect();

        if let Some(child) = build_hashed_node(subset, [prefix.clone(), vec![nib]].concat()) {
            children_hashes[nib as usize] = Some(child.hash);
            children_nodes.push(child);
        }
    }

    let node = TrieNode::Branch {
        children: children_hashes,
        value: value_at_node,
    };

    Some(HashedNode {
        hash: hash_trie_node(&node),
        node,
        children: children_nodes,
    })
}

fn longest_common_extension(entries: &[(Vec<u8>, Vec<u8>)], start: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut idx = start;

    loop {
        let Some(first) = entries[0].0.get(idx) else {
            break;
        };
        if entries.iter().all(|(k, _)| k.get(idx) == Some(first)) {
            out.push(*first);
            idx += 1;
        } else {
            break;
        }
    }

    out
}

fn collect_proof_nodes(node: &HashedNode, key: &[u8], out: &mut Vec<ProofNode>) -> Option<()> {
    match &node.node {
        TrieNode::Leaf { path, value } => {
            if path == key {
                out.push(ProofNode::Leaf {
                    path: path.clone(),
                    value: value.clone(),
                });
                Some(())
            } else {
                None
            }
        }
        TrieNode::Extension { path, child } => {
            if !key.starts_with(path) {
                return None;
            }
            out.push(ProofNode::Extension {
                path: path.clone(),
                child: *child,
            });
            let next = &key[path.len()..];
            collect_proof_nodes(&node.children[0], next, out)
        }
        TrieNode::Branch { children, value } => {
            out.push(ProofNode::Branch {
                children: *children,
                value: value.clone(),
            });
            if key.is_empty() {
                return value.as_ref().map(|_| ());
            }
            let nib = key[0] as usize;
            let _ = children[nib]?;
            for child in &node.children {
                if Some(child.hash) == children[nib] {
                    return collect_proof_nodes(child, &key[1..], out);
                }
            }
            None
        }
    }
}

fn bytes_to_nibbles(bytes: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    for b in bytes {
        out.push((b >> 4) & 0x0F);
        out.push(b & 0x0F);
    }
    out
}

fn hash_trie_node(node: &TrieNode) -> [u8; 32] {
    let encoded = bincode::serialize(node).expect("trie node serialization");
    *blake3::hash(&encoded).as_bytes()
}

fn hash_proof_node(node: &ProofNode) -> [u8; 32] {
    let encoded = bincode::serialize(node).expect("proof node serialization");
    *blake3::hash(&encoded).as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_insert_update_get_delete() {
        let mut trie = MerklePatriciaTrie::new();
        let key = [1u8; 32];

        trie.insert(key, b"hello".to_vec());
        assert_eq!(trie.get(&key), Some(b"hello".to_vec()));

        let before = trie.root_hash();
        trie.insert(key, b"world".to_vec());
        assert_eq!(trie.get(&key), Some(b"world".to_vec()));
        assert_ne!(before, trie.root_hash());

        trie.delete(&key);
        assert_eq!(trie.get(&key), None);
    }

    #[test]
    fn test_deterministic_root() {
        let mut trie1 = MerklePatriciaTrie::new();
        trie1.insert([1u8; 32], b"a".to_vec());
        trie1.insert([2u8; 32], b"b".to_vec());

        let mut trie2 = MerklePatriciaTrie::new();
        trie2.insert([2u8; 32], b"b".to_vec());
        trie2.insert([1u8; 32], b"a".to_vec());

        assert_eq!(trie1.root_hash(), trie2.root_hash());
    }

    #[test]
    fn test_global_state_root_matches_trie() {
        let mut state = GlobalState::default();
        let mut account = AccountState::default();
        account.lusd_balance = 42;
        state.accounts.insert([9u8; 32], account);

        let trie = MerklePatriciaTrie::from_global_state(&state);
        assert_eq!(state.root_hash(), trie.root_hash());
        assert_eq!(state_root_from_global_state(&state), state.root_hash());
    }

    #[test]
    fn test_proof_generation_and_verification() {
        let mut trie = MerklePatriciaTrie::new();
        let key = [3u8; 32];
        trie.insert(key, b"proof_data".to_vec());

        let proof = trie.prove(&key).expect("proof exists");
        assert!(MerklePatriciaTrie::verify_proof(trie.root_hash(), &proof));
    }
}
