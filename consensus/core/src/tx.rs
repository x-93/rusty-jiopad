//! Transaction data structures.

use crate::{hashing, Hash, errors::ConsensusResult};

pub mod script_public_key;

/// Transaction input.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TxInput {
    pub prev_tx_hash: Hash,
    pub index: u32,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

/// Transaction output.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

/// Transaction structure.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub version: u16,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(version: u16, inputs: Vec<TxInput>, outputs: Vec<TxOutput>, lock_time: u32) -> Self {
        Self { version, inputs, outputs, lock_time }
    }

    /// Computes the transaction hash.
    pub fn hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.version.to_le_bytes());
        for input in &self.inputs {
            data.extend_from_slice(input.prev_tx_hash.as_bytes());
            data.extend_from_slice(&input.index.to_le_bytes());
            data.extend_from_slice(&input.script_sig);
            data.extend_from_slice(&input.sequence.to_le_bytes());
        }
        for output in &self.outputs {
            data.extend_from_slice(&output.value.to_le_bytes());
            data.extend_from_slice(&output.script_pubkey);
        }
        data.extend_from_slice(&self.lock_time.to_le_bytes());

        hashing::hash_transaction(&data)
    }

    /// Validates the transaction.
    pub fn validate(&self) -> ConsensusResult<()> {
        if self.inputs.is_empty() {
            return Err(crate::errors::ConsensusError::TransactionValidation {
                msg: "Transaction must have at least one input".to_string(),
            });
        }
        if self.outputs.is_empty() {
            return Err(crate::errors::ConsensusError::TransactionValidation {
                msg: "Transaction must have at least one output".to_string(),
            });
        }

        // Check for duplicate inputs
        let mut seen = std::collections::HashSet::new();
        for input in &self.inputs {
            let key = (input.prev_tx_hash, input.index);
            if !seen.insert(key) {
                return Err(crate::errors::ConsensusError::TransactionValidation {
                    msg: "Duplicate transaction inputs".to_string(),
                });
            }
        }

        // Additional validations (e.g., script validation) can be added
        Ok(())
    }

    /// Checks if the transaction is a coinbase transaction.
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].prev_tx_hash == Hash::default()
    }

    /// Calculates the mass of the transaction.
    pub fn mass(&self) -> u64 {
        // Simplified mass calculation: base mass + input mass + output mass
        let base_mass = 100; // Fixed base
        let input_mass = self.inputs.len() as u64 * 50;
        let output_mass = self.outputs.len() as u64 * 30;
        base_mass + input_mass + output_mass
    }
}

/// Mutable transaction.
#[derive(Debug, Clone, Default)]
pub struct MutableTransaction {
    pub version: u16,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

/// Signable transaction.
#[derive(Debug, Clone, Default)]
pub struct SignableTransaction {
    pub version: u16,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

/// Transaction outpoint.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionOutpoint {
    pub transaction_id: Hash,
    pub index: u32,
}

/// UTXO entry.
#[derive(Debug, Clone, Default)]
pub struct UtxoEntry {
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
    pub block_daa_score: u64,
    pub is_coinbase: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_new() {
        let tx = Transaction::new(1, vec![], vec![], 0);
        assert_eq!(tx.version, 1);
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::new(1, vec![], vec![], 0);
        let hash = tx.hash();
        assert!(!hash.as_bytes().is_empty());
    }

    #[test]
    fn test_transaction_validate_no_inputs() {
        let tx = Transaction::new(1, vec![], vec![TxOutput { value: 100, script_pubkey: vec![] }], 0);
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_transaction_validate_no_outputs() {
        let input = TxInput {
            prev_tx_hash: Hash::default(),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let tx = Transaction::new(1, vec![input], vec![], 0);
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_transaction_is_coinbase() {
        let input = TxInput {
            prev_tx_hash: Hash::default(),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let tx = Transaction::new(1, vec![input], vec![], 0);
        assert!(tx.is_coinbase());
    }
}
