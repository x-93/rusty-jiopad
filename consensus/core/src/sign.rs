//! Signature utilities.

use crate::errors::ConsensusResult;

/// Signs data with a private key (placeholder).
pub fn sign_data(_data: &[u8], _private_key: &[u8]) -> Vec<u8> {
    // Placeholder: return dummy signature
    vec![0; 64]
}

/// Verifies a signature (placeholder).
pub fn verify_signature(_data: &[u8], signature: &[u8], _public_key: &[u8]) -> ConsensusResult<()> {
    if signature.len() != 64 {
        return Err(crate::errors::ConsensusError::InvalidSignature);
    }
    // Placeholder: always valid
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_data() {
        let sig = sign_data(b"test", &[0; 32]);
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_verify_signature_valid() {
        let sig = sign_data(b"test", &[0; 32]);
        assert!(verify_signature(b"test", &sig, &[0; 33]).is_ok());
    }

    #[test]
    fn test_verify_signature_invalid() {
        assert!(verify_signature(b"test", &[0; 32], &[0; 33]).is_err());
    }
}
