# Runtime Spec Version 24 Changes

This document details all interface changes between runtime spec version 23 and 24, including modified extrinsics, events, storage changes, and significant behavioral updates.

## Extrinsics

### `permission0::delegate_stream_permission` (renamed from `delegate_emission_permission`)

```diff
- #[pallet::call_index(0)]
- pub fn delegate_emission_permission(
+ #[pallet::call_index(0)]
+ pub fn delegate_stream_permission(
      origin: OriginFor<T>,
-     recipient: T::AccountId,
+     recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
-     allocation: EmissionAllocation<T>,
+     allocation: StreamAllocation<T>,
-     targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
      distribution: DistributionControl<T>,
      duration: PermissionDuration<T>,
      revocation: RevocationTerms<T>,
      enforcement: EnforcementAuthority<T>,
+     recipient_manager: Option<T::AccountId>,
+     weight_setter: Option<T::AccountId>,
  ) -> DispatchResult
```

The emission permission delegation has been renamed to stream permission delegation and restructured to consolidate recipients and their weights into a single parameter. Two new optional manager fields enable delegated management: `recipient_manager` can modify the list of recipients, while the `weight_setter` can adjust distribution weights. See [Stream Permission Management](#stream-permission-management).

### `permission0::update_stream_permission` (renamed from `update_emission_permission`)

```diff
- #[pallet::call_index(8)]
- pub fn update_emission_permission(
+ #[pallet::call_index(8)]
+ pub fn update_stream_permission(
      origin: OriginFor<T>,
      permission_id: PermissionId,
-     new_targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
+     new_recipients: Option<BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>>,
      new_streams: Option<BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>>,
      new_distribution_control: Option<DistributionControl<T>>,
+     new_recipient_manager: Option<Option<T::AccountId>>,
+     new_weight_setter: Option<Option<T::AccountId>>,
  ) -> DispatchResult
```

The stream permission update function now accepts optional parameters for all fields, allowing partial updates. New manager fields can be updated by the delegator to change who can manage recipients and weights. See [Delegated Permission Management](#delegated-permission-management).

### `permission0::update_namespace_permission` (NEW)

```diff
+ #[pallet::call_index(9)]
+ pub fn update_namespace_permission(
+     origin: OriginFor<T>,
+     permission_id: PermissionId,
+     max_instances: u32,
+ ) -> DispatchResult
```

A new extrinsic that allows permission delegators to update the maximum number of instances allowed for a namespace permission, enabling dynamic adjustment of permission limits after initial creation. See [Dynamic Instance Management](#dynamic-instance-management).

### `permission0::bulk_delegate_namespace_permission` (NEW)

```diff
+ #[pallet::call_index(10)]
+ pub fn bulk_delegate_namespace_permission(
+     origin: OriginFor<T>,
+     recipients: BoundedBTreeSet<T::AccountId, T::MaxBulkOperationsPerCall>,
+     paths: BoundedBTreeMap<
+         Option<PermissionId>,
+         BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
+         T::MaxNamespacesPerPermission,
+     >,
+     duration: PermissionDuration<T>,
+     revocation: RevocationTerms<T>,
+     instances: u32,
+ ) -> DispatchResult
```

Enables delegating namespace permissions to multiple recipients in a single transaction, creating multiple permission contracts with identical properties. This optimization reduces transaction costs for batch operations. See [Bulk Permission Operations](#bulk-permission-operations).

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

The `recipient` field has been removed and `revoked_by` is now optional to handle system-automatic revocations (e.g., expiration) rather than user-initiated ones.

### `permission0::PermissionExpired`

```diff
 PermissionExpired {
     delegator: T::AccountId,
-    recipient: T::AccountId,
     permission_id: PermissionId,
 }
```

Consistent with other event changes, the `recipient` field has been removed from expiration events.

### `permission0::StreamDistribution` (renamed from `EmissionDistribution`)

```diff
- EmissionDistribution {
+ StreamDistribution {
      permission_id: PermissionId,
      stream_id: Option<StreamId>,
-     target: T::AccountId,
+     recipient: T::AccountId,
      amount: BalanceOf<T>,
      reason: DistributionReason,
  }
```

The event has been renamed to reflect the terminology change from emissions to streams, and the `target` field renamed to `recipient` for consistency.

## Storage Items

### `permission0::PermissionsByParticipants`

```diff
  pub type PermissionsByParticipants<T: Config> = StorageMap<
      _,
      Identity,
      (T::AccountId, T::AccountId),
-     BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
+     BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
      ValueQuery,
  >;
```

Changed from `BoundedVec` to `BoundedBTreeSet` for more efficient permission lookups and to prevent duplicate entries.

### `permission0::PermissionsByDelegator`

```diff
  pub type PermissionsByDelegator<T: Config> = StorageMap<
      _,
      Identity,
      T::AccountId,
-     BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
+     BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
      ValueQuery,
  >;
```

Changed from `BoundedVec` to `BoundedBTreeSet` for consistency with other index storages.

### `permission0::PermissionsByRecipient`

```diff
  pub type PermissionsByRecipient<T: Config> = StorageMap<
      _,
      Identity,
      T::AccountId,
-     BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
+     BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
      ValueQuery,
  >;
```

Changed from `BoundedVec` to `BoundedBTreeSet` for improved index management.

## Structs & Enums

### `permission0::PermissionContract<T>`

```diff
 pub struct PermissionContract<T: Config> {
     pub delegator: T::AccountId,
-    pub recipient: T::AccountId,
     pub scope: PermissionScope<T>,
     pub duration: PermissionDuration<T>,
     pub revocation: RevocationTerms<T>,
     pub enforcement: EnforcementAuthority<T>,
+    pub last_update: BlockNumberFor<T>,
     ...
 }
```

The `recipient` field has been removed from the main contract structure as recipients are now scope-specific. Added `last_update` field to track modification timestamps. See [Multi-Recipient Permissions](#multi-recipient-permissions).

### `permission0::StreamScope<T>` (renamed from `EmissionScope`)

```diff
- pub struct EmissionScope<T: Config> {
+ pub struct StreamScope<T: Config> {
+     pub recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
-     pub allocation: EmissionAllocation<T>,
+     pub allocation: StreamAllocation<T>,
      pub distribution: DistributionControl<T>,
-     pub targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
      pub accumulating: bool,
+     pub recipient_managers: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
+     pub weight_setters: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
 }
```

The `targets` field has been renamed to `recipients` for clarity. New manager fields enable delegated management of recipients and weights within stream permissions. See [Stream Permission Management](#stream-permission-management).

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

Added `recipient` field to explicitly track the curator permission recipient within the scope.

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

Added `recipient` field to track the specific recipient of namespace permissions.

### `permission0::StreamAllocation<T>` (renamed from `EmissionAllocation`)

```diff
- pub enum EmissionAllocation<T: Config> {
+ pub enum StreamAllocation<T: Config> {
     Streams(BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>),
     FixedAmount(BalanceOf<T>),
 }
```

Renamed to reflect the terminology change from emissions to streams throughout the permission system.

### `permission0::PermissionScope<T>`

```diff
 pub enum PermissionScope<T: Config> {
-    Emission(EmissionScope<T>),
+    Stream(StreamScope<T>),
     Curator(CuratorScope<T>),
     Namespace(NamespaceScope<T>),
 }
```

The `Emission` variant has been renamed to `Stream` to align with the new terminology.

## Behavior Changes

### Multi-Recipient Permissions

**What changed**: Permission contracts no longer store a single recipient in the main contract structure. Recipients are now managed within each permission scope type, with stream permissions supporting multiple recipients with individual weights, while curator and namespace permissions track a single recipient within their scope.

**Why it matters**: This enables more flexible permission models, particularly for stream distributions where multiple accounts can receive tokens from a single permission contract. It allows for complex distribution patterns without requiring multiple permission contracts.

**Migration needed**: Existing permission contracts are automatically migrated during runtime upgrade through the v6 migration. Client applications must be updated to extract recipient information from the appropriate scope structure rather than the main contract.

*Tests*: Migration validated through comprehensive tests in `pallets/permission0/src/migrations.rs::v6` that ensure all permission types are correctly transformed and indices are properly updated.

*Cross-pallet impact*: The emission0 pallet's integration with permission0 is updated to use the new multi-recipient structure when accumulating and distributing streams.

### Stream Permission Management

**What changed**: Stream permissions now support designated `recipient_managers` and `weight_setters` accounts that can modify distribution parameters without the delegator's signature. The permission validation logic checks these manager accounts when processing updates, allowing delegated operational control.

**Why it matters**: This enables delegated management of stream distributions where the permission creator can designate trusted accounts to handle operational aspects like adjusting weights or managing recipient lists. This is particularly useful for automated systems, DAOs, or scenarios requiring multi-signature operations.

**Migration needed**: Existing stream permissions will have the delegator set as the default manager for both roles. New permissions can optionally specify different managers during creation.

*Tests*: Manager functionality is validated through permission update tests verifying proper access control for different management roles.

*Cross-pallet impact*: The emission0 pallet can leverage these management capabilities for more flexible weight control delegation scenarios.

### Delegated Permission Management

**What changed**: The `update_stream_permission` function now supports partial updates through optional parameters and enforces role-based access control. Delegators can update all fields, recipient managers can modify the recipient list (adding/removing recipients), and weight setters can adjust weights for existing recipients only.

**Why it matters**: This granular access control allows for separation of concerns in permission management, where different aspects of a permission can be managed by different trusted parties without requiring the delegator's constant involvement.

**Migration needed**: No migration required. Existing permissions continue to work with delegator-only management by default.

*Tests*: Access control validated through tests in `pallets/permission0/tests/` ensuring proper authorization for each role.

*Cross-pallet impact*: None directly, but provides more flexible integration points for other pallets.

### Permission Index Restructuring

**What changed**: The permission indexing system has been redesigned to handle multiple recipients per permission. The storage types changed from `BoundedVec` to `BoundedBTreeSet` for better performance and to prevent duplicates. Index management now properly handles permissions with multiple recipients in stream permissions.

**Why it matters**: This ensures efficient permission queries even with the new multi-recipient model and prevents index corruption from duplicate entries. The restructuring also improves cleanup operations when permissions are modified or revoked.

**Migration needed**: The v6 migration automatically rebuilds all permission indices using the new structure. No client-side changes required for querying.

*Tests*: Index consistency validated through migration tests checking all three index types (delegator, recipient, participants) for correctness.

*Cross-pallet impact*: Improved query performance for any pallet looking up permissions by recipient or delegator.

### Dynamic Instance Management

**What changed**: The new `update_namespace_permission` extrinsic allows delegators to modify the maximum instances of a namespace permission after creation. The system validates that increasing instances doesn't exceed parent limits and decreasing doesn't invalidate existing child permissions.

**Why it matters**: This provides flexibility in managing namespace permissions over time, allowing adjustments based on actual usage patterns without requiring permission revocation and re-creation. It's particularly useful for managing hierarchical namespace delegations.

**Migration needed**: No migration required. This is a new capability that doesn't affect existing permissions.

*Tests*: Instance management validated through tests ensuring proper parent-child instance tracking and validation.

*Cross-pallet impact*: None directly, but enhances namespace management capabilities for the torus0 pallet.

### Bulk Permission Operations

**What changed**: The new `bulk_delegate_namespace_permission` extrinsic enables creating multiple namespace permissions with identical properties in a single transaction. Each recipient gets their own permission contract with the same paths, duration, and revocation terms.

**Why it matters**: This significantly reduces transaction costs and complexity when delegating the same namespace permissions to multiple recipients, such as granting access to a group of agents or distributing permissions across a team.

**Migration needed**: No migration required. This is a new optimization feature.

*Tests*: Bulk operations validated through benchmarking tests ensuring correct permission creation for all recipients.

*Cross-pallet impact*: None directly, but improves efficiency for batch permission management scenarios.

### Recipient Self-Revocation for Multi-Recipient Permissions

**What changed**: When a stream permission has multiple recipients, individual recipients can now remove themselves from the permission without revoking the entire permission. The system updates indices accordingly and continues distributing to remaining recipients.

**Why it matters**: This provides recipients with autonomy to opt-out of stream distributions without affecting other recipients or requiring delegator intervention. It's particularly useful for compliance or when recipients no longer wish to receive distributions.

**Migration needed**: No migration required. This is a new capability for multi-recipient permissions.

*Tests*: Self-revocation tested in permission revocation tests ensuring proper index cleanup and continued operation for remaining recipients.

*Cross-pallet impact*: Stream distributions in emission0 automatically adjust to the reduced recipient set.
