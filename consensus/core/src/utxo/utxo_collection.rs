//! UTXO collection for storage.

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::tx::{TransactionOutpoint, TxOutput};
use crate::muhash::MuHash;
use super::utxo_error::UtxoError;

/// OutPoint representing a transaction output reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub tx_hash: crate::Hash,
    pub index: u32,
}

/// UTXO entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub output: TxOutput,
}

/// Thread-safe UTXO collection.
#[derive(Debug, Clone)]
pub struct UtxoCollection {
    pub(crate) utxos: Arc<RwLock<HashMap<OutPoint, TxOutput>>>,
    muhash: Arc<RwLock<MuHash>>,
}

impl UtxoCollection {
    /// Creates a new UTXO collection.
    pub fn new() -> Self {
        Self {
            utxos: Arc::new(RwLock::new(HashMap::new())),
            muhash: Arc::new(RwLock::new(MuHash::new())),
        }
    }

    /// Inserts a UTXO.
    pub fn insert(&self, outpoint: OutPoint, output: TxOutput) -> Result<(), UtxoError> {
        let mut utxos = self.utxos.write().unwrap();
        if utxos.contains_key(&outpoint) {
            return Err(UtxoError::AlreadySpent(TransactionOutpoint {
                transaction_id: outpoint.tx_hash,
                index: outpoint.index,
            }));
        }
        utxos.insert(outpoint.clone(), output.clone());
        let mut muhash = self.muhash.write().unwrap();
        muhash.add(&outpoint.tx_hash); // Simplified: hash tx_hash
        Ok(())
    }

    /// Removes a UTXO.
    pub fn remove(&self, outpoint: &OutPoint) -> Result<Option<TxOutput>, UtxoError> {
        let mut utxos = self.utxos.write().unwrap();
        let output = utxos.remove(outpoint);
        if output.is_some() {
            let mut muhash = self.muhash.write().unwrap();
            muhash.remove(&outpoint.tx_hash);
        }
        Ok(output)
    }

    /// Gets a UTXO.
    pub fn get(&self, outpoint: &OutPoint) -> Option<TxOutput> {
        let utxos = self.utxos.read().unwrap();
        utxos.get(outpoint).cloned()
    }

    /// Gets the length.
    pub fn len(&self) -> usize {
        let utxos = self.utxos.read().unwrap();
        utxos.len()
    }

    /// Checks if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Applies a diff.
    pub fn apply_diff(&self, diff: &super::utxo_diff::UtxoDiff) -> Result<(), UtxoError> {
        for (outpoint, output) in &diff.added {
            self.insert(outpoint.clone(), output.clone())?;
        }
        for outpoint in &diff.removed {
            self.remove(outpoint)?;
        }
        Ok(())
    }

    /// Gets the MuHash.
    pub fn muhash(&self) -> crate::Hash {
        let muhash = self.muhash.read().unwrap();
        muhash.finalize()
    }
}

impl Default for UtxoCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hash;

    #[test]
    fn test_insert_remove() {
        let collection = UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        assert!(collection.insert(outpoint.clone(), output.clone()).is_ok());
        assert_eq!(collection.len(), 1);
        assert!(collection.remove(&outpoint).is_ok());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_get() {
        let collection = UtxoCollection::new();
        let outpoint = OutPoint {
            tx_hash: Hash::default(),
            index: 0,
        };
        let output = TxOutput {
            value: 100,
            script_pubkey: vec![],
        };
        collection.insert(outpoint.clone(), output.clone()).unwrap();
        assert_eq!(collection.get(&outpoint), Some(output));
    }
}
