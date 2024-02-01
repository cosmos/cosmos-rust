#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(
    clippy::checked_conversions,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_lifetimes,
    unused_import_braces
)]

//! ## Re-exports
//!
//! CosmRS re-exports the following crates for easy access:
//!
//! - `bip32`: re-exported as `cosmrs::bip32`
//! - `cosmos-sdk-proto`: re-exported as `cosmrs::proto`
//! - `tendermint`: re-exported as `cosmrs::tendermint`
//! - `tendermint-rpc`: re-exported as `cosmrs::rpc` (requires `rpc` crate feature)

pub mod abci;
pub mod auth;
pub mod bank;
pub mod crypto;
pub mod distribution;
pub mod feegrant;
pub mod slashing;
pub mod staking;
pub mod tx;
pub mod vesting;

#[cfg(feature = "cosmwasm")]
pub mod cosmwasm;

#[cfg(feature = "dev")]
pub mod dev;

mod base;
mod error;

pub use crate::{
    base::{query, AccountId, Amount, Coin, Denom, Gas},
    error::{Error, Result},
    tx::Tx,
};

pub use cosmos_sdk_proto::{self as proto, Any};
pub use eyre::Report as ErrorReport;
pub use tendermint;

#[cfg(feature = "bip32")]
pub use bip32;

#[cfg(feature = "rpc")]
pub use tendermint_rpc as rpc;
