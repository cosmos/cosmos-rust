use crate::{proto, tx::Msg, Coin, ErrorReport, Result};
use std::time::SystemTime;

/// BasicAllowance implements Allowance with a one-time grant of tokens
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of tokens that can be spent
    /// by this allowance and will be updated as tokens are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    pub spend_limit: Vec<Coin>,

    /// expiration specifies an optional time when this allowance expires
    pub expiration: Option<SystemTime>,
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
        Ok(BasicAllowance {
            spend_limit: proto
                .spend_limit
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            expiration: proto
                .expiration
                .clone()
                .map(TryFrom::try_from)
                .transpose()?,
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
            expiration: allowance.expiration.map(Into::into),
        }
    }
}
