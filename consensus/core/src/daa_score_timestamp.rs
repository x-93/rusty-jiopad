//! DAA score and timestamp utilities.

use crate::errors::ConsensusResult;

/// DAA score and timestamp data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaaScoreTimestamp {
    pub daa_score: u64,
    pub timestamp: u64,
}

impl DaaScoreTimestamp {
    /// Creates new DAA data.
    pub fn new(daa_score: u64, timestamp: u64) -> Self {
        Self { daa_score, timestamp }
    }

    /// Validates the DAA data.
    pub fn validate(&self) -> ConsensusResult<()> {
        if self.timestamp == 0 {
            return Err(crate::errors::ConsensusError::Generic {
                msg: "Timestamp cannot be zero".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daa_score_timestamp_new() {
        let daa = DaaScoreTimestamp::new(100, 1234567890);
        assert_eq!(daa.daa_score, 100);
        assert_eq!(daa.timestamp, 1234567890);
    }

    #[test]
    fn test_daa_score_timestamp_validate() {
        let daa = DaaScoreTimestamp::new(100, 1234567890);
        assert!(daa.validate().is_ok());
    }

    #[test]
    fn test_daa_score_timestamp_validate_invalid() {
        let daa = DaaScoreTimestamp::new(100, 0);
        assert!(daa.validate().is_err());
    }
}
