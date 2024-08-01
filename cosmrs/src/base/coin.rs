use super::{Amount, Denom};
use crate::{proto, ErrorReport, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Amount,
}

impl Coin {
    /// Constructor
    pub fn new(amount: Amount, denom: &str) -> Result<Self> {
        Ok(Coin {
            amount,
            denom: denom.parse()?,
        })
    }
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
        // Support an empty denom when the amount is `0`. See cosmos/cosmos-rust#477
        if proto.denom.is_empty() && proto.amount == "0" {
            Ok(Coin::default())
        } else {
            Ok(Coin {
                denom: proto.denom.parse()?,
                amount: proto.amount.parse()?,
            })
        }
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

#[cfg(test)]
mod tests {
    use super::Coin;
    use crate::proto;

    #[test]
    fn new() {
        Coin::new(1000, "uatom").unwrap();
    }

    #[test]
    fn zero_value_coin_with_empty_denom() {
        let zero_proto = proto::cosmos::base::v1beta1::Coin::from(Coin::default());
        assert_eq!(&zero_proto.denom, "");
        assert_eq!(&zero_proto.amount, "0");

        let zero_coin = Coin::try_from(zero_proto).unwrap();
        assert_eq!(zero_coin, Coin::default())
    }
}
