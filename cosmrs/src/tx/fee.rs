//! Transaction fees

use super::Gas;
use crate::{
    proto::{self, traits::ParseOptional},
    AccountId, Coin, ErrorReport, Result,
};
use serde::{Deserialize, Serialize};

/// Fee includes the amount of coins paid in fees and the maximum gas to be
/// used by the transaction.
///
/// The ratio yields an effective “gasprice”, which must be above some minimum
/// to be accepted into the mempool.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Fee {
    /// Amount of coins to be paid as a fee.
    pub amount: Vec<Coin>,

    /// Maximum gas that can be used in transaction processing before an out
    /// of gas error occurs.
    pub gas_limit: Gas,

    /// Payer: if [`None`], the first signer is responsible for paying the fees.
    ///
    /// If [`Some`], the specified account must pay the fees. The payer must be
    /// a tx signer (and thus have signed this field in AuthInfo).
    ///
    /// Setting this field does not change the ordering of required signers for
    /// the transaction.
    pub payer: Option<AccountId>,

    /// Granter: if [`Some`], the fee payer (either the first signer or the
    /// value of the payer field) requests that a fee grant be used to pay fees
    /// instead of the fee payer’s own balance.
    ///
    /// If an appropriate fee grant does not exist or the chain does not
    /// support fee grants, this will fail.
    pub granter: Option<AccountId>,
}

impl Fee {
    /// Simple constructor for a single [`Coin`] amount and the given amount
    /// of [`Gas`].
    pub fn from_amount_and_gas(amount: Coin, gas_limit: impl Into<Gas>) -> Fee {
        Fee {
            amount: vec![amount],
            gas_limit: gas_limit.into(),
            payer: None,
            granter: None,
        }
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::Fee> for Fee {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::tx::v1beta1::Fee) -> Result<Fee> {
        Fee::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::tx::v1beta1::Fee> for Fee {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::tx::v1beta1::Fee) -> Result<Fee> {
        let amount = proto
            .amount
            .iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Fee {
            amount,
            gas_limit: proto.gas_limit,
            payer: proto.payer.parse_optional()?,
            granter: proto.granter.parse_optional()?,
        })
    }
}

impl From<Fee> for proto::cosmos::tx::v1beta1::Fee {
    fn from(fee: Fee) -> proto::cosmos::tx::v1beta1::Fee {
        proto::cosmos::tx::v1beta1::Fee::from(&fee)
    }
}

impl From<&Fee> for proto::cosmos::tx::v1beta1::Fee {
    fn from(fee: &Fee) -> proto::cosmos::tx::v1beta1::Fee {
        proto::cosmos::tx::v1beta1::Fee {
            amount: fee.amount.iter().map(Into::into).collect(),
            gas_limit: fee.gas_limit,
            payer: fee
                .payer
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_default(),
            granter: fee
                .granter
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_default(),
        }
    }
}
