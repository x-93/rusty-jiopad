use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

/// Thread-safe counters for consensus operations.
#[derive(Debug, Default)]
pub struct Counters {
    /// Number of blocks processed
    pub blocks_processed: AtomicU64,
    /// Number of transactions validated
    pub transactions_validated: AtomicU64,
    /// Number of validation errors
    pub validation_errors: AtomicU64,
    /// Number of blocks rejected
    pub blocks_rejected: AtomicU64,
    /// Number of pruning operations
    pub pruning_operations: AtomicU64,
}

impl Counters {
    /// Increment the blocks processed counter
    pub fn increment_blocks_processed(&self) {
        self.blocks_processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment the transactions validated counter
    pub fn increment_transactions_validated(&self, count: u64) {
        self.transactions_validated.fetch_add(count, Ordering::Relaxed);
    }

    /// Increment the validation errors counter
    pub fn increment_validation_errors(&self) {
        self.validation_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment the blocks rejected counter
    pub fn increment_blocks_rejected(&self) {
        self.blocks_rejected.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment the pruning operations counter
    pub fn increment_pruning_operations(&self) {
        self.pruning_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Get a snapshot of current counter values
    pub fn get_snapshot(&self) -> HashMap<&'static str, u64> {
        HashMap::from([
            ("blocks_processed", self.blocks_processed.load(Ordering::Relaxed)),
            ("transactions_validated", self.transactions_validated.load(Ordering::Relaxed)),
            ("validation_errors", self.validation_errors.load(Ordering::Relaxed)),
            ("blocks_rejected", self.blocks_rejected.load(Ordering::Relaxed)),
            ("pruning_operations", self.pruning_operations.load(Ordering::Relaxed)),
        ])
    }

    /// Reset all counters (for testing)
    pub fn reset(&self) {
        self.blocks_processed.store(0, Ordering::Relaxed);
        self.transactions_validated.store(0, Ordering::Relaxed);
        self.validation_errors.store(0, Ordering::Relaxed);
        self.blocks_rejected.store(0, Ordering::Relaxed);
        self.pruning_operations.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counters_increment() {
        let counters = Counters::default();
        counters.increment_blocks_processed();
        counters.increment_transactions_validated(5);
        counters.increment_validation_errors();

        let snapshot = counters.get_snapshot();
        assert_eq!(snapshot["blocks_processed"], 1);
        assert_eq!(snapshot["transactions_validated"], 5);
        assert_eq!(snapshot["validation_errors"], 1);
    }

    #[test]
    fn test_counters_reset() {
        let counters = Counters::default();
        counters.increment_blocks_processed();
        counters.reset();
        let snapshot = counters.get_snapshot();
        assert_eq!(snapshot["blocks_processed"], 0);
    }

    #[test]
    fn test_counters_thread_safety() {
        use std::thread;
        use std::sync::Arc;
        let counters = Arc::new(Counters::default());
        let handles: Vec<_> = (0..10).map(|_| {
            let c = Arc::clone(&counters);
            thread::spawn(move || {
                for _ in 0..100 {
                    c.increment_blocks_processed();
                }
            })
        }).collect();
        for h in handles {
            h.join().unwrap();
        }
        let snapshot = counters.get_snapshot();
        assert_eq!(snapshot["blocks_processed"], 1000);
    }

    #[test]
    fn test_counters_all_fields() {
        let counters = Counters::default();
        counters.increment_blocks_processed();
        counters.increment_transactions_validated(1);
        counters.increment_validation_errors();
        counters.increment_blocks_rejected();
        counters.increment_pruning_operations();

        let snapshot = counters.get_snapshot();
        assert_eq!(snapshot.len(), 5);
        assert_eq!(snapshot["blocks_processed"], 1);
        assert_eq!(snapshot["transactions_validated"], 1);
        assert_eq!(snapshot["validation_errors"], 1);
        assert_eq!(snapshot["blocks_rejected"], 1);
        assert_eq!(snapshot["pruning_operations"], 1);
    }
}
