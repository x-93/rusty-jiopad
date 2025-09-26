//! Xoshiro random number generator for HeavyHash.

/// Xoshiro256** random number generator.
pub struct Xoshiro256 {
    state: [u64; 4],
}

impl Xoshiro256 {
    /// Create new generator with seed.
    pub fn new(seed: u64) -> Self {
        let mut state = [0u64; 4];
        state[0] = seed;
        state[1] = seed.wrapping_mul(0x9E3779B97F4A7C15);
        state[2] = seed.wrapping_mul(0xB5297A4D3C2DB1EF);
        state[3] = seed.wrapping_mul(0x68BC384E9F5B8D3F);
        Self { state }
    }

    /// Generate next random u64.
    pub fn next(&mut self) -> u64 {
        let result = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }
}
