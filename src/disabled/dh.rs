#![forbid(unsafe_code)]

//! Stand-in for [`crate::hazmat::dh`] when the `rustcrypto-dh` family is not
//! compiled in: the API is unchanged and every call reports finite-field
//! Diffie-Hellman as unsupported.

use crate::backend::Operation;
use crate::error::{Error, Result};

/// Always fails: `rustcrypto-dh` is not enabled.
pub fn compute(
    _other_public: &[u8],
    _my_private: &[u8],
    _p: &[u8],
    _q: Option<&[u8]>,
) -> Result<Vec<u8>> {
    Err(Error::unsupported(
        Operation::DhAgreement,
        "finite-field Diffie-Hellman (enable the `rustcrypto-dh` feature)",
    ))
}
