use crate::{Error, ErrorReport, Result};
use serde::{de, de::Error as _, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Denomination.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Denom(String);

impl Denom {
    /// Minimum length of a [`Denom`].
    pub const MIN_LENGTH: usize = 3;

    /// Maximum length of a [`Denom`].
    pub const MAX_LENGTH: usize = 128;
}

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

    /// NOTE: implements the same checks as the `MatchDenom` function from the upstream Cosmos SDK.
    /// <https://github.com/cosmos/cosmos-sdk/blob/6a07568/types/coin.go#L885-L906>
    fn from_str(s: &str) -> Result<Self> {
        if s.len() < Self::MIN_LENGTH || s.len() > Self::MAX_LENGTH {
            return Err(Error::Denom { name: s.to_owned() }.into());
        }

        if !s.chars().all(is_valid_denom_char) {
            return Err(Error::Denom { name: s.to_owned() }.into());
        }

        Ok(Denom(s.to_owned()))
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

/// Check if a given character is allowed in a `Denom` name.
///
/// NOTE: implements the same checks as the `isValidRune` function from the upstream Cosmos SDK.
/// <https://github.com/cosmos/cosmos-sdk/blob/6a07568/types/coin.go#L879-L883>
#[inline]
fn is_valid_denom_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '/' | ':' | '.' | '_' | '-')
}

#[cfg(test)]
mod tests {
    use super::Denom;

    #[test]
    fn parse() {
        assert!(
            "ibc/9F53D255F5320A4BE124FF20C29D46406E126CE8A09B00CA8D3CFF7905119728"
                .parse::<Denom>()
                .is_ok()
        );
    }
}
