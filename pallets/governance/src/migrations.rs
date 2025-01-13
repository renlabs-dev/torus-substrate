pub mod next {
    use crate::{Config, GlobalGovernanceConfig, Pallet};
    use polkadot_sdk::frame_support::{
        migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
    };

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToNext<T>, Pallet<T>, W>;
    pub struct MigrateToNext<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToNext<T> {
        fn on_runtime_upgrade() -> Weight {
            GlobalGovernanceConfig::<T>::put(crate::config::GovernanceConfiguration::default());
            Weight::zero()
        }
    }
}
