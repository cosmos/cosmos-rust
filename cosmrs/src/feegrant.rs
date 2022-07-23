//! Fee grant module support
//!
//! <https://docs.cosmos.network/master/modules/feegrant/>

use std::time::SystemTime;

use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};
use prost_types::Any;

/// MsgGrantAllowance adds permission for Grantee to spend up to Allowance
/// of fees from the account of Granter.
#[derive(Clone, Debug, PartialEq)]
pub struct MsgGrantAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    pub granter: AccountId,

    /// grantee is the address of the user being granted an allowance of another user's funds.
    pub grantee: AccountId,

    /// allowance can be any of basic and filtered fee allowance.
    pub allowance: Option<Any>,
}

impl Msg for MsgGrantAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::MsgGrantAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::MsgGrantAllowance> for MsgGrantAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::feegrant::v1beta1::MsgGrantAllowance,
    ) -> Result<MsgGrantAllowance> {
        MsgGrantAllowance::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::feegrant::v1beta1::MsgGrantAllowance> for MsgGrantAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::feegrant::v1beta1::MsgGrantAllowance,
    ) -> Result<MsgGrantAllowance> {
        Ok(MsgGrantAllowance {
            granter: proto.granter.parse()?,
            grantee: proto.grantee.parse()?,
            allowance: proto.allowance.clone(),
        })
    }
}

impl From<MsgGrantAllowance> for proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
    fn from(coin: MsgGrantAllowance) -> proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
        proto::cosmos::feegrant::v1beta1::MsgGrantAllowance::from(&coin)
    }
}

impl From<&MsgGrantAllowance> for proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
    fn from(msg: &MsgGrantAllowance) -> proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
        proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
            granter: msg.granter.to_string(),
            grantee: msg.grantee.to_string(),
            allowance: msg.allowance.clone(),
        }
    }
}

/// MsgRevokeAllowance removes any existing Allowance from Granter to Grantee.
#[derive(Clone, Debug, PartialEq)]
pub struct MsgRevokeAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    pub granter: AccountId,

    /// grantee is the address of the user being granted an allowance of another user's funds.
    pub grantee: AccountId,
}

impl Msg for MsgRevokeAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance> for MsgRevokeAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance,
    ) -> Result<MsgRevokeAllowance> {
        MsgRevokeAllowance::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance> for MsgRevokeAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance,
    ) -> Result<MsgRevokeAllowance> {
        Ok(MsgRevokeAllowance {
            granter: proto.granter.parse()?,
            grantee: proto.grantee.parse()?,
        })
    }
}

impl From<MsgRevokeAllowance> for proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    fn from(allowance: MsgRevokeAllowance) -> proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
        proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance::from(&allowance)
    }
}

impl From<&MsgRevokeAllowance> for proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    fn from(msg: &MsgRevokeAllowance) -> proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
        proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
            granter: msg.granter.to_string(),
            grantee: msg.grantee.to_string(),
        }
    }
}

/// BasicAllowance implements Allowance with a one-time grant of tokens
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[derive(Clone, Debug, PartialEq)]
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

/// AllowedMsgAllowance creates allowance only for specified message types.
#[derive(Clone, Debug, PartialEq)]
pub struct AllowedMsgAllowance {
    /// allowance can be any of basic and filtered fee allowance.
    pub allowance: Option<Any>,

    /// allowed_messages are the messages for which the grantee has the access.
    pub allowed_messages: Vec<String>,
}

impl Msg for AllowedMsgAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance> for AllowedMsgAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance,
    ) -> Result<AllowedMsgAllowance> {
        AllowedMsgAllowance::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance> for AllowedMsgAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance,
    ) -> Result<AllowedMsgAllowance> {
        Ok(AllowedMsgAllowance {
            allowance: proto.allowance.clone(),
            allowed_messages: proto
                .allowed_messages
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<AllowedMsgAllowance> for proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    fn from(
        allowance: AllowedMsgAllowance,
    ) -> proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
        proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance::from(&allowance)
    }
}

impl From<&AllowedMsgAllowance> for proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    fn from(
        allowance: &AllowedMsgAllowance,
    ) -> proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
        proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
            allowance: allowance.allowance.clone().map(Into::into),
            allowed_messages: allowance.allowed_messages.iter().map(Into::into).collect(),
        }
    }
}
