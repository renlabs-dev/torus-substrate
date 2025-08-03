#![allow(dead_code)]

//! # Torus Client
//!
//! A client library for interacting with the Torus blockchain network using subxt.
//!
//! ## Overview
//!
//! The Torus client provides type-safe, ergonomic access to all Torus blockchain functionality. Built with auto-generated interfaces from runtime metadata, it ensures compatibility and type safety.
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! torus-client = "0.1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! Basic Usage
//!
//! ```rs
//!     #[tokio::main]
//!     async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!         // Connect to mainnet
//!         let client = TorusClient::for_mainnet().await?;
//!
//!         // Or to testnet
//!         let client = TorusClient::for_testnet().await?;
//!
//!         // Call extrinsics
//!         client.torus0().calls().register_agent(...);
//!
//!         // Fetch storages
//!         client.governance().storage().agent_applications_get(...);
//!
//!         // Subscribe to events
//!         client.events().subscribe::<...>();
//!
//!         // Call rpcs
//!         client.rpc().namespace_path_creation_cost(...);
//!
//!         // Access subxt client
//!         client.inner_client();
//!
//!         Ok(())
//!     }
//! ```

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
pub mod interfaces;
