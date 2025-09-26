use crate::{BlueWorkType, Hash};
use std::hash::Hasher;

pub trait HasherExtensions {
    /// Writes the len as u64 little endian bytes
    fn write_len(&mut self, len: usize) -> &mut Self;

    /// Writes the boolean as a u8  
    fn write_bool(&mut self, element: bool) -> &mut Self;

    /// Writes a single u8  
    fn write_u8(&mut self, element: u8) -> &mut Self;

    /// Writes the u16 as a little endian u8 array  
    fn write_u16(&mut self, element: u16) -> &mut Self;

    /// Writes the u32 as a little endian u8 array  
    fn write_u32(&mut self, element: u32) -> &mut Self;

    /// Writes the u64 as a little endian u8 array  
    fn write_u64(&mut self, element: u64) -> &mut Self;

    /// Writes blue work as big endian bytes w/o the leading zeros
    /// (emulates bigint.bytes() in the jiopad golang ref)
    fn write_blue_work(&mut self, work: BlueWorkType) -> &mut Self;

    /// Writes the number of bytes followed by the bytes themselves
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self;

    /// Writes the array len followed by each element as [[u8]]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self;
}

/// Fails at compile time if `usize::MAX > u64::MAX`.
/// If `usize` will ever grow larger than `u64`, we need to verify
/// that the lossy conversion below at `write_len` remains precise.
const _: usize = u64::MAX as usize - usize::MAX;

impl<T: Hasher> HasherExtensions for T {
    #[inline(always)]
    fn write_len(&mut self, len: usize) -> &mut Self {
        self.write(&(len as u64).to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_bool(&mut self, element: bool) -> &mut Self {
        self.write(if element { &[1u8] } else { &[0u8] });
        self
    }

    fn write_u8(&mut self, element: u8) -> &mut Self {
        self.write(&element.to_le_bytes());
        self
    }

    fn write_u16(&mut self, element: u16) -> &mut Self {
        self.write(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_u32(&mut self, element: u32) -> &mut Self {
        self.write(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_u64(&mut self, element: u64) -> &mut Self {
        self.write(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_blue_work(&mut self, work: BlueWorkType) -> &mut Self {
        let be_bytes = work.to_le_bytes();
        let start = be_bytes.iter().copied().position(|byte| byte != 0).unwrap_or(be_bytes.len());

        self.write_var_bytes(&be_bytes[start..])
    }

    #[inline(always)]
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.write_len(bytes.len()).write(bytes);
        self
    }

    #[inline(always)]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self {
        self.write_len(arr.len());
        for d in arr {
            self.write(d.as_ref());
        }
        self
    }
}

/// Hash data using SHA256.
pub fn hash_data(data: &[u8]) -> Hash {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    Hash::from_slice(&result)
}

/// Hash block header.
pub fn hash_block_header(data: &[u8]) -> Hash {
    hash_data(data)
}

/// Hash merkle root.
pub fn hash_merkle_root(hashes: &[Hash]) -> Hash {
    let mut data = Vec::new();
    for hash in hashes {
        data.extend_from_slice(hash.as_bytes());
    }
    hash_data(&data)
}

/// Double SHA256 hash.
pub fn double_sha256(data: &[u8]) -> Hash {
    let first = hash_data(data);
    hash_data(first.as_bytes())
}

/// Hash script.
pub fn hash_script(data: &[u8]) -> Hash {
    hash_data(data)
}

/// Hash transaction.
pub fn hash_transaction(data: &[u8]) -> Hash {
    hash_data(data)
}

/// Calculate the target from compact bits representation.
pub fn target_from_bits(bits: u32) -> [u8; 32] {
    let mut target = [0u8; 32];
    let exponent = (bits >> 24) as usize;
    let mantissa = bits & 0x00ffffff;
    if exponent <= 3 {
        let shift = 3 - exponent;
        target[32 - shift..32].copy_from_slice(&(mantissa << (8 * shift)).to_be_bytes()[..shift]);
    } else {
        let shift = exponent - 3;
        if shift < 29 {
            target[32 - 4 - shift..32 - shift].copy_from_slice(&mantissa.to_be_bytes());
        }
    }
    target
}

/// Check if hash meets the target.
pub fn meets_target(hash: &Hash, target: &[u8; 32]) -> bool {
    hash.as_bytes() < target
}

