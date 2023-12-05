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
                .map(|jailed_until| {
                    cosmos_sdk_proto::tendermint::google::protobuf::Timestamp {
                        seconds: jailed_until.seconds,
                        nanos: jailed_until.nanos,
                    }
                    .try_into()
                })
                .transpose()?,
        })
    }
}

impl From<Commission> for proto::cosmos::staking::v1beta1::Commission {
    fn from(commission: Commission) -> Self {
        proto::cosmos::staking::v1beta1::Commission {
            commission_rates: commission.commission_rates.map(Into::into),
            update_time: commission
                .update_time
                .map(cosmos_sdk_proto::tendermint::google::protobuf::Timestamp::from)
                .map(|t| Timestamp {
                    seconds: t.seconds,
                    nanos: t.nanos,
                }),
        }
    }
}
