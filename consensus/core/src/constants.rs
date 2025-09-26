//! Consensus constants for the Jio blockchain.

use crate::BlueWorkType;

/// Maximum mass allowed for a block in grams.
pub const MAX_BLOCK_MASS: u64 = 500_000;

/// Target time between blocks in seconds.
pub const TARGET_BLOCK_TIME: u64 = 1;

/// Number of blocks between difficulty adjustments.
pub const DIFFICULTY_ADJUSTMENT_WINDOW: u32 = 1024;

/// Initial target difficulty.
pub const INITIAL_TARGET: BlueWorkType = BlueWorkType::from_u64(0x1d00ffff);

/// Halving interval in blocks.
pub const HALVING_INTERVAL: u64 = 210_000;

/// Maximum number of transactions per block.
pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 10_000;

/// Minimum transaction fee in sompi (smallest unit).
pub const MIN_TRANSACTION_FEE: u64 = 1;

/// Coinbase maturity in blocks.
pub const COINBASE_MATURITY: u64 = 100;

/// Maximum script size in bytes.
pub const MAX_SCRIPT_SIZE: usize = 10_000;

/// Maximum stack size for script execution.
pub const MAX_STACK_SIZE: usize = 1000;

/// Maximum number of signature operations per block.
pub const MAX_SIGOPS_PER_BLOCK: u32 = 20_000;

/// Network magic bytes.
pub const NETWORK_MAGIC: [u8; 4] = [0xAB, 0xCD, 0xEF, 0x12];

/// Protocol version.
pub const PROTOCOL_VERSION: u32 = 1;

/// Maximum orphan blocks to keep.
pub const MAX_ORPHAN_BLOCKS: usize = 100;

/// DAA window size.
pub const DAA_WINDOW_SIZE: usize = 1024;

/// GHOSTDAG K parameter default.
pub const DEFAULT_GHOSTDAG_K: u16 = 18;
