//! Chain selection and virtual state management.

use std::collections::HashSet;
use std::sync::Arc;
use parking_lot::RwLock;
use rayon::prelude::*;
use crate::{Hash, errors::ConsensusResult, Block, ghostdag::GhostDag};

/// Virtual state of the blockchain.
#[derive(Debug, Clone)]
pub struct VirtualState {
    pub selected_tip: Hash,
    pub blue_score: u64,
    pub daa_score: u64,
    pub merge_set: Vec<Hash>,
}

impl Default for VirtualState {
    fn default() -> Self {
        Self {
            selected_tip: Hash::default(),
            blue_score: 0,
            daa_score: 0,
            merge_set: Vec::new(),
        }
    }
}

/// Chain selector implementing tip selection and virtual state management.
pub struct ChainSelector {
    ghostdag: Arc<GhostDag>,
    virtual_state: RwLock<VirtualState>,
}

impl ChainSelector {
    /// Creates a new chain selector.
    pub fn new(ghostdag: Arc<GhostDag>) -> Self {
        Self {
            ghostdag,
            virtual_state: RwLock::new(VirtualState::default()),
        }
    }

    /// Selects the current tip of the chain based on blue score.
    pub async fn select_tip(&self) -> ConsensusResult<Hash> {
        let tips = self.get_all_tips().await?;

        if tips.is_empty() {
            return Err(crate::errors::ConsensusError::NoTips);
        }

        // Select tip with highest blue score
        let best_tip = tips
            .par_iter()
            .max_by_key(|tip| {
                self.ghostdag.get_blue_score(tip).unwrap_or(0)
            })
            .cloned()
            .unwrap(); // Safe because tips is not empty

        Ok(best_tip)
    }

    /// Gets all current tips (blocks with no children).
    pub async fn get_all_tips(&self) -> ConsensusResult<Vec<Hash>> {
        let mut tips = Vec::new();

        // Find blocks that have no children
        for entry in self.ghostdag.block_relations.iter() {
            let block_hash = *entry.key();
            let relations = entry.value();

            if relations.children.read().is_empty() {
                tips.push(block_hash);
            }
        }

        Ok(tips)
    }

    /// Updates the virtual state when a new block is added.
    pub async fn update_virtual_state(&self, new_block: &Block) -> ConsensusResult<()> {
        let current_blue_score = {
            let state = self.virtual_state.read();
            state.blue_score
        };

        let new_blue_score = new_block.header.blue_score;

        // Update if new block has higher blue score
        if new_blue_score > current_blue_score {
            let mut state = self.virtual_state.write();
            state.selected_tip = new_block.hash();
            state.blue_score = new_blue_score;
            state.daa_score = new_block.header.daa_score;
            state.merge_set = new_block.ghostdag_data.as_ref()
                .map(|data| data.merge_set_blues.clone())
                .unwrap_or_default();
        }

        Ok(())
    }

    /// Gets the current virtual state.
    pub fn get_virtual_state(&self) -> VirtualState {
        self.virtual_state.read().clone()
    }

    /// Handles chain reorganization.
    pub async fn handle_reorg(&self, old_tip: Hash, new_tip: Hash) -> ConsensusResult<()> {
        // Calculate blocks to add and remove during reorg
        let (_added, _removed) = self.calculate_reorg_path(old_tip, new_tip).await?;

        // Update virtual state
        let new_state = self.calculate_virtual_state_for_tip(new_tip).await?;
        *self.virtual_state.write() = new_state;

        Ok(())
    }

    /// Calculates the reorganization path between two tips.
    async fn calculate_reorg_path(&self, old_tip: Hash, new_tip: Hash) -> ConsensusResult<(Vec<Hash>, Vec<Hash>)> {
        let mut added = Vec::new();
        let mut removed = Vec::new();

        // Simple implementation: find common ancestor and calculate paths
        // In a real implementation, this would use more sophisticated algorithms
        let common_ancestor = self.find_common_ancestor(old_tip, new_tip).await?;

        // Blocks to remove: from old_tip back to common ancestor
        let mut current = old_tip;
        while current != common_ancestor {
            removed.push(current);
            // Find parent (simplified - in real impl, use selected_parent from GhostDAG)
            if let Some(relations) = self.ghostdag.get_relations(&current) {
                if let Some(parent) = relations.selected_parent {
                    current = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Blocks to add: from new_tip back to common ancestor
        current = new_tip;
        while current != common_ancestor {
            added.push(current);
            if let Some(relations) = self.ghostdag.get_relations(&current) {
                if let Some(parent) = relations.selected_parent {
                    current = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Reverse added to get correct order
        added.reverse();

        Ok((added, removed))
    }

    /// Finds the common ancestor of two blocks.
    async fn find_common_ancestor(&self, block1: Hash, block2: Hash) -> ConsensusResult<Hash> {
        let mut ancestors1 = HashSet::new();
        let mut current = block1;

        // Collect ancestors of block1
        loop {
            ancestors1.insert(current);
            if let Some(relations) = self.ghostdag.get_relations(&current) {
                if let Some(parent) = relations.selected_parent {
                    current = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Find first common ancestor with block2
        current = block2;
        loop {
            if ancestors1.contains(&current) {
                return Ok(current);
            }
            if let Some(relations) = self.ghostdag.get_relations(&current) {
                if let Some(parent) = relations.selected_parent {
                    current = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Err(crate::errors::ConsensusError::NoCommonAncestor)
    }

    /// Calculates virtual state for a given tip.
    async fn calculate_virtual_state_for_tip(&self, tip: Hash) -> ConsensusResult<VirtualState> {
        let blue_score = self.ghostdag.get_blue_score(&tip).unwrap_or(0);

        // Simplified DAA score calculation
        let daa_score = if let Some(relations) = self.ghostdag.get_relations(&tip) {
            relations.blue_score // Placeholder
        } else {
            0
        };

        let merge_set = if let Some(relations) = self.ghostdag.get_relations(&tip) {
            relations.merge_set_blues.clone()
        } else {
            Vec::new()
        };

        Ok(VirtualState {
            selected_tip: tip,
            blue_score,
            daa_score,
            merge_set,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ghostdag::GhostDag;

    #[tokio::test]
    async fn test_chain_selector_new() {
        let ghostdag = Arc::new(GhostDag::new(10));
        let selector = ChainSelector::new(ghostdag);
        let state = selector.get_virtual_state();
        assert_eq!(state.blue_score, 0);
    }

    #[tokio::test]
    async fn test_select_tip_no_blocks() {
        let ghostdag = Arc::new(GhostDag::new(10));
        let selector = ChainSelector::new(ghostdag);
        let result = selector.select_tip().await;
        assert!(result.is_err());
    }
}
