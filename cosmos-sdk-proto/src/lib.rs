#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk-proto/0.8.0"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use tendermint_proto as tendermint;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/COSMOS_SDK_COMMIT");

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos.auth.v1beta1.rs");
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Key-value pairs.
        pub mod kv {
            pub mod v1beta1 {
                include!("prost/cosmos.base.kv.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos.base.reflection.v1beta1.rs");
            }
        }

        /// Snapshots containing Tendermint state sync info.
        pub mod snapshots {
            pub mod v1beta1 {
                include!("prost/cosmos.base.snapshots.v1beta1.rs");
            }
        }

        /// Data structure that holds the state of the application.
        pub mod store {
            pub mod v1beta1 {
                include!("prost/cosmos.base.store.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }

        pub mod tendermint {
            pub mod v1beta1 {
                include!("prost/cosmos.base.tendermint.v1beta1.rs");
            }
        }
    }

    /// Crisis handling
    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos.crisis.v1beta1.rs");
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        /// Multi-signature support.
        pub mod multisig {
            include!("prost/cosmos.crypto.multisig.rs");
            pub mod v1beta1 {
                include!("prost/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
        pub mod ed25519 {
            include!("prost/cosmos.crypto.ed25519.rs");
        }
        pub mod secp256k1 {
            include!("prost/cosmos.crypto.secp256k1.rs");
        }
    }

    /// Messages and services handling token distribution
    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos.distribution.v1beta1.rs");
        }
    }

    /// Messages and services handling evidence
    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos.evidence.v1beta1.rs");
        }
    }

    /// Messages and services handling gentx's
    pub mod genutil {
        pub mod v1beta1 {
            include!("prost/cosmos.genutil.v1beta1.rs");
        }
    }

    /// Messages and services handling governance
    pub mod gov {
        pub mod v1beta1 {
            include!("prost/cosmos.gov.v1beta1.rs");
        }
    }

    /// Messages and services handling minting
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos.mint.v1beta1.rs");
        }
    }

    /// Messages and services handling chain parameters
    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos.params.v1beta1.rs");
        }
    }

    /// Handling slashing parameters and unjailing
    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos.slashing.v1beta1.rs");
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            include!("prost/cosmos.staking.v1beta1.rs");
        }
    }

    /// Transactions.
    pub mod tx {
        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos.tx.v1beta1.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos.upgrade.v1beta1.rs");
        }
    }

    /// Services and tx's for the vesting module.
    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos.vesting.v1beta1.rs");
        }
    }
}

/// CosmWasm protobuf definitions.
#[cfg(feature = "cosmwasm")]
#[cfg_attr(docsrs, doc(cfg(feature = "cosmwasm")))]
pub mod cosmwasm {
    /// Messages and services handling CosmWasm.
    pub mod wasm {
        pub mod v1 {
            include!("prost/cosmwasm.wasm.v1.rs");
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc.applications.transfer.v1.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("prost/ibc.core.channel.v1.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc.core.client.v1.rs");
            }
        }

        /// IBC commitments.
        pub mod commitment {
            pub mod v1 {
                include!("prost/ibc.core.commitment.v1.rs");
            }
        }

        /// IBC connections.
        pub mod connection {
            pub mod v1 {
                include!("prost/ibc.core.connection.v1.rs");
            }
        }

        /// IBC types.
        pub mod types {
            pub mod v1 {
                include!("prost/ibc.core.types.v1.rs");
            }
        }
    }

    /// IBC light clients.
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include!("prost/ibc.lightclients.localhost.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include!("prost/ibc.lightclients.solomachine.v1.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("prost/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}

/// ICS23 protobuf definitions.
pub mod ics23 {
    include!("prost/ics23.rs");
}
