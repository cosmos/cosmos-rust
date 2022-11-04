use crate::{proto, tx::Msg, AccountId, Any, ErrorReport, Result};

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
