//!
//! # Consensus Core
//!
//! This crate implements primitives used in the Jio node consensus processing.
//!

extern crate alloc;
extern crate core;
extern crate self as consensus_core;

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hasher};

pub use jio_hashes::Hash;

pub mod acceptance_data;
pub mod api;
pub mod block;
pub mod blockhash;
pub mod blockstatus;
pub mod coinbase;
pub mod config;

pub mod constants;
pub mod daa_score_timestamp;
pub mod errors;

pub mod header;
pub mod mass;
pub mod merkle;
pub mod mining_rules;
pub mod muhash;
pub mod network;
pub mod pruning;
pub mod sign;
pub mod subnets;
pub mod trusted;
pub mod tx;
pub mod utxo;
pub mod hashing;
pub mod ghostdag;
pub mod chain_selection;


// Re-export implemented modules

pub use network::*;
pub use merkle::*;

/// Integer type for accumulated PoW of blue blocks. We expect no more than
/// 2^128 work in a single block (btc has ~2^80), and no more than 2^64
/// overall blocks, so 2^192 is definitely a justified upper-bound.
pub type BlueWorkType = jio_math::Uint192;

/// The extends directly from the expectation above about having no more than
/// 2^128 work in a single block
pub const MAX_WORK_LEVEL: BlockLevel = 128;

/// The type used to represent the GHOSTDAG K parameter
pub type KType = u16;

/// Map from Block hash to K type
pub type HashKTypeMap = std::sync::Arc<BlockHashMap<KType>>;

/// This HashMap skips the hashing of the key and uses the key directly as the hash.
/// Should only be used for block hashes that have correct DAA,
/// otherwise it is susceptible to DOS attacks via hash collisions.
pub type BlockHashMap<V> = HashMap<Hash, V, BlockHasher>;

/// Same as `BlockHashMap` but a `HashSet`.
pub type BlockHashSet = HashSet<Hash, BlockHasher>;

pub trait HashMapCustomHasher {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

// HashMap::new and HashMap::with_capacity are only implemented on Hasher=RandomState
// to avoid type inference problems, so we need to provide our own versions.
impl<V> HashMapCustomHasher for BlockHashMap<V> {
    #[inline(always)]
    fn new() -> Self {
        Self::with_hasher(BlockHasher::new())
    }
    #[inline(always)]
    fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_and_hasher(cap, BlockHasher::new())
    }
}

impl HashMapCustomHasher for BlockHashSet {
    #[inline(always)]
    fn new() -> Self {
        Self::with_hasher(BlockHasher::new())
    }
    #[inline(always)]
    fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_and_hasher(cap, BlockHasher::new())
    }
}

#[derive(Default, Debug)]
pub struct ChainPath {
    pub added: Vec<Hash>,
    pub removed: Vec<Hash>,
}

/// `hashes::Hash` writes 4 u64s so we just use the last one as the hash here
#[derive(Default, Clone, Copy)]
pub struct BlockHasher(u64);

impl BlockHasher {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Hasher for BlockHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline(always)]
    fn write_u64(&mut self, v: u64) {
        self.0 = v;
    }
    #[cold]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() >= 8 {
            self.write_u64(u64::from_le_bytes(bytes[bytes.len() - 8..bytes.len()].try_into().unwrap()));
        } else {
            let mut buf = [0u8; 8];
            buf[8 - bytes.len()..].copy_from_slice(bytes);
            self.write_u64(u64::from_le_bytes(buf));
        }
    }
}

impl BuildHasher for BlockHasher {
    type Hasher = Self;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self(0)
    }
}

pub type BlockLevel = u8;

#[cfg(test)]
mod tests {
    use super::BlockHasher;
    use jio_hashes::Hash;
    use std::hash::{Hash as _, Hasher as _};
    #[test]
    fn test_block_hasher() {
        let hash = Hash::from_le_u64([1, 2, 3, 4]);
        let mut hasher = BlockHasher::default();
        hash.hash(&mut hasher);
        assert_eq!(hasher.finish(), 4);
    }
}

// Re-export modules for public API
pub use acceptance_data::AcceptanceData;
pub use api::{ConsensusApi, DefaultConsensusApi};
pub use block::Block;
pub use blockhash::{block_hash, is_valid_block_hash};
pub use blockstatus::BlockStatus;
pub use coinbase::{create_coinbase_transaction, validate_coinbase};
pub use config::Config as ConsensusConfig;
pub use constants::*;
pub use daa_score_timestamp::DaaScoreTimestamp;
pub use errors::{ConsensusError, ConsensusResult};
pub use hashing::{hash_data, hash_block_header};
pub use header::Header;
pub use mass::{calculate_block_mass, validate_block_mass, BlockMass};
pub use merkle::{MerkleTree, calculate_merkle_root};
pub use mining_rules::{validate_mining_rules, check_proof_of_work};
pub use muhash::MuHash;
pub use network::{NetworkId, PeerAddress, NetworkMessage};
pub use pruning::PruningManager;
pub use sign::{sign_data, verify_signature};
pub use subnets::{Subnet, SubnetId};
pub use trusted::{TrustedNode, TrustedData};
pub use tx::{Transaction, TxInput, TxOutput};
pub use utxo::{UtxoCollection, OutPoint};

