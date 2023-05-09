use crate::{proto, tx::Msg, Coin, ErrorReport, Result};
use core::convert::TryFrom;
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;
/// BasicAllowance implements Allowance with a one-time grant of tokens
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of tokens that can be spent
    /// by this allowance and will be updated as tokens are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    pub spend_limit: Vec<Coin>,

    /// expiration specifies an optional time when this allowance expires
    pub expiration: Option<Time>,
}

impl Msg for BasicAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::BasicAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::BasicAllowance> for BasicAllowance {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::feegrant::v1beta1::BasicAllowance) -> Result<BasicAllowance> {
        BasicAllowance::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::feegrant::v1beta1::BasicAllowance> for BasicAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::feegrant::v1beta1::BasicAllowance,
    ) -> Result<BasicAllowance> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = proto
            .clone()
            .expiration
            .ok_or(eyre::eyre!("missing expiration"))?;

        // FIXME: shunts like this are necessary due to
        // https://github.com/informalsystems/tendermint-rs/issues/1053
        let proto_expiration = tpb::Timestamp { seconds, nanos };
        let expiration = Time::try_from(proto_expiration)
            .map_err(|e| eyre::eyre!(format!("invalid timestamp: {e}")))?;

        Ok(BasicAllowance {
            spend_limit: proto
                .spend_limit
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            expiration: Some(expiration),
        })
    }
}

impl From<BasicAllowance> for proto::cosmos::feegrant::v1beta1::BasicAllowance {
    fn from(allowance: BasicAllowance) -> proto::cosmos::feegrant::v1beta1::BasicAllowance {
        proto::cosmos::feegrant::v1beta1::BasicAllowance::from(&allowance)
    }
}

impl From<&BasicAllowance> for proto::cosmos::feegrant::v1beta1::BasicAllowance {
    fn from(allowance: &BasicAllowance) -> proto::cosmos::feegrant::v1beta1::BasicAllowance {
        proto::cosmos::feegrant::v1beta1::BasicAllowance {
            spend_limit: allowance.spend_limit.iter().map(Into::into).collect(),
            expiration: allowance.expiration.map(|v| {
                // FIXME: shunts like this are necessary due to
                // https://github.com/informalsystems/tendermint-rs/issues/1053
                let tpb::Timestamp { seconds, nanos } = v.into();
                let expiration = ibc_proto::google::protobuf::Timestamp { seconds, nanos };
                expiration
            }),
        }
    }
}
