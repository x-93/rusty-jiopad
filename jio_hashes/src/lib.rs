//! Jio hashes library.

use std::fmt;
use std::hash::Hasher;

/// Trait for extending hashers with additional methods.
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
    fn write_blue_work(&mut self, work: u64) -> &mut Self;

    /// Writes the number of bytes followed by the bytes themselves
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self;

    /// Writes the array len followed by each element as [[u8]]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self;
}

impl HasherExtensions for BlockHash {
    #[inline(always)]
    fn write_len(&mut self, len: usize) -> &mut Self {
        self.update(&(len as u64).to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_bool(&mut self, element: bool) -> &mut Self {
        self.update(if element { &[1u8] } else { &[0u8] });
        self
    }

    fn write_u8(&mut self, element: u8) -> &mut Self {
        self.update(&element.to_le_bytes());
        self
    }

    fn write_u16(&mut self, element: u16) -> &mut Self {
        self.update(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_u32(&mut self, element: u32) -> &mut Self {
        self.update(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_u64(&mut self, element: u64) -> &mut Self {
        self.update(&element.to_le_bytes());
        self
    }

    #[inline(always)]
    fn write_blue_work(&mut self, work: u64) -> &mut Self {
        let be_bytes = work.to_le_bytes();
        let start = be_bytes.iter().copied().position(|byte| byte != 0).unwrap_or(be_bytes.len());
        self.write_var_bytes(&be_bytes[start..])
    }

    #[inline(always)]
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.write_len(bytes.len()).update(bytes);
        self
    }

    #[inline(always)]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self {
        self.write_len(arr.len());
        for d in arr {
            self.update(d.as_ref());
        }
        self
    }
}

/// A 256-bit hash.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, serde::Serialize, serde::Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Create a hash from little-endian u64 array.
    pub fn from_le_u64(data: [u64; 4]) -> Self {
        let mut bytes = [0u8; 32];
        for (i, &val) in data.iter().enumerate() {
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&val.to_le_bytes());
        }
        Self(bytes)
    }

    /// Create a hash from a byte slice.
    pub fn from_slice(data: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        let len = data.len().min(32);
        bytes[..len].copy_from_slice(&data[..len]);
        Self(bytes)
    }

    /// Get the hash as bytes.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Get as little-endian u64 array.
    pub fn as_le_u64(&self) -> [u64; 4] {
        let mut arr = [0u64; 4];
        for (i, chunk) in self.0.chunks(8).enumerate() {
            arr[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        arr
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", self)
    }
}

impl std::hash::Hash for Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &u64_val in &self.as_le_u64() {
            state.write_u64(u64_val);
        }
    }
}

/// Block hasher for consensus operations.
#[derive(Clone)]
pub struct BlockHash {
    hasher: sha3::Sha3_256,
}

impl BlockHash {
    /// Creates a new block hasher.
    pub fn new() -> Self {
        Self {
            hasher: sha3::Sha3_256::default(),
        }
    }

    /// Updates the hasher with data.
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        use sha3::Digest;
        self.hasher.update(data);
        self
    }

    /// Finalizes the hash.
    pub fn finalize(self) -> Hash {
        use sha3::Digest;
        let result = self.hasher.finalize();
        Hash::from_slice(&result)
    }
}

impl Default for BlockHash {
    fn default() -> Self {
        Self::new()
    }
}

/// PoW hasher for HeavyHash algorithm.
#[derive(Clone)]
pub struct PowHash {
    hasher: sha3::Sha3_256,
}

impl PowHash {
    /// Creates a new PoW hasher with pre_pow_hash and timestamp.
    pub fn new(pre_pow_hash: Hash, timestamp: u64) -> Self {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_256::default();
        hasher.update(pre_pow_hash.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        // Add 32 zero bytes padding
        hasher.update(&[0u8; 32]);
        Self { hasher }
    }

    /// Finalizes the hash with a nonce.
    pub fn finalize_with_nonce(mut self, nonce: u64) -> Hash {
        use sha3::Digest;
        self.hasher.update(&nonce.to_le_bytes());
        let result = self.hasher.finalize();
        Hash::from_slice(&result)
    }
}
