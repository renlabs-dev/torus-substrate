# Namespace Permission Check with Redelegation

## Complete Example: Alice → Bob → Charlie

### Setup

- **Alice** (AccountId: `5GrwvaEF...`) owns namespaces:
  - `memory`
  - `memory.write`
  - `memory.read`
- **Dave** (AccountId: `5DAve...`) owns namespaces:
  - `memory`
  - `memory.config`
- **Bob** (AccountId: `5FHneW...`) receives permissions
- **Charlie** (AccountId: `5Charlie...`) receives redelegated permissions

### Step 1: Initial Namespace Creation

```rust
// Alice creates her namespaces
Torus0::create_namespace(origin: Alice, path: b"memory")
Torus0::create_namespace(origin: Alice, path: b"memory.write")
Torus0::create_namespace(origin: Alice, path: b"memory.read")

// Dave creates his namespaces
Torus0::create_namespace(origin: Dave, path: b"memory")
Torus0::create_namespace(origin: Dave, path: b"memory.config")
```

**Storage State:**

```rust
Namespaces:
  (Alice, "memory") -> NamespaceMetadata { ... }
  (Alice, "memory.write") -> NamespaceMetadata { ... }
  (Alice, "memory.read") -> NamespaceMetadata { ... }
  (Dave, "memory") -> NamespaceMetadata { ... }
  (Dave, "memory.config") -> NamespaceMetadata { ... }
```

### Step 2: Alice Delegates to Bob

```rust
Permission0::grant_namespace_permission(
    origin: Alice,
    grantee: Bob,
    paths: ["memory"],  // Just the suffix!
    duration: Indefinite,
    revocation: None,
)
```

**Creates Permission:**

```rust
permission_id_123 -> PermissionContract {
    grantor: Alice,
    grantee: Bob,
    scope: NamespaceScope { paths: ["memory"] },  // Only suffix stored!
    parent: None,
    // ...
}
```

### Step 3: Bob Redelegates to Charlie

```rust
Permission0::grant_namespace_permission(
    origin: Bob,
    grantee: Charlie,
    paths: ["memory.write"],  // Which agent's memory.write???
    duration: Indefinite,
    revocation: None,
)
```

**Creates Permission:**

```rust
permission_id_456 -> PermissionContract {
    grantor: Bob,
    grantee: Charlie,
    scope: NamespaceScope { paths: ["memory.write"] },  // Ambiguous!
    parent: Some(permission_id_123),
    // ...
}
```

## The Core Problem

### The Ambiguity Issue

When Charlie wants to access a namespace, the system doesn't know which agent's namespace to check:

```rust
// Charlie wants to access a memory.write endpoint
// But which one?
// - Alice's "memory.write"?
// - Dave's "memory.write"? 
// - Some other agent's?

// The permission only says Charlie can access "memory.write"
// It doesn't specify WHOSE "memory.write"!
```

### Current Algorithm (with Chain Traversal)

```rust
fn has_namespace_permission<T: Config>(
    agent: &T::AccountId,
    target_owner: &T::AccountId,  // We need to know WHOSE namespace!
    path: &NamespacePath,
) -> bool {
    // Step 1: Check direct ownership
    if agent == target_owner && Namespaces::<T>::contains_key(agent, path) {
        return true;
    }

    // Step 2: Check all permissions for this agent
    let current_block = <frame_system::Pallet<T>>::block_number();

    for permission_id in PermissionsByGrantee::<T>::get(agent) {
        if check_permission_chain::<T>(permission_id, target_owner, path, current_block) {
            return true;
        }
    }

    false
}

fn check_permission_chain<T: Config>(
    permission_id: PermissionId,
    target_owner: &T::AccountId,
    path: &NamespacePath,
    current_block: BlockNumberFor<T>,
) -> bool {
    let Some(permission) = Permissions::<T>::get(permission_id) else {
        return false;
    };

    // Check if expired
    if permission.is_expired(current_block) {
        return false;
    }

    // Check if this permission covers the requested path
    let PermissionScope::Namespace(scope) = &permission.scope else {
        return false;
    };

    let has_matching_path = scope.paths.iter().any(|granted_path| {
        granted_path == path || granted_path.is_parent_of(path)
    });

    if !has_matching_path {
        return false;
    }

    // Now we need to trace back to the root to find the actual owner
    let root_owner = find_root_owner::<T>(permission_id)?;

    // Check if this permission chain leads to the target owner
    root_owner == target_owner
}

fn find_root_owner<T: Config>(
    permission_id: PermissionId,
) -> Result<T::AccountId, ()> {
    let mut current_id = permission_id;
    let mut visited = BTreeSet::new();

    loop {
        // Prevent infinite loops
        if !visited.insert(current_id) {
            return Err(());
        }

        let Some(permission) = Permissions::<T>::get(current_id) else {
            return Err(());
        };

        // If no parent, the grantor must be the owner
        let Some(parent_id) = permission.parent else {
            // Verify the grantor actually owns the namespace
            if let PermissionScope::Namespace(scope) = &permission.scope {
                for path in &scope.paths {
                    if Namespaces::<T>::contains_key(&permission.grantor, path) {
                        return Ok(permission.grantor);
                    }
                }
            }
            return Err(());
        };

        current_id = parent_id;
    }
}
```

### Time Complexity Analysis

**Let:**

- `P` = Number of permissions for the agent
- `N` = Number of paths per permission  
- `L` = Average path length
- `D` = Maximum delegation chain depth

**Current Algorithm Complexity**: O(P × N × L × D)

Each permission check now requires:

1. Loading the permission: O(1)
2. Checking paths: O(N × L)
3. Traversing the chain to find root: O(D)
4. Additional storage reads: O(D) for chain traversal

**Total Storage Reads**: 1 + P × (1 + D)

## Alternative Solution: Full Path Storage

### Modified Structure

Store complete paths including agent context:

```rust
// Instead of: (AccountId, "memory")
// Store: (AccountId, "agent.alice.memory")

pub type Namespaces<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,        // Owner
    Blake2_128Concat,
    FullNamespacePath,   // "agent.alice.memory"
    NamespaceMetadata<T>,
>;
```

### Example with Full Paths

```rust
// Alice creates namespaces
Namespaces:
  (Alice, "agent.alice.memory") -> NamespaceMetadata { ... }
  (Alice, "agent.alice.memory.write") -> NamespaceMetadata { ... }
  (Alice, "agent.alice.memory.read") -> NamespaceMetadata { ... }

// Alice delegates to Bob
permission_id_123 -> PermissionContract {
    grantor: Alice,
    grantee: Bob,
    scope: NamespaceScope { 
        paths: ["agent.alice.memory"]  // Full path!
    },
    parent: None,
}

// Bob redelegates to Charlie
permission_id_456 -> PermissionContract {
    grantor: Bob,
    grantee: Charlie,
    scope: NamespaceScope { 
        paths: ["agent.alice.memory.write"]  // Unambiguous!
    },
    parent: Some(permission_id_123),
}
```

### Simplified Algorithm

```rust
fn has_namespace_permission_full_path<T: Config>(
    agent: &T::AccountId,
    full_path: &FullNamespacePath,  // e.g., "agent.alice.memory.write"
) -> bool {
    // Extract owner from path
    let owner = extract_owner_from_path(full_path)?;

    // Step 1: Check direct ownership
    if agent == owner && Namespaces::<T>::contains_key(agent, full_path) {
        return true;
    }

    // Step 2: Check permissions - No chain traversal needed!
    let current_block = <frame_system::Pallet<T>>::block_number();

    for permission_id in PermissionsByGrantee::<T>::get(agent) {
        let Some(permission) = Permissions::<T>::get(permission_id) else {
            continue;
        };

        if permission.is_expired(current_block) {
            continue;
        }

        if let PermissionScope::Namespace(scope) = &permission.scope {
            for granted_path in &scope.paths {
                // Direct comparison - no ambiguity!
                if granted_path == full_path || granted_path.is_parent_of(full_path) {
                    return true;
                }
            }
        }
    }

    false
}
```

**Time Complexity: O(P × N × L)** - No chain traversal needed!

## Trade-offs Analysis

### Current Approach (Suffix-Only Paths)

**Advantages:**

- **Smaller storage footprint**: Shorter paths to store

**Disadvantages:**

- **Ambiguity with redelegation**: Cannot determine which agent's namespace without chain traversal
- **Complex permission checking**: O(P × N × L × D) with chain traversal
- **More storage reads**: Need to traverse permission chain
- **Circular dependency risk**: Need careful cycle detection

### Alternative Approach (Full Paths)

**Advantages:**

- **Unambiguous permissions**: Always know exactly which namespace
- **Simple permission checking**: O(P × N × L) - no chain traversal
- **Fewer storage reads**: Direct permission lookup
- **Clear audit trail**: Can see full paths in permissions

**Disadvantages:**

- **Longer paths**: `"agent.alice.memory.write"` takes more storage

## Recommendation

For a system with redelegation support, **full path storage is strongly recommended** because:

1. **Correctness**: Eliminates ambiguity about which namespace is being accessed
2. **Performance**: Avoids expensive chain traversal (O(D) operations per check)
3. **Simplicity**: Much simpler algorithm to implement and reason about
4. **Security**: Clearer audit trail and permission boundaries

The storage overhead of longer paths is a reasonable trade-off for the significant gains in correctness, performance, and simplicity. Without full paths, the permission system becomes fragile and complex when redelegation is introduced.
