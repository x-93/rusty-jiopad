//! Block data structures.

use crate::{header::Header, hashing, Hash, errors::ConsensusResult};

/// Block template for mining.
#[derive(Debug, Clone, Default)]
pub struct BlockTemplate {
    pub header: Header,
    pub transactions: Vec<Hash>,
}

/// Template build mode.
#[derive(Debug, Clone, Copy)]
pub enum TemplateBuildMode {
    Standard,
}

/// Template transaction selector.
pub trait TemplateTransactionSelector {
    fn select_transactions(&self) -> Vec<Hash>;
}

/// Virtual state approximation ID.
#[derive(Debug, Clone, Default)]
pub struct VirtualStateApproxId(pub u64);

/// Block structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Hash>, // Placeholder for actual transaction hashes; will be replaced with Tx type
    pub ghostdag_data: Option<crate::ghostdag::GhostDagData>,
}

impl Block {
    /// Creates a new block with the given header and transactions.
    pub fn new(header: Header, transactions: Vec<Hash>) -> Self {
        Self { header, transactions, ghostdag_data: None }
    }

    /// Validates the block.
    pub fn validate(&self) -> ConsensusResult<()> {
        // Basic validation: check merkle root
        let merkle_root = hashing::hash_merkle_root(&self.transactions);
        if self.header.merkle_root != merkle_root {
            return Err(crate::errors::ConsensusError::MerkleRootMismatch);
        }

        // Additional validations can be added here (e.g., transaction count, mass, etc.)
        Ok(())
    }

    /// Computes the block hash (which is the header hash).
    pub fn hash(&self) -> Hash {
        self.header.hash()
    }

    /// Checks if the block is a genesis block.
    pub fn is_genesis(&self) -> bool {
        self.header.parents_by_level.iter().all(|level| level.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hash;

    #[test]
    fn test_block_new() {
        let header = Header::new();
        let txs = vec![Hash::default()];
        let block = Block::new(header, txs);
        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_block_validate_merkle_mismatch() {
        let mut header = Header::new();
        header.merkle_root = Hash::from_slice(b"wrong");
        let block = Block::new(header, vec![]);
        assert!(block.validate().is_err());
    }

    #[test]
    fn test_block_hash() {
        let header = Header::new();
        let block = Block::new(header, vec![]);
        let hash = block.hash();
        assert!(!hash.as_bytes().is_empty());
    }

    #[test]
    fn test_block_is_genesis() {
        let header = Header::new();
        let block = Block::new(header, vec![]);
        assert!(block.is_genesis());
    }
}
