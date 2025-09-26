//! Jio math library.

use std::fmt;

pub mod uint256;

/// A 192-bit unsigned integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct Uint192([u8; 24]);

impl Uint192 {
    /// Create from u64.
    pub const fn from_u64(val: u64) -> Self {
        let bytes = [
            (val & 0xFF) as u8,
            ((val >> 8) & 0xFF) as u8,
            ((val >> 16) & 0xFF) as u8,
            ((val >> 24) & 0xFF) as u8,
            ((val >> 32) & 0xFF) as u8,
            ((val >> 40) & 0xFF) as u8,
            ((val >> 48) & 0xFF) as u8,
            ((val >> 56) & 0xFF) as u8,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        Self(bytes)
    }

    /// Get as bytes.
    pub fn as_bytes(&self) -> &[u8; 24] {
        &self.0
    }

    /// Get as little-endian bytes.
    pub fn to_le_bytes(&self) -> [u8; 24] {
        self.0
    }
}

impl fmt::Display for Uint192 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Uint192 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uint192({})", self)
    }
}

pub use uint256::Uint256;
