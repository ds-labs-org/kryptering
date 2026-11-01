#![forbid(unsafe_code)]

//! Stand-in for [`crate::hazmat::aes_cbc`] when the `rustcrypto-sym` family is
//! not compiled in: the API is unchanged and every call reports AES-CBC as
//! unsupported.

use crate::algorithm::{AesKeySize, CipherAlgorithm};
use crate::backend::Operation;
use crate::error::{Error, Result};

const DISABLED: &str = "AES-CBC (enable the `rustcrypto-sym` feature)";

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn encrypt(size: AesKeySize, _key: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(
        Operation::Encrypt(CipherAlgorithm::AesCbc(size)),
        DISABLED,
    ))
}

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn decrypt(size: AesKeySize, _key: &[u8], _data: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(
        Operation::Decrypt(CipherAlgorithm::AesCbc(size)),
        DISABLED,
    ))
}
