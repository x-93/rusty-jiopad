use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Runtime statistics for consensus operations.
#[derive(Debug)]
pub struct Stats {
    /// Timestamps of recent transactions
    transaction_timestamps: VecDeque<Instant>,
    /// Durations of recent block processing
    block_processing_times: VecDeque<Duration>,
    /// Window size for sliding averages (in seconds)
    window_seconds: u64,
}

impl Stats {
    /// Create a new Stats instance with a given window size.
    pub fn new(window_seconds: u64) -> Self {
        Self {
            transaction_timestamps: VecDeque::new(),
            block_processing_times: VecDeque::new(),
            window_seconds,
        }
    }

    /// Max entries to keep (prevents memory bloat if cleanup lags).
    const MAX_ENTRIES: usize = 10000; // Tune based on expected TPS * window

    /// Record a transaction for TPS calculation.
    pub fn record_transaction(&mut self) {
        let now = Instant::now();
        self.transaction_timestamps.push_back(now);
        if self.transaction_timestamps.len() > Self::MAX_ENTRIES {
            self.transaction_timestamps.pop_front();
        }
        self.cleanup_old_entries();
    }

    /// Record block processing time.
    pub fn record_block_processing_time(&mut self, duration: Duration) {
        self.block_processing_times.push_back(duration);
        // Keep only recent entries, say last 100
        if self.block_processing_times.len() > 100 {
            self.block_processing_times.pop_front();
        }
    }

    /// Get transactions per second over the window.
    pub fn tps(&self) -> f64 {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(self.window_seconds);
        let mut count = 0;
        for &t in &self.transaction_timestamps {
            if t > cutoff {
                count += 1;
            } else {
                break; // Since sorted, early exit
            }
        }
        count as f64 / self.window_seconds.max(1) as f64 // Avoid div0
    }

    /// Get average block processing time.
    pub fn average_block_processing_time(&self) -> Duration {
        if self.block_processing_times.is_empty() {
            Duration::default()
        } else {
            let total: Duration = self.block_processing_times.iter().sum();
            total / self.block_processing_times.len() as u32
        }
    }

    /// Get current statistics as a hashmap.
    pub fn get_stats(&self) -> std::collections::HashMap<&'static str, String> {
        std::collections::HashMap::from([
            ("tps", format!("{:.2}", self.tps())),
            ("avg_block_time_ms", format!("{:.2}", self.average_block_processing_time().as_millis())),
        ])
    }

    /// Clean up entries older than the window.
    fn cleanup_old_entries(&mut self) {
        let cutoff = Instant::now() - Duration::from_secs(self.window_seconds);
        while let Some(&front) = self.transaction_timestamps.front() {
            if front < cutoff {
                self.transaction_timestamps.pop_front();
            } else {
                break;
            }
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new(60) // 1 minute window
    }
}

/// Block count statistics.
#[derive(Debug, Clone, Copy, Default)]
pub struct BlockCount {
    pub total: u64,
    pub chain: u64,
}

/// Consensus statistics.
#[derive(Debug, Clone, Default)]
pub struct ConsensusStats {
    pub block_count: BlockCount,
    pub tps: f64,
    pub avg_block_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_stats_record_transaction() {
        let mut stats = Stats::new(10);
        stats.record_transaction();
        assert_eq!(stats.transaction_timestamps.len(), 1);
    }

    #[test]
    fn test_stats_tps() {
        let mut stats = Stats::new(10);
        for _ in 0..5 {
            stats.record_transaction();
            thread::sleep(Duration::from_millis(100));
        }
        let tps = stats.tps();
        assert!(tps > 0.0 && tps <= 5.0);
    }

    #[test]
    fn test_stats_average_block_time() {
        let mut stats = Stats::default();
        stats.record_block_processing_time(Duration::from_millis(100));
        stats.record_block_processing_time(Duration::from_millis(200));
        let avg = stats.average_block_processing_time();
        assert_eq!(avg, Duration::from_millis(150));
    }

    #[test]
    fn test_stats_get_stats() {
        let mut stats = Stats::default();
        stats.record_transaction();
        let stats_map = stats.get_stats();
        assert!(stats_map.contains_key("tps"));
        assert!(stats_map.contains_key("avg_block_time_ms"));
    }

    #[test]
    fn test_stats_cleanup_old_entries() {
        let mut stats = Stats::new(1); // 1 second window
        stats.record_transaction();
        thread::sleep(Duration::from_secs(2));
        stats.record_transaction(); // This should trigger cleanup
        assert_eq!(stats.transaction_timestamps.len(), 1);
    }

    #[test]
    fn test_stats_empty_average() {
        let stats = Stats::default();
        let avg = stats.average_block_processing_time();
        assert_eq!(avg, Duration::default());
    }

    #[test]
    fn test_stats_window_edge_cases() {
        let mut stats = Stats::new(1); // 1 second window
        stats.record_transaction();
        let _tps = stats.tps();
        // Just ensure no panic
        assert_eq!(stats.transaction_timestamps.len(), 1);
    }
}
