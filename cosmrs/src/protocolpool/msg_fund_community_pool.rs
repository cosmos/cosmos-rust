use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};

/// MsgFundCommunityPool represents a message to send coins from depositor to the community pool.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgFundCommunityPool {
    /// Depositor's address.
    pub depositor: AccountId,

    /// Amount to deposit.
    pub amount: Vec<Coin>,
}

impl Msg for MsgFundCommunityPool {
    type Proto = proto::cosmos::protocolpool::v1::MsgFundCommunityPool;
}

impl TryFrom<proto::cosmos::protocolpool::v1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::protocolpool::v1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        MsgFundCommunityPool::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::protocolpool::v1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::protocolpool::v1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        let mut amounts = Vec::with_capacity(proto.amount.len());
        for amount in &proto.amount {
            amounts.push(Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            })
        }
        Ok(MsgFundCommunityPool {
            depositor: proto.depositor.parse()?,
            amount: amounts,
        })
    }
}

impl From<MsgFundCommunityPool> for proto::cosmos::protocolpool::v1::MsgFundCommunityPool {
    fn from(
        coin: MsgFundCommunityPool,
    ) -> proto::cosmos::protocolpool::v1::MsgFundCommunityPool {
        proto::cosmos::protocolpool::v1::MsgFundCommunityPool::from(&coin)
    }
}

impl From<&MsgFundCommunityPool> for proto::cosmos::protocolpool::v1::MsgFundCommunityPool {
    fn from(
        msg: &MsgFundCommunityPool,
    ) -> proto::cosmos::protocolpool::v1::MsgFundCommunityPool {
        let mut amounts = Vec::with_capacity(msg.amount.len());
        for amount in &msg.amount {
            amounts.push(proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            })
        }
        proto::cosmos::protocolpool::v1::MsgFundCommunityPool {
            depositor: "".to_string(),
            amount: amounts,
        }
    }
}
