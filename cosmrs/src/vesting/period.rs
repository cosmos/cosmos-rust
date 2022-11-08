use crate::{proto, Coin, ErrorReport, Result};

/// [`Period`] defines a length of time and amount of coins that will vest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Period {
    /// Length of this vesting period in seconds.
    pub length: i64,

    /// The amount of coins (per denomination) that will vest upon this period finishing.
    pub amount: Vec<Coin>,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::Period> for Period {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::vesting::v1beta1::Period) -> Result<Period> {
        Ok(Period {
            length: proto.length,
            amount: proto
                .amount
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Period> for proto::cosmos::vesting::v1beta1::Period {
    fn from(period: Period) -> Self {
        proto::cosmos::vesting::v1beta1::Period {
            length: period.length,
            amount: period.amount.into_iter().map(Into::into).collect(),
        }
    }
}
