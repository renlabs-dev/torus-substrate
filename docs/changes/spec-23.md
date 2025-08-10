# Runtime Spec Version 23 Changelog

This document details all runtime interface changes introduced in spec version 23.

## 1. Extrinsics

### permission0 pallet

- ```diff
  #[pallet::call_index(6)]
  pub fn delegate_curator_permission(
      origin: OriginFor<T>,
      recipient: T::AccountId,
  -   flags: u32,
  +   flags: BoundedBTreeMap<
  +       Option<PermissionId>,
  +       u32,
  +       T::MaxCuratorSubpermissionsPerPermission,
  +   >,
      cooldown: Option<BlockNumberFor<T>>,
      duration: PermissionDuration<T>,
      revocation: RevocationTerms<T>,
  +   instances: u32,
  ) -> DispatchResult
  ```
  Modified the `delegate_curator_permission` extrinsic to support curator re-delegation. The `flags` parameter is now a map of optional parent permission IDs to permission flags, allowing delegated curators to further delegate specific permissions. Added `instances` parameter to control the number of permission instances that can be created. This enables hierarchical curator permission delegation with fine-grained control. See [Curator Re-delegation](#curator-re-delegation).

## 2. Events

No event changes in this version.

## 3. Storage Items

### permission0 pallet

- ```diff
  - const STORAGE_VERSION: StorageVersion = StorageVersion::new(4);
  + const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);
  ```
  Storage version incremented from 4 to 5 due to structural changes in `CuratorScope`. Includes migration v5 to transform existing curator permissions into the new format with support for sub-delegations.

## 4. Structs & Enums

### permission0 pallet

- ```diff
  pub struct CuratorScope<T: Config> {
  -   pub flags: CuratorPermissions,
  +   pub flags: BoundedBTreeMap<
  +       Option<PermissionId>,
  +       CuratorPermissions,
  +       T::MaxCuratorSubpermissionsPerPermission,
  +   >,
      pub cooldown: Option<BlockNumberFor<T>>,
  }
  ```
  Modified `CuratorScope` to support hierarchical curator permissions. The `flags` field is now a map where each entry represents permissions that can be traced back to their parent permission ID, enabling curator re-delegation capabilities. See [Curator Re-delegation](#curator-re-delegation).

### permission0 pallet errors

- ```diff
  + /// Too many curator permissions being delegated in a single permission.
  + TooManyCuratorPermissions,
  + /// Namespace delegation depth exceeded the maximum allowed limit.
  + DelegationDepthExceeded,
  ```
  Added new error types to handle curator permission limits and namespace delegation depth restrictions. These errors ensure the permission system remains manageable and prevents excessive nesting of delegations.

### permission0 pallet configuration

- ```diff
  + /// Maximum number of curator subpermissions a single permission can delegate.
  + #[pallet::constant]
  + type MaxCuratorSubpermissionsPerPermission: Get<u32>;
  ```
  Added configuration parameter to limit the number of curator sub-permissions that can be delegated in a single permission contract. Set to 16 in the runtime configuration.

## 5. Behavior Changes

### Curator Re-delegation

**What changed**: Curators can now re-delegate their permissions to other agents, creating a hierarchical permission structure. The implementation tracks parent-child relationships between permissions and enforces inheritance rules where child permissions cannot exceed parent permission scopes.

**Why it matters**: This enables more flexible governance structures where curators can delegate specific subsets of their permissions to trusted agents, creating delegation chains. This is particularly useful for scaling curator operations and distributing workload while maintaining accountability through the permission hierarchy.

**Migration needed**: Existing curator permissions are automatically migrated to the new structure with their flags mapped to a single entry with no parent permission ID.

*Tests*: See `pallets/permission0/tests/curator_tests.rs` for re-delegation scenarios.
*Cross-pallet impact*: The governance pallet's curator operations now support hierarchical permission checking through the Permission0CuratorApi trait.

### Namespace Re-delegation

**What changed**: Added support for namespace permission re-delegation with depth limiting. Agents can now delegate namespace permissions they've received to other agents, creating delegation chains up to 5 levels deep. The system validates that child namespace paths exist and are properly owned before allowing delegation.

**Why it matters**: This allows namespace owners to create more complex delegation structures for namespace management. Organizations can delegate namespace control to team leads who can further delegate to team members, enabling hierarchical namespace administration.

**Migration needed**: No migration needed for existing namespace permissions as they remain compatible with the new system.

*Tests*: See `pallets/permission0/tests/namespace_tests.rs` for namespace re-delegation scenarios.
*Cross-pallet impact*: The torus0 pallet's namespace operations now check delegation depth through the Permission0NamespacesApi trait.

### Permission Instance Tracking

**What changed**: All permission types now support instance tracking through the `instances` parameter. This allows delegators to specify how many times a permission can be used concurrently, providing better control over permission usage.

**Why it matters**: This prevents permission abuse by limiting concurrent usage and provides better resource management for delegated operations. It's particularly important for curator and namespace permissions where parallel operations could cause conflicts.

**Migration needed**: Existing permissions are migrated with a default instance count of 1, maintaining current behavior.

*Tests*: See `pallets/permission0/tests/permission_tests.rs` for instance tracking validation.
*Cross-pallet impact*: All pallets using the Permission0 API must now check available instances before executing permission-gated operations.

### Storage Migration v5

**What changed**: Migrated the `CuratorScope` structure to support the new hierarchical permission model. The migration transforms the flat `CuratorPermissions` flags into a `BoundedBTreeMap` structure where existing permissions are mapped with no parent permission ID.

**Why it matters**: This ensures backward compatibility while enabling the new curator re-delegation features. All existing curator permissions continue to work as before while gaining the ability to be re-delegated.

**Migration needed**: Automatic - the migration runs during runtime upgrade and transforms all existing curator permissions to the new format.

*Tests*: See `pallets/permission0/src/migrations.rs` for migration logic.
*Cross-pallet impact*: None - the migration is transparent to other pallets.