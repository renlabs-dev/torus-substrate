use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
};

use crate::{Config, Pallet};

pub mod v3 {
    use super::*;

    pub type Migration<T, W> = VersionedMigration<2, 3, MigrateToV3<T>, Pallet<T>, W>;
    pub struct MigrateToV3<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV3<T> {
        fn on_runtime_upgrade() -> Weight {
            Weight::zero()
        }
    }
}
