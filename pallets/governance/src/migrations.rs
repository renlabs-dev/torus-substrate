use polkadot_sdk::{
    frame_support::{
        migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
    },
    sp_runtime::Percent,
};

use crate::{Config, Pallet};

pub type Migrations<T, W> = (v1::Migration<T, W>, v2::Migration<T, W>);

pub mod v2 {
    use super::*;

    use crate::TreasuryEmissionFee;

    pub type Migration<T, W> = VersionedMigration<1, 2, MigrateToV2<T>, Pallet<T>, W>;
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            // Set Treasury emission fee to 5%
            TreasuryEmissionFee::<T>::put(Percent::from_percent(5));

            Weight::zero()
        }
    }
}

pub mod v1 {
    use super::*;

    use crate::GlobalGovernanceConfig;

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            GlobalGovernanceConfig::<T>::put(crate::config::GovernanceConfiguration::default());
            Weight::zero()
        }
    }
}
