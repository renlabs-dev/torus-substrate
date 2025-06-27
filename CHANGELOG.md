# Changelog

## Spec 2

- fix: expired expire applications only if they are Open
- fix: `torus0` on_initialize hook

## Spec 12

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

We moved the `StakingTo` and `StakedBy` removals **after** the call to `remove_stake`, ensuring the unstake happens first.

#### Reimbursement

All balances lost to this bug have been restored. Funds were sent to a recovery account, then redistributed to the original stakers via script:

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

#### Migration script

You can find the migration here:
[https://github.com/renlabs-dev/torus-substrate/blob/main/pallets/torus0/src/migrations.rs#L68](https://github.com/renlabs-dev/torus-substrate/blob/main/pallets/torus0/src/migrations.rs#L68)

The account address was **not** hard-coded. The script calculates the difference between `TotalStake` (which remained unchanged) and the sum of all actual stakes, then transfers that difference back to each staker.

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
- Applied saturating arithmetic in various runtime and pallet modules to prevent overflow issues.
- Improved arithmetic safety and robustness in emission and governance modules.
- Refined author identification and gas limit calculations with safer arithmetic operations.

