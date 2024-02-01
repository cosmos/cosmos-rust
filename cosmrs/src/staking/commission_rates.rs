use crate::proto;

/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CommissionRates {
    /// rate is the commission rate charged to delegators, as a fraction.
    pub rate: String,

    /// max_rate defines the maximum commission rate which validator can ever charge, as a fraction.
    pub max_rate: String,

    /// max_change_rate defines the maximum daily increase of the validator commission, as a fraction.
    pub max_change_rate: String,
}

impl From<proto::cosmos::staking::v1beta1::CommissionRates> for CommissionRates {
    fn from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::CommissionRates) -> Self {
        CommissionRates {
            rate: proto.rate,
            max_rate: proto.max_rate,
            max_change_rate: proto.max_change_rate,
        }
    }
}

impl From<CommissionRates> for proto::cosmos::staking::v1beta1::CommissionRates {
    fn from(commission_rates: CommissionRates) -> Self {
        proto::cosmos::staking::v1beta1::CommissionRates {
            rate: commission_rates.rate,
            max_rate: commission_rates.max_rate,
            max_change_rate: commission_rates.max_change_rate,
        }
    }
}
