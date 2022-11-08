use crate::{auth::BaseAccount, proto, Coin, ErrorReport, Result};

/// [`BaseVestingAccount`] implements the `VestingAccount` interface.
///
/// It contains all the necessary fields needed for any vesting account implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaseVestingAccount {
    /// [`BaseAccount`] specification of this vesting account.
    pub base_account: Option<BaseAccount>,

    /// The amount of coins (per denomination) that are initially part of a vesting account.
    /// These coins are set at genesis.
    pub original_vesting: Vec<Coin>,

    /// The tracked amount of coins (per denomination) that are delegated from a vesting account
    /// that have been fully vested at time of delegation.
    pub delegated_free: Vec<Coin>,

    /// The tracked amount of coins (per denomination) that are delegated from a vesting account
    /// that were vesting at time of delegation.
    pub delegated_vesting: Vec<Coin>,

    /// The BFT time at which a vesting account is fully vested
    pub end_time: i64,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::BaseVestingAccount> for BaseVestingAccount {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::vesting::v1beta1::BaseVestingAccount,
    ) -> Result<BaseVestingAccount> {
        Ok(BaseVestingAccount {
            base_account: proto.base_account.map(TryFrom::try_from).transpose()?,
            original_vesting: proto
                .original_vesting
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            delegated_free: proto
                .delegated_free
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            delegated_vesting: proto
                .delegated_vesting
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            end_time: proto.end_time,
        })
    }
}

impl From<BaseVestingAccount> for proto::cosmos::vesting::v1beta1::BaseVestingAccount {
    fn from(account: BaseVestingAccount) -> Self {
        proto::cosmos::vesting::v1beta1::BaseVestingAccount {
            base_account: account.base_account.map(Into::into),
            original_vesting: account
                .original_vesting
                .into_iter()
                .map(Into::into)
                .collect(),
            delegated_free: account.delegated_free.into_iter().map(Into::into).collect(),
            delegated_vesting: account
                .delegated_vesting
                .into_iter()
                .map(Into::into)
                .collect(),
            end_time: account.end_time,
        }
    }
}
