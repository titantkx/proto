#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos.auth.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.auth.v1beta1.tonic.rs");
        }
    }

    /// Granting of arbitrary privileges from one account to another.
    pub mod authz {
        pub mod v1beta1 {
            include!("prost/cosmos.authz.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.authz.v1beta1.tonic.rs");
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.bank.v1beta1.tonic.rs");
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

    pub mod consensus {
        pub mod v1 {
            include!("prost/cosmos.consensus.v1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.consensus.v1.tonic.rs");
        }
    }

    /// Crisis handling
    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos.crisis.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.crisis.v1beta1.tonic.rs");
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
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.distribution.v1beta1.tonic.rs");
        }
    }

    /// Messages and services handling evidence
    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos.evidence.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.evidence.v1beta1.tonic.rs");
        }
    }

    /// Allows accounts to grant fee allowances and to use fees from their accounts.
    pub mod feegrant {
        pub mod v1beta1 {
            include!("prost/cosmos.feegrant.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.feegrant.v1beta1.tonic.rs");
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
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.gov.v1beta1.tonic.rs");
        }
    }

    /// Messages and services handling minting
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos.mint.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.mint.v1beta1.tonic.rs");
        }
    }

    /// Messages and services handling chain parameters
    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos.params.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.params.v1beta1.tonic.rs");
        }
    }

    /// Handling slashing parameters and unjailing
    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos.slashing.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.slashing.v1beta1.tonic.rs");
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            // WARNING: This file is problematic due to a namespace conflict, see the README for more info
            include!("prost/cosmos.staking.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.staking.v1beta1.tonic.rs");
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
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.tx.v1beta1.tonic.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos.upgrade.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.upgrade.v1beta1.tonic.rs");
        }
    }

    /// Services and tx's for the vesting module.
    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos.vesting.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmos.vesting.v1beta1.tonic.rs");
        }
    }

    pub mod ics23 {
        pub mod v1 {
            include!("prost/cosmos.ics23.v1.rs");
        }
    }
}

/// Tendermint proto definitions
pub mod tendermint {
    pub mod libs {
        pub mod bits {
            include!("prost/tendermint.libs.bits.rs");
        }
    }
    pub mod abci {
        include!("prost/tendermint.abci.rs");
        #[cfg(feature = "grpc")]
        include!("prost/tendermint.abci.tonic.rs");
    }
    pub mod crypto {
        include!("prost/tendermint.crypto.rs");
    }
    pub mod p2p {
        include!("prost/tendermint.p2p.rs");
    }
    pub mod types {
        include!("prost/tendermint.types.rs");
    }
    pub mod version {
        include!("prost/tendermint.version.rs");
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// fee
        pub mod fee {
            pub mod v1 {
                include!("prost/ibc.applications.fee.v1.rs");
                #[cfg(feature = "grpc")]
                include!("prost/ibc.applications.fee.v1.tonic.rs");
            }
        }
        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc.applications.transfer.v1.rs");
            }

            pub mod v2 {
                include!("prost/ibc.applications.transfer.v2.rs");
            }
        }

        /// ICA
        pub mod interchain_accounts {
            pub mod v1 {
                include!("prost/ibc.applications.interchain_accounts.v1.rs");
            }
            pub mod controller {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.controller.v1.rs");
                    #[cfg(feature = "grpc")]
                    include!("prost/ibc.applications.interchain_accounts.controller.v1.tonic.rs");
                }
            }

            pub mod genesis {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.genesis.v1.rs");
                }
            }
            pub mod host {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("prost/ibc.core.channel.v1.rs");
                #[cfg(feature = "grpc")]
                include!("prost/ibc.core.channel.v1.tonic.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc.core.client.v1.rs");
                #[cfg(feature = "grpc")]
                include!("prost/ibc.core.client.v1.tonic.rs");
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
                #[cfg(feature = "grpc")]
                include!("prost/ibc.core.connection.v1.tonic.rs");
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
            pub mod v2 {
                include!("prost/ibc.lightclients.localhost.v2.rs");
            }
        }
        pub mod solomachine {
            pub mod v2 {
                include!("prost/ibc.lightclients.solomachine.v2.rs");
            }

            pub mod v3 {
                include!("prost/ibc.lightclients.solomachine.v3.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("prost/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}
