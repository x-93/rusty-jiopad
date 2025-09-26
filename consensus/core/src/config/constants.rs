/// Performance-related parameters for consensus operations.
pub mod perf {
    use crate::config::params::Params;

    /// Performance parameters for tuning consensus behavior.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PerfParams {
        /// Maximum mass per transaction
        pub max_mass_per_tx: u64,
        /// Maximum mass per block
        pub max_mass_per_block: u64,
        /// Maximum number of transactions per second
        pub max_tps: u64,
        /// Block processing timeout in milliseconds
        pub block_processing_timeout_ms: u64,
        /// Memory limit for UTXO cache
        pub utxo_cache_memory_limit: usize,
        /// Number of parallel validation threads
        pub validation_threads: usize,
    }

    impl PerfParams {
        /// Adjust performance parameters based on consensus parameters
        pub fn adjust_to_consensus_params(&mut self, _params: &Params) {
            // Example adjustment: scale TPS based on block time
            // self.max_tps = (1000 / params.target_time_per_block) * 100; // Simplified
        }
    }

    impl Default for PerfParams {
        fn default() -> Self {
            Self {
                max_mass_per_tx: 100_000,
                max_mass_per_block: 500_000,
                max_tps: 1000,
                block_processing_timeout_ms: 5000,
                utxo_cache_memory_limit: 1_000_000_000, // 1GB
                validation_threads: num_cpus::get(),
            }
        }
    }

    /// Default performance parameters
    pub const PERF_PARAMS: PerfParams = PerfParams {
        max_mass_per_tx: 100_000,
        max_mass_per_block: 500_000,
        max_tps: 1000,
        block_processing_timeout_ms: 5000,
        utxo_cache_memory_limit: 1_000_000_000,
        validation_threads: 4, // Conservative default
    };
}

#[cfg(test)]
mod tests {
    use super::perf::*;

    #[test]
    fn test_perf_params_default() {
        let params = PerfParams::default();
        assert_eq!(params.max_mass_per_tx, 100_000);
    }

    #[test]
    fn test_perf_params_adjust() {
        let mut params = PerfParams::default();
        let consensus_params = crate::config::params::Params::default();
        params.adjust_to_consensus_params(&consensus_params);
        // Add assertions based on adjustment logic
    }
}
