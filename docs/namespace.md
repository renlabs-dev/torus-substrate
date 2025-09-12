# Torus Namespaces: Hierarchical Resource Organization and Delegation

## Overview

The namespace system provides a hierarchical naming structure for the Torus Protocol, enabling agents to organize and delegate access to their off-chain services. Agents control their own namespace trees and can share branches with others through the permission system, delegating authority over resources and APIs.

An agent running a Twitter memory service might register `agent.alice.memory.twitter`, while another agent providing market data could own `agent.bob.data.markets.crypto`. These dot-separated paths create a natural hierarchy that mirrors how we organize resources, with agents serving as the root nodes of their namespace trees.

As agents offer specialized off-chain services, they need a way to organize these services and delegate access to specific components. The namespace system provides this organizational structure while integrating with the permission system for access control.

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
- Segments must begin and end with alphanumerics
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

The namespace pricing model uses a sigmoid curve that adjusts fees based on how many namespaces an agent already owns. This creates progressive pricing that becomes more expensive as agents accumulate namespaces, preventing spam while allowing reasonable usage.

```rust
pub struct NamespacePricingConfig<T: Config> {
    pub deposit_per_byte: BalanceOf<T>,
    pub base_fee: BalanceOf<T>,
    pub count_midpoint: u32,
    pub fee_steepness: Percent,
    pub max_fee_multiplier: u32,
}
```

The fee calculation uses the agent's current namespace count to determine their position on the sigmoid curve. Below the midpoint, fees remain relatively low. As agents approach and pass the midpoint (typically 10 namespaces), fees increase following the curve's steepness parameter. The calculation uses `libm::exp` to create a smooth S-curve that eventually plateaus at `base_fee * (1 + max_fee_multiplier)`.

The steepness parameter controls how rapidly fees increase around the midpoint. A steepness of 0% creates a flat fee at the midpoint value. Higher steepness values create sharper transitions - at 100%, the curve becomes nearly vertical at the midpoint. This allows the network to adjust pricing dynamics through governance without changing the core mechanism.

Beyond the creation fee, agents must reserve a deposit proportional to the namespace's storage consumption (`deposit_per_byte * path_length`). These deposits are fully refundable when namespaces are deleted, ensuring agents only pay for active storage use. Creation fees go to the treasury, funding network development.

## Storage Architecture

The storage uses a double map structure with namespace ownership as the first key and the namespace path as the second:

```rust
pub type Namespaces<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    NamespaceOwnership<T>,
    Blake2_128Concat,
    NamespacePath,
    NamespaceMetadata<T>,
>;
```

Namespaces are owned by either the system or an account:

```rust
pub enum NamespaceOwnership<T: Config> {
    System,
    Account(T::AccountId),
}
```

The `System` ownership is used for root-level namespaces like `agent`, while `Account` ownership represents agent-owned namespaces. This separation allows different fee and permission models for system versus user namespaces.

Each namespace stores minimal metadata:

```rust
pub struct NamespaceMetadata<T: Config> {
    pub created_at: BlockNumberFor<T>,
    pub deposit: BalanceOf<T>,
}
```

The system also tracks namespace counts per owner to enable the sigmoid pricing model:

```rust
pub type NamespaceCount<T: Config> = StorageMap<_, Blake2_128Concat, NamespaceOwnership<T>, u32>;
```

This count increments when namespaces are created and decrements when deleted, allowing the pricing algorithm to calculate fees based on current usage.

## Creating Namespaces

When creating a deep path like `agent.alice.memory.twitter.v2`, the system automatically creates any missing intermediate namespaces. This saves users from manual step-by-step creation while ensuring the tree remains consistent.

```rust
fn create_namespace(origin: OriginFor<T>, path: Vec<u8>) -> DispatchResult;
```

> This extrinsic lives inside the Torus0 pallet.

The creation process validates that agent-owned namespaces follow the pattern `agent.<agentname>` where the agent name must match the owner's registered name exactly. This coupling prevents agents from creating namespaces under other agents' names.

The algorithm works backward from the deepest path, checking which parents already exist. Once it finds an existing parent (or reaches the root), it stops checking and creates only the missing paths. For each missing namespace, the system calculates the fee based on the owner's current namespace count and reserves the storage deposit. All namespace creation happens atomically - if any step fails, the entire operation rolls back.

## Agent Registration and Namespaces

When an agent registers with the network, the system automatically creates their root namespace `agent.<agentname>`. This happens within the registration transaction itself:

```rust
// During agent registration
let namespace_path = NamespacePath::new_agent_root(&name)?;
namespace::create_namespace::<T>(
    NamespaceOwnership::Account(agent_key.clone()),
    namespace_path,
)?;
```

If namespace creation fails (for example, if the name contains invalid characters), the entire agent registration fails. This ensures every registered agent has a valid namespace root from which they can create sub-namespaces.

## Deletion Strategy

The deletion process ensures that namespaces with active permission delegations cannot be deleted, preventing disruption to services depending on those paths. It automatically removes all child namespaces, maintaining tree consistency. All deposits are refunded to the owner, making deletion economically neutral beyond the initial fee.

```rust
fn delete_namespace(origin: OriginFor<T>, path: Vec<u8>) -> DispatchResult;
```

> This extrinsic lives inside the Torus0 pallet.

When an agent deregisters, the system automatically deletes their root namespace and all child namespaces. The deletion happens recursively - the system iterates through all namespaces owned by the agent, identifies those that are children of the root namespace, and deletes them all in a single transaction. The total deposits from all deleted namespaces are calculated and refunded atomically using `T::Currency::unreserve`.

The check for active delegations uses the permission system's `is_delegating_namespace` function, which examines all permissions held by the delegator to see if any reference the namespace being deleted or its children.

## Permission Integration

Namespaces integrate with the permission system to enable controlled delegation of access to off-chain services. An agent can delegate access to specific namespace paths or entire subtrees.

```rust
pub struct NamespaceScope<T: Config> {
    pub recipient: T::AccountId,
    pub paths: BoundedBTreeMap<
        Option<PermissionId>,
        BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
        T::MaxNamespacesPerPermission,
    >,
    pub max_instances: u32,
    pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
}
```

The namespace permission scope contains:
- `recipient`: The account that receives the namespace permission
- `paths`: A map from parent permission IDs to sets of namespace paths, creating a hierarchical permission tree
- `max_instances`: Maximum number of child permissions that can be created from this permission
- `children`: Set of child permissions already created from this permission

### Delegation Depth and Hierarchy

The system enforces a maximum delegation depth of 5 levels to prevent infinite delegation chains. When creating a namespace permission, the system traverses the parent permission chain to calculate the current depth. If the new permission would exceed the maximum depth, the operation fails.

Permissions track their parent-child relationships through the `paths` map. When a permission references a parent permission ID, it inherits access only to namespaces that the parent has access to. This creates a tree structure where each level can only delegate a subset of what it received.

### Permission Cleanup

When a namespace permission expires or is revoked, the system performs cleanup operations to maintain consistency:

```rust
// During cleanup
for pid in self.paths.keys().cloned().flatten() {
    Permissions::<T>::mutate_extant(pid, |parent| {
        if let Some(children) = parent.children_mut() {
            children.remove(&permission_id);
        }
    });
}
```

The cleanup process removes the expired permission from all parent permissions' child tracking sets. This prevents the accumulation of dead references in the permission tree.

### Validation and Constraints

When delegating namespace permissions, the system validates:
- The delegator owns the namespace being delegated
- Parent permissions (if specified) grant access to the requested namespaces
- The delegation depth does not exceed the maximum
- Revocation terms are not stronger than parent permissions
- The recipient is a registered agent

The permission system's existing infrastructure handles duration, revocation terms, and enforcement authorities. Namespace permissions can be temporary, require multi-signature revocation, or include third-party controllers. See [permission0.md](permission0.md) for details on the permission system.

An agent running a data aggregation service could delegate read access to `agent.alice.data.public` while keeping `agent.alice.data.private` restricted. The delegation could be time-limited, revocable by designated arbiters, or controlled by enforcement authorities who verify off-chain conditions.

## Practical Applications

A memory service agent registers `agent.memory` and creates specialized sub-namespaces like `agent.memory.twitter`, `agent.memory.discord`, and `agent.memory.telegram`. Each represents a different data source with potentially different access requirements. The agent can delegate read access to `agent.memory.twitter` to analytics agents while keeping other sources private.

A compute marketplace might use `agent.compute.gpu.nvidia.a100` to represent specific hardware resources. Delegating this namespace delegates access to submit jobs to those specific GPUs. The hierarchical structure naturally represents the hardware taxonomy while permissions control access.

API versioning is viable with paths like `agent.api.v1` and `agent.api.v2`. Services can maintain backward compatibility by keeping old namespaces active while encouraging migration to newer versions. Permissions can be time-limited to enforce deprecation schedules.

Data feeds benefit from hierarchical organization: `agent.data.markets.crypto.btc.price` clearly indicates the data type and source. Subscribers can receive permissions for specific data points or entire categories, with granular control over access duration and revocation.

## Implementation Trade-offs

By optimizing for direct lookups, we sacrificed on-chain traversal capabilities. Services cannot efficiently query "all namespaces under `agent.alice.memory`" without iterating through all of Alice's namespaces. This pushes complexity to off-chain indexers, which can build specialized data structures for such queries. The current implementation expects moderate namespace counts per agent, with the sigmoid pricing mechanism naturally limiting excessive creation.

Storage efficiency took precedence over feature richness. Each namespace stores only creation time and deposit amount rather than extensive configuration. This keeps the on-chain footprint small but means additional features require off-chain coordination or separate storage structures.

The coupling between agent names and namespace roots (requiring `agent.<agentname>` to match the registered name) provides security but reduces flexibility. Agents cannot create namespaces that don't follow this pattern, even if they might have legitimate use cases for alternative naming schemes.

The maximum delegation depth of 5 levels balances security with usability. Deeper delegation chains could enable more complex permission structures but would increase traversal costs and potential attack vectors. The fixed limit keeps the system predictable and performant.

The sigmoid pricing curve parameters are currently fixed in storage rather than computed dynamically based on network conditions. While governance can adjust these parameters, the system cannot automatically respond to usage spikes or market conditions without a governance proposal.

## Future Evolution

The namespace system's design anticipates future growth while maintaining backward compatibility. The versioned storage pattern allows seamless upgrades if requirements change. Several enhancements are possible without breaking existing namespaces:

The pricing parameters could become more dynamic, adjusting automatically based on network usage patterns. The sigmoid curve formula could be extended to consider factors beyond just namespace count, such as total network namespace usage or time-based decay of older namespaces.

The metadata structure could be extended to include additional fields like expiration times, usage counters, or custom properties. The storage migration system makes such upgrades straightforward while preserving existing data.

Off-chain indexing services will emerge to provide query capabilities the on-chain system doesn't support. These could offer GraphQL APIs for namespace exploration, real-time updates via WebSocket, and specialized search functionality for discovering available namespaces or tracking delegation chains.

The permission integration could support more complex access patterns, such as conditional permissions based on on-chain state or multi-party approval requirements for sensitive namespaces.

## Security Considerations

The sigmoid pricing curve provides the primary anti-spam mechanism by making namespace accumulation progressively expensive. Agents creating many namespaces face exponentially increasing costs, naturally limiting spam without hard caps. Future versions may introduce additional curator controls for namespace management.

The delegation check during deletion prevents griefing attacks where namespace owners could disrupt dependent services. By requiring all delegations to be revoked before deletion, services have time to migrate or negotiate continued access. The system checks not just direct delegations but also parent-child relationships in the permission tree.

Path validation enforces strict rules on namespace format:
- Segments must contain only lowercase alphanumerics, hyphens, underscores, and plus signs
- Each segment must be 1-63 characters long
- Segments must begin and end with alphanumerics
- Maximum 10 segments (depth limitation)
- Total path length cannot exceed 255 bytes

The coupling between agent names and namespace roots prevents impersonation. An agent cannot create namespaces under another agent's name, as the system validates that `agent.<name>` matches the creator's registered name exactly.

The maximum delegation depth of 5 levels prevents infinite delegation chains that could cause stack overflows during traversal or create confusion about access rights. Each delegation must specify weaker or equal revocation terms compared to its parent, preventing privilege escalation.

Storage deposits use Substrate's `ReservableCurrency` trait, ensuring deposits are genuinely locked and can always be refunded. This prevents scenarios where promised refunds cannot be delivered due to account drainage.
