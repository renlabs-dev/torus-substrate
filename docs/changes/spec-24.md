# Runtime Spec Version 24 Changes

This document outlines all interface changes between runtime spec version 23 and 24, including new extrinsics, modified events, storage changes, and behavioral updates.

## Extrinsics

### `permission0::update_namespace_permission` (NEW)

```diff
+#[pallet::call_index(9)]
+pub fn update_namespace_permission(
+    origin: OriginFor<T>,
+    permission_id: PermissionId,
+    max_instances: u32,
+) -> DispatchResult
```

A new extrinsic that allows permission delegators to update the maximum number of instances allowed for a namespace permission. This provides delegators with the ability to dynamically adjust permission limits after initial creation, enabling more flexible permission management.

### `permission0::delegate_emission_permission`

```diff
 pub fn delegate_emission_permission(
     delegator: T::AccountId,
-    recipient: T::AccountId,
+    recipients: Vec<(T::AccountId, u16)>,
     allocation: EmissionAllocation<Balance>,
-    targets: Vec<(T::AccountId, u16)>,
     distribution: DistributionControl<Balance, BlockNumber>,
     duration: PermissionDuration<BlockNumber>,
     revocation: RevocationTerms<AccountId, BlockNumber>,
     enforcement: EnforcementAuthority<AccountId>,
+    recipient_manager: Option<AccountId>,
+    weight_setter: Option<AccountId>,
 ) -> Result<PermissionId, DispatchError>
```

The emission permission delegation has been restructured to consolidate recipients and their weights into a single parameter, while adding new optional manager fields. The `recipient_manager` can modify the list of recipients, while the `weight_setter` can adjust distribution weights, enabling more granular permission management.

## Events

### `permission0::PermissionDelegated`

```diff
 PermissionDelegated {
     delegator: T::AccountId,
-    recipient: T::AccountId,
     permission_id: PermissionId,
 }
```

The `recipient` field has been removed from the `PermissionDelegated` event since permissions can now have multiple recipients. Client applications should query the permission contract directly to determine recipients.

### `permission0::PermissionRevoked`

```diff
 PermissionRevoked {
     delegator: T::AccountId,
-    recipient: T::AccountId,
-    revoked_by: T::AccountId,
+    revoked_by: Option<T::AccountId>,
     permission_id: PermissionId,
 }
```

The `recipient` field has been removed and `revoked_by` is now optional to handle cases where permissions are revoked by the system automatically (e.g., expiration) rather than by a specific account.

### `permission0::PermissionExpired`

```diff
 PermissionExpired {
     delegator: T::AccountId,
-    recipient: T::AccountId,
     permission_id: PermissionId,
 }
```

Consistent with other events, the `recipient` field has been removed from `PermissionExpired` events.

## Storage Items

No storage item names were changed in this version, but several storage values were migrated through the permission pallet's v6 migration to update the internal structure of permission contracts.

## Structs & Enums

### `permission0::PermissionContract<T>`

```diff
 pub struct PermissionContract<T: Config> {
     pub delegator: T::AccountId,
-    pub recipient: T::AccountId,
     pub scope: PermissionScope<T>,
     // ... other fields
 }
```

The `recipient` field has been removed from the main contract structure since recipients are now scope-specific and can be multiple accounts for emission permissions.

### `permission0::EmissionScope<T>`

```diff
 pub struct EmissionScope<T: Config> {
+    pub recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
     pub allocation: EmissionAllocation<T>,
     pub distribution: DistributionControl<T>,
-    pub targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
     pub accumulating: bool,
+    pub recipient_manager: Option<T::AccountId>,
+    pub weight_setter: Option<T::AccountId>,
 }
```

The `targets` field has been renamed to `recipients` for clarity, and new manager fields enable delegated management of recipients and weights within emission permissions.

### `permission0::CuratorScope<T>`

```diff
 pub struct CuratorScope<T: Config> {
+    pub recipient: T::AccountId,
     pub flags: BoundedBTreeMap<
         Option<PermissionId>,
         CuratorPermissions,
         T::MaxCuratorSubpermissionsPerPermission,
     >,
     pub cooldown: Option<BlockNumberFor<T>>,
 }
```

The `recipient` field has been added to track the specific recipient of curator permissions, and the flags structure now supports parent-child relationships through the optional PermissionId key.

### `permission0::NamespaceScope<T>`

```diff
 pub struct NamespaceScope<T: Config> {
+    pub recipient: T::AccountId,
     pub paths: BoundedBTreeMap<
         Option<PermissionId>,
         BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
         T::MaxNamespacesPerPermission,
     >,
 }
```

Similar to curator permissions, namespace permissions now explicitly track their recipient and support hierarchical path delegation through the optional PermissionId structure.

## Behavior Changes

### Permission Recipients Management

**What changed**: Permission contracts no longer store a single recipient in the main contract structure. Instead, recipients are managed within each permission scope type, allowing for multiple recipients in emission permissions and explicit recipient tracking in curator and namespace permissions.

**Why it matters**: This change enables more flexible permission models, particularly for emission permissions where multiple accounts can receive distributions from a single permission contract. It also provides clearer separation of concerns between permission metadata and scope-specific recipient management.

**Migration needed**: Existing permission contracts are automatically migrated during runtime upgrade. Client applications that previously relied on the `recipient` field in events or contract queries must be updated to extract recipient information from the appropriate scope structure.

*Tests*: The migration is validated through comprehensive tests in `pallets/permission0/src/migrations.rs` that ensure all existing permissions are correctly transformed and all storage indices are properly updated.

*Cross-pallet impact*: Changes affect any pallet that queries permission contracts, though the API layer abstracts most of these changes from external consumers.

### Emission Permission Weight Management

**What changed**: Emission permissions now support designated `weight_setter` and `recipient_manager` accounts that can modify distribution parameters without requiring the original delegator's signature. The permission validation logic checks these manager accounts when processing updates.

**Why it matters**: This enables delegated management of emission streams where the original permission creator can designate trusted accounts to handle operational aspects like weight adjustments or recipient list management. This is particularly useful for automated systems or multi-signature scenarios.

**Migration needed**: Existing emission permissions will have `None` values for the new manager fields and continue to work with delegator-only management. New permissions can optionally specify managers during creation.

*Tests*: Manager functionality is validated through permission update tests that verify proper access control for different management roles.

*Cross-pallet impact*: The emission0 pallet can leverage these new management capabilities for more flexible weight control delegation scenarios.

### Permission Index Management

**What changed**: The permission indexing system has been redesigned to handle multiple recipients per permission contract. The new system maintains separate indices for delegators and participants (recipients), with proper cleanup when permissions are modified or revoked.

**Why it matters**: This ensures that permission queries remain efficient even with the new multi-recipient model, and that storage cleanup properly handles all affected accounts when permissions change.

**Migration needed**: The migration automatically rebuilds all permission indices using the new structure. No client-side changes are required.

*Tests*: Index management is validated through migration tests that verify index consistency before and after the upgrade, including edge cases with complex permission hierarchies.

*Cross-pallet impact*: Any pallet querying permissions by recipient will benefit from the improved index structure, with better performance for multi-recipient scenarios.
