pub mod v4 {
    use num_traits::Zero;
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_runtime::BoundedBTreeMap,
        sp_tracing::warn,
        sp_weights::Weight,
    };

    use crate::{
        Config, EmissionAllocation, Pallet, PermissionId, PermissionScope,
        permission::NamespaceScope,
    };

    pub type Migration<T, W> = VersionedMigration<3, 4, MigrateToV4<T>, Pallet<T>, W>;
    pub struct MigrateToV4<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use pallet_torus0_api::NamespacePath;
        use polkadot_sdk::{
            frame_support::Identity,
            frame_support_procedural::storage_alias,
            polkadot_sdk_frame::prelude::BlockNumberFor,
            sp_runtime::{BoundedBTreeSet, BoundedVec},
        };
        use scale_info::TypeInfo;

        use crate::{
            AccountIdOf, Config, CuratorScope, EmissionScope, Pallet, PermissionId,
            permission::{EnforcementAuthority, PermissionDuration},
        };

        #[storage_alias]
        pub type Permissions<T: Config> =
            StorageMap<Pallet<T>, Identity, PermissionId, PermissionContract<T>>;

        #[storage_alias]
        pub type PermissionsByGrantor<T: Config> = StorageMap<
            Pallet<T>,
            Identity,
            AccountIdOf<T>,
            BoundedVec<PermissionId, <T as Config>::MaxTargetsPerPermission>,
        >;

        #[storage_alias]
        pub type PermissionsByGrantee<T: Config> = StorageMap<
            Pallet<T>,
            Identity,
            AccountIdOf<T>,
            BoundedVec<PermissionId, <T as Config>::MaxTargetsPerPermission>,
        >;

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct PermissionContract<T: crate::Config> {
            pub grantor: T::AccountId,
            pub grantee: T::AccountId,
            pub scope: PermissionScope<T>,
            pub duration: PermissionDuration<T>,
            pub revocation: RevocationTerms<T>,
            /// Enforcement authority that can toggle the permission
            pub enforcement: EnforcementAuthority<T>,
            /// Last execution block
            pub last_execution: Option<BlockNumberFor<T>>,
            /// Number of times the permission was executed
            pub execution_count: u32,
            /// Parent permission ID (None for root permissions)
            pub parent: Option<PermissionId>,
            pub created_at: BlockNumberFor<T>,
        }

        /// Defines what the permission applies to
        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub enum PermissionScope<T: Config> {
            Emission(EmissionScope<T>),
            Curator(CuratorScope<T>),
            Namespace(NamespaceScope<T>),
        }

        /// Scope for namespace permissions
        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct NamespaceScope<T: Config> {
            /// Set of namespace paths this permission delegates access to
            pub paths: BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub enum RevocationTerms<T: Config> {
            /// Cannot be revoked
            Irrevocable,
            /// Can be revoked by the grantor at any time
            RevocableByGrantor,
            /// Can be revoked by third party arbiters
            RevocableByArbiters {
                accounts: BoundedVec<T::AccountId, T::MaxRevokersPerPermission>,
                required_votes: u32,
            },
            /// Time-based revocation
            RevocableAfter(BlockNumberFor<T>),
        }
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV4<T> {
        fn on_runtime_upgrade() -> Weight {
            let _ = old_storage::PermissionsByGrantor::<T>::clear(u32::MAX, None);
            let _ = old_storage::PermissionsByGrantee::<T>::clear(u32::MAX, None);

            for (pid, contract) in old_storage::Permissions::<T>::iter() {
                if let old_storage::PermissionScope::Emission(scope) = &contract.scope {
                    if let EmissionAllocation::Streams(streams) = &scope.allocation {
                        for stream in streams.keys() {
                            if !crate::AccumulatedStreamAmounts::<T>::contains_key((
                                &contract.grantor,
                                &stream,
                                &pid,
                            )) {
                                warn!(
                                    "inserting accumulated stream value for broken contract: {pid}, stream: {stream}"
                                );
                                crate::AccumulatedStreamAmounts::<T>::set(
                                    (&contract.grantor, &stream, &pid),
                                    Some(Zero::zero()),
                                );
                            }
                        }
                    }
                }

                crate::PermissionsByDelegator::<T>::mutate(&contract.grantor, |pids| {
                    let _ = pids.try_push(pid);
                });

                crate::PermissionsByRecipient::<T>::mutate(&contract.grantee, |pids| {
                    let _ = pids.try_push(pid);
                });

                let scope = match contract.scope {
                    old_storage::PermissionScope::Emission(scope) => {
                        PermissionScope::Emission(scope)
                    }
                    old_storage::PermissionScope::Curator(scope) => PermissionScope::Curator(scope),
                    old_storage::PermissionScope::Namespace(scope) => PermissionScope::Namespace({
                        let mut paths = BoundedBTreeMap::new();
                        let _ = paths.try_insert(Option::<PermissionId>::None, scope.paths);
                        NamespaceScope { paths }
                    }),
                };

                let revocation = match contract.revocation {
                    old_storage::RevocationTerms::Irrevocable => {
                        crate::RevocationTerms::Irrevocable
                    }
                    old_storage::RevocationTerms::RevocableByGrantor => {
                        crate::RevocationTerms::RevocableByDelegator
                    }
                    old_storage::RevocationTerms::RevocableByArbiters {
                        accounts,
                        required_votes,
                    } => crate::RevocationTerms::RevocableByArbiters {
                        accounts,
                        required_votes,
                    },
                    old_storage::RevocationTerms::RevocableAfter(block) => {
                        crate::RevocationTerms::RevocableAfter(block)
                    }
                };

                let mut new = crate::PermissionContract::<T>::new(
                    contract.grantor,
                    contract.grantee,
                    scope,
                    contract.duration,
                    revocation,
                    contract.enforcement,
                    1,
                );

                new.created_at = contract.created_at;
                #[allow(deprecated)]
                new.set_execution_info(contract.last_execution, contract.execution_count);

                crate::Permissions::<T>::set(pid, Some(new));
            }

            Weight::zero()
        }
    }
}

pub mod v5 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_runtime::BoundedBTreeMap,
        sp_weights::Weight,
    };

    use crate::{Config, CuratorScope, Pallet, PermissionId, PermissionScope};

    pub type Migration<T, W> = VersionedMigration<4, 5, MigrateToV5<T>, Pallet<T>, W>;
    pub struct MigrateToV5<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::{
            frame_support::Identity, frame_support_procedural::storage_alias,
            polkadot_sdk_frame::prelude::BlockNumberFor, sp_runtime::BoundedBTreeSet,
        };
        use scale_info::TypeInfo;

        use crate::{
            Config, CuratorPermissions, EmissionScope, Pallet, PermissionId, RevocationTerms,
            permission::{EnforcementAuthority, NamespaceScope, PermissionDuration},
        };

        #[storage_alias]
        pub type Permissions<T: Config> =
            StorageMap<Pallet<T>, Identity, PermissionId, PermissionContract<T>>;

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct PermissionContract<T: Config> {
            pub delegator: T::AccountId,
            pub recipient: T::AccountId,
            pub scope: PermissionScope<T>,
            pub duration: PermissionDuration<T>,
            pub revocation: RevocationTerms<T>,
            /// Enforcement authority that can toggle the permission
            pub enforcement: EnforcementAuthority<T>,
            /// Last execution block
            pub last_execution: Option<BlockNumberFor<T>>,
            /// Number of times the permission was executed
            pub execution_count: u32,
            /// Maximum number of instances of this permission
            pub max_instances: u32,
            /// Children permissions
            pub children: BoundedBTreeSet<PermissionId, T::MaxChildrenPerPermission>,
            pub created_at: BlockNumberFor<T>,
        }

        /// Defines what the permission applies to
        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub enum PermissionScope<T: Config> {
            Emission(EmissionScope<T>),
            Curator(CuratorScope<T>),
            Namespace(NamespaceScope<T>),
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct CuratorScope<T: Config> {
            pub flags: CuratorPermissions,
            pub cooldown: Option<BlockNumberFor<T>>,
        }
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV5<T> {
        fn on_runtime_upgrade() -> Weight {
            for (pid, contract) in old_storage::Permissions::<T>::iter() {
                let scope = match contract.scope {
                    old_storage::PermissionScope::Emission(scope) => {
                        PermissionScope::Emission(scope)
                    }
                    old_storage::PermissionScope::Namespace(scope) => {
                        PermissionScope::Namespace(scope)
                    }
                    old_storage::PermissionScope::Curator(scope) => PermissionScope::Curator({
                        let mut flags = BoundedBTreeMap::new();
                        let _ = flags.try_insert(Option::<PermissionId>::None, scope.flags);
                        CuratorScope {
                            flags,
                            cooldown: scope.cooldown,
                        }
                    }),
                };

                let mut new = crate::PermissionContract::<T>::new(
                    contract.delegator,
                    contract.recipient,
                    scope,
                    contract.duration,
                    contract.revocation,
                    contract.enforcement,
                    1,
                );

                new.created_at = contract.created_at;
                #[allow(deprecated)]
                new.set_execution_info(contract.last_execution, contract.execution_count);

                crate::Permissions::<T>::set(pid, Some(new));
            }

            Weight::zero()
        }
    }
}
