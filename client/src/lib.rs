#![allow(dead_code)]

//! # Torus Client
//!
//! A client library for interacting with the Torus blockchain network using subxt.

pub mod utils;

mod chain;
mod client;
pub mod error;
pub(crate) mod macros;
mod pallets;

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

// pub mod wrappers {
//     #[cfg(feature = "mainnet")]
//     pub mod mainnet;
// }

#[cfg(feature = "testnet")]
mod faucet;
