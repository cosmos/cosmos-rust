use super::BasicAllowance;
use crate::{proto, tx::Msg, Coin, ErrorReport, Result};
use core::convert::TryFrom;
use core::time::Duration;
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;
/// PeriodicAllowance extends Allowance to allow for both a maximum cap,
/// as well as a limit per time period.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicAllowance {
    /// basic specifies a struct of `BasicAllowance`
    pub basic: Option<BasicAllowance>,

    /// period specifies the time duration in which period_spend_limit coins can
    /// be spent before that allowance is reset
    pub period: Option<Duration>,

    /// period_spend_limit specifies the maximum number of coins that can be spent
    /// in the period
    pub period_spend_limit: Vec<Coin>,

    /// period_can_spend is the number of coins left to be spent before the period_reset time
    pub period_can_spend: Vec<Coin>,

    /// period_reset is the time at which this period resets and a new one begins,
    /// it is calculated from the start time of the first transaction after the
    /// last period ended
    pub period_reset: Option<Time>,
}

impl Msg for PeriodicAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::PeriodicAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::PeriodicAllowance> for PeriodicAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::feegrant::v1beta1::PeriodicAllowance,
    ) -> Result<PeriodicAllowance> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = proto
            .period_reset
            .ok_or(eyre::eyre!("missing period_reset"))?;

        // FIXME: shunts like this are necessary due to
        // https://github.com/informalsystems/tendermint-rs/issues/1053
        let proto_period_reset = tpb::Timestamp { seconds, nanos };
        let period_reset = Time::try_from(proto_period_reset)
            .map_err(|e| eyre::eyre!(format!("invalid Period reset: {e}")))?;

        Ok(PeriodicAllowance {
            basic: proto.basic.map(TryFrom::try_from).transpose()?,
            period: proto
                .period
                .map(TryFrom::try_from)
                .transpose()
                .map_err(|e| eyre::eyre!(format!("{:?}", e)))?,
            period_spend_limit: proto
                .period_spend_limit
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            period_can_spend: proto
                .period_can_spend
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            period_reset: Some(period_reset),
        })
    }
}

impl From<PeriodicAllowance> for proto::cosmos::feegrant::v1beta1::PeriodicAllowance {
    fn from(allowance: PeriodicAllowance) -> proto::cosmos::feegrant::v1beta1::PeriodicAllowance {
        proto::cosmos::feegrant::v1beta1::PeriodicAllowance {
            basic: allowance.basic.map(Into::into),
            period: allowance
                .period
                .map(TryInto::try_into)
                .transpose()
                .expect("invalid allowance period"), // TODO(tarcieri): fallible serialization?
            period_spend_limit: allowance
                .period_spend_limit
                .iter()
                .map(Into::into)
                .collect(),
            period_can_spend: allowance.period_can_spend.iter().map(Into::into).collect(),
            period_reset: allowance.period_reset.map(|v| {
                // FIXME: shunts like this are necessary due to
                // https://github.com/informalsystems/tendermint-rs/issues/1053
                let tpb::Timestamp { seconds, nanos } = v.into();
                let period_reset = ibc_proto::google::protobuf::Timestamp { seconds, nanos };
                period_reset
            }),
        }
    }
}
