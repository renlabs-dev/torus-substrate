# Spec 26 Changelog

This release introduces wallet stake delegation permissions, enabling secure delegation of staking operations to trusted accounts while maintaining control over funds. This feature creates a cold-hot wallet architecture for enhanced security.

## 1. Extrinsics

### permission0 pallet

- ```diff
  + #[pallet::call_index(11)]
  + pub fn delegate_wallet_stake_permission(
  +     origin: OriginFor<T>,
  +     recipient: T::AccountId,
  +     stake_details: WalletStake,
  +     duration: PermissionDuration<T>,
  +     revocation: RevocationTerms<T>,
  + ) -> DispatchResult
  ```
  Creates a wallet permission that delegates stake management capabilities to a recipient account. The delegator can specify whether the recipient can transfer stake between validators and whether the delegation is exclusive (blocking the delegator from managing their own stake). See [Wallet Stake Delegation](#wallet-stake-delegation).

- ```diff
  + #[pallet::call_index(12)]
  + pub fn execute_wallet_stake_permission(
  +     caller: OriginFor<T>,
  +     permission_id: PermissionId,
  +     op: WalletStakeOperation<T>,
  + ) -> DispatchResult
  ```
  Allows a permission recipient to execute delegated stake operations (unstaking or transferring stake between validators) on behalf of the delegator. The operation type and parameters are specified through the `WalletStakeOperation` enum. See [Wallet Stake Delegation](#wallet-stake-delegation).

## 2. Events

No new events were added in this release. The existing `PermissionDelegated` event is emitted when wallet permissions are created.

## 3. Storage Items

No new storage items were added. Wallet permissions are stored in the existing `Permissions` storage map with the new `Wallet` variant of `PermissionScope`.

## 4. Structs & Enums

### permission0 pallet

- ```diff
  + pub struct WalletScope<T: Config> {
  +     pub recipient: T::AccountId,
  +     pub r#type: WalletScopeType,
  + }
  ```
  Defines the wallet permission scope containing the recipient and the type of wallet operations they can perform.

- ```diff
  + pub enum WalletScopeType {
  +     Stake(WalletStake),
  + }
  ```
  Specifies the type of wallet operations that can be delegated. Currently only supports stake operations, but designed to be extensible for future wallet operation types.

- ```diff
  + pub struct WalletStake {
  +     pub can_transfer_stake: bool,
  +     pub exclusive_stake_access: bool,
  + }
  ```
  Configuration for stake delegation permissions. `can_transfer_stake` allows transferring stake between validators, while `exclusive_stake_access` blocks the delegator from managing their own stake while the permission is active.

- ```diff
  + pub enum WalletStakeOperation<T: Config> {
  +     Unstake { staked: T::AccountId, amount: BalanceOf<T> },
  +     Transfer { from: T::AccountId, to: T::AccountId, amount: BalanceOf<T> },
  + }
  ```
  Specifies the stake operation to execute when calling `execute_wallet_stake_permission`. Supports unstaking from a validator or transferring stake between validators.

- ```diff
    pub enum PermissionScope<T: Config> {
        Stream(StreamScope<T>),
        Curator(CuratorScope<T>),
        Namespace(NamespaceScope<T>),
  +     Wallet(WalletScope<T>),
    }
  ```
  Extended the permission scope enum to include the new `Wallet` variant for wallet operation delegations.

### torus0 pallet

- ```diff
    pub enum Error<T> {
        ...
  +     StakeIsDelegated,
    }
  ```
  New error returned when a user attempts to manage their stake directly while an exclusive wallet permission is active.

## Behavior Changes

### Wallet Stake Delegation

**What changed**: A new permission system allows accounts to delegate their staking operations to trusted recipients. The delegator can grant permission to unstake tokens and optionally transfer stake between validators. When exclusive access is granted, the delegator cannot perform stake operations directly.

**Why it matters**: This feature enables cold-hot wallet architectures where high-value accounts can remain offline while still participating in network staking. Cold wallets delegate staking management to hot wallets without exposing funds to transfer risks. If a hot wallet is compromised, attackers can only manage existing stakes, not drain accounts.

**Migration needed**: No migration required. Existing staking operations continue to work normally unless users explicitly create wallet permissions.

*Tests*: Tests validate permission creation, exclusive access enforcement, and proper execution of delegated operations.
*Cross-pallet impact*: The torus0 pallet now checks for active wallet permissions before allowing direct stake operations, integrating with the permission0 pallet's wallet permission system.

### Stake Operation Access Control

**What changed**: The `remove_stake` and `transfer_stake` extrinsics in the torus0 pallet now check for exclusive wallet permissions before allowing operations. If an exclusive stake permission exists for the account, direct stake management is blocked with the `StakeIsDelegated` error.

**Why it matters**: This ensures that when exclusive delegation is active, only the designated recipient can manage stakes, preventing conflicts between delegator and recipient operations and maintaining security guarantees.

**Migration needed**: None. The check only applies to accounts that create exclusive wallet permissions.

*Tests*: Integration tests verify that exclusive permissions properly block direct stake operations.
*Cross-pallet impact*: Torus0 now depends on Permission0WalletApi trait to query active wallet permissions.

### Permission System Extension

**What changed**: The permission system's `PermissionScope` enum was extended with a `Wallet` variant, and the permission cleanup, revocation, and execution logic was updated to handle wallet permissions alongside existing stream, curator, and namespace permissions.

**Why it matters**: The permission system is now more versatile, supporting not just emission and governance delegations but also operational delegations for wallet management. This creates a foundation for future wallet operation types beyond staking.

**Migration needed**: The migration from v5 to v7 is already complete (spec 25). No additional migration needed for spec 26.

*Tests*: Existing permission system tests extended to cover wallet permission lifecycle.
*Cross-pallet impact*: All pallets using the permission system can now interact with wallet permissions through the unified API.