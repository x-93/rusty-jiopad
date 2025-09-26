use crate::Hash;

#[cfg(feature = "devnet-prealloc")]
use crate::utxo::utxo_collection::UtxoCollection;
#[cfg(feature = "devnet-prealloc")]
use std::sync::Arc;

/// Configuration for the genesis block and initial network state.
#[derive(Clone, Debug)]
pub struct GenesisParams {
    /// Hash of the genesis block
    pub genesis_hash: Hash,
    /// Timestamp of the genesis block
    pub genesis_timestamp: u64,
    /// Initial difficulty
    pub initial_difficulty: u64,
    /// Pre-allocated UTXO set for devnet
    #[cfg(feature = "devnet-prealloc")]
    pub initial_utxo_set: Arc<UtxoCollection>,
    /// Enable genesis processing
    pub process_genesis: bool,
}

impl GenesisParams {
    /// Create genesis params for mainnet
    pub fn mainnet() -> Self {
        Self {
            genesis_hash: Hash::from_le_u64([0; 4]), // Placeholder
            genesis_timestamp: 1_600_000_000,
            initial_difficulty: 1,
            #[cfg(feature = "devnet-prealloc")]
            initial_utxo_set: Arc::new(UtxoCollection::new()),
            process_genesis: true,
        }
    }

    /// Create genesis params for testnet
    pub fn testnet() -> Self {
        Self {
            genesis_hash: Hash::from_le_u64([1; 4]), // Placeholder
            genesis_timestamp: 1_600_000_000,
            initial_difficulty: 1,
            #[cfg(feature = "devnet-prealloc")]
            initial_utxo_set: Arc::new(UtxoCollection::new()),
            process_genesis: true,
        }
    }
}

impl Default for GenesisParams {
    fn default() -> Self {
        Self::mainnet()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_params_default() {
        let params = GenesisParams::default();
        assert_eq!(params.genesis_timestamp, 1_600_000_000);
        assert!(params.process_genesis);
    }

    #[test]
    fn test_genesis_params_mainnet() {
        let params = GenesisParams::mainnet();
        assert_eq!(params.initial_difficulty, 1);
    }
}
