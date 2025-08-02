use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
};

use crate::{Config, Pallet};

pub mod v4 {
    use super::*;
    use crate::{Agent, Agents};
    use scale_info::prelude::vec::Vec;

    pub mod storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::frame_support::{DebugNoBound, Identity, storage_alias};
        use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
        use polkadot_sdk::sp_runtime::{BoundedVec, Percent};
        use scale_info::TypeInfo;

        use crate::AccountIdOf;

        #[derive(DebugNoBound, Encode, Decode, MaxEncodedLen, TypeInfo)]
        #[scale_info(skip_type_params(T))]
        pub struct Agent<T: crate::Config> {
            pub key: AccountIdOf<T>,
            pub name: BoundedVec<u8, T::MaxAgentNameLengthConstraint>,
            pub url: BoundedVec<u8, T::MaxAgentUrlLengthConstraint>,
            pub metadata: BoundedVec<u8, T::MaxAgentMetadataLengthConstraint>,
            pub weight_penalty_factor: Percent,
            pub registration_block: BlockNumberFor<T>,
            pub fees: crate::fee::ValidatorFee<T>,
        }

        #[storage_alias]
        pub type Agents<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, AccountIdOf<T>, Agent<T>>;
    }

    pub type Migration<T, W> = VersionedMigration<3, 4, MigrateToV4<T>, Pallet<T>, W>;
    pub struct MigrateToV4<T>(core::marker::PhantomData<T>);
    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV4<T> {
        fn on_runtime_upgrade() -> Weight {
            let old_agents = storage::Agents::<T>::iter().collect::<Vec<_>>();
            let _ = storage::Agents::<T>::clear(u32::MAX, None);

            for (id, old_agent) in old_agents {
                Agents::<T>::insert(
                    id,
                    Agent {
                        key: old_agent.key,
                        name: old_agent.name,
                        url: old_agent.url,
                        metadata: old_agent.metadata,
                        registration_block: old_agent.registration_block,
                        weight_penalty_factor: old_agent.weight_penalty_factor,
                        fees: old_agent.fees,
                        last_update_block: old_agent.registration_block,
                    },
                )
            }

            Weight::zero()
        }
    }
}

pub mod v5 {
    use pallet_torus0_api::NamespacePath;
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_tracing::{error, info},
        sp_weights::Weight,
    };

    use crate::{
        Agents, BurnConfig, Config, Pallet, burn::BurnConfiguration, namespace::NamespaceOwnership,
    };

    pub mod storage {
        use polkadot_sdk::frame_support::{pallet_prelude::*, storage_alias};

        use crate::AccountIdOf;

        #[storage_alias]
        pub type Namespaces<T: crate::Config> = StorageDoubleMap<
            crate::Pallet<T>,
            Blake2_128Concat,
            AccountIdOf<T>,
            Blake2_128Concat,
            pallet_torus0_api::NamespacePath,
            crate::namespace::NamespaceMetadata<T>,
        >;

        #[storage_alias]
        pub type NamespaceCount<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Blake2_128Concat, AccountIdOf<T>, u32, ValueQuery>;
    }

    pub type Migration<T, W> = VersionedMigration<4, 5, MigrateToV5<T>, Pallet<T>, W>;
    pub struct MigrateToV5<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV5<T> {
        fn on_runtime_upgrade() -> Weight {
            BurnConfig::<T>::set(BurnConfiguration {
                min_burn: 15000000000000000000,   // 15tors
                max_burn: 1000000000000000000000, // 1000tors
                max_registrations_per_interval: 16,
                ..BurnConfig::<T>::get()
            });

            let _ = storage::Namespaces::<T>::clear(u32::MAX, None);
            let _ = storage::NamespaceCount::<T>::clear(u32::MAX, None);

            let path = NamespacePath::agent_root();
            #[allow(deprecated)]
            if let Err(err) =
                crate::namespace::create_namespace0::<T>(NamespaceOwnership::System, path, false)
            {
                error!("failed to create root agent namespace: {err:?}");
                return Weight::default();
            }

            info!("created root agent namespace");

            for (id, agent) in Agents::<T>::iter() {
                let old_name = agent.name.clone();
                let Ok(agent_name) = core::str::from_utf8(&agent.name) else {
                    error!("agent name is not utf-8: {:?}", agent.name);
                    continue;
                };

                let agent_name = agent_name.trim().to_ascii_lowercase().replace(' ', "-");

                let Ok(bounded_name) = agent_name.as_bytes().to_vec().try_into() else {
                    error!("cannot lower case agent {agent_name:?}");
                    continue;
                };

                let path = match NamespacePath::new_agent_root(agent_name.as_bytes()) {
                    Ok(path) => path,
                    Err(err) => {
                        error!("cannot create path for agent {agent_name:?}: {err:?}");
                        continue;
                    }
                };

                Agents::<T>::mutate_extant(id.clone(), |agent| {
                    agent.name = bounded_name;
                });

                #[allow(deprecated)]
                if let Err(err) = crate::namespace::create_namespace0::<T>(
                    NamespaceOwnership::Account(id.clone()),
                    path.clone(),
                    false,
                ) {
                    error!("cannot create namespace for agent {agent_name:?}: {err:?}");

                    Agents::<T>::mutate_extant(id.clone(), |agent| {
                        agent.name = old_name;
                    });
                } else {
                    info!("created namespace entry for agent {agent_name:?}: {path:?}");
                }
            }

            Weight::default()
        }
    }
}

pub mod v6 {

    use polkadot_sdk::{
        frame_support::{
            migrations::VersionedMigration,
            traits::{Currency, Imbalance, NamedReservableCurrency, UncheckedOnRuntimeUpgrade},
        },
        sp_core::Get,
        sp_std::collections::btree_set::BTreeSet,
        sp_tracing::{error, info, warn},
        sp_weights::Weight,
    };

    use crate::{Config, Pallet, stake::STAKE_IDENTIFIER};

    pub type Migration<T, W> = VersionedMigration<5, 6, MigrateToV6<T>, Pallet<T>, W>;
    pub struct MigrateToV6<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV6<T> {
        fn on_runtime_upgrade() -> Weight {
            let ed = T::ExistentialDeposit::get();
            let mut minted = 0u128;

            let total_stake_before = crate::TotalStake::<T>::get();
            let total_issuance_before = T::Currency::total_issuance();
            let total_tokens_before = total_stake_before.saturating_add(total_issuance_before);

            for (staker, staked, original_stake) in crate::StakingTo::<T>::iter() {
                let free = T::Currency::free_balance(&staker);

                let mint = original_stake;
                let stake = if free < ed {
                    original_stake.checked_sub(ed).unwrap_or_default()
                } else {
                    original_stake
                };

                let imbalance = T::Currency::deposit_creating(&staker, mint);
                if imbalance.peek() != mint {
                    error!(
                        "failed to mint {mint} stake tokens for account {staker:?}: actual {}",
                        imbalance.peek()
                    );
                    continue;
                }

                if stake > 0 {
                    if let Err(err) = T::Currency::reserve_named(STAKE_IDENTIFIER, &staker, stake) {
                        error!(
                            "failed to reserve {stake} stake tokens for account {staker:?}: {err:?}"
                        );
                    }
                }

                if original_stake != stake {
                    let diff = mint.saturating_sub(stake);
                    warn!(
                        "updating total stake for a difference of {diff} tokens for stake {staker:?} -> {staked:?}",
                    );

                    crate::TotalStake::<T>::mutate(|total| {
                        *total = total.saturating_sub(diff);
                    });
                    crate::StakingTo::<T>::mutate(&staker, &staked, |total| {
                        if let Some(total) = total {
                            *total = stake;
                        }
                    });
                    crate::StakedBy::<T>::mutate(&staked, &staker, |total| {
                        if let Some(total) = total {
                            *total = stake;
                        }
                    });
                }

                minted = minted.saturating_add(mint);
            }

            let total_staking_to: u128 = crate::StakingTo::<T>::iter_values().sum();
            let total_staked_by: u128 = crate::StakedBy::<T>::iter_values().sum();
            let total_reserved: u128 = crate::StakingTo::<T>::iter_keys()
                .map(|(staker, _)| staker)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|staker| T::Currency::reserved_balance_named(STAKE_IDENTIFIER, &staker))
                .sum();

            let total_stake_after = crate::TotalStake::<T>::get();
            let total_issuance_after = T::Currency::total_issuance();

            info!(
                "total stake minted: {minted}. total issuance: {total_issuance_after}. total issuance before: {total_issuance_before}"
            );

            if total_issuance_after != total_tokens_before {
                error!("total issuance does not match total tokens before migration");
            }

            if total_staking_to != total_staked_by {
                error!("total staking to does not match total staked by");
            }

            if total_staking_to != total_stake_after {
                error!("total staking to does not match total staked after");
            }

            if total_reserved != total_stake_after {
                error!(
                    "total reserved balance does not match total tracked state: {total_reserved} != {total_stake_after}"
                );
            }

            Weight::default()
        }
    }
}
