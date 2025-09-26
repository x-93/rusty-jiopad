//! MuHash for efficient UTXO set hashing.

use crate::Hash;

/// MuHash state for incremental hashing.
#[derive(Debug, Clone)]
pub struct MuHash {
    state: Hash,
}

impl MuHash {
    /// Creates a new MuHash instance.
    pub fn new() -> Self {
        Self { state: Hash::default() }
    }

    /// Adds an element to the hash.
    pub fn add(&mut self, element: &Hash) {
        // Placeholder: XOR for simplicity
        self.state = Hash::from_le_u64([
            self.state.as_le_u64()[0] ^ element.as_le_u64()[0],
            self.state.as_le_u64()[1] ^ element.as_le_u64()[1],
            self.state.as_le_u64()[2] ^ element.as_le_u64()[2],
            self.state.as_le_u64()[3] ^ element.as_le_u64()[3],
        ]);
    }

    /// Removes an element from the hash.
    pub fn remove(&mut self, element: &Hash) {
        self.add(element); // XOR is its own inverse
    }

    /// Gets the current hash.
    pub fn finalize(&self) -> Hash {
        self.state
    }
}

impl Default for MuHash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muhash_add_remove() {
        let mut muhash = MuHash::new();
        let hash1 = Hash::from_le_u64([1, 0, 0, 0]);
        let hash2 = Hash::from_le_u64([2, 0, 0, 0]);

        muhash.add(&hash1);
        let h1 = muhash.finalize();
        muhash.add(&hash2);
        let h2 = muhash.finalize();
        muhash.remove(&hash2);
        let h3 = muhash.finalize();

        assert_eq!(h1, h3);
        assert_ne!(h1, h2);
    }
}
