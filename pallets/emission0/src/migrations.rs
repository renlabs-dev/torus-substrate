use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
};

use crate::{Config, Pallet};

pub mod v2 {
    use polkadot_sdk::{sp_std::collections::btree_set::BTreeSet, sp_tracing::error};

    use pallet_emission0_api::Emission0Api;
    use pallet_governance_api::GovernanceApi;
    use pallet_torus0_api::Torus0Api;

    use super::*;

    pub type Migration<T, W> = VersionedMigration<1, 2, MigrateToV2<T>, Pallet<T>, W>;
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let allocators: BTreeSet<_> = <T::Governance>::get_allocators().collect();
            if let Some(allocator) = allocators.first() {
                for agent in <T::Torus>::agent_ids() {
                    if allocators.contains(&agent) {
                        continue;
                    }

                    if let Err(err) =
                        <Pallet<T> as Emission0Api<T::AccountId>>::delegate_weight_control(
                            &agent, allocator,
                        )
                    {
                        error!(
                            "failed to delegate weight control from {agent:?} to {allocator:?}: {err:?}"
                        );
                    }
                }
            } else {
                error!("no allocators available");
            }

            Weight::zero()
        }
    }
}
