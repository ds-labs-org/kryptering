#![forbid(unsafe_code)]

//! Stand-in for [`crate::keywrap`] when the `rustcrypto-sym` family is not
//! compiled in: the API is unchanged and every call reports the algorithm as
//! unsupported.

use crate::algorithm::KeyWrapAlgorithm;
use crate::backend::Operation;
use crate::error::{Error, Result};

const DISABLED: &str = "key wrapping (enable the `rustcrypto-sym` feature)";

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn wrap(algorithm: KeyWrapAlgorithm, _kek: &[u8], _key_data: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(Operation::Wrap(algorithm), DISABLED))
}

/// Always fails: `rustcrypto-sym` is not enabled.
pub fn unwrap(algorithm: KeyWrapAlgorithm, _kek: &[u8], _wrapped: &[u8]) -> Result<Vec<u8>> {
    Err(Error::unsupported(Operation::Unwrap(algorithm), DISABLED))
}
