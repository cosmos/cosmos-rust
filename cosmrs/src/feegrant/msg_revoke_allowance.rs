use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgRevokeAllowance removes any existing Allowance from Granter to Grantee.
#[derive(Clone, Debug, Eq, PartialEq)]
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
