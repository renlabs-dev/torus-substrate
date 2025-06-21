# Torus Namespaces: Integrating Off-chain Agent Capabilities with the Chain Permission System

## Overview

The namespace system provides a hierarchical tree naming structure for the Torus Protocol, enabling agents to organize and delegate access to their off-chain services. Think of it as a decentralized DNS where agents control their own namespace trees and can share branches with others through the permission system, delegating authority over APIs.

An agent running a Twitter memory service might register agent.alice.memory.twitter, while another agent providing market data could own agent.bob.data.markets.crypto. These dot-separated paths create a natural hierarchy that mirrors how we think about organizing resources, with agents serving as the root nodes of their namespace trees.

The system emerges from a practical need, as agents begin offering specialized off-chain services, they need a way to organize these services and delegate access to specific components.

## Namespace Paths

Every namespace follows a hierarchical dot-separated format with strict validation rules. Each segment can contain alphanumeric characters, hyphens, and underscores, with a maximum length of 63 characters per segment.

```rust
pub struct NamespacePath {
    inner: BoundedVec<u8, ConstU32<MAX_NAMESPACE_PATH_LENGTH>>,
}
```

The path validation ensures consistency across the network:

- Maximum 255 bytes total length
- Maximum 10 segments (depth limitation)
- Each segment between 1-63 characters
- Segments must begin with alphanumerics
- Valid characters: ASCII alphanumerics, `-`, `_`, and `+`

> Implementations MUST NOT assume the character set will be ASCII only forever. Paths are UTF-8 encoded strings.

This structure creates clear ownership: `agent.alice` owns all paths under that prefix, from `agent.alice.api` to `agent.alice.memory.twitter.v2`. The depth limitation prevents excessive nesting while still allowing meaningful organization.

## Design Philosophy

Initially, we considered complex tree structures like Patricia tries or custom prefix trees that would enable on-chain traversal. However, analyzing actual usage, we see that almost all operations are simple existence checks.

When an off-chain service receives a request, it needs to verify that a namespace exists and check permissions. Both operations are direct lookups. The service already knows the exact path it's checking, there's no need for prefix searches or tree traversal on-chain.

```rust
pub type Namespaces<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Blake2_128Concat,
    NamespacePath,
    NamespaceMetadata<T>,
>;
```

By using a double map with the agent as the first key and the full path as the second, we achieve O(1) lookups for all common operations. Agent-level enumeration remains efficient, and the storage structure is straightforward to understand and maintain.

## Economic Model

The namespace pricing model balances accessibility with spam prevention through a simple yet effective fee structure. Rather than the sigmoid curve described in early designs, the implemented system uses a base fee that goes to the treasury plus a refundable deposit based on storage consumption.

```rust
pub struct NamespacePricingConfig<T: Config> {
    pub deposit_per_byte: BalanceOf<T>,
    pub base_fee: BalanceOf<T>,
    pub count_midpoint: u32,
    pub fee_steepness: Percent,
    pub max_fee_multiplier: u32,
}
```

Currently, the fee calculation returns a flat base fee, though the structure allows other pricing engines in the future. The deposit ensures that agents must lock tokens proportional to the storage they consume, which are returned when the namespace is deleted.

This approach creates natural incentives. Agents think carefully about namespace creation since deposits are locked. The base fee contributes to the treasury, funding network development. Storage deposits scale linearly with path length, discouraging excessively long names.

## Storage Architecture

Each namespace stores minimal metadata:

```rust
pub struct NamespaceMetadata<T: Config> {
    pub created_at: BlockNumberFor<T>,
    pub deposit: BalanceOf<T>,
}
```

This approach means each namespace consumes minimal storage. `deposit` tracks the locked amount for refunds.

## Creating Namespaces

When creating a deep path like `agent.alice.memory.twitter.v2`, the system automatically creates any missing intermediate namespaces. This saves users from manual step-by-step creation while ensuring the tree remains consistent.

```rust
fn create_namespace(origin: OriginFor<T>, path: Vec<u8>) -> DispatchResult;
```

> This extrinsic lives inside the Torus0 pallet.

The algorithm determines which parent paths need creation by checking from the deepest level upward. It calculates the total fee and deposit required, processes payment atomically, then creates all namespaces in a single transaction.

## Deletion Strategy

The deletion process ensures that namespaces with active permission delegations cannot be deleted, preventing disruption to services depending on those paths. It automatically removes all child namespaces, maintaining tree consistency. All deposits are refunded to the owner, making deletion economically neutral beyond the initial fee.

```rust
fn delete_namespace(origin: OriginFor<T>, path: Vec<u8>) -> DispatchResult;
```

> This extrinsic lives inside the Torus0 pallet.

## Permission Integration

Namespaces gain their true power through integration with the permission system. An agent can delegate access to specific namespace paths or entire subtrees, enabling access control for off-chain services.

```rust
pub struct NamespaceScope<T: Config> {
    pub paths: BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
}
```

The namespace permission scope contains a set of paths that the grantee can access. The permission system's existing infrastructure handles the complexity of duration, revocation terms, and enforcement authorities. This means namespace permissions can be temporary, require multi-signature revocation, or include third-party controllers. Read more in [permission0.md](permission0.md).

This integration creates composition possibilities. An agent running a data aggregation service could delegate read access to `agent.alice.data.public` while keeping `agent.alice.data.private` restricted, or delegate the entire data scope: `agent.alice.data`. The delegation could be time-limited, revocable by designated arbiters, or controlled by enforcement authorities who verify off-chain conditions.

## Practical Applications

A memory service agent registers `agent.memory` and creates specialized sub-namespaces like `agent.memory.twitter`, `agent.memory.discord`, and `agent.memory.telegram`. Each represents a different data source with potentially different access requirements. The agent can delegate read access to `agent.memory.twitter` to analytics agents while keeping other sources private.

A compute marketplace might use `agent.compute.gpu.nvidia.a100` to represent specific hardware resources. Delegating this namespace grants access to submit jobs to those specific GPUs. The hierarchical structure naturally represents the hardware taxonomy while permissions control access.

API versioning is viable with paths like `agent.api.v1` and `agent.api.v2`. Services can maintain backward compatibility by keeping old namespaces active while encouraging migration to newer versions. Permissions can be time-limited to enforce deprecation schedules.

Data feeds benefit from hierarchical organization: `agent.data.markets.crypto.btc.price` clearly indicates the data type and source. Subscribers can receive permissions for specific data points or entire categories, with granular control over access duration and revocation.

## Implementation Trade-offs

By optimizing for direct lookups, we sacrificed on-chain traversal capabilities. Services cannot efficiently query "all namespaces under `agent.alice.memory`" without iterating through all of Alice's namespaces. This pushes complexity to off-chain indexers, which can build specialized data structures for such queries. Also, we don't expect huge amounts of namespaces from the beginning, and the pricing mechanism should counter that problem to a certain degree.

Storage efficiency took precedence over feature richness. Each namespace stores minimal metadata rather than extensive configuration. This keeps the on-chain footprint small but means additional features require off-chain coordination or separate storage.

The flat fee structure, while simple, doesn't capture the true cost difference between shallow and deep namespaces. This may be refined in future versions as usage patterns emerge and economic requirements become clearer.

## Future Evolution

The namespace system's design anticipates future growth while maintaining backward compatibility. The versioned storage pattern allows seamless upgrades if requirements change. Several enhancements are possible without breaking existing namespaces:

The pricing configuration structure already supports the sigmoid-based fee calculation described in the original design. As the network grows and usage patterns emerge, this more sophisticated pricing can be enabled to better balance accessibility with resource consumption.

The metadata structure could be extended to include additional fields like expiration times, usage counters, or permission defaults. The storage migration system makes such upgrades straightforward.

Off-chain indexing services will likely emerge to provide sophisticated query capabilities. These could offer GraphQL APIs for namespace exploration, real-time updates via WebSocket, and specialized search functionality.

## Security Considerations

We will need a anti-spam system to emerge in the near future. The current version will, however, allow curators to delete/toggle namespaces.

The delegation check during deletion prevents a class of griefing attacks where namespace owners could disrupt dependent services. By requiring delegations to be revoked before deletion, services have warning and can negotiate continued access or migration paths.

Path validation prevents injection attacks and ensures consistent parsing across different implementations. The character restrictions and length limits bound resource consumption while allowing meaningful names.

The economic model creates natural spam resistance. The combination of base fees and storage deposits means namespace squatting has real costs. The treasury receives fees from creation, funding network development rather than enriching early adopters.
