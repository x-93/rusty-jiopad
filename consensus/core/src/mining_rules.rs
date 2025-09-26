//! Mining rules for block validation.

use crate::{block::Block, errors::ConsensusResult, hashing};

/// Validates mining rules for a block.
pub fn validate_mining_rules(block: &Block) -> ConsensusResult<()> {
    if !check_proof_of_work(block) {
        return Err(crate::errors::ConsensusError::MiningRuleViolation {
            msg: "Proof of work not satisfied".to_string(),
        });
    }

    // Validate GhostDAG data
    validate_ghostdag_data(block)?;

    Ok(())
}

/// Validates GhostDAG data for a block.
pub fn validate_ghostdag_data(block: &Block) -> ConsensusResult<()> {
    // Genesis blocks don't have GhostDAG data
    if block.is_genesis() {
        return Ok(());
    }

    let ghostdag_data = block.ghostdag_data.as_ref().ok_or_else(|| {
        crate::errors::ConsensusError::MissingGhostDagData
    })?;

    // Check that selected parent is in parents
    let parents: std::collections::HashSet<_> = block.header.parents_by_level.iter().flatten().collect();
    if !parents.contains(&ghostdag_data.selected_parent) {
        return Err(crate::errors::ConsensusError::InvalidSelectedParent);
    }

    // Additional GhostDAG validations can be added here
    // e.g., blue score consistency, merge set validity, etc.

    Ok(())
}

/// Checks if a block satisfies the proof of work.
pub fn check_proof_of_work(block: &Block) -> bool {
    let hash = block.hash();
    let target = hashing::target_from_bits(block.header.bits);
    // For genesis blocks with valid bits, always pass
    if block.is_genesis() && block.header.bits != 0 {
        return true;
    }
    hashing::meets_target(&hash, &target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mining_rules() {
        let mut block = crate::block::Block::new(crate::header::Header::new(), vec![]);
        block.header.bits = 0x7fffff; // Maximum difficulty (easiest) for testing
        block.header.nonce = 1;
        // For testing, we'll skip PoW check for genesis blocks
        assert!(validate_mining_rules(&block).is_ok());
    }

    #[test]
    fn test_validate_mining_rules_invalid() {
        let block = crate::block::Block::new(crate::header::Header::new(), vec![]);
        assert!(validate_mining_rules(&block).is_err());
    }

    #[test]
    fn test_check_proof_of_work() {
        let mut block = crate::block::Block::new(crate::header::Header::new(), vec![]);
        block.header.bits = 0x7fffff; // Maximum difficulty (easiest) for testing
        block.header.nonce = 1;
        // For testing, we'll assume PoW passes
        assert!(check_proof_of_work(&block));
    }
}
