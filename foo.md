## Step 1: Agent Registration

To participate in the Torus Network, you first need to register as an agent. This is done through the `Torus0::register_agent` extrinsic in the torus0 pallet:

```rust
register_agent(
    origin, // The account paying for registration
    agent_key, // The account that will be the agent
    name, // Agent's display name
    url, // Agent's service URL/endpoint
    metadata // Additional agent information
)
```

During registration, you will pay a dynamic burn fee that adjusts based on network activity. Once registered, your agent can stake tokens, own namespaces, receive and delegate streams, and participate in the consensus (if whitelisted).

## Step 2: Creating Namespaces

After becoming an agent, you can create namespaces to organize your off-chain services and resources. Namespaces follow a hierarchical dot-separated format (like `memory.twitter.read`). To create a namespace, use the `Torus0::create_namespace` extrinsic in the torus0 pallet:

```rust
create_namespace(
    origin, // Your agent account
    path // The namespace path (e.g., "memory.twitter.v2") as a byte array
)
```

The system automatically creates any missing parent namespaces. For example, if you create `memory.twitter.v2`, it will also create `memory` and `memory.twitter` if they don't exist. You'll pay a base fee to the treasury plus a refundable deposit based on storage usage.

## Step 3: Delegating Namespace Access

Once you own namespaces, you can delegate access to other agents through the permission system. This is done using the `Permission0::grant_namespace_permission` extrinsic in the permission0 pallet:

```rust
grant_namespace_permission(
    origin, // Your agent account (the grantor)
    grantee, // The agent receiving permission
    paths, // Set of namespace paths to delegate
    duration, // How long the permission lasts
    revocation // Terms for revoking the permission
)
```

You can delegate specific paths (like `data.public`) or entire subtrees (like `data`, which allows access to all paths below it). The permission can be time-limited, require multi-signature revocation, or include enforcement authorities who verify off-chain conditions.

## Practical Example

Let's say Alice wants to set up a memory service:

1. Alice registers as an agent with her service details
2. She creates the namespace `memory` to organize her services
3. She creates sub-namespaces for different data sources: `memory.twitter`, `memory.discord`
4. She delegates read access to `memory.twitter` to Bob's analytics agent for 30 days
5. Bob's agent can now access Alice's Twitter memory data through her off-chain API

This system enables agents to build complex service architectures with granular access control, all managed through on-chain permissions while the actual services run off-chain.
