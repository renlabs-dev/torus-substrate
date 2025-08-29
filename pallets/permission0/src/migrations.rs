pub mod v7 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_std::{vec, vec::Vec},
        sp_tracing::{error, info, warn},
        sp_weights::Weight,
    };

    use crate::{
        Config, Pallet, PermissionContract, PermissionScope, Permissions, PermissionsByDelegator,
        PermissionsByParticipants, PermissionsByRecipient, StreamScope,
        permission::add_permission_indices,
        permission::{CuratorScope, NamespaceScope},
    };

    pub type Migration<T, W> = VersionedMigration<6, 7, MigrateToV7<T>, Pallet<T>, W>;
    pub struct MigrateToV7<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use pallet_torus0_api::NamespacePath;
        use polkadot_sdk::{
            frame_support::Identity,
            frame_support_procedural::storage_alias,
            polkadot_sdk_frame::prelude::{BlockNumberFor, ValueQuery},
            sp_runtime::{BoundedBTreeMap, BoundedBTreeSet, BoundedVec},
        };
        use scale_info::TypeInfo;

        use crate::{
            AccountIdOf, Config, CuratorPermissions, EnforcementAuthority, Pallet,
            PermissionDuration, PermissionId, RevocationTerms, StreamScope,
        };

        #[storage_alias]
        pub type Permissions<T: Config> =
            StorageMap<Pallet<T>, Identity, PermissionId, OldPermissionContract<T>>;

        #[storage_alias]
        pub type PermissionsByParticipants<T: Config> = StorageMap<
            Pallet<T>,
            Identity,
            (AccountIdOf<T>, AccountIdOf<T>),
            BoundedVec<PermissionId, <T as Config>::MaxRecipientsPerPermission>,
            ValueQuery,
        >;

        #[storage_alias]
        pub type PermissionsByDelegator<T: Config> = StorageMap<
            Pallet<T>,
            Identity,
            AccountIdOf<T>,
            BoundedVec<PermissionId, <T as Config>::MaxRecipientsPerPermission>,
            ValueQuery,
        >;

        #[storage_alias]
        pub type PermissionsByRecipient<T: Config> = StorageMap<
            Pallet<T>,
            Identity,
            AccountIdOf<T>,
            BoundedVec<PermissionId, <T as Config>::MaxRecipientsPerPermission>,
            ValueQuery,
        >;

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldCuratorScope<T: Config> {
            pub recipient: T::AccountId,
            pub flags: BoundedBTreeMap<
                Option<PermissionId>,
                CuratorPermissions,
                T::MaxCuratorSubpermissionsPerPermission,
            >,
            pub cooldown: Option<BlockNumberFor<T>>,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldNamespaceScope<T: Config> {
            pub recipient: T::AccountId,
            pub paths: BoundedBTreeMap<
                Option<PermissionId>,
                BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
                T::MaxNamespacesPerPermission,
            >,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub enum OldPermissionScope<T: Config> {
            Stream(StreamScope<T>),
            Curator(OldCuratorScope<T>),
            Namespace(OldNamespaceScope<T>),
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldPermissionContract<T: Config> {
            pub delegator: T::AccountId,
            pub scope: OldPermissionScope<T>,
            pub duration: PermissionDuration<T>,
            pub revocation: RevocationTerms<T>,
            /// Enforcement authority that can toggle the permission
            pub enforcement: EnforcementAuthority<T>,
            /// Last update block
            pub last_update: BlockNumberFor<T>,
            /// Last execution block
            #[doc(hidden)]
            pub last_execution: Option<BlockNumberFor<T>>,
            /// Number of times the permission was executed
            #[doc(hidden)]
            pub execution_count: u32,
            pub max_instances: u32,
            pub children: polkadot_sdk::sp_runtime::BoundedBTreeSet<
                PermissionId,
                T::MaxChildrenPerPermission,
            >,
            pub created_at: BlockNumberFor<T>,
        }
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV7<T> {
        fn on_runtime_upgrade() -> Weight {
            let _ = old_storage::PermissionsByDelegator::<T>::clear(u32::MAX, None);
            let _ = old_storage::PermissionsByRecipient::<T>::clear(u32::MAX, None);
            let _ = old_storage::PermissionsByParticipants::<T>::clear(u32::MAX, None);

            let _ = crate::PermissionsByDelegator::<T>::clear(u32::MAX, None);
            let _ = crate::PermissionsByRecipient::<T>::clear(u32::MAX, None);
            let _ = crate::PermissionsByParticipants::<T>::clear(u32::MAX, None);

            for (permission_id, old_contract) in old_storage::Permissions::<T>::iter() {
                let new_scope = match old_contract.scope {
                    old_storage::OldPermissionScope::Stream(stream) => {
                        if let Err(e) = add_permission_indices::<T>(
                            &old_contract.delegator,
                            stream.recipients.keys(),
                            permission_id,
                        ) {
                            error!(
                                "Failed to add permission indices for stream permission {permission_id:?}: {e:?}"
                            );
                        }

                        PermissionScope::Stream(stream)
                    }
                    old_storage::OldPermissionScope::Curator(old_curator) => {
                        if let Err(e) = add_permission_indices::<T>(
                            &old_contract.delegator,
                            core::iter::once(&old_curator.recipient),
                            permission_id,
                        ) {
                            error!(
                                "Failed to add permission indices for curator permission {permission_id:?}: {e:?}"
                            );
                        }

                        let new_curator = crate::permission::CuratorScope::<T> {
                            recipient: old_curator.recipient,
                            flags: old_curator.flags,
                            cooldown: old_curator.cooldown,
                            max_instances: old_contract.max_instances,
                            children: old_contract.children,
                        };

                        PermissionScope::Curator(new_curator)
                    }
                    old_storage::OldPermissionScope::Namespace(old_namespace) => {
                        if let Err(e) = add_permission_indices::<T>(
                            &old_contract.delegator,
                            core::iter::once(&old_namespace.recipient),
                            permission_id,
                        ) {
                            error!(
                                "Failed to add permission indices for namespace permission {permission_id:?}: {e:?}"
                            );
                        }

                        let new_namespace = crate::permission::NamespaceScope::<T> {
                            recipient: old_namespace.recipient,
                            paths: old_namespace.paths,
                            max_instances: old_contract.max_instances,
                            children: old_contract.children,
                        };

                        PermissionScope::Namespace(new_namespace)
                    }
                };

                let new_contract = PermissionContract::<T> {
                    delegator: old_contract.delegator,
                    scope: new_scope,
                    duration: old_contract.duration,
                    revocation: old_contract.revocation,
                    enforcement: old_contract.enforcement,
                    last_update: old_contract.created_at,
                    last_execution: old_contract.last_execution,
                    execution_count: old_contract.execution_count,
                    created_at: old_contract.created_at,
                };

                crate::Permissions::<T>::set(permission_id, Some(new_contract));
            }

            check_all_indices_consistency::<T>();

            Weight::zero()
        }
    }

    /// Check consistency of all permission indices after migration
    fn check_all_indices_consistency<T: Config>() {
        info!("Starting permission0 index consistency checks...");

        let mut total_inconsistencies = 0u32;
        total_inconsistencies =
            total_inconsistencies.saturating_add(check_delegator_indices_consistency::<T>());
        total_inconsistencies =
            total_inconsistencies.saturating_add(check_recipient_indices_consistency::<T>());
        total_inconsistencies =
            total_inconsistencies.saturating_add(check_participant_indices_consistency::<T>());
        total_inconsistencies =
            total_inconsistencies.saturating_add(check_permissions_are_indexed::<T>());

        if total_inconsistencies == 0 {
            info!("All permission0 indices are consistent!");
        } else {
            error!("Found {total_inconsistencies} total index inconsistencies in permission0!");
        }
    }

    /// Check that all permissions in delegator indices exist and have correct delegator
    fn check_delegator_indices_consistency<T: Config>() -> u32 {
        let mut inconsistencies = 0u32;

        for (delegator, permission_ids) in PermissionsByDelegator::<T>::iter() {
            for permission_id in permission_ids.iter() {
                if let Some(contract) = Permissions::<T>::get(permission_id) {
                    if contract.delegator != delegator {
                        error!(
                            "Delegator index inconsistency: Permission {permission_id:?} \
                            indexed under delegator {delegator:?} but actual delegator is {:?}",
                            contract.delegator
                        );
                        inconsistencies = inconsistencies.saturating_add(1);
                    }
                } else {
                    error!(
                        "Delegator index inconsistency: Permission {permission_id:?} \
                        indexed under delegator {delegator:?} but permission doesn't exist"
                    );
                    inconsistencies = inconsistencies.saturating_add(1);
                }
            }
        }

        if inconsistencies > 0 {
            warn!("Found {inconsistencies} delegator index inconsistencies");
        } else {
            info!("Delegator indices are consistent");
        }

        inconsistencies
    }

    /// Check that all permissions in recipient indices exist and recipient is listed in the permission scope
    fn check_recipient_indices_consistency<T: Config>() -> u32 {
        let mut inconsistencies = 0u32;

        for (recipient, permission_ids) in PermissionsByRecipient::<T>::iter() {
            for permission_id in permission_ids.iter() {
                if let Some(contract) = Permissions::<T>::get(permission_id) {
                    let is_valid_recipient = match &contract.scope {
                        PermissionScope::Stream(StreamScope { recipients, .. }) => {
                            recipients.contains_key(&recipient)
                        }
                        PermissionScope::Curator(CuratorScope {
                            recipient: scope_recipient,
                            ..
                        })
                        | PermissionScope::Namespace(NamespaceScope {
                            recipient: scope_recipient,
                            ..
                        }) => scope_recipient == &recipient,
                    };

                    if !is_valid_recipient {
                        error!(
                            "Recipient index inconsistency: Permission {permission_id:?} \
                            indexed under recipient {recipient:?} but recipient is not in the permission scope"
                        );
                        inconsistencies = inconsistencies.saturating_add(1);
                    }
                } else {
                    error!(
                        "Recipient index inconsistency: Permission {permission_id:?} \
                        indexed under recipient {recipient:?} but permission doesn't exist"
                    );
                    inconsistencies = inconsistencies.saturating_add(1);
                }
            }
        }

        if inconsistencies > 0 {
            warn!("Found {inconsistencies} recipient index inconsistencies");
        } else {
            info!("Recipient indices are consistent");
        }

        inconsistencies
    }

    /// Check that all permissions in participant indices exist and both participants are correct
    fn check_participant_indices_consistency<T: Config>() -> u32 {
        let mut inconsistencies = 0u32;

        for ((delegator, recipient), permission_ids) in PermissionsByParticipants::<T>::iter() {
            for permission_id in permission_ids.iter() {
                if let Some(contract) = Permissions::<T>::get(permission_id) {
                    if contract.delegator != delegator {
                        error!(
                            "Participant index inconsistency: Permission {permission_id:?} \
                            indexed under delegator {delegator:?} but actual delegator is {:?}",
                            contract.delegator
                        );
                        inconsistencies = inconsistencies.saturating_add(1);
                        continue;
                    }

                    let is_valid_recipient = match &contract.scope {
                        PermissionScope::Stream(StreamScope { recipients, .. }) => {
                            recipients.contains_key(&recipient)
                        }
                        PermissionScope::Curator(CuratorScope {
                            recipient: scope_recipient,
                            ..
                        })
                        | PermissionScope::Namespace(NamespaceScope {
                            recipient: scope_recipient,
                            ..
                        }) => scope_recipient == &recipient,
                    };

                    if !is_valid_recipient {
                        error!(
                            "Participant index inconsistency: Permission {permission_id:?} \
                            indexed under participant pair ({delegator:?}, {recipient:?}) \
                            but recipient is not in the permission scope"
                        );
                        inconsistencies = inconsistencies.saturating_add(1);
                    }
                } else {
                    error!(
                        "Participant index inconsistency: Permission {permission_id:?} \
                        indexed under participant pair ({delegator:?}, {recipient:?}) \
                        but permission doesn't exist"
                    );
                    inconsistencies = inconsistencies.saturating_add(1);
                }
            }
        }

        if inconsistencies > 0 {
            warn!("Found {inconsistencies} participant index inconsistencies");
        } else {
            info!("Participant indices are consistent");
        }

        inconsistencies
    }

    /// Check that all permissions in storage are properly indexed
    /// This is the counterpart function that verifies all permissions have correct index entries
    pub fn check_permissions_are_indexed<T: Config>() -> u32 {
        let mut inconsistencies = 0u32;

        info!("Checking that all permissions in storage are properly indexed...");

        for (permission_id, contract) in Permissions::<T>::iter() {
            let delegator = &contract.delegator;

            let delegator_indices = PermissionsByDelegator::<T>::get(delegator);
            if !delegator_indices.contains(&permission_id) {
                error!(
                    "Missing delegator index: Permission {permission_id:?} with delegator {delegator:?} \
                    is not in PermissionsByDelegator index"
                );
                inconsistencies = inconsistencies.saturating_add(1);
            }

            let recipients: Vec<T::AccountId> = match &contract.scope {
                PermissionScope::Stream(StreamScope { recipients, .. }) => {
                    recipients.keys().cloned().collect()
                }
                PermissionScope::Curator(CuratorScope { recipient, .. })
                | PermissionScope::Namespace(NamespaceScope { recipient, .. }) => {
                    vec![recipient.clone()]
                }
            };

            for recipient in recipients.iter() {
                let recipient_indices = PermissionsByRecipient::<T>::get(recipient);
                if !recipient_indices.contains(&permission_id) {
                    error!(
                        "Missing recipient index: Permission {permission_id:?} with recipient {recipient:?} \
                        is not in PermissionsByRecipient index"
                    );
                    inconsistencies = inconsistencies.saturating_add(1);
                }

                let participant_indices =
                    PermissionsByParticipants::<T>::get((delegator.clone(), recipient.clone()));
                if !participant_indices.contains(&permission_id) {
                    error!(
                        "Missing participant index: Permission {permission_id:?} with participants \
                        ({delegator:?}, {recipient:?}) is not in PermissionsByParticipants index"
                    );
                    inconsistencies = inconsistencies.saturating_add(1);
                }
            }
        }

        if inconsistencies > 0 {
            warn!("Found {inconsistencies} missing index entries for existing permissions");
        } else {
            info!("All permissions are properly indexed");
        }

        inconsistencies
    }
}
