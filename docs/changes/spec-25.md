# Runtime Spec Version 25 Changes

This document outlines all interface changes between runtime spec version 24 and 25, focusing on structural refactoring of the permission system to move instance management into individual permission scopes.

## Extrinsics

No extrinsic signatures were changed in this version. The permission0 pallet maintains the same external interface with internal restructuring only.

## Events

No event changes were made in this version.

## Storage Items

No storage item changes were made in this version. A migration (v7) handles the internal restructuring of existing data.

## Structs & Enums

### `permission0::PermissionContract<T>`

```diff
 pub struct PermissionContract<T: Config> {
     pub delegator: T::AccountId,
     pub scope: PermissionScope<T>,
     pub duration: PermissionDuration<T>,
     pub revocation: RevocationTerms<T>,
     pub enforcement: EnforcementAuthority<T>,
     pub last_update: BlockNumberFor<T>,
     pub last_execution: Option<BlockNumberFor<T>>,
     pub execution_count: u32,
-    pub max_instances: u32,
-    pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
     pub created_at: BlockNumberFor<T>,
 }
```

The `max_instances` and `children` fields have been removed from the main contract structure and moved into the individual permission scope types that require them (CuratorScope and NamespaceScope). This architectural change better encapsulates scope-specific functionality and eliminates unnecessary fields for stream permissions which don't use instance management. See [Instance Management Refactoring](#instance-management-refactoring).

### `permission0::CuratorScope<T>`

```diff
 pub struct CuratorScope<T: Config> {
     pub recipient: T::AccountId,
     pub flags: BoundedBTreeMap<
         Option<PermissionId>,
         CuratorPermissions,
         T::MaxCuratorSubpermissionsPerPermission,
     >,
     pub cooldown: Option<BlockNumberFor<T>>,
+    pub max_instances: u32,
+    pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
 }
```

The curator scope now directly contains instance management fields, allowing curator permissions to manage their own hierarchical relationships and instance limits independently. See [Scope-Specific Instance Management](#scope-specific-instance-management).

### `permission0::NamespaceScope<T>`

```diff
 pub struct NamespaceScope<T: Config> {
     pub recipient: T::AccountId,
     pub paths: BoundedBTreeMap<
         Option<PermissionId>,
         BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
         T::MaxNamespacesPerPermission,
     >,
+    pub max_instances: u32,
+    pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
 }
```

Similar to curator permissions, namespace permissions now maintain their own instance tracking and children collections, enabling proper encapsulation of hierarchical permission structures. See [Scope-Specific Instance Management](#scope-specific-instance-management).

### `permission0::PermissionContract<T>::new()` signature

```diff
 pub(crate) fn new(
     delegator: T::AccountId,
     scope: PermissionScope<T>,
     duration: PermissionDuration<T>,
     revocation: RevocationTerms<T>,
     enforcement: EnforcementAuthority<T>,
-    max_instances: u32,
 ) -> Self
```

The constructor no longer accepts `max_instances` as a parameter since this is now managed within the individual scope structures, simplifying permission creation for stream permissions which don't require instance management.

## Behavior Changes

### Instance Management Refactoring

**What changed**: Instance management (max_instances and children tracking) has been moved from the PermissionContract level into the specific permission scopes that require it (CuratorScope and NamespaceScope). The PermissionContract methods `max_instances()`, `available_instances()`, `used_instances()`, `children()`, and `children_mut()` now return `Option` types and delegate to the underlying scope implementations.

**Why it matters**: This refactoring improves the architecture by eliminating unnecessary fields for stream permissions, which don't use instance management. It creates better separation of concerns where each permission type manages only the data it needs, reducing storage overhead for stream permissions and making the codebase more maintainable.

**Migration needed**: The v7 migration automatically restructures existing permissions by moving the max_instances and children fields from the contract level into the appropriate scope structures. No manual intervention is required.

*Tests*: Migration validated through comprehensive tests in `pallets/permission0/src/migrations.rs::v7` ensuring all permission types are correctly transformed.

*Cross-pallet impact*: None. The external API remains unchanged, and other pallets interact with permissions through the same interface.

### Scope-Specific Instance Management

**What changed**: CuratorScope and NamespaceScope now directly manage their instance limits and children collections. The cleanup logic for removing children from parent permissions when a permission is revoked has been updated to use the new `children_mut()` method that safely accesses the children collection only when present in the scope.

**Why it matters**: This change creates a cleaner separation where only permission types that support hierarchical delegation (curator and namespace) carry the overhead of tracking instances and children. Stream permissions, which are flat structures, no longer carry unnecessary instance-related data.

**Migration needed**: Handled automatically by the v7 migration. Existing permissions are restructured to move instance data into the appropriate scope types.

*Tests*: Instance management behavior is validated through existing permission delegation and revocation tests.

*Cross-pallet impact*: None. The changes are internal to the permission0 pallet structure.

### Dynamic Instance Updates for Namespace Permissions

**What changed**: The `update_namespace_permission` function now correctly updates the max_instances field within the NamespaceScope rather than at the PermissionContract level. The validation logic checks parent permissions' available instances and ensures that reducing instances doesn't invalidate existing child permissions.

**Why it matters**: This ensures that instance management for namespace permissions works correctly with the new structure where instances are tracked at the scope level. The change maintains the same external behavior while operating on the restructured internal data.

**Migration needed**: None. This is an internal implementation change that maintains the same external behavior.

*Tests*: Instance update validation tested through namespace permission update tests ensuring proper parent-child instance tracking.

*Cross-pallet impact*: None. The external behavior of updating namespace permissions remains unchanged.

### Optional Instance Methods

**What changed**: The methods `max_instances()`, `available_instances()`, and `children()` on PermissionContract now return `Option` types instead of direct values. These methods return `Some(value)` for curator and namespace permissions that support instances, and `None` for stream permissions that don't.

**Why it matters**: This change makes the API more explicit about which permission types support instance management, preventing incorrect assumptions about stream permissions having instance limits. It provides compile-time safety for code that works with instance management.

**Migration needed**: None. Internal implementation detail.

*Tests*: Method behavior validated through permission contract unit tests.

*Cross-pallet impact*: None. These are internal methods not exposed through the pallet's external API.