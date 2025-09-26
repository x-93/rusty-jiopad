use crate::{network::NetworkId, BlueWorkType};

/// Consensus parameters defining the network rules and constants.
#[derive(Clone, Debug, PartialEq)]
pub struct Params {
    /// The network identifier
    pub network_id: NetworkId,
    /// Target time per block in milliseconds
    pub target_time_per_block: u64,
    /// Maximum block mass
    pub max_block_mass: u64,
    /// Maximum transaction mass
    pub max_tx_mass: u64,
    /// Halving interval for block rewards
    pub halving_interval: u64,
    /// Maximum number of blocks in a chain
    pub max_block_parents: u8,
    /// Timestamp deviation tolerance
    pub timestamp_deviation_tolerance: u64,
    /// Genesis timestamp
    pub genesis_timestamp: u64,
    /// Maximum number of transactions per block
    pub max_txs_per_block: usize,
    /// Difficulty adjustment window
    pub difficulty_adjustment_window: usize,
    /// Minimum difficulty
    pub min_difficulty: BlueWorkType,
    /// Skip proof of work (for testing)
    pub skip_proof_of_work: bool,
}

impl Params {
    /// Validate the parameters for consistency
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.target_time_per_block == 0 {
            return Err("target_time_per_block must be positive");
        }
        if self.max_block_mass == 0 {
            return Err("max_block_mass must be positive");
        }
        Ok(())
    }
}

impl Default for Params {
    fn default() -> Self {
        // Mainnet defaults
        Self {
            network_id: NetworkId::Mainnet,
            target_time_per_block: 1000, // 1 second
            max_block_mass: 500_000, // 500KB
            max_tx_mass: 100_000, // 100KB
            halving_interval: 2_100_000,
            max_block_parents: 10,
            timestamp_deviation_tolerance: 132,
            genesis_timestamp: 1_600_000_000, // Example timestamp
            max_txs_per_block: 1000,
            difficulty_adjustment_window: 2646,
            min_difficulty: BlueWorkType::from_u64(1),
            skip_proof_of_work: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_default() {
        let params = Params::default();
        assert_eq!(params.network_id, NetworkId::Mainnet);
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_params_validation() {
        let mut params = Params::default();
        params.target_time_per_block = 0;
        assert!(params.validate().is_err());
    }
}
