//! Acceptance data for block validation.

use crate::{errors::ConsensusResult, Hash};

/// Acceptance data structure for block acceptance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceData {
    pub accepted_tx_ids: Vec<Hash>,
    pub accepted_block_hashes: Vec<Hash>,
}

impl AcceptanceData {
    /// Creates new acceptance data.
    pub fn new(accepted_tx_ids: Vec<Hash>, accepted_block_hashes: Vec<Hash>) -> Self {
        Self {
            accepted_tx_ids,
            accepted_block_hashes,
        }
    }

    /// Validates the acceptance data.
    pub fn validate(&self) -> ConsensusResult<()> {
        if self.accepted_tx_ids.is_empty() {
            return Err(crate::errors::ConsensusError::Generic {
                msg: "No accepted transactions".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptance_data_new() {
        let data = AcceptanceData::new(vec![Hash::default()], vec![Hash::default()]);
        assert_eq!(data.accepted_tx_ids.len(), 1);
    }

    #[test]
    fn test_acceptance_data_validate() {
        let data = AcceptanceData::new(vec![Hash::default()], vec![Hash::default()]);
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_acceptance_data_validate_invalid() {
        let data = AcceptanceData::new(vec![], vec![Hash::default()]);
        assert!(data.validate().is_err());
    }
}
