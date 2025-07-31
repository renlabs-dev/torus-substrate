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

```
    use torus_client::{Client, Config};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Connect to testnet
        let client = Client::from_url("wss://testnet.torus.dev").await?;

        // Get chain info
        let info = client.chain_info().await?;
        println!("Connected to {} v{}", info.chain, info.version);

        // Subscribe to events
        let mut events = client.events().subscribe().await?;
        while let Some(event) = events.next().await {
            println!("Event: {:?}", event);
        }

        Ok(())
    }
```

## Contributing

See ../CONTRIBUTING.md for development setup and guidelines.

## License

Licensed under ../LICENSE.