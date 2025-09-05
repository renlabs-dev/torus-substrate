# Wallet Scope Implementation - Stake Delegation

## Overview

The Wallet scope introduces secure delegation of wallet operations, starting with staking as the initial implementation. This feature enables cold-hot wallet behavior, where high-value accounts can delegate specific operations to more accessible accounts without exposing private keys. This significantly improves security by allowing cold storage accounts to remain offline while still participating in network staking activities.

## Security Motivation

Traditional blockchain accounts face a dilemma: keep keys accessible for regular operations (increasing theft risk) or store them securely offline (losing operational flexibility). The Wallet scope solves this by allowing a cold wallet to delegate specific permissions to a hot wallet, maintaining security while enabling day-to-day operations.

For staking specifically, this means:
- Cold wallets can delegate staking management without exposing funds to transfer risks
- Hot wallets can optimize staking positions without accessing the principal
- Compromised hot wallets cannot drain accounts, only manage existing stakes

## Implementation Structure

### Core Types

The Wallet scope follows the permission system's modular design:

```rust
pub enum PermissionScope {
    Curator(CuratorScope),
    Namespace(NamespaceScope),
    Stream(StreamScope),
    Wallet(WalletScope),  // New scope type
}

pub struct WalletScope {
    pub recipient: AccountId,
    pub wallet_type: WalletPermissionType,
}

pub enum WalletPermissionType {
    Stake(StakePermissions),
    // Future: Transfer, Governance, etc.
}

pub struct StakePermissions {
    pub can_unstake: bool,
    pub can_transfer_stake: bool,
}
```

### Delegation

Creating a wallet delegation is straightforward:

```rust
pub fn delegate_wallet_permission(
    origin: OriginFor<T>,
    recipient: T::AccountId,
    wallet_type: WalletPermissionType,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    enforcement: EnforcementAuthority<T>,
)
```

### Execution

The recipient executes delegated operations through a dedicated extrinsic:

```rust
pub fn execute_stake_permission(
    origin: OriginFor<T>,
    permission_id: PermissionId,
    operation: StakeOperation,
)

pub enum StakeOperation {
    Unstake { 
        validator: AccountId, 
        amount: Balance 
    },
    TransferStake { 
        from_validator: AccountId,
        to_validator: AccountId, 
        amount: Balance 
    },
}
```

## Integration Points

The permission system integrates with the torus0 pallet's staking module:

```rust
// Check for delegated permissions before allowing stake operations
fn can_perform_stake_operation<T: Config>(
    account: &T::AccountId,
    operation: &StakeOperationType,
) -> Result<Option<T::AccountId>, Error<T>> {
    if let Some(permission) = Permission0Api::get_active_wallet_permission(
        account, 
        WalletType::Stake
    ) {
        // Operation must be performed by the authorized recipient
        Ok(Some(permission.recipient))
    } else {
        // Account performs its own operations
        Ok(None)
    }
}
```

## Behavior and Constraints

When a stake delegation is active:
1. The delegator cannot directly perform staking operations
2. Only the designated recipient can manage the delegator's stakes
3. The recipient can unstake from any validator
4. The recipient can transfer stake between validators
5. All operations affect only existing stakes (no new staking from balance)

## Revocation

The Wallet scope supports the standard revocation patterns:
- **Immediate**: Permission ends instantly
- **Delayed**: Permission expires after a set period
- **Conditional**: Permission ends when specific conditions are met

This ensures delegators maintain ultimate control over their permissions while providing flexibility for different use cases.

## Future Extensions

While starting with stake management, the Wallet scope architecture supports future operation types:
- Transfer permissions (with amount and recipient limits)
- Governance voting delegation
- Fee payment authorization
- Contract interaction permissions

Each new type will follow the same security-first approach, ensuring delegators never lose ultimate control of their assets.