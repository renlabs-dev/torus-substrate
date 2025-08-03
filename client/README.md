# Torus Client

[![Crates.io](https://img.shields.io/crates/v/torus-client.svg)](https://crates.io/crates/torus-client)
[![Documentation](https://docs.rs/torus-client/badge.svg)](https://docs.rs/torus-client)
[![Build Status](https://github.com/renlabs-dev/torus-substrate/workflows/CI/badge.svg)](https://github.com/renlabs-dev/torus-substrate/actions)

A Rust client library for interacting with the Torus blockchain.

## Overview

The Torus client provides type-safe, ergonomic access to all Torus blockchain functionality. Built with auto-generated interfaces from runtime metadata, it ensures compatibility and type safety.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
torus-client = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Basic Usage

```rs
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Connect to mainnet
        let client = TorusClient::for_mainnet().await?;

        // Or to testnet
        let client = TorusClient::for_testnet().await?;

        // Call extrinsics
        client.torus0().calls().register_agent(...);

        // Fetch storages
        client.governance().storage().agent_applications_get(...);

        // Subscribe to events
        client.events().subscribe::<...>();

        // Call rpcs
        client.rpc().namespace_path_creation_cost(...);

        // Access subxt client
        client.inner_client();

        Ok(())
    }
```

## Contributing

See ../CONTRIBUTING.md for development setup and guidelines.

## License

Licensed under ../LICENSE.
