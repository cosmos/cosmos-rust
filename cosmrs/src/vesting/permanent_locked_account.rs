use super::BaseVestingAccount;
use crate::{proto, ErrorReport, Result};

/// [`PermanentLockedAccount`] implements the `VestingAccount` interface. It does
/// not ever release coins, locking them indefinitely.
///
/// Coins in this account can still be used for delegating and for governance
/// votes even while locked.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermanentLockedAccount {
    /// Base vesting account specification required for this vesting implementation.
    pub base_vesting_account: Option<BaseVestingAccount>,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::PermanentLockedAccount> for PermanentLockedAccount {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::vesting::v1beta1::PermanentLockedAccount,
    ) -> Result<PermanentLockedAccount> {
        Ok(PermanentLockedAccount {
            base_vesting_account: proto
                .base_vesting_account
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<PermanentLockedAccount> for proto::cosmos::vesting::v1beta1::PermanentLockedAccount {
    fn from(account: PermanentLockedAccount) -> Self {
        proto::cosmos::vesting::v1beta1::PermanentLockedAccount {
            base_vesting_account: account.base_vesting_account.map(Into::into),
        }
    }
}
