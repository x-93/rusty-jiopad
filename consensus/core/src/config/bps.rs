/// Configuration for blocks per second (BPS) limits and rate limiting.
#[derive(Clone, Debug, PartialEq)]
pub struct BpsParams {
    /// Maximum blocks per second allowed
    pub max_bps: f64,
    /// Window size in seconds for BPS calculation
    pub bps_window_seconds: u64,
    /// Enable BPS rate limiting
    pub enable_bps_limiting: bool,
    /// Burst allowance for BPS
    pub bps_burst_allowance: u32,
}

impl BpsParams {
    /// Check if a BPS rate is within limits
    pub fn is_within_limit(&self, current_bps: f64) -> bool {
        if !self.enable_bps_limiting {
            return true;
        }
        current_bps <= self.max_bps
    }

    /// Calculate allowed blocks in a given time window
    pub fn allowed_blocks_in_window(&self, window_seconds: u64) -> u64 {
        (self.max_bps * window_seconds as f64) as u64 + self.bps_burst_allowance as u64
    }
}

impl Default for BpsParams {
    fn default() -> Self {
        Self {
            max_bps: 10.0, // 10 blocks per second
            bps_window_seconds: 60,
            enable_bps_limiting: true,
            bps_burst_allowance: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bps_params_default() {
        let params = BpsParams::default();
        assert_eq!(params.max_bps, 10.0);
        assert!(params.enable_bps_limiting);
    }

    #[test]
    fn test_bps_within_limit() {
        let params = BpsParams::default();
        assert!(params.is_within_limit(5.0));
        assert!(!params.is_within_limit(15.0));
    }

    #[test]
    fn test_allowed_blocks_in_window() {
        let params = BpsParams::default();
        let allowed = params.allowed_blocks_in_window(10);
        assert_eq!(allowed, 100 + 5); // 10 * 10 + burst
    }
}
