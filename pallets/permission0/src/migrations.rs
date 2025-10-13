use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, pallet_prelude::*, traits::UncheckedOnRuntimeUpgrade,
};

use crate::{Config, PermissionId, Permissions};

pub mod v8 {

    use super::*;

    pub type MigrationToV8<T, W> = VersionedMigration<7, 8, MigrateToV8<T>, crate::Pallet<T>, W>;

    pub struct MigrateToV8<T>(core::marker::PhantomData<T>);

    mod old {
        use codec::{Decode, Encode};
        use polkadot_sdk::{
            polkadot_sdk_frame::prelude::BlockNumberFor,
            sp_runtime::{BoundedBTreeMap, BoundedBTreeSet},
        };

        use super::*;
        use crate::permission::*;

        #[derive(Encode, Decode)]
        pub struct StreamScope<T: Config> {
            pub recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
            pub allocation: StreamAllocation<T>,
            pub distribution: DistributionControl<T>,
            pub accumulating: bool,
            pub recipient_managers: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
            pub weight_setters: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
        }

        #[derive(Encode, Decode)]
        pub enum PermissionScope<T: Config> {
            Stream(StreamScope<T>),
            Curator(CuratorScope<T>),
            Namespace(NamespaceScope<T>),
            Wallet(wallet::WalletScope<T>),
        }

        #[derive(Encode, Decode)]
        pub struct PermissionContract<T: Config> {
            pub delegator: T::AccountId,
            pub scope: PermissionScope<T>,
            pub duration: PermissionDuration<T>,
            pub revocation: RevocationTerms<T>,
            pub enforcement: EnforcementAuthority<T>,
            pub last_update: BlockNumberFor<T>,
            pub last_execution: Option<BlockNumberFor<T>>,
            pub execution_count: u32,
            pub created_at: BlockNumberFor<T>,
        }
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV8<T> {
        fn on_runtime_upgrade() -> Weight {
            let mut weight = Weight::zero();
            let mut translated = 0u64;

            Permissions::<T>::translate(|_key: PermissionId, old: old::PermissionContract<T>| {
                weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
                translated = translated.saturating_add(1);

                let new_scope = match old.scope {
                    old::PermissionScope::Stream(old_stream) => {
                        crate::PermissionScope::Stream(crate::StreamScope {
                            recipients: old_stream.recipients,
                            allocation: old_stream.allocation,
                            distribution: old_stream.distribution,
                            accumulating: old_stream.accumulating,
                            recipient_managers: old_stream.recipient_managers,
                            weight_setters: old_stream.weight_setters,
                            funnels: BoundedVec::default(),
                        })
                    }
                    old::PermissionScope::Curator(curator) => {
                        crate::PermissionScope::Curator(curator)
                    }
                    old::PermissionScope::Namespace(namespace) => {
                        crate::PermissionScope::Namespace(namespace)
                    }
                    old::PermissionScope::Wallet(wallet) => crate::PermissionScope::Wallet(wallet),
                };

                Some(crate::PermissionContract {
                    delegator: old.delegator,
                    scope: new_scope,
                    duration: old.duration,
                    revocation: old.revocation,
                    enforcement: old.enforcement,
                    last_update: old.last_update,
                    last_execution: old.last_execution,
                    execution_count: old.execution_count,
                    created_at: old.created_at,
                })
            });

            weight
        }
    }
}
