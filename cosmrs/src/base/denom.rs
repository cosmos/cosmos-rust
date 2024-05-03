use crate::{Error, ErrorReport, Result};
use serde::{de, de::Error as _, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

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
            .all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '/' | ':' | '.' | '_' | '-'))
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
