//! Error types

pub use eyre::{Report, Result};

use thiserror::Error;

/// Kinds of errors
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// Invalid decimal value
    #[error("invalid decimal value")]
    Decimal,
}
