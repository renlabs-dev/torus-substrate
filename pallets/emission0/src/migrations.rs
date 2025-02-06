use polkadot_sdk::{
    frame_support::{
        migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
    },
    sp_runtime::Percent,
};

use crate::{Config, Pallet};

pub mod v1 {
    use super::*;
    use crate::{EmissionRecyclingPercentage, IncentivesRatio};

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            EmissionRecyclingPercentage::<T>::set(Percent::from_percent(81));
            IncentivesRatio::<T>::set(Percent::from_percent(30));

            Weight::zero()
        }
    }
}
