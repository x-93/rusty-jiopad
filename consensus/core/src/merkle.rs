//! Merkle tree implementation for consensus.

use crate::{hashing, Hash, errors::ConsensusResult};

/// Merkle tree node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MerkleNode {
    Leaf(Hash),
    Internal(Hash, Box<MerkleNode>, Box<MerkleNode>),
}

/// Merkle tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleTree {
    root: MerkleNode,
}

impl MerkleTree {
    /// Builds a Merkle tree from transaction hashes.
    pub fn from_tx_hashes(tx_hashes: &[Hash]) -> ConsensusResult<Self> {
        if tx_hashes.is_empty() {
            return Ok(Self { root: MerkleNode::Leaf(Hash::default()) });
        }

        let root = Self::build_tree(tx_hashes, 0, tx_hashes.len() - 1)?;
        Ok(Self { root })
    }

    /// Computes the Merkle root hash.
    pub fn root(&self) -> Hash {
        match &self.root {
            MerkleNode::Leaf(h) => *h,
            MerkleNode::Internal(h, _, _) => *h,
        }
    }

    fn build_tree(tx_hashes: &[Hash], start: usize, end: usize) -> ConsensusResult<MerkleNode> {
        if start == end {
            return Ok(MerkleNode::Leaf(tx_hashes[start]));
        }

        let mid = start + (end - start) / 2;
        let left = Self::build_tree(tx_hashes, start, mid)?;
        let right = Self::build_tree(tx_hashes, mid + 1, end)?;

        let left_hash = match &left {
            MerkleNode::Leaf(h) => *h,
            MerkleNode::Internal(h, _, _) => *h,
        };
        let right_hash = match &right {
            MerkleNode::Leaf(h) => *h,
            MerkleNode::Internal(h, _, _) => *h,
        };

        let combined = left_hash.as_bytes().iter().chain(right_hash.as_bytes().iter()).cloned().collect::<Vec<u8>>();
        let node_hash = hashing::double_sha256(&combined);

        Ok(MerkleNode::Internal(node_hash, Box::new(left), Box::new(right)))
    }

    /// Verifies a Merkle proof (placeholder for full proof verification).
    pub fn verify_proof(_tx_hash: Hash, _root: Hash, _proof: &[Hash]) -> bool {
        // Placeholder; implement actual proof verification
        true
    }
}

/// Simple Merkle root calculation (for compatibility with existing code).
pub fn calculate_merkle_root(tx_hashes: &[Hash]) -> Hash {
    if tx_hashes.is_empty() {
        return Hash::default();
    }

    let tree = MerkleTree::from_tx_hashes(tx_hashes).unwrap_or_else(|_| MerkleTree { root: MerkleNode::Leaf(Hash::default()) });
    tree.root()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hash;

    #[test]
    fn test_merkle_tree_single_tx() {
        let tx_hash = Hash::from_slice(b"single_tx");
        let tree = MerkleTree::from_tx_hashes(&[tx_hash]).unwrap();
        assert_eq!(tree.root(), tx_hash);
    }

    #[test]
    fn test_merkle_tree_two_txs() {
        let tx1 = Hash::from_slice(b"tx1");
        let tx2 = Hash::from_slice(b"tx2");
        let tree = MerkleTree::from_tx_hashes(&[tx1, tx2]).unwrap();

        let combined = tx1.as_bytes().iter().chain(tx2.as_bytes().iter()).cloned().collect::<Vec<u8>>();
        let expected_root = hashing::double_sha256(&combined);
        assert_eq!(tree.root(), expected_root);
    }

    #[test]
    fn test_merkle_tree_empty() {
        let tree = MerkleTree::from_tx_hashes(&[]).unwrap();
        assert_eq!(tree.root(), Hash::default());
    }

    #[test]
    fn test_calculate_merkle_root() {
        let tx_hashes = vec![Hash::from_slice(b"tx1")];
        let root = calculate_merkle_root(&tx_hashes);
        assert_eq!(root, tx_hashes[0]);
    }
}
