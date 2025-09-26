//! Matrix for HeavyHash algorithm.

use jio_hashes::Hash;

/// Matrix for HeavyHash computation.
pub struct Matrix {
    // Simplified matrix for demonstration - in real implementation this would be much more complex
    data: [u8; 64],
}

impl Matrix {
    /// Generate matrix from pre_pow_hash.
    pub fn generate(pre_pow_hash: Hash) -> Self {
        let mut data = [0u8; 64];
        let hash_bytes = pre_pow_hash.as_bytes();
        // Simple matrix generation - copy hash bytes and repeat
        for i in 0..64 {
            data[i] = hash_bytes[i % 32];
        }
        Self { data }
    }

    /// Apply heavy hash to input hash.
    pub fn heavy_hash(&self, input: Hash) -> Hash {
        let mut result = [0u8; 32];
        let input_bytes = input.as_bytes();

        // Simple heavy hash simulation - XOR with matrix data
        for i in 0..32 {
            result[i] = input_bytes[i] ^ self.data[i] ^ self.data[i + 32];
        }

        Hash::from_slice(&result)
    }
}
