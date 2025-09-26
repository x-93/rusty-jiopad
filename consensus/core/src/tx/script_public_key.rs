//! Script public key for transaction outputs.

use crate::{hashing, Hash, errors::ConsensusResult};

/// Script public key types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptPublicKeyType {
    /// Pay to public key hash.
    PayToPubkeyHash,
    /// Pay to script hash.
    PayToScriptHash,
    /// Pay to public key.
    PayToPubkey,
    /// Unknown script type.
    Unknown,
}

/// Script public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptPublicKey {
    pub script: Vec<u8>,
    pub version: u16,
}

impl ScriptPublicKey {
    /// Creates a new script public key.
    pub fn new(script: Vec<u8>, version: u16) -> Self {
        Self { script, version }
    }

    /// Creates a pay-to-pubkey-hash script.
    pub fn pay_to_pubkey_hash(pubkey_hash: &Hash) -> Self {
        let mut script = vec![0x76, 0xa9, 0x20]; // OP_DUP OP_HASH160 OP_PUSHBYTES_32
        script.extend_from_slice(pubkey_hash.as_bytes());
        script.extend_from_slice(&[0x88, 0xac]); // OP_EQUALVERIFY OP_CHECKSIG
        Self::new(script, 0)
    }

    /// Gets the script type.
    pub fn script_type(&self) -> ScriptPublicKeyType {
        if self.is_pay_to_pubkey_hash() {
            ScriptPublicKeyType::PayToPubkeyHash
        } else if self.is_pay_to_script_hash() {
            ScriptPublicKeyType::PayToScriptHash
        } else if self.is_pay_to_pubkey() {
            ScriptPublicKeyType::PayToPubkey
        } else {
            ScriptPublicKeyType::Unknown
        }
    }

    /// Checks if it's a pay-to-pubkey-hash script.
    pub fn is_pay_to_pubkey_hash(&self) -> bool {
        self.script.len() == 37 &&
        self.script[0] == 0x76 && // OP_DUP
        self.script[1] == 0xa9 && // OP_HASH160
        self.script[2] == 0x20 && // OP_PUSHBYTES_32
        self.script[35] == 0x88 && // OP_EQUALVERIFY
        self.script[36] == 0xac    // OP_CHECKSIG
    }

    /// Checks if it's a pay-to-script-hash script.
    pub fn is_pay_to_script_hash(&self) -> bool {
        self.script.len() == 23 &&
        self.script[0] == 0xa9 && // OP_HASH160
        self.script[1] == 0x14 && // OP_PUSHBYTES_20
        self.script[22] == 0x87    // OP_EQUAL
    }

    /// Checks if it's a pay-to-pubkey script.
    pub fn is_pay_to_pubkey(&self) -> bool {
        (self.script.len() == 35 || self.script.len() == 67) &&
        (self.script.last() == Some(&0xac)) // OP_CHECKSIG
    }

    /// Extracts the pubkey hash from a P2PKH script.
    pub fn pubkey_hash(&self) -> Option<Hash> {
        if self.is_pay_to_pubkey_hash() {
            Some(Hash::from_slice(&self.script[3..35]))
        } else {
            None
        }
    }

    /// Validates the script (basic checks).
    pub fn validate(&self) -> ConsensusResult<()> {
        if self.script.is_empty() {
            return Err(crate::errors::ConsensusError::ScriptValidation {
                msg: "Script public key is empty".to_string(),
            });
        }
        // Additional validation logic can be added
        Ok(())
    }

    /// Computes the script hash.
    pub fn script_hash(&self) -> Hash {
        hashing::hash_script(&self.script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pay_to_pubkey_hash() {
        let hash = Hash::from_le_u64([1, 0, 0, 0]);
        let script = ScriptPublicKey::pay_to_pubkey_hash(&hash);
        assert!(script.is_pay_to_pubkey_hash());
        assert_eq!(script.pubkey_hash(), Some(hash));
    }

    #[test]
    fn test_script_type() {
        let hash = Hash::from_le_u64([1, 0, 0, 0]);
        let script = ScriptPublicKey::pay_to_pubkey_hash(&hash);
        assert_eq!(script.script_type(), ScriptPublicKeyType::PayToPubkeyHash);
    }

    #[test]
    fn test_validate_empty_script() {
        let script = ScriptPublicKey::new(vec![], 0);
        assert!(script.validate().is_err());
    }
}
