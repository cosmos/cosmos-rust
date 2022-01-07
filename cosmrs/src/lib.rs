#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmrs/0.4.0"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod bank;
pub mod crypto;
pub mod distribution;
pub mod staking;
pub mod tx;

#[cfg(feature = "cosmwasm")]
pub mod cosmwasm;

#[cfg(feature = "dev")]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
pub mod dev;

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
pub use eyre::Report as ErrorReport;
pub use prost_types::Any;
pub use tendermint;

#[cfg(feature = "bip32")]
#[cfg_attr(docsrs, doc(cfg(feature = "bip32")))]
pub use bip32;

#[cfg(feature = "rpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "rpc")))]
pub use tendermint_rpc as rpc;
