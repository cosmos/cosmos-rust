//! # Cosmos SDK for Rust
//!
//! Framework for building [Cosmos] blockchain applications in Rust, modeled off
//! of the [Cosmos SDK for Golang].
//!
//! ## Features
//!
//! - [Transactions][`tx`]: build, sign, and/or parse Cosmos SDK transactions
//!
//! [Cosmos]: https://cosmos.network/
//! [Cosmos SDK for Golang]: https://github.com/cosmos/cosmos-sdk

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk-rs/0.1.0-pre"
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod bank;
pub mod crypto;
pub mod tx;

mod base;
mod decimal;
mod error;
mod prost_ext;

pub use crate::{
    base::{AccountId, Coin, Denom},
    decimal::Decimal,
    error::{Error, Result},
    tx::Tx,
};

pub use cosmos_sdk_proto as proto;
pub use tendermint;

#[cfg(feature = "rpc")]
pub use tendermint_rpc as rpc;
