//! Decimal type with equivalent semantics to the [Cosmos `sdk.Dec`][1] type.
//!
//! [1]: https://pkg.go.dev/github.com/cosmos/cosmos-sdk/types#Dec

use crate::{ErrorReport, Result};
use std::{
    fmt,
    ops::{Add, AddAssign},
    str::FromStr,
};

/// Decimal type which follows Cosmos [Cosmos `sdk.Dec`][1] conventions.
///
/// [1]: https://pkg.go.dev/github.com/cosmos/cosmos-sdk/types#Dec
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Decimal(u64);

impl FromStr for Decimal {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<Self> {
        Ok(s.parse().map(Self)?)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    #[inline]
    fn add(self, rhs: Decimal) -> Decimal {
        Decimal(self.0 + rhs.0)
    }
}

impl AddAssign for Decimal {
    #[inline]
    fn add_assign(&mut self, rhs: Decimal) {
        self.0 += rhs.0;
    }
}

macro_rules! impl_from_primitive_int_for_decimal {
    ($($int:ty),+) => {
        $(impl From<$int> for Decimal {
            fn from(num: $int) -> Decimal {
                #[allow(trivial_numeric_casts)]
                Decimal(num.into())
            }
        })+
    };
}

impl_from_primitive_int_for_decimal!(u8, u16, u32, u64);
