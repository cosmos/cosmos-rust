use super::{BaseVestingAccount, Period};
use crate::{proto, ErrorReport, Result};

/// [`PeriodicVestingAccount`] implements the `VestingAccount` interface.
///
/// It periodically vests by unlocking coins during each specified period.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicVestingAccount {
    /// Base vesting account specification required for this vesting implementation.
    pub base_vesting_account: Option<BaseVestingAccount>,

    /// The BFT time at which a vesting account starts to vest.
    pub start_time: i64,

    /// Vesting [`Period`]s associated with this account. Periods are sequential,
    /// in that the duration of a period only starts at the end of the previous period.
    pub vesting_periods: Vec<Period>,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::PeriodicVestingAccount> for PeriodicVestingAccount {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::vesting::v1beta1::PeriodicVestingAccount,
    ) -> Result<PeriodicVestingAccount> {
        Ok(PeriodicVestingAccount {
            base_vesting_account: proto
                .base_vesting_account
                .map(TryFrom::try_from)
                .transpose()?,
            start_time: proto.start_time,
            vesting_periods: proto
                .vesting_periods
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<PeriodicVestingAccount> for proto::cosmos::vesting::v1beta1::PeriodicVestingAccount {
    fn from(account: PeriodicVestingAccount) -> Self {
        proto::cosmos::vesting::v1beta1::PeriodicVestingAccount {
            base_vesting_account: account.base_vesting_account.map(Into::into),
            start_time: account.start_time,
            vesting_periods: account
                .vesting_periods
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
