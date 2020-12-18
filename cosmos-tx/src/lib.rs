//! Transaction builder and signer for Cosmos-based blockchains

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk-proto/0.1.2"
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod builder;
pub mod decimal;
pub mod error;
pub mod msg;

pub use crate::{builder::Builder, decimal::Decimal, error::Error};
pub use k256::ecdsa::{Signature, VerifyingKey};

/// Transaction signer for ECDSA/secp256k1 signatures
pub type Signer = dyn ecdsa::signature::Signer<Signature>;
