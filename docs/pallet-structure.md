# Torus Pallet Structure

This guide explains how our pallets are organized and why we've set them up this way. Understanding this structure will help you navigate and contribute to the codebase more effectively.

## Core Pallets

Torus has three main pallets, each with their own specialized role. The "0" suffix in the pallet names (`torus0`, `emission0`) indicates that these implementations are early versions that will undergo significant changes for the v1 release.

The `torus0` pallet serves as the foundation of our network, handling agent registration, staking, and the core mechanics of the protocol. It manages the lifecycle of agents, from registration to pruning, and maintains the staking relationships between users and validators.

The `emission0` pallet handles our token distribution system, ensuring that everyone gets their fair share of tokens based on their contributions. It implements a sophisticated linear distribution algorithm that calculates incentives for miners and dividends for validators using matrix operations. The algorithm accumulates pending emissions over time and distributes them at regular intervals based on validator weights and stakes. For a detailed explanation of how this distribution works, refer to the [linear-emission.md](linear-emission.md) document.

The `governance` pallet provides the democracy layer, allowing the community to propose changes, vote, and manage the network's evolution. It processes proposals for network parameter changes, counts votes, implements approved changes, and manages special roles like allocators and curators.

These three pallets work together to create a functional blockchain system.

## Pallet Directory Structure

Each pallet follows a consistent structure that promotes organization and maintainability:

```
pallets/pallet_name/
├── Cargo.toml                   # Dependency management
├── api/                         # External interface
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs               # API trait definition
├── src/                         # Main pallet code
│   ├── lib.rs                   # Pallet definition
│   ├── ext.rs                   # Helper utilities
│   ├── migrations.rs            # Storage migrations
│   ├── weights.rs               # Transaction weights
│   └── domain_modules.rs        # Business logic modules
└── tests/                       # Integration tests
    └── module_tests.rs          # Tests by domain
```

This structure separates concerns and makes it easier to navigate and understand the codebase. The `api` directory contains the public interface, the `src` directory holds the implementation, and the `tests` directory contains integration tests for the pallet.

## API-Based Design

We use a separation between API definitions and implementations, with each pallet having its own `api` crate that defines a trait for other pallets to interact with it. This approach is necessary because Rust crates need to depend on each other, and having separate API crates prevents circular dependencies. It also makes it much simpler to create mock implementations for testing.

For example, the `Emission0Api` trait defines methods like `consensus_stats` and `delegate_weight_control` that other pallets can use to interact with the `emission0` pallet. The `torus0` pallet implements its own API, which `emission0` uses to get staking information and agent details.

This separation creates clear boundaries between pallets, allowing them to evolve independently as long as they maintain their API contracts. The implementation details can change drastically between versions, but as long as the API remains stable, other pallets won't be affected.

## Inside a Pallet

Each pallet is structured around the Substrate pallet framework, with a few Torus-specific conventions that align with our coding guidelines outlined in [CONTRIBUTING.md](CONTRIBUTING.md).

The main pallet definition lives in `lib.rs`, where we define the `Config` trait for runtime configuration, storage items for on-chain state, extrinsic calls that users can execute, events for notifications, and errors to handle failure cases.

### Documentation and Safety

All functions, types, storage items, extrinsics, and errors must be thoroughly documented, as this documentation is exported for other language-based clients like TypeScript and Python. Clear documentation ensures that developers working with different languages can understand and use our blockchain correctly.

We strictly avoid any operations that can cause panics in runtime code. The WASM runtime that Substrate runs on cannot unwind, so panics in functions like `on_initialize` will halt the chain. This means:

- We never use `unwrap()`, `expect()`, or other panic-causing operations in pallet code
- We always use checked arithmetic operations (`checked_add`, `saturating_sub`, etc.) instead of operators that can panic (`+`, `-`, etc.)
- We properly handle all `Option` and `Result` types, and never ignore errors
- We use `ensure!` macros for validation instead of `assert!` which can panic

### Numeric Types and Balance Operations

For balance operations and fixed-point arithmetic, we use `FixedU128` instead of raw `u128` values. This type provides methods for safe arithmetic operations and better handles the decimal precision needed for operations like percentages and ratios.

```rust
// Instead of:
let result = (amount * percentage) / 100;

// We use:
let percentage = FixedU128::from_rational(percentage_value, 100);
let result = percentage.mul_floor(amount);
```

This approach prevents overflows and ensures consistent handling of decimal precision across the codebase.

### Code Organization

Our extrinsic calls always use manual call indexing with `#[pallet::call_index(n)]` to ensure backward compatibility during updates. This approach prevents issues that can arise from automatic indexing when adding or removing calls.

Instead of having large implementations directly inside the `lib.rs` file, we break down complex logic into domain-specific modules. For example, in the `torus0` pallet, `agent.rs` handles agent registration and management, `stake.rs` manages staking mechanics and rewards, and `fee.rs` controls fee configuration and processing.

These modules typically implement their functionality using generic functions that take `Config` as a type parameter, rather than implementing methods directly on the `Pallet` struct. This approach improves code organization and testability.

```rust
// Instead of:
impl<T: Config> Pallet<T> {
    pub fn some_function(...) { ... }
}

// We prefer:
pub fn some_function<T: Config>(...) { ... }
```

## Storage Efficiency and Container Types

We are careful about storage usage since every storage item adds to the blockchain's state size. Instead of having multiple storage values for related data, we prefer using container types to group related values together. For example, instead of having separate storage items for different burn-related values, we might define a `BurnConfiguration` struct that contains all burn parameters.

```rust
pub struct BurnConfiguration {
    pub burn_amount: Balance,
    pub target_registrations: u16,
    pub target_registrations_interval: u16,
    pub burn_adjust_factor: Percent,
}
```

This approach reduces the number of storage reads and writes, minimizing the blockchain's state bloat. However, we sometimes need to index "hot" values that are frequently accessed, such as staking relationships. In these cases, we might use separate storage maps for performance reasons, even if it means some data duplication.

When updating structured data, we follow patterns that ensure data consistency and validation. For instance, when updating agent data, we retrieve the full `Agent` struct, validate the changes, and then store it back, rather than updating individual fields directly. This approach encapsulates the update logic and ensures all validation rules are applied consistently.

```rust
// Update an agent's data with proper validation
fn update_agent_metadata<T: Config>(
    agent_id: &T::AccountId,
    new_metadata: Vec<u8>,
) -> DispatchResult {
    Agents::<T>::try_mutate(agent_id, |maybe_agent| {
        let agent = maybe_agent.as_mut().ok_or(Error::<T>::AgentNotFound)?;
        ensure!(new_metadata.len() <= MaxMetadataLength::<T>::get() as usize, Error::<T>::MetadataTooLong);
        agent.metadata = new_metadata;
        Ok(())
    })
}
```

We take input validation very seriously, and domain functions should only be updatable through private functions that correctly verify the data. This prevents invalid data from entering the blockchain state and causing issues down the line.

## Storage and Migrations

Blockchains require careful handling of storage changes since data persists across runtime upgrades. Each pallet tracks its storage version and includes a `migrations.rs` file to handle transitions between versions.

We use the `VersionedMigration` pattern from `frame_support` for all our migrations, which ensures that migrations run exactly once and in the correct order. Each migration is implemented as a separate module defining the version transition:

```rust
pub mod v2 {
    use super::*;
    use crate::{Agent, Agents};
    
    // Migration from version 1 to version 2
    pub type Migration<T, W> = VersionedMigration<1, 2, MigrateToV2<T>, Pallet<T>, W>;
    
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);
    
    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            // Actual migration logic
            Agents::<T>::translate(|_key, mut agent: Agent<T>| {
                agent.fees = Default::default();
                Some(agent)
            });
            
            Weight::zero()
        }
    }
}
```

When modifying storage structures (creating/deleting storages, changing names, switching types, altering structs, reordering enum variants), we create migration code that converts data from the old format to the new one. This approach allows us to evolve the codebase without requiring a chain restart or losing user data.

For example, in the `torus0` pallet, when we added the `fees` field to the `Agent` struct, we created a migration that reads each agent, adds the new field with a default value, and writes it back to storage.

These migrations are registered in the runtime and applied automatically during runtime upgrades. We carefully increment the storage version (checking git history to determine if it has already been incremented) and ensure proper migration paths between versions.

## The Torus0 Pallet in Detail

The `torus0` pallet manages agent registrations, stake, and the fundamental network participation rules. It maintains a registry of network agents, each with a name, URL, and metadata. Agents can be registered by anyone but must meet certain criteria, such as passing the burn requirement if the network is at capacity.

The pallet also handles staking relationships, allowing users to stake tokens on validators to show their support. These stakes influence the validator's standing in the network and are used by the `emission0` pallet to calculate rewards.

Network constraints are enforced, such as maximum agent count and minimum validator stake. When the network reaches capacity, the pallet implements a pruning mechanism that removes the least valuable agents to make room for new ones.

## The Emission0 Pallet in Detail

The `emission0` pallet implements token distribution based on network participation. It accumulates pending emissions over time, adding a fixed amount per block until reaching the distribution interval.

When distribution occurs, the pallet uses a linear algebra approach to calculate rewards. It builds a matrix of validator weights and stakes, performs various operations like sparse matrix multiplication and Hadamard products, and calculates incentives for miners and dividends for validators.

The distribution considers factors like weight control fees, delegation fees, and the balance between incentives and dividends set by governance. The result is a fair distribution that rewards both miners for their work and validators for their stake and weight assignment.

## The Governance Pallet in Detail

The `governance` pallet enables decentralized decision-making through proposals and voting. Network participants can create proposals to change parameters, add features, or implement policy changes.

The pallet manages the voting process, tracking support for proposals and executing approved changes. It also handles special roles like allocators who can assign tokens to new agents and curators who manage the whitelist.

The treasury is also managed by this pallet, collecting fees and emissions for community use. The treasury funds can be allocated through governance decisions to support network development and improvement.

## Cross-Pallet Communication

The pallets communicate with each other through their API traits. When one pallet needs information or functionality from another, it calls the appropriate API method.

For example, when the `emission0` pallet distributes rewards, it needs to know which agents are registered, their stakes, and other information maintained by `torus0`. It uses the `Torus0Api` trait to access this information without knowing the internal details of how `torus0` stores or processes it.

Similarly, `torus0` might need to check with `governance` to verify if an agent is whitelisted before processing certain actions. This loose coupling through APIs makes the system more maintainable and allows pallets to evolve independently.

## Testing Approach

Each pallet has its own test directory with integration tests for each domain module. These tests verify that the pallet behaves correctly under various conditions, including edge cases and error scenarios.

The API-based design makes testing easier by allowing the creation of mock implementations. When testing a pallet that depends on another, we can create a mock version of the dependency's API that provides controlled responses for testing purposes.

We always write Substrate benchmarks for our extrinsics because almost all operations in the runtime must be paid for (with very few exceptions, usually for operations that can only be executed by sudo keys). Benchmarking measures the computational and storage costs of each operation, allowing us to assign accurate weights that correspond to the resources consumed. These weights are then translated into transaction fees that users pay to execute operations on the chain.

Proper benchmarking is crucial for several reasons. First, it ensures the economic security of the chain by preventing resource exhaustion attacks - operations that consume significant resources must cost proportionally more. Second, it provides fair pricing for users, so they only pay for the actual computational cost of their transactions. Finally, it helps maintain a consistent user experience by ensuring the chain can process a predictable number of transactions per block without exceeding block time limits.

Every time we add or modify an extrinsic, we write or update its benchmark to reflect its current resource usage. This practice is non-negotiable for production-ready code.

## Extending the System

When adding new features to Torus, we follow a consistent approach to maintain the quality and organization of the codebase.

First, we decide which pallet should own the functionality based on its purpose and relationships. Then we add necessary storage items and parameters, implement the business logic in appropriate modules, and expose relevant functions through the pallet's API.

If other pallets need to use the new functionality, we update them to use the API methods. Finally, we write thorough tests to verify that the feature works as expected under all conditions.

Since storage changes require migrations, we take special care when modifying existing structures, ensuring that the chain can upgrade smoothly without losing data.

By following these patterns and conventions, we create a robust and maintainable blockchain system that can evolve over time while maintaining backward compatibility and performance.