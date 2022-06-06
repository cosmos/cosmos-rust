//! Vesting-related types

use crate::auth::BaseAccount;
use crate::Result;
use crate::{proto, Coin, ErrorReport};

/// BaseVestingAccount implements the VestingAccount interface. It contains all
/// the necessary fields needed for any vesting account implementation.
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

/// ContinuousVestingAccount implements the VestingAccount interface. It
/// continuously vests by unlocking coins linearly with respect to time.
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

/// DelayedVestingAccount implements the VestingAccount interface. It vests all
/// coins after a specific time, but non prior. In other words, it keeps them
/// locked until a specified time.
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

/// Period defines a length of time and amount of coins that will vest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Period {
    /// Length of this vesting period in seconds.
    pub length: i64,

    /// The amount of coins (per denomination) that will vest upon this period finishing.
    pub amount: Vec<Coin>,
}

impl TryFrom<proto::cosmos::vesting::v1beta1::Period> for Period {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::vesting::v1beta1::Period) -> Result<Period> {
        Ok(Period {
            length: proto.length,
            amount: proto
                .amount
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Period> for proto::cosmos::vesting::v1beta1::Period {
    fn from(period: Period) -> Self {
        proto::cosmos::vesting::v1beta1::Period {
            length: period.length,
            amount: period.amount.into_iter().map(Into::into).collect(),
        }
    }
}

/// PeriodicVestingAccount implements the VestingAccount interface. It
/// periodically vests by unlocking coins during each specified period.
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

/// PermanentLockedAccount implements the VestingAccount interface. It does
/// not ever release coins, locking them indefinitely. Coins in this account can
/// still be used for delegating and for governance votes even while locked.
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
