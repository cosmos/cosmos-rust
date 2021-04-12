//! Base functionality.

use crate::{proto, Decimal, Error, Result};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use subtle_encoding::bech32;

/// Account identifiers
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct AccountId {
    /// Account ID encoded as Bech32
    bech32: String,

    /// Length of the human-readable prefix of the address
    hrp_length: usize,
}

impl AccountId {
    /// Create an [`AccountId`] with the given human-readable prefix and
    /// public key hash.
    pub fn new(prefix: &str, bytes: [u8; tendermint::account::LENGTH]) -> Result<Self> {
        let id = bech32::encode(prefix, &bytes);

        // TODO(tarcieri): ensure this is the proper validation for an account prefix
        if prefix.chars().all(|c| matches!(c, 'a'..='z')) {
            Ok(Self {
                bech32: id,
                hrp_length: prefix.len(),
            })
        } else {
            Err(Error::AccountId { id }.into())
        }
    }

    /// Get the human-readable prefix of this account.
    pub fn prefix(&self) -> &str {
        &self.bech32[..self.hrp_length]
    }

    /// Decode an account ID from Bech32 to an inner byte value.
    pub fn to_bytes(&self) -> [u8; tendermint::account::LENGTH] {
        bech32::decode(&self.bech32)
            .ok()
            .and_then(|result| result.1.try_into().ok())
            .expect("malformed Bech32 AccountId")
    }
}

impl AsRef<str> for AccountId {
    fn as_ref(&self) -> &str {
        &self.bech32
    }
}

impl fmt::Debug for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountId").field(&self.as_ref()).finish()
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for AccountId {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let (hrp, bytes) = bech32::decode(s)?;

        if bytes.len() == tendermint::account::LENGTH {
            Ok(Self {
                bech32: s.to_owned(),
                hrp_length: hrp.len(),
            })
        } else {
            Err(Error::AccountId { id: s.to_owned() }.into())
        }
    }
}

impl From<AccountId> for tendermint::account::Id {
    fn from(id: AccountId) -> tendermint::account::Id {
        tendermint::account::Id::from(&id)
    }
}

impl From<&AccountId> for tendermint::account::Id {
    fn from(id: &AccountId) -> tendermint::account::Id {
        tendermint::account::Id::new(id.to_bytes())
    }
}

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Decimal,
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Coin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Ok(Coin {
            denom: proto.denom.parse()?,
            amount: proto.amount.parse()?,
        })
    }
}

impl From<Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin::from(&coin)
    }
}

impl From<&Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: &Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin {
            denom: coin.denom.to_string(),
            amount: coin.amount.to_string(),
        }
    }
}

/// Denomination.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Denom(String);

impl AsRef<str> for Denom {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for Denom {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        // TODO(tarcieri): ensure this is the proper validation for a denom name
        if s.chars().all(|c| matches!(c, 'a'..='z')) {
            Ok(Denom(s.to_owned()))
        } else {
            Err(Error::Denom { name: s.to_owned() }.into())
        }
    }
}
