//! Base functionality.

mod account_id;
mod coin;
mod denom;

pub mod query;

pub use self::{account_id::AccountId, coin::Coin, denom::Denom};

/// Amounts.
pub type Amount = u128;

/// Gas cost.
pub type Gas = u64;
