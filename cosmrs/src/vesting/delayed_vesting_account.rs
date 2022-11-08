use super::BaseVestingAccount;
use crate::{proto, ErrorReport, Result};

/// [`DelayedVestingAccount`] implements the `VestingAccount` interface.
///
/// It vests all coins after a specific time, but non prior. In other words,
/// it keeps them locked until a specified time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelayedVestingAccount {
    /// Base vesting account specification required for this vesting implementation.
    pub base_vesting_account: Option<BaseVestingAccount>,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::DelayedVestingAccount> for DelayedVestingAccount {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::vesting::v1beta1::DelayedVestingAccount,
    ) -> Result<DelayedVestingAccount> {
        Ok(DelayedVestingAccount {
            base_vesting_account: proto
                .base_vesting_account
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<DelayedVestingAccount> for proto::cosmos::vesting::v1beta1::DelayedVestingAccount {
    fn from(account: DelayedVestingAccount) -> Self {
        proto::cosmos::vesting::v1beta1::DelayedVestingAccount {
            base_vesting_account: account.base_vesting_account.map(Into::into),
        }
    }
}
