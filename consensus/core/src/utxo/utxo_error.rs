//! UTXO-specific errors.

use crate::tx::TransactionOutpoint;

/// Custom errors for UTXO operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtxoError {
    /// UTXO not found.
    NotFound(TransactionOutpoint),
    /// UTXO already spent.
    AlreadySpent(TransactionOutpoint),
    /// Invalid output.
    InvalidOutput(String),
    /// Diff application failed.
    DiffApplicationFailed(String),
}

impl std::fmt::Display for UtxoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtxoError::NotFound(outpoint) => write!(f, "UTXO not found: {:?}", outpoint),
            UtxoError::AlreadySpent(outpoint) => write!(f, "UTXO already spent: {:?}", outpoint),
            UtxoError::InvalidOutput(msg) => write!(f, "Invalid output: {}", msg),
            UtxoError::DiffApplicationFailed(msg) => write!(f, "Diff application failed: {}", msg),
        }
    }
}

impl std::error::Error for UtxoError {}


