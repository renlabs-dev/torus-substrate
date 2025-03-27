# Torus Client

A Rust client library for interacting with the Torus blockchain network using `subxt`.

## Features

- Connect to Torus Mainnet, Testnet, or Development nodes
- Type-safe interaction with the Torus runtime
- Network-specific metadata and interfaces
- Submit transactions
- Query blockchain state
- Subscribe to events
- Unified API across different networks

## Setup

### Prerequisites

- [Rust](https://rustup.rs/)
- [Nix](https://nixos.org/) (optional, for development environment)
- [just](https://github.com/casey/just) command runner

### Development Environment

The repository contains a Nix flake that provides a complete development environment:

```sh
nix develop
```

## Generating Network Interfaces

This crate uses `subxt` to generate Rust code from blockchain metadata. Generated code enables type-safe interaction with the Torus runtime.

### Available Commands

```sh
# Generate interfaces for mainnet (default)
just gen_interfaces

# Generate interfaces for testnet
just gen_interfaces testnet

# Generate interfaces for local development node
just gen_interfaces dev

# Generate interfaces for all live networks (mainnet and testnet)
just gen_for_live
```

## Usage

```rust
use torus_client::{TorusClient, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to mainnet
    let client = TorusClient::for_network(Network::Mainnet).await?;

    // Get the latest finalized block hash
    let block_hash = client.client().rpc().finalized_head().await?;
    println!("Latest finalized block: {:?}", block_hash);

    Ok(())
}
```

## Create features

- `mainnet` (enabled by default): Include mainnet-specific interfaces
- `testnet`: Include testnet-specific interfaces
- `dev`: Include development node interfaces
- `all-networks`: Include interfaces for all networks

## License

This project is licensed under the MIT License.
