use crate::{Error, ErrorReport, Result};
use eyre::WrapErr;
use serde::{de, de::Error as _, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};
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
    /// Maximum allowed length (in bytes) of an address.
    pub const MAX_LENGTH: usize = 255;

    /// Create an [`AccountId`] with the given human-readable prefix and
    /// public key hash.
    pub fn new(prefix: &str, bytes: &[u8]) -> Result<Self> {
        let id = bech32::encode(prefix, bytes);

        if !prefix.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9')) {
            return Err(Error::AccountId { id })
                .wrap_err("expected prefix to be lowercase alphanumeric characters only");
        }

        if matches!(bytes.len(), 1..=Self::MAX_LENGTH) {
            Ok(Self {
                bech32: id,
                hrp_length: prefix.len(),
            })
        } else {
            Err(Error::AccountId { id }).wrap_err_with(|| {
                format!(
                    "account ID should be at most {} bytes long, but was {} bytes long",
                    Self::MAX_LENGTH,
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
        let (hrp, bytes) = if s.starts_with(|c: char| c.is_uppercase()) {
            bech32::decode_upper(s)
        } else {
            bech32::decode(s)
        }
        .wrap_err(format!("invalid bech32: '{}'", s))?;
        Self::new(&hrp, &bytes)
    }
}

impl From<AccountId> for String {
    fn from(id: AccountId) -> Self {
        id.bech32
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

#[cfg(test)]
mod tests {
    use super::AccountId;

    #[test]
    fn parse() {
        "juno1cma4czt2jnydvrvz3lrc9jvcmhpjxtds95s3c6"
            .parse::<AccountId>()
            .unwrap();
    }

    #[test]
    fn with_digit() {
        "okp41urdh3smlstyafjtyg0d606egllhwp8kvnw0d2f"
            .parse::<AccountId>()
            .unwrap();
    }

    /// See https://en.bitcoin.it/wiki/BIP_0173 -- UPPERCASE is a valid bech32
    #[test]
    fn with_uppercase() {
        "STARS1JUME25TTJLCAQQJZJJQX9HUMVZE3VCC8QF2KWL"
            .parse::<AccountId>()
            .unwrap();
    }

    #[test]
    fn to_string() {
        let account_id = "juno10j9gpw9t4jsz47qgnkvl5n3zlm2fz72k67rxsg"
            .parse::<AccountId>()
            .unwrap();

        let s: String = account_id.into();
        assert_eq!(s, "juno10j9gpw9t4jsz47qgnkvl5n3zlm2fz72k67rxsg".to_string());
    }
}
