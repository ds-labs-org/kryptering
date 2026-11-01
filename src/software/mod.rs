//! Software cryptographic backend using RustCrypto crates.

#[cfg(feature = "rustcrypto-sym")]
pub mod cipher;
#[cfg(not(feature = "rustcrypto-sym"))]
#[path = "../disabled/cipher.rs"]
pub mod cipher;
#[cfg(feature = "post-quantum")]
pub mod composite;
#[cfg(feature = "post-quantum")]
pub mod kem;
pub mod keyagreement;
pub mod keytransport;
#[cfg(feature = "rustcrypto-sym")]
pub mod keywrap;
#[cfg(not(feature = "rustcrypto-sym"))]
#[path = "../disabled/keywrap.rs"]
pub mod keywrap;
pub mod sign;
