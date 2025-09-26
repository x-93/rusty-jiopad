//! UTXO view for immutable snapshots.

use crate::tx::Transaction;
use super::utxo_collection::{UtxoCollection, OutPoint};
use super::utxo_diff::UtxoDiff;
use super::utxo_error::UtxoError;

/// Immutable UTXO view.
#[derive(Debug, Clone)]
pub struct UtxoView {
    utxos: std::collections::HashMap<OutPoint, crate::tx::TxOutput>,
}

impl UtxoView {
    /// Creates a view from a collection.
    pub fn new_from_collection(collection: &UtxoCollection) -> Self {
        let utxos = collection.utxos.read().unwrap().clone();
        Self { utxos }
    }

    /// Applies a diff to the view.
    pub fn apply_diff(&mut self, diff: &UtxoDiff) {
        for (outpoint, output) in &diff.added {
            self.utxos.insert(outpoint.clone(), output.clone());
        }
        for outpoint in &diff.removed {
            self.utxos.remove(outpoint);
        }
    }

    /// Validates a transaction against the view.
    pub fn validate_tx(&self, tx: &Transaction) -> Result<(), UtxoError> {
        let mut seen = std::collections::HashSet::new();
        for input in &tx.inputs {
            let outpoint = OutPoint {
                tx_hash: input.prev_tx_hash,
                index: input.index,
            };
            if !self.utxos.contains_key(&outpoint) {
                return Err(UtxoError::NotFound(crate::tx::TransactionOutpoint {
                    transaction_id: outpoint.tx_hash,
                    index: outpoint.index,
                }));
            }
            if !seen.insert(outpoint.clone()) {
                return Err(UtxoError::AlreadySpent(crate::tx::TransactionOutpoint {
                    transaction_id: outpoint.tx_hash,
                    index: outpoint.index,
                }));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx::{TxInput, Transaction};
    use crate::Hash;

    #[test]
    fn test_new_from_collection() {
        let collection = UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = crate::tx::TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        collection.insert(outpoint.clone(), output.clone()).unwrap();
        let view = UtxoView::new_from_collection(&collection);
        assert!(view.utxos.contains_key(&outpoint));
    }

    #[test]
    fn test_validate_tx() {
        let collection = UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = crate::tx::TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        collection.insert(outpoint.clone(), output).unwrap();
        let view = UtxoView::new_from_collection(&collection);

        let input = TxInput {
            prev_tx_hash: Hash::default(),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let tx = Transaction::new(1, vec![input], vec![], 0);
        assert!(view.validate_tx(&tx).is_ok());
    }

    #[test]
    fn test_validate_invalid_tx() {
        let collection = UtxoCollection::new();
        let view = UtxoView::new_from_collection(&collection);

        let input = TxInput {
            prev_tx_hash: Hash::default(),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let tx = Transaction::new(1, vec![input], vec![], 0);
        assert!(view.validate_tx(&tx).is_err());
    }
}
