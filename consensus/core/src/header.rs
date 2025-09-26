//! Block header data structures.

use crate::{hashing, Hash, BlueWorkType};

/// Block header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub version: u16,
    pub parents_by_level: Vec<Vec<Hash>>,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
    pub daa_score: u64,
    pub blue_score: u64,
    pub blue_work: BlueWorkType,
    pub pruning_point: Hash,
    /// Cached hash to avoid recomputation.
    cached_hash: Option<Hash>,
}

impl Header {
    /// Creates a new header with default values.
    pub fn new() -> Self {
        Self {
            version: 1,
            parents_by_level: vec![vec![]], // Genesis has no parents
            merkle_root: Hash::default(),
            timestamp: 0,
            bits: 0,
            nonce: 0,
            daa_score: 0,
            blue_score: 0,
            blue_work: BlueWorkType::from_u64(0),
            pruning_point: Hash::default(),
            cached_hash: None,
        }
    }

    /// Computes the hash of the header.
    pub fn hash(&self) -> Hash {
        self.hash_with_nonce(self.nonce)
    }

    /// Computes the hash of the header with a specific nonce (for mining optimization).
    pub fn hash_with_nonce(&self, nonce: u64) -> Hash {
        // Serialize header fields except nonce, then append nonce
        let mut data = Vec::new();
        data.extend_from_slice(&self.version.to_le_bytes());
        // Serialize parents_by_level
        data.extend_from_slice(&(self.parents_by_level.len() as u32).to_le_bytes());
        for level in &self.parents_by_level {
            data.extend_from_slice(&(level.len() as u32).to_le_bytes());
            for parent in level {
                data.extend_from_slice(parent.as_bytes());
            }
        }
        data.extend_from_slice(self.merkle_root.as_bytes());
        data.extend_from_slice(&self.timestamp.to_le_bytes());
        data.extend_from_slice(&self.bits.to_le_bytes());
        data.extend_from_slice(&nonce.to_le_bytes());
        data.extend_from_slice(&self.daa_score.to_le_bytes());
        data.extend_from_slice(&self.blue_score.to_le_bytes());
        // BlueWorkType serialization placeholder
        data.extend_from_slice(&self.blue_work.to_le_bytes());
        data.extend_from_slice(self.pruning_point.as_bytes());

        hashing::hash_block_header(&data)
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_new() {
        let header = Header::new();
        assert_eq!(header.version, 1);
        assert_eq!(header.timestamp, 0);
    }

    #[test]
    fn test_header_hash() {
        let header = Header::new();
        let hash = header.hash();
        assert!(!hash.as_bytes().is_empty());
    }
}
