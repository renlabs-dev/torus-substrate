pub mod v2 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_weights::Weight,
    };

    use crate::{Config, Pallet};

    pub type Migration<T, W> = VersionedMigration<1, 2, MigrateToV2<T>, Pallet<T>, W>;
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            Weight::zero()
        }
    }
}

pub mod v3 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_weights::Weight,
    };

    use crate::{Config, Pallet};

    pub type Migration<T, W> = VersionedMigration<2, 3, MigrateToV3<T>, Pallet<T>, W>;
    pub struct MigrateToV3<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV3<T> {
        fn on_runtime_upgrade() -> Weight {
            Weight::zero()
        }
    }
}
