#![forbid(unsafe_code)]

//! Stand-in for [`crate::cipher`] when the `rustcrypto-sym` family is not
//! compiled in: the API is unchanged and every call reports the algorithm as
//! unsupported, exactly as [`crate::Backend::supports`] does.

use crate::algorithm::CipherAlgorithm;
use crate::backend::Operation;
use crate::error::{Error, Result};

const DISABLED: &str = "block ciphers (enable the `rustcrypto-sym` feature)";

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn encrypt(algorithm: CipherAlgorithm, _key: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(Operation::Encrypt(algorithm), DISABLED))
}

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn decrypt(algorithm: CipherAlgorithm, _key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(Operation::Decrypt(algorithm), DISABLED))
}
