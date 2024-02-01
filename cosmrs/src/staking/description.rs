use crate::proto;

/// Description defines a validator description.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Description {
    /// moniker defines a human-readable name for the validator.
    pub moniker: String,

    /// identity defines an optional identity signature (ex. UPort or Keybase).
    pub identity: String,

    /// website defines an optional website link.
    pub website: String,

    /// security_contact defines an optional email for security contact.
    pub security_contact: String,

    /// details define other optional details.
    pub details: String,
}

impl From<proto::cosmos::staking::v1beta1::Description> for Description {
    fn from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::Description) -> Self {
        Description {
            moniker: proto.moniker,
            identity: proto.identity,
            website: proto.website,
            security_contact: proto.security_contact,
            details: proto.details,
        }
    }
}

impl From<Description> for proto::cosmos::staking::v1beta1::Description {
    fn from(description: Description) -> Self {
        proto::cosmos::staking::v1beta1::Description {
            moniker: description.moniker,
            identity: description.identity,
            website: description.website,
            security_contact: description.security_contact,
            details: description.details,
        }
    }
}
