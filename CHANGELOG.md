# Changelog

## Spec 23

This release builds upon the permission delegation system with enhanced re-delegation capabilities, enabling more sophisticated hierarchical permission structures for curators and namespace management.

### Major Features

#### Curator Re-delegation

Curators can now re-delegate their permissions to other agents, creating a hierarchical permission structure:

- The implementation tracks parent-child relationships between permissions and enforces inheritance rules where child permissions cannot exceed parent permission scopes.
- Enables more flexible governance structures where curators can delegate specific subsets of their permissions to trusted agents, creating delegation chains.
- Particularly useful for scaling curator operations and distributing workload while maintaining accountability through the permission hierarchy.
- Existing curator permissions are automatically migrated to the new structure with their flags mapped to a single entry with no parent permission ID.

#### Namespace Re-delegation

Added support for namespace permission re-delegation with depth limiting:

- Agents can now delegate namespace permissions they've received to other agents, creating delegation chains up to 5 levels deep.
- The system validates that child namespace paths exist and are properly owned before allowing delegation.
- Allows namespace owners to create more complex delegation structures for namespace management.
- Organizations can delegate namespace control to team leads who can further delegate to team members, enabling hierarchical namespace administration.
- No migration needed for existing namespace permissions as they remain compatible with the new system.

#### Permission Instance Tracking

All permission types now support instance tracking through the instances parameter:

- Allows delegators to specify how many times a permission can be used concurrently, providing better control over permission usage.
- Prevents permission abuse by limiting concurrent usage and provides better resource management for delegated operations.
- Particularly important for curator and namespace permissions where parallel operations could cause conflicts.
- Existing permissions are migrated with a default instance count of 1, maintaining current behavior.

### Infrastructure Changes

#### Storage Migration v5

Migrated the CuratorScope structure to support the new hierarchical permission model:

- The migration transforms the flat CuratorPermissions flags into a BoundedBTreeMap structure where existing permissions are mapped with no parent permission ID.
- Ensures backward compatibility while enabling the new curator re-delegation features.
- All existing curator permissions continue to work as before while gaining the ability to be re-delegated.
- The migration runs automatically during runtime upgrade and is transparent to other pallets.

### Configuration Updates

- Added MaxCuratorSubpermissionsPerPermission parameter to limit the number of curator sub-permissions that can be delegated in a single permission contract (set to 16).
- Added new error types to handle curator permission limits and namespace delegation depth restrictions.

This release enhances the permission system's flexibility while maintaining security through proper validation and depth limiting, enabling more sophisticated governance and namespace management structures.

## Spec 22

This release introduces a major redesign of the permission system from a grant-based to a delegation-based model, along with significant improvements to staking, agent registration, and emission distribution.

### Major Features

#### Permission Delegation System Redesign

The entire permission system has been redesigned around delegation rather than granting:

- All functions, events, and storage items renamed from "grant/grantor/grantee" to "delegate/delegator/recipient" terminology.
- Better reflects the actual relationship between participants - one party delegates authority to another rather than simply granting permissions.
- Improves the mental model for developers and users interacting with the permission system.
- All client applications must update their calls to use the new extrinsic names and parameter names.

#### Hierarchical Namespace Permissions

Namespace permissions now support hierarchical delegation through a parent-child relationship system:

- The NamespaceScope structure changed from a simple set of paths to a map of parent permission IDs to path sets.
- Permission contracts now track children and have instance limits.
- Enables sophisticated permission delegation chains where a recipient of namespace permissions can re-delegate subsets of those permissions to other parties.
- The instance system prevents over-delegation by limiting how many active delegations can exist from a single permission.

#### Staking System Overhaul

The staking system now uses named reserves instead of withdrawing and issuing tokens:

- Stakes are tracked using the Balances pallet's named reserve functionality with identifier "torstake".
- Improves the economic model by ensuring staked tokens remain as part of the staker's balance but are properly reserved and cannot be spent.
- Ensures the total issuance remains consistent and stakes are properly tracked by the underlying currency system.
- The v6 migration automatically handles the conversion of existing stakes to the new reserve-based system.

#### Agent Registration Simplification

The register_agent extrinsic no longer takes a separate agent_key parameter:

- The agent key is now always the transaction signer.
- The registration process also now checks for duplicate agent names in addition to duplicate keys.
- Simplifies the registration process and ensures stronger consistency between the transaction signer and the registered agent.
- Name uniqueness prevents confusion and impersonation attempts.

#### Enhanced Emission Distribution Tracking

The emission distribution system now emits more granular events:

- Individual EmissionDistribution events for each target and AccumulatedEmission events when tokens are accumulated for permissions.
- Distribution functions now return DispatchResult and handle errors gracefully.
- Provides better observability into the emission system, allowing external systems to track exactly how tokens flow through the network.
- Error handling prevents the emission system from silently failing.

#### Whitelist-Based Consensus Participation

The emission system now considers agents to be eligible for consensus only if they are both registered and whitelisted:

- The registered field in ConsensusMemberInput was renamed to whitelisted to reflect this dual requirement.
- Provides finer-grained control over network participation.
- Allows the governance system to temporarily restrict agents from consensus without full deregistration.

#### Namespace Creation Event Granularity

Namespace creation and deletion now emit individual events for each namespace path rather than batch events:

- Provides more detailed tracking of namespace operations.
- Better supports indexing and monitoring systems that need to track individual namespace state changes.
- Applications listening to namespace events should expect multiple events per operation when multiple paths are involved.

### API Changes

- The Torus0Api stake_to method now returns DispatchResult instead of Result<(), Balance> for consistent error handling.
- Added find_agent_by_name method to the Torus0Api to support name-based agent lookups.
- Added agent_name method to NamespacePath to extract agent names from namespace paths.

This release represents a fundamental shift in how permissions are conceptualized and managed within the Torus Network, providing clearer semantics and more powerful delegation capabilities.

## Spec 21

This release introduces major architectural changes to enable decentralized economic relationships, improved governance flexibility, and preparation for off-chain service integration. The changes span several months of development focused on creating a more sophisticated and scalable network.

### Major Features

#### Permission System (permission0 pallet)

A comprehensive permission framework that revolutionizes how agents interact economically within the network:

- Agents can now share their token emissions with contributors through recursive permission trees. Supports percentage-based streams or fixed amounts, with flexible distribution controls (manual, automatic threshold, scheduled).
- Permissions can be managed by arbiters (multi-sig groups) for enhanced security and decentralized control.
- Governance powers are now granular with specific permissions for application review, whitelist management, and penalty control. Replaces the old curator system with a more flexible approach.
- Built-in protections against double-spending and infinite recursion ensure tokenomics.

#### Namespace System

A hierarchical naming system that functions as decentralized DNS/capability listing for agent services:

- Agents can create dot-separated paths like `agent.alice.api.v2` to organize their off-chain services.
- Owning a namespace gives control over all sub-paths, enabling structured delegation of services.
- Base fees go to treasury while refundable deposits incentivize efficient storage use.
- Namespaces can be delegated through the permission system, enabling complex service architectures.

#### Agent Management Improvements

- Agent names are automatically converted to namespace-compliant format (lowercase, no spaces) during the upgrade.
- Curators can now freeze/unfreeze agents, adding a new governance tool for spam control.
- Agent metadata updates now have cooldowns to prevent spam and ensure stability.
- Burn parameters adjusted for better economic balance during high registration activity.

#### Economic Enhancements

- All transaction fees now flow to the treasury instead of being burned, providing sustainable funding for network development.
- Only whitelisted agents receive emissions, ensuring quality participants chosen by the DAO are rewarded.
- Permission holders can dynamically adjust how they distribute received emissions.
- Distribution remainders are now properly accumulated, preventing token loss.

#### Developer Experience

- Proof-of-work based faucet for obtaining testnet tokens, preventing abuse while maintaining accessibility.
- Comprehensive docs for all pallets now auto-published to GitHub Pages.
- New `torus0_namespacePathCreationCost` RPC for calculating namespace fees upfront.
- Integrated coverage reporting with `cargo xtask coverage` for maintaining code quality.

### Bug Fixes & Safety Improvements

- Critical fix ensuring stakes are properly refunded before clearing storage during de-registration.
- Banned arithmetic side effects throughout the runtime, using saturating operations to prevent overflows.
- All agents now automatically delegate weights to allocators, simplifying the onboarding process.

### Governance Changes

- Instead of all-or-nothing curator status, specific permissions can be granted.
- Improved with better event emissions and clearer execution flows.
- Now integrated with the permission system for more flexible control.

### Infrastructure

- Enhanced GitHub Actions for automated testing, documentation, and Docker builds.
- Added development container configuration for consistent environments.
- Migrated CI infrastructure for better performance.

This release represents a fundamental evolution of the Torus Network, creating the foundation for a truly decentralized economy where agents can form complex relationships, share resources, and organize their services in meaningful ways.

## Spec version 12

### Critical Migration & Bug Fix

We were alerted that the agent

<https://torex.rs/account/5GvBntw5j45K7kMwj9XahfwEW7ByJHRNPrSFmBzUyHcnaYNT>

had been deregistered and that all stake delegated to it appeared to be "missing".

#### Root cause

The issue was introduced in the following code:

```rs
pub(crate) fn clear_key<T: crate::Config>(key: &AccountIdOf<T>) -> DispatchResult {
    for (staker, staked) in crate::StakingTo::<T>::iter() {
        if &staker == key || &staked == key {
            crate::StakingTo::<T>::remove(&staker, &staked);
            crate::StakedBy::<T>::remove(&staked, &staker);
            if let Err(err) = remove_stake::<T>(
                key: staker.clone(),
                agent_key: staked.clone(),
                amount,
            ) {
                error!(
                    "could not remove stake from {:?} to {:?}: {err:?}",
                    staker, staked
                );
            }
        }
    }
    Ok(())
}
```

`StakingTo` and `StakedBy` were deleted **before** the call to `remove_stake`.
Because those storage items no longer existed, `remove_stake` exited early and the affected stake was effectively erased.

#### Fix

The code was changed entirely to a generic function that correctly removes stake, remove_stake0

#### Migration script

You can find the migration here:
[https://github.com/renlabs-dev/torus-substrate/blob/954a2485badc49c1ed35e8015b396fb8bad8e4c7/pallets/torus0/src/migrations.rs#L68](https://github.com/renlabs-dev/torus-substrate/blob/954a2485badc49c1ed35e8015b396fb8bad8e4c7/pallets/torus0/src/migrations.rs#L68)

The migration calculates the difference between `TotalStake` (which remained **correctly** unchanged) and sums all actual stakes on the network. The difference is then sent to a recovery account, and redistributed to the original stakers via script

**Original Stakers (OS)**

```text
5CuBSdUuBeLVxtzrTtrdiCjipEgjbvUoMJjxrss4T9f1MEoZ: 178467095451535513057748
5CwHfGwRTnUuZkFG4M5x5ZaXEgmn8RfaiG7Zy7RYUceKcwZT: 13425244249634765265782
5DJNtxnttEeuRiMErC6V6CR8z4BXEksKFpBjBukGeEWDxN9g: 299250156775756214045518
5EUPK5qrbFU5wFzfAMe1xVT4SLAV53YgeeRB97Z4aPYZUWQk: 76583203738628718841313
5FEGvoqFHUSYSFd1mYbpbasZNatsKfMeMPLGzwTwk276QBP9: 1024384116344227564972
5FNRQAB4xzpru9Gcu4gGe58FivaBZNCGiLJQ7xfCYUimMVLR: 42796457538016798301186
5Giu5U5XeJuYtTeSWTYq5yp9rooSy3NGTgEh9i2cghfhPLsW: 207369620960478104891376
5Gubvc4bG9LzzxWBtx6MWXgHb27YvMKPjJ99YSyfMh1hf4RN: 207368595393608853664012
5GvBntw5j45K7kMwj9XahfwEW7ByJHRNPrSFmBzUyHcnaYNT: 8826118395848226775843
```

### Refactor

- Improved clarity and maintainability of stake removal logic.
- Enhanced parameter naming for better readability.
- Modified operation order in agent unregistration for consistency.

### Tests

- Revised and renamed tests to better reflect updated agent unregistration and staking behaviors.

### Chores

- Updated runtime and storage version numbers to reflect the latest changes.
- Introduced workspace-wide linting configurations for consistent code quality.
- Enforced stricter Clippy lint rules across the workspace.

## Spec version 2

- fix: expired expire applications only if they are Open
- fix: `torus0` on_initialize hook
- Applied saturating arithmetic in various runtime and pallet modules to prevent overflow issues.
- Improved arithmetic safety and robustness in emission and governance modules.
- Refined author identification and gas limit calculations with safer arithmetic operations.
