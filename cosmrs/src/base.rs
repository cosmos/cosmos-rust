//! Base functionality.

use crate::{proto, Error, ErrorReport, Result};
use eyre::WrapErr;
use serde::{de, de::Error as _, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};
use subtle_encoding::bech32;

/// Maximum allowed length (in bytes) for an address.
pub const MAX_ADDRESS_LENGTH: usize = 255;

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
    pub fn new(prefix: &str, bytes: &[u8]) -> Result<Self> {
        let id = bech32::encode(prefix, &bytes);

        if !prefix.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9')) {
            return Err(Error::AccountId { id })
                .wrap_err("expected prefix to be lowercase alphanumeric characters only");
        }

        if matches!(bytes.len(), 1..=MAX_ADDRESS_LENGTH) {
            Ok(Self {
                bech32: id,
                hrp_length: prefix.len(),
            })
        } else {
            Err(Error::AccountId { id }).wrap_err_with(|| {
                format!(
                    "account ID should be at most {} bytes long, but was {} bytes long",
                    MAX_ADDRESS_LENGTH,
                    bytes.len()
                )
            })
        }
    }

    /// Get the human-readable prefix of this account.
    pub fn prefix(&self) -> &str {
        &self.bech32[..self.hrp_length]
    }

    /// Decode an account ID from Bech32 to an inner byte value.
    pub fn to_bytes(&self) -> Vec<u8> {
        bech32::decode(&self.bech32)
            .expect("malformed Bech32 AccountId")
            .1
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
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<Self> {
        let (hrp, bytes) = bech32::decode(s).wrap_err(format!("invalid bech32: '{}'", s))?;
        Self::new(&hrp, &bytes)
    }
}

impl TryFrom<AccountId> for tendermint::account::Id {
    type Error = ErrorReport;

    fn try_from(id: AccountId) -> Result<tendermint::account::Id> {
        tendermint::account::Id::try_from(&id)
    }
}

// TODO(tarcieri): non-fixed-width account ID type
impl TryFrom<&AccountId> for tendermint::account::Id {
    type Error = ErrorReport;

    fn try_from(id: &AccountId) -> Result<tendermint::account::Id> {
        let bytes = id.to_bytes();
        let len = bytes.len();

        match bytes.try_into() {
            Ok(bytes) => Ok(tendermint::account::Id::new(bytes)),
            _ => Err(Error::AccountId {
                id: id.bech32.clone(),
            })
            .wrap_err_with(|| format!("invalid length for account ID: {}", len)),
        }
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl Serialize for AccountId {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bech32.serialize(serializer)
    }
}

/// Coin defines a token with a denomination and an amount.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: u128,
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Coin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = ErrorReport;

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

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // See: https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/types/coin.go#L643-L645
        write!(f, "{}{}", self.amount, self.denom)
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
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<Self> {
        // TODO(tarcieri): ensure this is the proper validation for a denom name
        if s.chars()
            .all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '/'))
        {
            Ok(Denom(s.to_owned()))
        } else {
            Err(Error::Denom { name: s.to_owned() }.into())
        }
    }
}

impl<'de> Deserialize<'de> for Denom {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl Serialize for Denom {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{AccountId, Denom};

    #[test]
    fn account_id() {
        "juno1cma4czt2jnydvrvz3lrc9jvcmhpjxtds95s3c6"
            .parse::<AccountId>()
            .unwrap();
    }

    #[test]
    fn account_id_with_digit() {
        "okp41urdh3smlstyafjtyg0d606egllhwp8kvnw0d2f"
            .parse::<AccountId>()
            .unwrap();
    }

    #[test]
    fn denom_from_str() {
        assert!(
            "ibc/9F53D255F5320A4BE124FF20C29D46406E126CE8A09B00CA8D3CFF7905119728"
                .parse::<Denom>()
                .is_ok()
        );
    }
}
