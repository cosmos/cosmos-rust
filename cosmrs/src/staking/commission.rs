use crate::staking::CommissionRates;
use crate::{proto, ErrorReport, Result};
use cosmos_sdk_proto::Timestamp;
use tendermint::Time;

/// Commission defines commission parameters for a given validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Commission {
    /// commission_rates defines the initial commission rates to be used for creating a validator.
    pub commission_rates: Option<CommissionRates>,

    /// update_time is the last time the commission rate was changed.
    pub update_time: Option<Time>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::Commission> for Commission {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::Commission) -> Result<Self> {
        Ok(Commission {
            commission_rates: proto.commission_rates.map(Into::into),
            update_time: proto
                .update_time
                .map(|timestamp| {
                    Time::from_unix_timestamp(timestamp.seconds, timestamp.nanos as u32)
                        .map_err(|_| crate::ErrorReport::msg("Invalid timestamp"))
                })
                .transpose()?,
        })
    }
}

impl From<Commission> for proto::cosmos::staking::v1beta1::Commission {
    fn from(commission: Commission) -> Self {
        proto::cosmos::staking::v1beta1::Commission {
            commission_rates: commission.commission_rates.map(Into::into),
            update_time: commission.update_time.map(|time| Timestamp {
                seconds: time.unix_timestamp(),
                nanos: time.unix_timestamp_nanos() as i32 % 1_000_000_000,
            }),
        }
    }
}
