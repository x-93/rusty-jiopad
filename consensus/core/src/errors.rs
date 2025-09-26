//! Error types for the consensus core.

use crate::{Hash, KType};
use std::fmt;

/// Block-related errors.
pub mod block {
    use crate::errors::ConsensusError;
    pub type RuleError = ConsensusError;
    pub type BlockProcessResult<T> = Result<T, RuleError>;
}

/// Coinbase-related errors.
pub mod coinbase {
    use crate::errors::ConsensusError;
    pub type CoinbaseResult<T> = Result<T, ConsensusError>;
}

/// Consensus-related errors.
pub mod consensus {
    use crate::errors::ConsensusError;
    pub type ConsensusResult<T> = Result<T, ConsensusError>;
}

/// Pruning-related errors.
pub mod pruning {
    use crate::errors::ConsensusError;
    pub type PruningImportResult<T> = Result<T, ConsensusError>;
    pub type PruningProofMetadata = ConsensusError; // Stub
}

/// Transaction-related errors.
pub mod tx {
    use crate::errors::ConsensusError;
    pub type TxResult<T> = Result<T, ConsensusError>;
}

/// Consensus core errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusError {
    BlockHashMismatch {
        expected: Hash,
        actual: Hash,
    },

    InvalidBlockHeader { msg: String },

    TransactionValidation { msg: String },

    UtxoNotFound { output: Hash },

    InsufficientFunds,

    InvalidSignature,

    ScriptValidation { msg: String },

    MerkleRootMismatch,

    MiningRuleViolation { msg: String },

    DaaScoreCalculationFailed,

    InvalidKParameter { k: KType },

    Pruning { msg: String },

    NetworkProtocol { msg: String },

    MissingGhostDagData,

    InvalidSelectedParent,

    NoValidParent,

    NoTips,

    NoCommonAncestor,

    InvalidAnticone,

    Generic { msg: String },
}

impl fmt::Display for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsensusError::BlockHashMismatch { expected, actual } => {
                write!(f, "Block hash mismatch: expected {}, got {}", expected, actual)
            }
            ConsensusError::InvalidBlockHeader { msg } => {
                write!(f, "Invalid block header: {}", msg)
            }
            ConsensusError::TransactionValidation { msg } => {
                write!(f, "Transaction validation failed: {}", msg)
            }
            ConsensusError::UtxoNotFound { output } => {
                write!(f, "UTXO not found for output {}", output)
            }
            ConsensusError::InsufficientFunds => {
                write!(f, "Insufficient funds in transaction")
            }
            ConsensusError::InvalidSignature => {
                write!(f, "Invalid signature")
            }
            ConsensusError::ScriptValidation { msg } => {
                write!(f, "Script validation failed: {}", msg)
            }
            ConsensusError::MerkleRootMismatch => {
                write!(f, "Merkle root mismatch")
            }
            ConsensusError::MiningRuleViolation { msg } => {
                write!(f, "Mining rule violation: {}", msg)
            }
            ConsensusError::DaaScoreCalculationFailed => {
                write!(f, "DAA score calculation failed")
            }
            ConsensusError::InvalidKParameter { k } => {
                write!(f, "GHOSTDAG K parameter out of bounds: {}", k)
            }
            ConsensusError::Pruning { msg } => {
                write!(f, "Pruning error: {}", msg)
            }
            ConsensusError::NetworkProtocol { msg } => {
                write!(f, "Network protocol error: {}", msg)
            }
            ConsensusError::MissingGhostDagData => {
                write!(f, "Missing GhostDAG data for block")
            }
            ConsensusError::InvalidSelectedParent => {
                write!(f, "Invalid selected parent in GhostDAG data")
            }
            ConsensusError::NoValidParent => {
                write!(f, "No valid parent found for block")
            }
            ConsensusError::NoTips => {
                write!(f, "No tips found in the DAG")
            }
            ConsensusError::NoCommonAncestor => {
                write!(f, "No common ancestor found for reorganization")
            }
            ConsensusError::InvalidAnticone => {
                write!(f, "Invalid anticone calculation")
            }
            ConsensusError::Generic { msg } => {
                write!(f, "Generic consensus error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConsensusError {}

/// Result type alias for consensus operations.
pub type ConsensusResult<T> = Result<T, ConsensusError>;

impl From<crate::utxo::UtxoError> for ConsensusError {
    fn from(err: crate::utxo::UtxoError) -> Self {
        ConsensusError::Generic { msg: err.to_string() }
    }
}
