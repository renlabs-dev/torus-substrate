pub mod v4 {
    use num_traits::Zero;
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_tracing::warn,
        sp_weights::Weight,
    };

    use crate::{Config, EmissionAllocation, Pallet, PermissionScope};

    pub type Migration<T, W> = VersionedMigration<3, 4, MigrateToV6<T>, Pallet<T>, W>;
    pub struct MigrateToV6<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::{
            frame_support::Identity, frame_support_procedural::storage_alias,
            polkadot_sdk_frame::prelude::BlockNumberFor, sp_runtime::BoundedVec,
        };
        use scale_info::TypeInfo;

        use crate::{
            permission::{EnforcementAuthority, PermissionDuration, PermissionScope},
            AccountIdOf, Config, Pallet, PermissionId,
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

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV6<T> {
        fn on_runtime_upgrade() -> Weight {
            let _ = old_storage::PermissionsByGrantor::<T>::clear(u32::MAX, None);
            let _ = old_storage::PermissionsByGrantee::<T>::clear(u32::MAX, None);

            for (pid, contract) in old_storage::Permissions::<T>::iter() {
                if let PermissionScope::Emission(scope) = &contract.scope {
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

                crate::Permissions::<T>::set(
                    pid,
                    Some(crate::PermissionContract {
                        delegator: contract.grantor,
                        recipient: contract.grantee,
                        scope: contract.scope,
                        duration: contract.duration,
                        revocation: match contract.revocation {
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
                        },
                        enforcement: contract.enforcement,
                        last_execution: contract.last_execution,
                        execution_count: contract.execution_count,
                        parent: contract.parent,
                        created_at: contract.created_at,
                    }),
                );
            }

            Weight::zero()
        }
    }
}
