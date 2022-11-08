use super::BaseVestingAccount;
use crate::{proto, ErrorReport, Result};

/// [`ContinuousVestingAccount`] implements the `VestingAccount` interface.
///
/// It continuously vests by unlocking coins linearly with respect to time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuousVestingAccount {
    /// Base vesting account specification required for this vesting implementation.
    pub base_vesting_account: Option<BaseVestingAccount>,

    /// The BFT time at which a vesting account starts to vest.
    pub start_time: i64,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::ContinuousVestingAccount>
    for ContinuousVestingAccount
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::vesting::v1beta1::ContinuousVestingAccount,
    ) -> Result<ContinuousVestingAccount> {
        Ok(ContinuousVestingAccount {
            base_vesting_account: proto
                .base_vesting_account
                .map(TryFrom::try_from)
                .transpose()?,
            start_time: proto.start_time,
        })
    }
}

impl From<ContinuousVestingAccount> for proto::cosmos::vesting::v1beta1::ContinuousVestingAccount {
    fn from(account: ContinuousVestingAccount) -> Self {
        proto::cosmos::vesting::v1beta1::ContinuousVestingAccount {
            base_vesting_account: account.base_vesting_account.map(Into::into),
            start_time: 0,
        }
    }
}
