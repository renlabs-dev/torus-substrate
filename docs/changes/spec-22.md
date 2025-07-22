# Runtime Interface Changes - Spec Version 22

This document outlines all runtime interface changes between spec version 21 and 22.

## Extrinsics

### Permission0 Pallet

- ```diff
  - #[pallet::weight(T::WeightInfo::grant_emission_permission())]
  - pub fn grant_emission_permission(
  + #[pallet::weight(T::WeightInfo::delegate_emission_permission())]
  + pub fn delegate_emission_permission(
      origin: OriginFor<T>,
  -   grantee: T::AccountId,
  +   recipient: T::AccountId,
      allocation: EmissionAllocation<T>,
      targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
      distribution: DistributionControl<T>,
      duration: PermissionDuration<T>,
      revocation: RevocationTerms<T>,
      enforcement: EnforcementAuthority<T>,
  ) -> DispatchResult
  ```
  Renames `grant_emission_permission` to `delegate_emission_permission` and changes the `grantee` parameter to `recipient`, reflecting the conceptual shift from granting permissions to delegating them. The functionality remains identical but the terminology better reflects the delegation relationship.

- ```diff
  - #[pallet::weight(T::WeightInfo::grant_curator_permission())]
  - pub fn grant_curator_permission(
  + #[pallet::weight(T::WeightInfo::delegate_curator_permission())]
  + pub fn delegate_curator_permission(
      origin: OriginFor<T>,
  -   grantee: T::AccountId,
  +   recipient: T::AccountId,
      flags: u32,
      cooldown: Option<BlockNumberFor<T>>,
      duration: PermissionDuration<T>,
      revocation: RevocationTerms<T>,
  ) -> DispatchResult
  ```
  Renames `grant_curator_permission` to `delegate_curator_permission` and updates parameter naming from `grantee` to `recipient`.

- ```diff
  - #[pallet::weight(T::WeightInfo::grant_curator_permission())]
  - pub fn grant_namespace_permission(
  + #[pallet::weight(T::WeightInfo::delegate_curator_permission())]
  + pub fn delegate_namespace_permission(
      origin: OriginFor<T>,
  -   grantee: T::AccountId,
  -   paths: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
  +   recipient: T::AccountId,
  +   paths: BoundedBTreeMap<
  +       Option<PermissionId>,
  +       BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
  +       T::MaxNamespacesPerPermission,
  +   >,
      duration: PermissionDuration<T>,
      revocation: RevocationTerms<T>,
  +   instances: u32,
  ) -> DispatchResult
  ```
  Significantly changes namespace permission delegation by introducing hierarchical permissions. The `paths` parameter now maps parent permission IDs to sets of namespace paths, enabling permission inheritance. Adds an `instances` parameter to control the number of times a permission can be used.

### Torus0 Pallet

- ```diff
  - pub fn register_agent(
      origin: OriginFor<T>,
  -   agent_key: T::AccountId,
      name: Vec<u8>,
      url: Vec<u8>,
      metadata: Vec<u8>,
  ) -> DispatchResult
  ```
  Removes the separate `agent_key` parameter. The agent key is now derived from the transaction signer, simplifying the registration process and ensuring the agent key matches the transaction origin.

- ```diff
  - #[pallet::weight((T::WeightInfo::unregister_agent(), DispatchClass::Normal, Pays::Yes))]
  - pub fn unregister_agent(origin: OriginFor<T>) -> DispatchResult
  + #[pallet::weight((T::WeightInfo::deregister_agent(), DispatchClass::Normal, Pays::Yes))]
  + pub fn deregister_agent(origin: OriginFor<T>) -> DispatchResult
  ```
  Renames `unregister_agent` to `deregister_agent` for consistency with internal function naming conventions.

## Events

### Permission0 Pallet

- ```diff
  - /// Permission granted from grantor to grantee with ID
  - PermissionGranted {
  -     grantor: T::AccountId,
  -     grantee: T::AccountId,
  + /// Permission delegated from delegator to recipient with ID
  + PermissionDelegated {
  +     delegator: T::AccountId,
  +     recipient: T::AccountId,
      permission_id: PermissionId,
  },
  ```
  Renames the event and field names to reflect delegation terminology.

- ```diff
  - /// Permission revoked with ID
  - PermissionRevoked {
  -     grantor: T::AccountId,
  -     grantee: T::AccountId,
  +     delegator: T::AccountId,
  +     recipient: T::AccountId,
      revoked_by: Option<T::AccountId>,
      permission_id: PermissionId,
  },
  ```
  Updates field names in the revocation event to match the new delegation terminology.

- ```diff
  - /// Permission expired with ID
  - PermissionExpired {
  -     grantor: T::AccountId,
  -     grantee: T::AccountId,
  +     delegator: T::AccountId,
  +     recipient: T::AccountId,
      permission_id: PermissionId,
  },
  ```
  Updates field names in the expiration event.

- ```diff
  - /// Permission executed (manual distribution) with ID
  - PermissionExecuted {
  -     grantor: T::AccountId,
  -     grantee: T::AccountId,
  -     permission_id: PermissionId,
  -     stream_id: Option<StreamId>,
  -     amount: BalanceOf<T>,
  - },
  - /// Auto-distribution executed
  - AutoDistributionExecuted {
  -     grantor: T::AccountId,
  -     grantee: T::AccountId,
  -     permission_id: PermissionId,
  -     stream_id: Option<StreamId>,
  -     amount: BalanceOf<T>,
  - },
  + /// An emission distribution happened
  + EmissionDistribution {
  +     permission_id: PermissionId,
  +     stream_id: Option<StreamId>,
  +     target: T::AccountId,
  +     amount: BalanceOf<T>,
  +     reason: permission::emission::DistributionReason,
  + },
  ```
  Consolidates manual and automatic distribution events into a single `EmissionDistribution` event that includes a `reason` field to distinguish between manual and automatic execution.

- ```diff
  + /// Accumulated emission for stream
  + AccumulatedEmission {
  +     permission_id: PermissionId,
  +     stream_id: StreamId,
  +     amount: BalanceOf<T>,
  + },
  ```
  Adds a new event to track emission accumulation for debugging and monitoring purposes.

### Governance Pallet

- ```diff
  - /// A vote has been unregistered from a proposal.
  - ProposalVoteUnregistered(u64, T::AccountId),
  + /// A vote has been deregistered from a proposal.
  + ProposalVoteUnregistered(u64, T::AccountId),
  ```
  Updates event documentation to use "deregistered" instead of "unregistered" for terminology consistency.

## Storage Items

### Permission0 Pallet

- ```diff
  - /// Permissions granted by a specific account
  - pub type PermissionsByGrantor<T: Config> = StorageMap<
  + /// Permissions delegated by a specific account
  + pub type PermissionsByDelegator<T: Config> = StorageMap<
      _,
      Identity,
      T::AccountId,
      BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
      ValueQuery,
  >;
  ```
  Renames storage map to reflect delegation terminology.

- ```diff
  - /// Permissions received by a specific account
  - pub type PermissionsByGrantee<T: Config> = StorageMap<
  + pub type PermissionsByRecipient<T: Config> = StorageMap<
      _,
      Identity,
      T::AccountId,
      BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
      ValueQuery,
  >;
  ```
  Renames storage map for recipients.

- ```diff
  - /// Mapping from (grantor, grantee) to permission IDs
  + /// Mapping from (delegator, recipient) to permission IDs
    pub type PermissionsByParticipants<T: Config> = StorageMap<
      _,
      Identity,
  -   (T::AccountId, T::AccountId),
  +   (T::AccountId, T::AccountId),
      BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
      ValueQuery,
  >;
  ```
  Updates documentation to reflect the terminology change while keeping the same structure.

## Structs & Enums

### Permission0 API Types

- ```diff
  /// Distribution control parameters
  pub enum DistributionControl<Balance, BlockNumber> {
  -   /// Manual distribution by the grantee
  +   /// Manual distribution by the recipient
      Manual,
      /// Automatic distribution after accumulation threshold
      Automatic(Balance),
      /// Distribution at specific block
      AtBlock(BlockNumber),
      /// Distribution at interval
      Interval(BlockNumber),
  }
  ```
  Updates documentation in the distribution control enum.

- ```diff
  pub enum RevocationTerms<AccountId, BlockNumber> {
      /// Cannot be revoked
      Irrevocable,
  -   /// Can be revoked by the grantor at any time
  -   RevocableByGrantor,
  +   /// Can be revoked by the delegator at any time
  +   RevocableByDelegator,
      /// Can be revoked by third party arbiters
      RevocableByArbiters {
          accounts: Vec<AccountId>,
          required_votes: u32,
      },
      /// Time-based revocation
      RevocableAfter(BlockNumber),
  }
  ```
  Renames the revocation term to match delegation terminology.

### Permission0 Internal Types

- ```diff
  pub struct PermissionContract<T: Config> {
  -   pub grantor: T::AccountId,
  -   pub grantee: T::AccountId,
  +   pub delegator: T::AccountId,
  +   pub recipient: T::AccountId,
      pub scope: PermissionScope<T>,
      pub duration: PermissionDuration<T>,
      pub revocation: RevocationTerms<T>,
      /// Enforcement authority that can toggle the permission
      pub enforcement: EnforcementAuthority<T>,
      /// Last execution block
  -   pub last_execution: Option<BlockNumberFor<T>>,
  +   last_execution: Option<BlockNumberFor<T>>,
      /// Number of times the permission was executed
  -   pub execution_count: u32,
  -   /// Parent permission ID (None for root permissions)
  -   pub parent: Option<PermissionId>,
  +   execution_count: u32,
  +   /// Maximum number of instances of this permission
  +   pub max_instances: u32,
  +   /// Children permissions
  +   pub children: BoundedBTreeSet<H256, T::MaxChildrenPerPermission>,
      pub created_at: BlockNumberFor<T>,
  }
  ```
  Major structural changes: renames fields to use delegation terminology, makes execution tracking fields private with accessor methods, removes parent field and adds children field for hierarchical permissions, and introduces max_instances for usage control.

- ```diff
  /// Scope for namespace permissions
  pub struct NamespaceScope<T: Config> {
  -   /// Set of namespace paths this permission grants access to
  -   pub paths: BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
  +   /// Set of namespace paths this permission delegates access to
  +   pub paths: BoundedBTreeMap<
  +       Option<PermissionId>,
  +       BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
  +       T::MaxNamespacesPerPermission,
  +   >,
  }
  ```
  Transforms the simple set of paths into a hierarchical structure mapping parent permission IDs to path sets, enabling permission inheritance and re-delegation.

### Emission0 Pallet Types

- ```diff
  pub struct ConsensusMemberInput<T: Config> {
      pub agent_id: T::AccountId,
      pub validator_permit: bool,
      pub weights: Vec<(T::AccountId, FixedU128)>,
      pub stakes: Vec<(T::AccountId, u128)>,
      pub total_stake: u128,
      pub normalized_stake: FixedU128,
      pub delegating_to: Option<T::AccountId>,
  -   pub registered: bool,
  +   pub whitelisted: bool,
  }
  ```
  Renames the field from `registered` to `whitelisted` to better reflect that agents must be both registered and whitelisted to participate in consensus.

- ```diff
  + #[derive(
  +     Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, TypeInfo, MaxEncodedLen,
  + )]
  + pub enum DistributionReason {
  +     Automatic,
  +     Manual,
  + }
  ```
  Adds a new enum to distinguish between automatic and manual distribution reasons in events and internal logic.

### Torus0 API Types

- ```diff
  pub trait Torus0Api<AccountId, Balance> {
      // ... other methods
  -   fn stake_to(staker: &AccountId, staked: &AccountId, amount: Balance) -> Result<(), Balance>;
  +   fn stake_to(staker: &AccountId, staked: &AccountId, amount: Balance) -> DispatchResult;
  +   fn find_agent_by_name(name: &[u8]) -> Option<AccountId>;
      // ... other methods
  }
  ```
  Changes the return type of `stake_to` from `Result<(), Balance>` to `DispatchResult` for consistent error handling, and adds a new method to find agents by name.

- ```diff
  impl NamespacePath {
      // ... existing methods
  +   /// Get the agent name from the agent root path: "agent.<name>".
  +   pub fn agent_name(&self) -> Option<&[u8]> {
  +       let mut segments = self.segments();
  +       if self.segments().next().is_some_and(|f| f == b"agent") {
  +           segments.nth(1)
  +       } else {
  +           None
  +       }
  +   }
  }
  ```
  Adds a new method to extract agent names from namespace paths.

## Behavior Changes

### Permission Delegation System Redesign

**What changed**: The entire permission system has been redesigned around delegation rather than granting. This includes renaming all functions, events, and storage items from "grant/grantor/grantee" to "delegate/delegator/recipient" terminology.

**Why it matters**: This change better reflects the actual relationship between participants - one party delegates authority to another rather than simply granting permissions. This conceptual clarity improves the mental model for developers and users interacting with the permission system.

**Migration needed**: All client applications must update their calls to use the new extrinsic names and parameter names. Event listeners must be updated to use the new event names and field names.

*Tests*: All permission-related benchmarks and tests have been updated to use the new terminology.
*Cross-pallet impact*: The Governance pallet migration code was updated to use the new delegation terminology when migrating curator permissions.

### Hierarchical Namespace Permissions

**What changed**: Namespace permissions now support hierarchical delegation through a parent-child relationship system. The `NamespaceScope` structure changed from a simple set of paths to a map of parent permission IDs to path sets, and permission contracts now track children and have instance limits.

**Why it matters**: This enables sophisticated permission delegation chains where a recipient of namespace permissions can re-delegate subsets of those permissions to other parties. The instance system prevents over-delegation by limiting how many active delegations can exist from a single permission.

**Migration needed**: Existing namespace permissions will be migrated to the new structure with all paths under a `None` parent (indicating root-level permissions).

*Tests*: New tests validate permission inheritance, instance counting, and proper parent-child relationship management.
*Cross-pallet impact*: The Permission0 pallet now enforces stricter validation of namespace existence through the Torus0 API.

### Staking System Overhaul

**What changed**: The staking system now uses named reserves instead of withdrawing and issuing tokens. Stakes are tracked using the Balances pallet's named reserve functionality with identifier `b"torstake"`. The migration mints tokens for existing stakers and reserves them appropriately.

**Why it matters**: This change improves the economic model by ensuring staked tokens remain as part of the staker's balance but are properly reserved and cannot be spent. It also ensures the total issuance remains consistent and stakes are properly tracked by the underlying currency system.

**Migration needed**: The v6 migration automatically handles the conversion of existing stakes to the new reserve-based system. Stakers with insufficient balance for the existential deposit will have their stakes reduced accordingly.

*Tests*: Comprehensive tests ensure stake operations work correctly with the reserve system.
*Cross-pallet impact*: The Emission0 pallet now properly handles staking failures with error logging rather than panicking.

### Agent Registration Simplification

**What changed**: The `register_agent` extrinsic no longer takes a separate `agent_key` parameter. The agent key is now always the transaction signer. The registration process also now checks for duplicate agent names in addition to duplicate keys.

**Why it matters**: This simplifies the registration process and ensures stronger consistency between the transaction signer and the registered agent. Name uniqueness prevents confusion and impersonation attempts.

**Migration needed**: Client applications must remove the `agent_key` parameter when calling `register_agent`.

*Tests*: Updated benchmarks and tests reflect the simplified registration process.
*Cross-pallet impact*: The Torus0 API now includes `find_agent_by_name` to support name-based lookups.

### Enhanced Emission Distribution Tracking

**What changed**: The emission distribution system now emits more granular events, including individual `EmissionDistribution` events for each target and `AccumulatedEmission` events when tokens are accumulated for permissions. Distribution functions now return `DispatchResult` and handle errors gracefully.

**Why it matters**: This provides better observability into the emission system, allowing external systems to track exactly how tokens flow through the network. Error handling prevents the emission system from silently failing.

**Migration needed**: Applications monitoring emission events should update to use the new `EmissionDistribution` event format instead of the separate manual/automatic events.

*Tests*: Tests verify proper event emission and error handling in distribution functions.
*Cross-pallet impact*: Improved error logging helps identify issues in cross-pallet interactions during emission distribution.

### Whitelist-Based Consensus Participation

**What changed**: The emission system now considers agents to be eligible for consensus only if they are both registered and whitelisted. The `registered` field in `ConsensusMemberInput` was renamed to `whitelisted` to reflect this dual requirement.

**Why it matters**: This provides finer-grained control over network participation, allowing the governance system to temporarily restrict agents from consensus without full deregistration.

**Migration needed**: No action required - this is an internal change that affects consensus calculation.

*Tests*: Tests ensure proper filtering of whitelisted agents in consensus calculations.
*Cross-pallet impact*: The emission system now queries both the Torus0 pallet for registration status and the Governance pallet for whitelist status.

### Namespace Creation Event Granularity

**What changed**: Namespace creation and deletion now emit individual events for each namespace path rather than batch events.

**Why it matters**: This provides more detailed tracking of namespace operations and better supports indexing and monitoring systems that need to track individual namespace state changes.

**Migration needed**: Applications listening to namespace events should expect multiple events per operation when multiple paths are involved.

*Tests*: Tests verify proper event emission for both single and multiple namespace operations.
*Cross-pallet impact*: No direct impact on other pallets, but improves overall system observability.