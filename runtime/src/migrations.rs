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

use crate::Runtime;

type AccountId = <Runtime as frame_system::Config>::AccountId;
type Agent = pallet_torus0::agent::Agent<Runtime>;

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
