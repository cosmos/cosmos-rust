//! Transaction builder and signer for Cosmos-based blockchains

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk-proto/0.2.0-pre"
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod bank;
pub mod tx;

mod base;
mod builder;
mod decimal;
mod error;
mod prost_ext;
mod public_key;
mod signing_key;

pub use crate::{
    base::{AccountId, Coin, Denom},
    builder::Builder,
    decimal::Decimal,
    error::{Error, Result},
    public_key::PublicKey,
    signing_key::SigningKey,
};

pub use k256::ecdsa::Signature;
pub use tendermint;

#[cfg(feature = "rpc")]
pub use tendermint_rpc as rpc;
