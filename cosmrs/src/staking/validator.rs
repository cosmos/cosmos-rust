use crate::crypto::PublicKey;
use crate::staking::{Commission, Description};
use crate::{proto, AccountId, ErrorReport, Result};
use cosmos_sdk_proto::Timestamp;
use tendermint::Time;

/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
#[derive(Clone, Debug, PartialEq)]
pub struct Validator {
    /// operator_address defines the address of the validator's operator;
    pub operator_address: AccountId,

    /// consensus_pubkey is the consensus public key of the validator.
    pub consensus_pubkey: Option<PublicKey>,

    /// jailed defined whether the validator has been jailed from bonded status or not.
    pub jailed: bool,

    /// status is the validator status (bonded/unbonding/unbonded).
    pub status: i32,

    /// tokens define the delegated tokens (incl. self-delegation).
    pub tokens: String,

    /// delegator_shares defines total shares issued to a validator's delegators.
    pub delegator_shares: String,

    /// description defines the description terms for the validator.
    pub description: Option<Description>,

    /// unbonding_height defines, if unbonding, the height at which this validator has begun unbonding.
    pub unbonding_height: i64,

    /// unbonding_time defines, if unbonding, the min time for the validator to complete unbonding.
    pub unbonding_time: Option<Time>,

    /// commission defines the commission parameters.
    pub commission: Option<Commission>,
    /// min_self_delegation is the validator's self declared minimum self delegation.
    ///
    /// Since: cosmos-sdk 0.46
    pub min_self_delegation: String,
}

impl TryFrom<proto::cosmos::staking::v1beta1::Validator> for Validator {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::Validator) -> Result<Self> {
        Ok(Validator {
            operator_address: proto.operator_address.parse()?,
            consensus_pubkey: proto.consensus_pubkey.map(TryInto::try_into).transpose()?,
            jailed: proto.jailed,
            status: proto.status,
            tokens: proto.tokens,
            delegator_shares: proto.delegator_shares,
            description: proto.description.map(Into::into),
            unbonding_height: proto.unbonding_height,
            unbonding_time: proto
                .unbonding_time
                .map(|jailed_until| {
                    cosmos_sdk_proto::tendermint::google::protobuf::Timestamp {
                        seconds: jailed_until.seconds,
                        nanos: jailed_until.nanos,
                    }
                    .try_into()
                })
                .transpose()?,
            commission: proto.commission.map(TryInto::try_into).transpose()?,
            min_self_delegation: proto.min_self_delegation,
        })
    }
}

impl From<Validator> for proto::cosmos::staking::v1beta1::Validator {
    fn from(validator: Validator) -> Self {
        proto::cosmos::staking::v1beta1::Validator {
            operator_address: validator.operator_address.to_string(),
            consensus_pubkey: validator.consensus_pubkey.map(Into::into),
            jailed: validator.jailed,
            status: validator.status,
            tokens: validator.tokens,
            delegator_shares: validator.delegator_shares,
            description: validator.description.map(Into::into),
            unbonding_height: validator.unbonding_height,
            unbonding_time: validator
                .unbonding_time
                .map(cosmos_sdk_proto::tendermint::google::protobuf::Timestamp::from)
                .map(|t| Timestamp {
                    seconds: t.seconds,
                    nanos: t.nanos,
                }),
            commission: validator.commission.map(Into::into),
            min_self_delegation: validator.min_self_delegation,
        }
    }
}
