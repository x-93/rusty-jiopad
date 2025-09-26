use std::fmt;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

/// A 256-bit unsigned integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Uint256([u8; 32]);

impl Uint256 {
    /// Create from compact target bits (Bitcoin-style).
    pub fn from_compact_target_bits(bits: u32) -> Self {
        let mut bytes = [0u8; 32];
        let exponent = (bits >> 24) as usize;
        let mantissa = bits & 0x00FF_FFFF;
        if exponent <= 3 {
            let shift = 3 - exponent;
            let mantissa_shifted = (mantissa as u32) << (8 * shift);
            let mantissa_bytes = mantissa_shifted.to_be_bytes();
            bytes[32 - shift..32].copy_from_slice(&mantissa_bytes[4 - shift..]);
        } else {
            let shift = exponent - 3;
            if shift < 29 {
                let mantissa_bytes = (mantissa as u32).to_be_bytes();
                let start = 32 - 4 - shift;
                let end = 32 - shift;
                bytes[start..end].copy_from_slice(&mantissa_bytes);
            }
        }
        Self(bytes)
    }

    /// Get the number of bits in the integer.
    pub fn bits(&self) -> u32 {
        let mut bits = 256;
        for &byte in self.0.iter().rev() {
            if byte != 0 {
                bits -= self.0.iter().rev().position(|&b| b != 0).unwrap() as u32 * 8;
                bits += (byte as u32).leading_zeros() as u32;
                break;
            }
        }
        256 - bits
    }

    /// Compare with another Uint256.
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl From<[u8; 32]> for Uint256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uint256({})", self)
    }
}
