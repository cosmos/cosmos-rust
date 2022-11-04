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
    type Proto = proto::cosmos::distribution::v1beta1::MsgFundCommunityPool;
}

impl TryFrom<proto::cosmos::distribution::v1beta1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::distribution::v1beta1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        MsgFundCommunityPool::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::distribution::v1beta1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::distribution::v1beta1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        let mut amounts = vec![];
        for amount in proto.amount.iter() {
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

impl From<MsgFundCommunityPool> for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    fn from(
        coin: MsgFundCommunityPool,
    ) -> proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
        proto::cosmos::distribution::v1beta1::MsgFundCommunityPool::from(&coin)
    }
}

impl From<&MsgFundCommunityPool> for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    fn from(
        msg: &MsgFundCommunityPool,
    ) -> proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
        let mut amounts = vec![];
        for amount in msg.amount.iter() {
            amounts.push(proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            })
        }
        proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
            depositor: "".to_string(),
            amount: amounts,
        }
    }
}
