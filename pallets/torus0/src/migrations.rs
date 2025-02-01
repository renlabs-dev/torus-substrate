use crate::Config;
use crate::Pallet;
use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade,
};

pub mod _v1 {
    use super::*;

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {}
}
