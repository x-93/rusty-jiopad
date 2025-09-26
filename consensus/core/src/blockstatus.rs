//! Block status definitions.

/// Status of a block in the consensus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStatus {
    /// Block is invalid.
    Invalid,
    /// Block is valid but not accepted.
    Valid,
    /// Block is accepted into the chain.
    Accepted,
    /// Block is part of the main chain.
    MainChain,
}

impl BlockStatus {
    /// Checks if the block is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, BlockStatus::Valid | BlockStatus::Accepted | BlockStatus::MainChain)
    }

    /// Checks if the block is accepted.
    pub fn is_accepted(&self) -> bool {
        matches!(self, BlockStatus::Accepted | BlockStatus::MainChain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_status_is_valid() {
        assert!(!BlockStatus::Invalid.is_valid());
        assert!(BlockStatus::Valid.is_valid());
        assert!(BlockStatus::Accepted.is_valid());
        assert!(BlockStatus::MainChain.is_valid());
    }

    #[test]
    fn test_block_status_is_accepted() {
        assert!(!BlockStatus::Invalid.is_accepted());
        assert!(!BlockStatus::Valid.is_accepted());
        assert!(BlockStatus::Accepted.is_accepted());
        assert!(BlockStatus::MainChain.is_accepted());
    }
}
