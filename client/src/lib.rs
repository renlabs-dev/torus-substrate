#![allow(dead_code)]

//! # Torus Client
//!
//! A client library for interacting with the Torus blockchain network using subxt.

pub mod utils;

mod chain;
pub mod client;
mod error;
pub use error::*;

#[allow(clippy::too_many_arguments, dead_code)]
pub mod wrappers;

// Generated interfaces for different networks
pub(crate) mod interfaces {
    // These modules will be populated by the `just gen_interfaces` command
    #[cfg(feature = "mainnet")]
    pub mod mainnet;

    #[cfg(feature = "testnet")]
    pub mod testnet;

    // #[cfg(feature = "dev")]
    // pub mod dev;
}

#[cfg(feature = "testnet")]
mod faucet;
