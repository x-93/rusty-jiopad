//! Pruning utilities for consensus data.

use crate::Hash;
use std::collections::HashSet;

/// Pruning manager for managing pruned data.
#[derive(Debug)]
pub struct PruningManager {
    pub pruning_point: Hash,
    pub pruned_blocks: HashSet<Hash>,
}

impl PruningManager {
    /// Creates a new pruning manager.
    pub fn new() -> Self {
        Self {
            pruning_point: Hash::default(),
            pruned_blocks: HashSet::new(),
        }
    }

    /// Sets the pruning point.
    pub fn set_pruning_point(&mut self, point: Hash) {
        self.pruning_point = point;
    }

    /// Adds a pruned block.
    pub fn prune_block(&mut self, block_hash: Hash) {
        self.pruned_blocks.insert(block_hash);
    }

    /// Checks if a block is pruned.
    pub fn is_pruned(&self, block_hash: &Hash) -> bool {
        self.pruned_blocks.contains(block_hash)
    }
}

impl Default for PruningManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Pruning point proof.
#[derive(Debug, Clone, Default)]
pub struct PruningPointProof {
    pub data: Vec<u8>,
}

/// Trusted data for pruning point.
#[derive(Debug, Clone, Default)]
pub struct PruningPointTrustedData {
    pub data: Vec<u8>,
}

/// List of pruning points.
#[derive(Debug, Clone, Default)]
pub struct PruningPointsList {
    pub points: Vec<Hash>,
}

/// Metadata for pruning proof.
#[derive(Debug, Clone, Default)]
pub struct PruningProofMetadata {
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pruning_manager() {
        let mut manager = PruningManager::new();
        let hash = Hash::from_le_u64([1, 0, 0, 0]);
        manager.prune_block(hash);
        assert!(manager.is_pruned(&hash));
    }

    #[test]
    fn test_pruning_manager_no_duplicates() {
        let mut manager = PruningManager::new();
        let hash = Hash::from_le_u64([1, 0, 0, 0]);
        manager.prune_block(hash.clone());
        manager.prune_block(hash.clone());
        assert_eq!(manager.pruned_blocks.len(), 1);
    }
}
