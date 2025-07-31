#![allow(dead_code)]

//! # Torus Client
//!
//! A client library for interacting with the Torus blockchain network using subxt.

pub use subxt;
pub use subxt_signer;

pub mod utils;

pub mod chain;
pub mod client;
mod error;
pub use error::*;
pub mod events;
pub mod rpc;

#[allow(clippy::too_many_arguments, dead_code)]
pub(crate) mod interfaces;

// #[cfg(feature = "testnet")]
// mod faucet;
