# Torus Client

A Rust client library for interacting with the Torus blockchain network using `subxt`.

## Features

- **Automatic Storage Wrapper Generation**: Ergonomic wrapper functions are automatically generated for all Substrate storage items
- **Multi-Network Support**: Support for mainnet, testnet, and development networks
- **Type-Safe Access**: Fully typed access to storage items with automatic type inference
- **Efficient Querying**: Both single-item access and bulk querying supported
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

## Generating Network Interfaces & Storage Wrappers

This crate uses `subxt` to generate Rust code from blockchain metadata and automatically generates ergonomic storage wrapper functions. All generated code is placed in the `src/interfaces/` directory for each network.

### Available Commands

```sh
# Generate interfaces only
just gen_interfaces              # mainnet (default)
just gen_interfaces testnet      # testnet  
just gen_interfaces dev          # local development node

# Generate storage wrappers only
just gen_wrappers               # mainnet (default)
just gen_wrappers testnet       # testnet

# Generate both interfaces and wrappers
just gen_complete               # mainnet (default)
just gen_complete testnet       # testnet

# Generate for all live networks
just gen_for_live              # interfaces only for mainnet + testnet
just gen_complete_live         # interfaces + wrappers for mainnet + testnet
```

### Generated Files Structure

```
src/
├── interfaces/
│   ├── mainnet.rs          # Subxt-generated mainnet API
│   └── testnet.rs          # Subxt-generated testnet API
└── wrappers/
    ├── mainnet.rs          # Generated storage wrappers for mainnet
    └── testnet.rs          # Generated storage wrappers for testnet
```

## Storage Wrapper Generation

This library automatically generates ergonomic wrapper functions for Substrate storage items at build time. The system:

1. **Parses** the subxt-generated API files (`mainnet.rs`, `testnet.rs`)
2. **Identifies** storage patterns (Value, Map, DoubleMap, NMap)
3. **Generates** typed wrapper functions for each storage item
4. **Provides** both individual access and bulk query methods

### Generated Function Patterns

#### Storage Values
```rust
// For: pub type TotalStake<T> = StorageValue<_, Balance>
pub async fn get_total_stake(client: &OnlineClient<PolkadotConfig>) -> Result<Balance, Error>
```

#### Storage Maps
```rust
// For: pub type Agents<T> = StorageMap<_, Identity, AccountId, Agent>
pub async fn get_agents_by_account_id(client: &OnlineClient<PolkadotConfig>, account_id: AccountId) -> Result<Option<Agent>, Error>
pub async fn query_map_agents(client: &OnlineClient<PolkadotConfig>) -> Result<HashMap<AccountId, Agent>, Error>
```

#### Storage Double Maps
```rust
// For: pub type StakedBy<T> = StorageDoubleMap<_, Identity, AccountId, Identity, AccountId, Balance>
pub async fn get_staked_by_by_account_id_by_account_id(
    client: &OnlineClient<PolkadotConfig>, 
    agent: AccountId, 
    staker: AccountId
) -> Result<Option<Balance>, Error>

pub async fn query_map_staked_by(
    client: &OnlineClient<PolkadotConfig>
) -> Result<HashMap<AccountId, HashMap<AccountId, Balance>>, Error>
```

#### Storage N Maps
```rust
// For: StorageNMap with 3+ keys
pub async fn get_storage_by_key1_by_key2_by_key3(
    client: &OnlineClient<PolkadotConfig>,
    key1: Key1Type,
    key2: Key2Type, 
    key3: Key3Type
) -> Result<Option<ValueType>, Error>

pub async fn query_map_storage(
    client: &OnlineClient<PolkadotConfig>
) -> Result<Vec<((Key1Type, Key2Type, Key3Type), ValueType)>, Error>
```

## Usage

### Basic Setup

```rust
use torus_client::{TorusClient, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Torus mainnet
    let client = TorusClient::for_network(Network::Mainnet).await?;
    
    // Use generated wrapper functions
    use torus_client::wrappers::mainnet::*;
    
    // Get total stake (Storage Value)
    let total_stake = get_total_stake(client.client()).await?;
    println!("Total network stake: {}", total_stake);
    
    // Get agent info (Storage Map)
    let account_id = AccountId32::from([0u8; 32]);
    if let Some(agent) = get_agents_by_account_id32(client.client(), account_id).await? {
        println!("Agent: {:?}", agent);
    }
    
    // Query all agents
    let all_agents = query_map_agents(client.client()).await?;
    println!("Total agents: {}", all_agents.len());
    
    Ok(())
}
```

### Network-Specific Features

```rust
// Mainnet wrappers
#[cfg(feature = "mainnet")]
use torus_client::wrappers::mainnet::*;

// Testnet wrappers  
#[cfg(feature = "testnet")]
use torus_client::wrappers::testnet::*;
```

### Examples

The `examples` module provides comprehensive examples for all storage patterns:

```rust
use torus_client::examples::StorageWrapperExamples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run all examples
    StorageWrapperExamples::run_all_examples().await?;
    Ok(())
}
```

## Build System

The wrapper generation happens automatically at build time through `build.rs`:

1. **Parser** (`utils/parser.rs`): Analyzes subxt-generated API files to extract storage patterns
2. **Code Generator** (`utils/codegen.rs`): Generates ergonomic wrapper functions
3. **Build Script** (`build.rs`): Orchestrates the generation process

### Build Dependencies

```toml
[build-dependencies]
syn = { version = "2.0", features = ["full", "parsing"] }
quote = "1.0"
proc-macro2 = "1.0"
thiserror = "1.0"
```

### Regeneration

Wrappers are automatically regenerated when:
- API interface files change (`src/interfaces/mainnet.rs`, `src/interfaces/testnet.rs`)
- Parser or codegen modules change (`src/utils/parser.rs`, `src/utils/codegen.rs`)

Force regeneration:
```bash
cargo clean
cargo build
```

## Features

- `mainnet` (default): Enable mainnet API and wrappers
- `testnet`: Enable testnet API and wrappers  
- `all-networks`: Enable all network support

## Error Handling

All generated functions return `Result<T, Box<dyn std::error::Error>>` for consistent error handling:

```rust
match get_total_stake(&client).await {
    Ok(stake) => println!("Total stake: {}", stake),
    Err(e) => eprintln!("Failed to get total stake: {}", e),
}
```

## Type Safety

The generated wrappers maintain full type safety:
- Parameter types match the storage key types exactly
- Return types match the storage value types
- Optional storage items return `Option<T>`
- Required storage items return `T` with default values

## Performance

- **Individual Access**: Direct storage queries with minimal overhead
- **Bulk Queries**: Efficient iteration over storage maps with key decoding
- **Lazy Loading**: Storage connections are established on-demand
- **Memory Efficient**: Large storage maps are streamed rather than loaded entirely into memory

## License

This project is licensed under the MIT License.
