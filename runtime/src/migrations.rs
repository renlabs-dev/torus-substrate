use alloc::vec::Vec;

use polkadot_sdk::{
    frame_support::{
        pallet_prelude::*,
        storage_alias,
        traits::{Currency, ExistenceRequirement, OnRuntimeUpgrade, StorageVersion},
    },
    frame_system,
    sp_weights::Weight,
};

#[cfg(feature = "try-runtime")]
use polkadot_sdk::sp_runtime::{TryRuntimeError, traits::Zero};

use crate::Runtime;

type AccountId = <Runtime as frame_system::Config>::AccountId;
type Agent = pallet_torus0::agent::Agent<Runtime>;

#[cfg(feature = "try-runtime")]
type Balance = <<Runtime as pallet_torus0::Config>::Currency as Currency<AccountId>>::Balance;

#[storage_alias(pallet_name)]
type Key = StorageValue<polkadot_sdk::pallet_sudo::Pallet<Runtime>, AccountId, OptionQuery>;

/// Moves the allocator role to the current sudo key.
pub struct MigrateAllocator;

impl OnRuntimeUpgrade for MigrateAllocator {
    fn on_runtime_upgrade() -> Weight {
        if StorageVersion::get::<pallet_governance::Pallet<Runtime>>() == StorageVersion::new(5) {
            if let Some(new_allocator) = sudo_key() {
                migrate_allocator(new_allocator);
            }

            StorageVersion::new(6).put::<pallet_governance::Pallet<Runtime>>();
        }

        Weight::zero()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
        let new_allocator = sudo_key();

        let old_allocators: Vec<AccountId> = pallet_governance::Allocators::<Runtime>::iter_keys()
            .filter(|allocator| Some(allocator) != new_allocator.as_ref())
            .collect();

        // Whether any prior allocator carries an agent identity to inherit. When
        // true, the migration is expected to register the sudo allocator and move
        // the prior allocators' stake to it.
        let old_agent_exists = old_allocators
            .iter()
            .any(pallet_torus0::Agents::<Runtime>::contains_key);

        let stake_to_old = sum_inbound_stake(&old_allocators);
        let prior_new_stake = new_allocator
            .as_ref()
            .map(|new| sum_inbound_stake(core::slice::from_ref(new)))
            .unwrap_or_else(Zero::zero);

        Ok((
            new_allocator,
            old_allocators,
            old_agent_exists,
            stake_to_old,
            prior_new_stake,
        )
            .encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
        let (new_allocator, old_allocators, old_agent_exists, stake_to_old, prior_new_stake): (
            Option<AccountId>,
            Vec<AccountId>,
            bool,
            Balance,
            Balance,
        ) = Decode::decode(&mut &state[..])
            .map_err(|_| "MigrateAllocator: failed to decode pre_upgrade state")?;

        ensure!(
            StorageVersion::get::<pallet_governance::Pallet<Runtime>>() == StorageVersion::new(6),
            "MigrateAllocator: governance storage version was not bumped to 6"
        );

        let Some(new_allocator) = new_allocator else {
            // No sudo key set: migration only bumps the storage version.
            return Ok(());
        };

        let allocators: Vec<AccountId> =
            pallet_governance::Allocators::<Runtime>::iter_keys().collect();
        ensure!(
            allocators == [new_allocator.clone()],
            "MigrateAllocator: sudo key is not the sole allocator"
        );

        ensure!(
            pallet_governance::Whitelist::<Runtime>::contains_key(&new_allocator),
            "MigrateAllocator: sudo allocator is not whitelisted"
        );

        for old_allocator in &old_allocators {
            ensure!(
                !pallet_torus0::Agents::<Runtime>::contains_key(old_allocator),
                "MigrateAllocator: a prior allocator agent is still registered"
            );
        }

        // When a prior allocator identity existed, the sudo allocator must have
        // been registered as an agent and must have received all of the prior
        // allocators' inbound stake.
        if old_agent_exists {
            ensure!(
                pallet_torus0::Agents::<Runtime>::contains_key(&new_allocator),
                "MigrateAllocator: sudo allocator was not registered as an agent"
            );

            let new_stake = sum_inbound_stake(core::slice::from_ref(&new_allocator));
            ensure!(
                new_stake == prior_new_stake.saturating_add(stake_to_old),
                "MigrateAllocator: prior allocator stake was not transferred to the sudo allocator"
            );
        }

        Ok(())
    }
}

/// Sums all stake delegated *to* the given agents (their inbound stake).
#[cfg(feature = "try-runtime")]
fn sum_inbound_stake(agents: &[AccountId]) -> Balance {
    agents
        .iter()
        .flat_map(pallet_torus0::StakedBy::<Runtime>::iter_prefix_values)
        .fold(Zero::zero(), |acc: Balance, amount| {
            acc.saturating_add(amount)
        })
}

fn sudo_key() -> Option<AccountId> {
    Key::get()
}

fn migrate_allocator(new_allocator: AccountId) {
    let old_allocators = pallet_governance::Allocators::<Runtime>::iter_keys()
        .filter(|allocator| allocator != &new_allocator)
        .collect::<Vec<_>>();

    let old_agent = old_allocators
        .iter()
        .find_map(pallet_torus0::Agents::<Runtime>::get);

    pallet_governance::Whitelist::<Runtime>::insert(&new_allocator, ());
    replace_allocators(&new_allocator);
    register_allocator(&new_allocator, old_agent.as_ref());
    transfer_allocator_stake(&old_allocators, &new_allocator);

    for old_allocator in old_allocators {
        let _ = pallet_torus0::agent::deregister::<Runtime>(old_allocator);
    }
}

fn register_allocator(new_allocator: &AccountId, old_agent: Option<&Agent>) {
    if pallet_torus0::Agents::<Runtime>::contains_key(new_allocator) {
        return;
    }

    if let Some(old_agent) = old_agent {
        fund_registration_burn(new_allocator);
        let _ = pallet_torus0::agent::register::<Runtime>(
            new_allocator.clone(),
            old_agent.name.to_vec(),
            old_agent.url.to_vec(),
            old_agent.metadata.to_vec(),
        );
    }
}

fn replace_allocators(new_allocator: &AccountId) {
    let _ = pallet_governance::Allocators::<Runtime>::clear(u32::MAX, None);
    pallet_governance::Allocators::<Runtime>::insert(new_allocator, ());
}

fn fund_registration_burn(new_allocator: &AccountId) {
    let _ = <Runtime as pallet_torus0::Config>::Currency::transfer(
        &pallet_governance::DaoTreasuryAddress::<Runtime>::get(),
        new_allocator,
        pallet_torus0::Burn::<Runtime>::get(),
        ExistenceRequirement::AllowDeath,
    );
}

fn transfer_allocator_stake(old_allocators: &[AccountId], new_allocator: &AccountId) {
    if !pallet_torus0::Agents::<Runtime>::contains_key(new_allocator) {
        return;
    }

    let stakes = pallet_torus0::StakingTo::<Runtime>::iter()
        .filter(|(_, staked, _)| old_allocators.contains(staked))
        .collect::<Vec<_>>();

    for (staker, old_allocator, amount) in stakes {
        let _ = pallet_torus0::stake::transfer_stake::<Runtime>(
            staker,
            old_allocator,
            new_allocator.clone(),
            amount,
        );
    }
}
