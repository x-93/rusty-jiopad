//! Coinbase transaction utilities.

use crate::{tx::{Transaction, TxInput, TxOutput}, Hash, errors::ConsensusResult};

/// Miner data for coinbase transactions.
#[derive(Debug, Clone, Default)]
pub struct MinerData {
    pub extra_data: Vec<u8>,
}

/// Creates a coinbase transaction for mining rewards.
/// Coinbase transactions have one input with null prev_tx_hash and one output with the reward.
pub fn create_coinbase_transaction(reward: u64, script_pubkey: Vec<u8>) -> Transaction {
    let input = TxInput {
        prev_tx_hash: Hash::default(),
        index: 0,
        script_sig: vec![],
        sequence: 0,
    };
    let output = TxOutput { value: reward, script_pubkey };
    Transaction::new(1, vec![input], vec![output], 0)
}

/// Validates a coinbase transaction.
/// Coinbase must have exactly one input with null prev_tx_hash, exactly one output, and the output value must be valid.
pub fn validate_coinbase(tx: &Transaction) -> ConsensusResult<()> {
    if !tx.is_coinbase() {
        return Err(crate::errors::ConsensusError::TransactionValidation {
            msg: "Not a coinbase transaction".to_string(),
        });
    }
    if tx.outputs.len() != 1 {
        return Err(crate::errors::ConsensusError::TransactionValidation {
            msg: "Coinbase must have exactly one output".to_string(),
        });
    }
    // Additional validations (e.g., reward amount) can be added
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_coinbase() {
        let tx = create_coinbase_transaction(50, vec![0x01]);
        assert!(tx.is_coinbase());
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].value, 50);
    }

    #[test]
    fn test_validate_coinbase_valid() {
        let tx = create_coinbase_transaction(50, vec![0x01]);
        assert!(validate_coinbase(&tx).is_ok());
    }

    #[test]
    fn test_validate_coinbase_invalid() {
        let input = TxInput {
            prev_tx_hash: Hash::from_slice(b"non_default"),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let output = TxOutput { value: 50, script_pubkey: vec![] };
        let tx = Transaction::new(1, vec![input], vec![output], 0);
        assert!(validate_coinbase(&tx).is_err());
    }
}
