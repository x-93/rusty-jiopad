//! UTXO diff for incremental changes.

use crate::tx::{Transaction, TxOutput};
use super::utxo_collection::OutPoint;
use super::utxo_error::UtxoError;

/// Incremental UTXO changes.
#[derive(Debug, Clone, Default)]
pub struct UtxoDiff {
    pub added: Vec<(OutPoint, TxOutput)>,
    pub removed: Vec<OutPoint>,
}

impl UtxoDiff {
    /// Creates a new diff.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a UTXO.
    pub fn add(&mut self, outpoint: OutPoint, output: TxOutput) {
        self.added.push((outpoint, output));
    }

    /// Removes a UTXO.
    pub fn remove(&mut self, outpoint: OutPoint) {
        self.removed.push(outpoint);
    }

    /// Applies the diff to a collection.
    pub fn apply_to(&self, collection: &super::utxo_collection::UtxoCollection) -> Result<(), UtxoError> {
        collection.apply_diff(self)
    }

    /// Reverses the diff.
    pub fn reverse(&self) -> UtxoDiff {
        let mut reversed = UtxoDiff::new();
        // Note: Reverse is incomplete without collection access
        for (outpoint, _) in &self.added {
            reversed.remove(outpoint.clone());
        }
        // For removed, we can't add back without knowing the output
        reversed
    }

    /// Creates a diff from a transaction.
    pub fn from_transaction(tx: &Transaction) -> Self {
        let mut diff = UtxoDiff::new();
        // Spend inputs
        for input in &tx.inputs {
            let outpoint = OutPoint {
                tx_hash: input.prev_tx_hash,
                index: input.index,
            };
            diff.remove(outpoint);
        }
        // Add outputs
        let tx_hash = tx.hash();
        for (index, output) in tx.outputs.iter().enumerate() {
            let outpoint = OutPoint {
                tx_hash,
                index: index as u32,
            };
            diff.add(outpoint, output.clone());
        }
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx::{TxInput, Transaction};
    use crate::Hash;

    #[test]
    fn test_from_transaction() {
        let input = TxInput {
            prev_tx_hash: Hash::default(),
            index: 0,
            script_sig: vec![],
            sequence: 0,
        };
        let output = TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        let tx = Transaction::new(1, vec![input], vec![output.clone()], 0);
        let diff = UtxoDiff::from_transaction(&tx);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.added.len(), 1);
    }

    #[test]
    fn test_apply_diff() {
        let collection = crate::UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        let mut diff = UtxoDiff::new();
        diff.add(outpoint.clone(), output.clone());
        assert!(diff.apply_to(&collection).is_ok());
        assert_eq!(collection.get(&outpoint), Some(output));
    }
}
