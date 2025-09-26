//! GhostDAG consensus implementation using PHANTOM algorithm.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use crate::{Hash, KType, BlueWorkType, errors::ConsensusResult, Block};

/// GhostDAG data for a block.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GhostDagData {
    pub blue_score: u64,
    pub blue_work: BlueWorkType,
    pub selected_parent: Hash,
    pub merge_set_blues: Vec<Hash>,
    pub merge_set_reds: Vec<Hash>,
    pub blues_anticone_sizes: HashMap<Hash, u64>,
}

impl Default for GhostDagData {
    fn default() -> Self {
        Self {
            blue_score: 0,
            blue_work: BlueWorkType::from_u64(0),
            selected_parent: Hash::default(),
            merge_set_blues: Vec::new(),
            merge_set_reds: Vec::new(),
            blues_anticone_sizes: HashMap::new(),
        }
    }
}

/// Block relations in the DAG.
#[derive(Debug, Clone)]
pub struct BlockRelations {
    pub parents: Vec<Hash>,
    pub children: Arc<RwLock<Vec<Hash>>>,
    pub is_blue: bool,
    pub blue_score: u64,
    pub selected_parent: Option<Hash>,
    pub merge_set_blues: Vec<Hash>,
    pub merge_set_reds: Vec<Hash>,
}

/// GhostDAG manager implementing PHANTOM algorithm.
pub struct GhostDag {
    k: KType,
    pub block_relations: DashMap<Hash, BlockRelations>,
    blue_scores: DashMap<Hash, u64>,
}

impl GhostDag {
    /// Creates a new GhostDAG with the given k parameter.
    pub fn new(k: KType) -> Self {
        Self {
            k,
            block_relations: DashMap::new(),
            blue_scores: DashMap::new(),
        }
    }

    /// Adds a block to the DAG and calculates its GhostDAG data.
    pub async fn add_block(&self, block: &Block) -> ConsensusResult<GhostDagData> {
        // Collect all parents across levels
        let all_parents: Vec<Hash> = block.header.parents_by_level
            .iter()
            .flatten()
            .cloned()
            .collect();

        // Calculate blue and red sets using PHANTOM algorithm
        let (blue_set, red_set) = self.calculate_blue_set(block, &all_parents).await?;

        // Select parent with highest blue score
        let selected_parent = self.select_parent(&all_parents).await?;

        // Calculate blue work
        let blue_work = self.calculate_blue_work_proper(&blue_set).await?;

        // Calculate blue score
        let blue_score = blue_set.len() as u64;

        // Store block relations
        let relations = BlockRelations {
            parents: all_parents.clone(),
            children: Arc::new(RwLock::new(Vec::new())),
            is_blue: blue_set.contains(&block.hash()),
            blue_score,
            selected_parent: Some(selected_parent),
            merge_set_blues: blue_set.clone(),
            merge_set_reds: red_set.clone(),
        };

        self.block_relations.insert(block.hash(), relations);
        self.blue_scores.insert(block.hash(), blue_score);

        // Update children for parent blocks
        for parent in &all_parents {
            if let Some(parent_relations) = self.block_relations.get_mut(parent) {
                parent_relations.children.write().push(block.hash());
            }
        }

        // Calculate anticone sizes for blue blocks
        let parents_set = HashSet::from_iter(all_parents.iter().cloned());
        let blues_anticone_sizes = self.calculate_blues_anticone_sizes(&blue_set, &parents_set).await?;

        Ok(GhostDagData {
            blue_score,
            blue_work,
            selected_parent,
            merge_set_blues: blue_set,
            merge_set_reds: red_set,
            blues_anticone_sizes,
        })
    }

    /// Calculates blue and red sets using PHANTOM algorithm.
    async fn calculate_blue_set(&self, _block: &Block, parents: &[Hash]) -> ConsensusResult<(Vec<Hash>, Vec<Hash>)> {
        let mut blue_set = Vec::new();
        let mut red_set = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Start with parents
        for parent in parents {
            queue.push_back(*parent);
        }

        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            // Calculate anticone size with optimization
            let anticone_size = self.calculate_anticone_size_optimized(&current, &HashSet::new()).await?;

            if anticone_size <= self.k as u64 {
                blue_set.push(current);
            } else {
                red_set.push(current);
            }

            // Add ancestors to queue
            if let Some(relations) = self.block_relations.get(&current) {
                for parent in &relations.parents {
                    queue.push_back(*parent);
                }
            }
        }

        Ok((blue_set, red_set))
    }

    /// Selects the parent with the highest blue score.
    async fn select_parent(&self, parents: &[Hash]) -> ConsensusResult<Hash> {
        if parents.is_empty() {
            // Genesis block has no parents, return default hash
            return Ok(Hash::default());
        }

        let selected = parents
            .par_iter()
            .max_by_key(|parent| {
                self.blue_scores.get(parent).map(|s| *s).unwrap_or(0)
            })
            .cloned()
            .ok_or(crate::errors::ConsensusError::NoValidParent)?;

        Ok(selected)
    }

    /// Calculates the accumulated blue work for a set of blocks.
    async fn calculate_blue_work_proper(&self, blue_set: &[Hash]) -> ConsensusResult<BlueWorkType> {
        let mut total_work: u128 = 0;

        for &block_hash in blue_set {
            // Accumulate actual work (placeholder - implement proper work calculation)
            let _block_work = self.get_block_work(&block_hash).await?;
            // For now, convert to u128 for accumulation (simplified)
            // In real implementation, proper big integer addition needed
            total_work += 1; // Placeholder
        }

        Ok(BlueWorkType::from_u64(total_work as u64))
    }

    /// Gets the work contributed by a block.
    async fn get_block_work(&self, _block_hash: &Hash) -> ConsensusResult<BlueWorkType> {
        // Placeholder: implement based on difficulty target
        // Work = 2^256 / (target + 1) for Bitcoin-style
        Ok(BlueWorkType::from_u64(1))
    }

    /// Calculates anticone size for a block with optimization.
    async fn calculate_anticone_size_optimized(
        &self,
        block_hash: &Hash,
        visited: &HashSet<Hash>
    ) -> ConsensusResult<u64> {
        let mut size = 0u64;
        let mut to_visit = VecDeque::new();
        let mut visited_local = HashSet::new();

        // Start from block's future (descendants)
        to_visit.push_back(*block_hash);

        while let Some(current) = to_visit.pop_front() {
            if visited_local.contains(&current) || visited.contains(&current) {
                continue;
            }
            visited_local.insert(current);

            if current != *block_hash {
                size += 1;
            }
            // Add children to visit
            if let Some(relations) = self.block_relations.get(&current) {
                for child in relations.children.read().iter() {
                    to_visit.push_back(*child);
                }
            }
        }

        Ok(size)
    }

    /// Checks if a candidate block is in the past cone of a reference block.
    async fn is_in_past_cone(&self, candidate: &Hash, reference: &Hash) -> ConsensusResult<bool> {
        let mut current = *candidate;
        while current != *reference {
            if let Some(relations) = self.block_relations.get(&current) {
                if let Some(parent) = relations.selected_parent {
                    current = parent;
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Calculates anticone sizes for blue blocks.
    async fn calculate_blues_anticone_sizes(&self, blue_set: &[Hash], parents: &HashSet<Hash>) -> ConsensusResult<HashMap<Hash, u64>> {
        let mut sizes = HashMap::new();

        // Parallel calculation for performance
        let results: Vec<_> = blue_set.par_iter()
            .map(|blue_block| {
                let size = self.calculate_anticone_size_optimized(blue_block, parents);
                (blue_block, size)
            })
            .collect();

        for (blue_block, size_result) in results {
            let size = size_result.await?;
            sizes.insert(*blue_block, size);
        }

        Ok(sizes)
    }

    /// Gets the blue score for a block.
    pub fn get_blue_score(&self, block_hash: &Hash) -> Option<u64> {
        self.blue_scores.get(block_hash).map(|s| *s)
    }

    /// Gets block relations.
    pub fn get_relations(&self, block_hash: &Hash) -> Option<BlockRelations> {
        self.block_relations.get(block_hash).map(|r| r.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::Header;

    fn create_test_block(parents: Vec<Hash>) -> Block {
        let mut header = Header::new();
        header.parents_by_level = vec![parents];
        Block::new(header, vec![])
    }

    #[tokio::test]
    async fn test_ghostdag_add_block() {
        let ghostdag = GhostDag::new(10);
        let block = create_test_block(vec![]);

        let result = ghostdag.add_block(&block).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.blue_score, 0); // Genesis has no parents
    }

    #[tokio::test]
    async fn test_calculate_anticone_size() {
        let ghostdag = GhostDag::new(10);
        let block = create_test_block(vec![]);

        // Add genesis block
        ghostdag.add_block(&block).await.unwrap();

        let visited = HashSet::new();
        let size = ghostdag.calculate_anticone_size_optimized(&block.hash(), &visited).await.unwrap();
        assert_eq!(size, 0); // No other blocks
    }

    #[tokio::test]
    async fn test_complex_dag_scenario() {
        let ghostdag = GhostDag::new(3);

        // Create genesis
        let genesis = create_test_block(vec![]);
        ghostdag.add_block(&genesis).await.unwrap();

        // Add multiple children
        let child1 = create_test_block(vec![genesis.hash()]);
        let child2 = create_test_block(vec![genesis.hash()]);
        ghostdag.add_block(&child1).await.unwrap();
        ghostdag.add_block(&child2).await.unwrap();

        // Add merge block
        let merge = create_test_block(vec![child1.hash(), child2.hash()]);
        let data = ghostdag.add_block(&merge).await.unwrap();

        // Verify blue set contains expected blocks
        assert!(data.merge_set_blues.contains(&child1.hash()));
        assert!(data.merge_set_blues.contains(&child2.hash()));
        assert!(data.merge_set_reds.is_empty()); // Should be blue with k=3
        assert_eq!(data.blue_score, 2); // child1 + child2
    }

    #[tokio::test]
    async fn test_multi_level_parents() {
        let ghostdag = GhostDag::new(10);

        // Create genesis
        let genesis = create_test_block(vec![]);
        ghostdag.add_block(&genesis).await.unwrap();

        // Create block with multi-level parents (simulate)
        let mut header = Header::new();
        header.parents_by_level = vec![
            vec![genesis.hash()], // Level 0
            vec![], // Level 1 (empty for test)
        ];
        let block = Block::new(header, vec![]);

        let result = ghostdag.add_block(&block).await;
        assert!(result.is_ok());
    }
}
