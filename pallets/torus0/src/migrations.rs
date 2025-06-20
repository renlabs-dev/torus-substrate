use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
};

use crate::{Config, Pallet};

pub mod v3 {
    use super::*;
    use crate::{Agent, Agents};
    use scale_info::prelude::vec::Vec;

    pub mod storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::frame_support::{storage_alias, DebugNoBound, Identity};
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

    pub type Migration<T, W> = VersionedMigration<2, 3, MigrateToV3<T>, Pallet<T>, W>;
    pub struct MigrateToV3<T>(core::marker::PhantomData<T>);
    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV3<T> {
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

pub mod v4 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_weights::Weight,
    };

    use crate::{Config, Pallet};

    pub type Migration<T, W> = VersionedMigration<3, 4, MigrateToV4<T>, Pallet<T>, W>;
    pub struct MigrateToV4<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV4<T> {
        fn on_runtime_upgrade() -> Weight {
            Weight::zero()
        }
    }
}

pub mod v5 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_weights::Weight,
    };

    use crate::{burn::BurnConfiguration, BurnConfig, Config, Pallet};

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

            Weight::default()
        }
    }
}
