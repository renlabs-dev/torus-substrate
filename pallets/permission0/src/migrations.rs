pub mod v6 {
    use polkadot_sdk::{
        frame_support::{migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade},
        sp_runtime::BoundedBTreeSet,
        sp_tracing::{error, info, warn},
        sp_weights::Weight,
    };

    use crate::{
        Config, Pallet, PermissionContract, PermissionScope, Permissions, PermissionsByDelegator,
        PermissionsByParticipants, PermissionsByRecipient, StreamScope,
        permission::{CuratorScope, NamespaceScope},
        permission::{add_permission_indices, remove_permission_from_indices},
    };

    pub type Migration<T, W> = VersionedMigration<5, 6, MigrateToV6<T>, Pallet<T>, W>;
    pub struct MigrateToV6<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::{
            frame_support::Identity,
            frame_support_procedural::storage_alias,
            polkadot_sdk_frame::prelude::BlockNumberFor,
            sp_runtime::{BoundedBTreeMap, BoundedBTreeSet},
        };
        use scale_info::TypeInfo;

        use crate::{Config, DistributionControl, Pallet, PermissionId, StreamAllocation};

        #[storage_alias]
        pub type Permissions<T: Config> =
            StorageMap<Pallet<T>, Identity, PermissionId, OldPermissionContract<T>>;

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldEmissionScope<T: Config> {
            pub allocation: StreamAllocation<T>,
            pub distribution: DistributionControl<T>,
            pub targets: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
            pub accumulating: bool,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldCuratorScope<T: Config> {
            pub flags: BoundedBTreeMap<
                Option<PermissionId>,
                crate::CuratorPermissions,
                T::MaxCuratorSubpermissionsPerPermission,
            >,
            pub cooldown: Option<BlockNumberFor<T>>,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldNamespaceScope<T: Config> {
            pub paths: BoundedBTreeMap<
                Option<PermissionId>,
                BoundedBTreeSet<pallet_torus0_api::NamespacePath, T::MaxNamespacesPerPermission>,
                T::MaxNamespacesPerPermission,
            >,
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub enum OldPermissionScope<T: Config> {
            Emission(OldEmissionScope<T>),
            Curator(OldCuratorScope<T>),
            Namespace(OldNamespaceScope<T>),
        }

        #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct OldPermissionContract<T: Config> {
            pub delegator: T::AccountId,
            pub recipient: T::AccountId,
            pub scope: OldPermissionScope<T>,
            pub duration: crate::permission::PermissionDuration<T>,
            pub revocation: crate::RevocationTerms<T>,
            pub enforcement: crate::permission::EnforcementAuthority<T>,
            pub last_execution: Option<BlockNumberFor<T>>,
            pub execution_count: u32,
            pub max_instances: u32,
            pub children: polkadot_sdk::sp_runtime::BoundedBTreeSet<
                PermissionId,
                T::MaxChildrenPerPermission,
            >,
            pub created_at: BlockNumberFor<T>,
        }
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV6<T> {
        fn on_runtime_upgrade() -> Weight {
            let mut migrated_count = 0u32;
            let mut emission_permissions_updated = 0u32;
            let mut curator_permissions_updated = 0u32;
            let mut namespace_permissions_updated = 0u32;

            for (permission_id, old_contract) in old_storage::Permissions::<T>::iter() {
                let new_scope = match old_contract.scope {
                    old_storage::OldPermissionScope::Emission(old_emission) => {
                        emission_permissions_updated =
                            emission_permissions_updated.saturating_add(1);

                        // For emission permissions, we need to update storage indices:
                        // 1. Remove the old recipient index (permission recipient)
                        // 2. Add indices for all the emission targets

                        // Remove old recipient index
                        remove_permission_from_indices::<T>(
                            &old_contract.delegator,
                            core::iter::once(&old_contract.recipient),
                            permission_id,
                        );

                        let mut managers = BoundedBTreeSet::new();
                        let _ = managers.try_insert(old_contract.delegator.clone());

                        let new_emission = StreamScope::<T> {
                            recipients: old_emission.targets, // Field renamed from targets to recipients
                            allocation: old_emission.allocation,
                            distribution: old_emission.distribution,
                            accumulating: old_emission.accumulating,
                            // New manager fields introduced in v6
                            recipient_managers: managers.clone(),
                            weight_setters: managers,
                        };

                        // Add new indices for all emission targets
                        if let Err(e) = add_permission_indices::<T>(
                            &old_contract.delegator,
                            new_emission.recipients.keys(),
                            permission_id,
                        ) {
                            error!(
                                "Failed to add permission indices for emission permission {permission_id:?}: {e:?}"
                            );
                        }

                        PermissionScope::Stream(new_emission)
                    }
                    old_storage::OldPermissionScope::Curator(old_curator) => {
                        curator_permissions_updated = curator_permissions_updated.saturating_add(1);

                        let new_curator = crate::permission::CuratorScope::<T> {
                            recipient: old_contract.recipient.clone(), // Add recipient field from contract
                            flags: old_curator.flags,
                            cooldown: old_curator.cooldown,
                        };

                        PermissionScope::Curator(new_curator)
                    }
                    old_storage::OldPermissionScope::Namespace(old_namespace) => {
                        namespace_permissions_updated =
                            namespace_permissions_updated.saturating_add(1);

                        let new_namespace = crate::permission::NamespaceScope::<T> {
                            recipient: old_contract.recipient.clone(), // Add recipient field from contract
                            paths: old_namespace.paths,
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
                    last_execution: old_contract.last_execution,
                    execution_count: old_contract.execution_count,
                    children: old_contract.children,
                    created_at: old_contract.created_at,
                    max_instances: old_contract.max_instances,
                };

                crate::Permissions::<T>::set(permission_id, Some(new_contract));
                migrated_count = migrated_count.saturating_add(1);
            }

            info!(
                "Permission0 migration v5→v6 completed: {migrated_count} total permissions migrated \
| Emission: {emission_permissions_updated} (targets→recipients, manager fields added) \
| Curator: {curator_permissions_updated} (recipient field added) \
| Namespace: {namespace_permissions_updated} (recipient field added)",
            );

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
}
