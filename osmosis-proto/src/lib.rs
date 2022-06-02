#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use tendermint_proto as tendermint;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const OSMOSISD_VERSION: &str = include_str!("prost/osmosisd/OSMOSISD_COMMIT");


#[cfg(feature = "cosmos")]
#[cfg_attr(docsrs, doc(cfg(feature = "cosmos")))]
/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/osmosisd/cosmos.auth.v1beta1.rs");
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/osmosisd/cosmos.bank.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/osmosisd/cosmos.base.query.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/osmosisd/cosmos.base.v1beta1.rs");
        }
    }
}

/// Osmosis protobuf definitions.
#[cfg(feature = "osmosis")]
#[cfg_attr(docsrs, doc(cfg(feature = "osmosis")))]
pub mod osmosis {
    /// Messages and services handling CosmWasm.
    pub mod claim {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.claim.v1beta1.rs");
        }
    }

    pub mod epochs {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.epochs.v1beta1.rs");
        }
    }

    pub mod gamm {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.gamm.v1beta1.rs");
        }
    }
    pub mod incentives {
        include!("prost/osmosisd/osmosis.incentives.rs");
    }

    pub mod lockup {
        include!("prost/osmosisd/osmosis.lockup.rs");
    }

    pub mod mint {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.mint.v1beta1.rs");
        }
    }

    pub mod poolincentives {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.poolincentives.v1beta1.rs");
        }
    }

    pub mod store {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.store.v1beta1.rs");
        }
    }

    pub mod superfluid {
        include!("prost/osmosisd/osmosis.superfluid.rs");
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.superfluid.v1beta1.rs");
        }
    }

    pub mod txfees {
        pub mod v1beta1 {
            include!("prost/osmosisd/osmosis.txfees.v1beta1.rs");
        }
    }
}
