use polkadot_sdk::{
    frame_support::{
        migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
    },
    sp_runtime::Percent,
};

use crate::{Config, Pallet};

pub mod v1 {
    use super::*;

    use crate::{Agent, Agents};

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            Agents::<T>::translate(|_key, mut agent: Agent<T>| {
                agent.weight_penalty_factor = Percent::from_percent(0);
                Some(agent)
            });
            Weight::zero()
        }
    }
}

pub mod v2 {
    use super::*;

    use crate::{Agent, Agents};

    pub type Migration<T, W> = VersionedMigration<0, 2, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            Agents::<T>::translate(|_key, mut agent: Agent<T>| {
                agent.fees = Default::default();
                Some(agent)
            });
            Weight::zero()
        }
    }
}
