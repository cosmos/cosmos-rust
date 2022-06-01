//! Error types

pub use eyre::{Report, Result};

use crate::tx;
use thiserror::Error;

/// Kinds of errors.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    /// Invalid account.
    #[error("invalid account ID: {id:?}")]
    AccountId {
        /// Malformed account ID
        id: String,
    },

    /// Cryptographic errors.
    #[error("cryptographic error")]
    Crypto,

    /// Invalid decimal value.
    #[error("invalid decimal value: {value:?}")]
    Decimal {
        /// Invalid decimal value
        value: String,
    },

    /// Invalid denomination.
    #[error("invalid denomination: {name:?}")]
    Denom {
        /// Invalid name
        name: String,
    },

    /// Protobuf is missing a field.
    #[error("missing proto field: {name:?}")]
    MissingField {
        /// Name of the missing field
        name: &'static str,
    },

    /// Unexpected message type.
    #[error("unexpected Msg type: {found:?}, expected {expected:?}")]
    MsgType {
        /// Expected type URL.
        expected: &'static str,

        /// Actual type URL found in the [`prost_types::Any`] message.
        found: String,
    },

    /// Transaction not found.
    #[error("transaction not found: {hash:?}")]
    TxNotFound {
        /// Transaction hash that wasn't found.
        hash: tx::Hash,
    },

    /// Invalid value for the given field of an enum.
    #[error("invalid proto enum value: {name:?}, value: {found_value:?}")]
    InvalidEnumValue {
        /// Name of the enum field
        name: &'static str,

        /// Actual value of the field found
        found_value: i32,
    },
}

impl Eq for Error {}
