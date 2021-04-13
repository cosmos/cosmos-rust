//! ECDSA/secp256k1 support

pub mod signing_key;

pub use self::signing_key::SigningKey;
pub use k256::ecdsa::{Signature, VerifyingKey};
