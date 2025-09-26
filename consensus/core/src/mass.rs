//! Block mass calculation utilities.

use crate::errors::ConsensusResult;

/// Contextual masses for transactions.
#[derive(Debug, Clone, Default)]
pub struct ContextualMasses(pub u64);

/// Non-contextual masses for transactions.
#[derive(Debug, Clone, Default)]
pub struct NonContextualMasses(pub u64);

/// Block mass type.
pub type BlockMass = u64;

/// Calculates the mass of a block based on its transactions.
pub fn calculate_block_mass(transactions: &[crate::tx::Transaction]) -> BlockMass {
    let mut mass = 0;
    for tx in transactions {
        mass += tx.mass();
    }
    mass
}

/// Validates block mass against the maximum allowed.
pub fn validate_block_mass(mass: BlockMass) -> ConsensusResult<()> {
    if mass > crate::constants::MAX_BLOCK_MASS {
        return Err(crate::errors::ConsensusError::MiningRuleViolation {
            msg: format!("Block mass {} exceeds maximum {}", mass, crate::constants::MAX_BLOCK_MASS),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_block_mass() {
        let tx = crate::tx::Transaction::new(1, vec![], vec![], 0);
        let mass = calculate_block_mass(&[tx]);
        assert_eq!(mass, 100);
    }

    #[test]
    fn test_validate_block_mass_valid() {
        assert!(validate_block_mass(crate::constants::MAX_BLOCK_MASS).is_ok());
    }

    #[test]
    fn test_validate_block_mass_invalid() {
        assert!(validate_block_mass(crate::constants::MAX_BLOCK_MASS + 1).is_err());
    }
}
