use cosmos_sdk_proto::cosmos::protocolpool::v1::Params;

use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// WithdrawValidatorCommission submits a proposal to update protocolpool module params. Note: the entire params must be provided.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUpdateParams {
    pub authority: String,
    pub params: Option<Params>,
}

impl Msg for MsgUpdateParams {
    type Proto = proto::cosmos::protocolpool::v1::MsgUpdateParams;
}

impl TryFrom<proto::cosmos::protocolpool::v1::MsgUpdateParams> for MsgUpdateParams {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::protocolpool::v1::MsgUpdateParams,
    ) -> Result<MsgUpdateParams> {
        MsgUpdateParams::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::protocolpool::v1::MsgUpdateParams> for MsgUpdateParams {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::protocolpool::v1::MsgUpdateParams,
    ) -> Result<MsgUpdateParams> {
        Ok(MsgUpdateParams {
            authority: proto.authority.parse()?,
            params: proto.params.parse()?,
        })
    }
}

impl From<MsgUpdateParams> for proto::cosmos::protocolpool::v1::MsgUpdateParams {
    fn from(coin: MsgUpdateParams) -> proto::cosmos::protocolpool::v1::MsgUpdateParams {
        proto::cosmos::protocolpool::v1::MsgUpdateParams::from(&coin)
    }
}

impl From<&MsgUpdateParams> for proto::cosmos::protocolpool::v1::MsgUpdateParams {
    fn from(msg: &MsgUpdateParams) -> proto::cosmos::protocolpool::v1::MsgUpdateParams {
        proto::cosmos::protocolpool::v1::MsgUpdateParams {
            authority: msg.authority.to_string(),
            params: msg.params,
        }
    }
}
