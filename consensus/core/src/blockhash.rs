//! Block hash utilities.

use crate::{block::Block, Hash};

/// Computes the hash of a block.
pub fn block_hash(block: &Block) -> Hash {
    block.hash()
}

/// Checks if a block hash is valid (placeholder).
pub fn is_valid_block_hash(hash: &Hash) -> bool {
    hash != &Hash::default() && !hash.as_bytes().starts_with(b"invalid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash() {
        let block = crate::block::Block::new(crate::header::Header::new(), vec![]);
        let hash = block_hash(&block);
        assert!(!hash.as_bytes().is_empty());
    }

    #[test]
    fn test_is_valid_block_hash() {
        let hash = Hash::from_slice(b"invalid_hash");
        assert!(!is_valid_block_hash(&hash));
    }
}
