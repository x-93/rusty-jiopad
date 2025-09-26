//! UTXO inquirer for read-only queries.

use crate::tx::TxOutput;
use super::utxo_collection::{UtxoCollection, OutPoint};
use super::utxo_error::UtxoError;

/// Read-only UTXO inquirer.
pub trait UtxoInquirer {
    /// Gets a UTXO.
    fn get_utxo(&self, outpoint: &OutPoint) -> Option<TxOutput>;

    /// Checks if a UTXO exists.
    fn exists(&self, outpoint: &OutPoint) -> bool {
        self.get_utxo(outpoint).is_some()
    }

    /// Gets the balance for a script pubkey.
    fn get_balance(&self, script_pubkey: &[u8]) -> u64;
}

impl UtxoInquirer for UtxoCollection {
    fn get_utxo(&self, outpoint: &OutPoint) -> Option<TxOutput> {
        self.get(outpoint)
    }

    fn get_balance(&self, script_pubkey: &[u8]) -> u64 {
        let utxos = self.utxos.read().unwrap();
        utxos.values()
            .filter(|output| output.script_pubkey == script_pubkey)
            .map(|output| output.value)
            .sum()
    }
}

/// Error type for inquirer.
pub type UtxoInquirerError = UtxoError;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hash;

    #[test]
    fn test_get_utxo() {
        let collection = UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = TxOutput {
            value: 100,
            script_pubkey: vec![1, 2, 3],
        };
        collection.insert(outpoint.clone(), output.clone()).unwrap();
        assert_eq!(collection.get_utxo(&outpoint), Some(output));
    }

    #[test]
    fn test_get_balance() {
        let collection = UtxoCollection::new();
        let script = vec![1, 2, 3];
        let outpoint1 = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let outpoint2 = OutPoint {
            tx_hash: Hash::from_le_u64([1, 0, 0, 0]),
            index: 0,
        };
        let output1 = TxOutput {
            value: 100,
            script_pubkey: script.clone(),
        };
        let output2 = TxOutput {
            value: 200,
            script_pubkey: script.clone(),
        };
        collection.insert(outpoint1, output1).unwrap();
        collection.insert(outpoint2, output2).unwrap();
        assert_eq!(collection.get_balance(&script), 300);
    }
}
