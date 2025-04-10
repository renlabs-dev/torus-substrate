use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
};

use crate::{Config, Pallet};

pub mod v1 {
    use super::*;
    use crate::permission::PermissionContract;
    use crate::PermissionId;
    use polkadot_sdk::sp_runtime::BoundedVec;
    use scale_info::prelude::{collections::BTreeMap, vec::Vec};

    pub mod storage {
        use super::*;
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::frame_support::{storage_alias, CloneNoBound, DebugNoBound, Identity};
        use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
        use scale_info::TypeInfo;

        use crate::{EnforcementAuthority, PermissionDuration, PermissionScope, RevocationTerms};

        #[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
        #[scale_info(skip_type_params(T))]
        pub struct PermissionContract<T: crate::Config> {
            pub grantor: T::AccountId,
            pub grantee: T::AccountId,
            pub scope: PermissionScope<T>,
            pub duration: PermissionDuration<T>,
            pub revocation: RevocationTerms<T>,
            pub enforcement: EnforcementAuthority<T>,
            pub last_execution: Option<BlockNumberFor<T>>,
            pub execution_count: u32,
            pub parent: Option<PermissionId>,
            pub created_at: BlockNumberFor<T>,
        }

        #[storage_alias]
        pub type Permissions<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, PermissionId, PermissionContract<T>>;
    }

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);
    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let old_permissions = storage::Permissions::<T>::iter().collect::<Vec<_>>();
            let _ = storage::Permissions::<T>::clear(u32::MAX, None);

            let mut children = old_permissions.iter().fold(
                BTreeMap::<PermissionId, Vec<PermissionId>>::new(),
                |mut map, (id, contract)| {
                    if let Some(parent) = contract.parent {
                        map.entry(parent).or_default().push(*id);
                    }
                    map
                },
            );

            for (id, contract) in old_permissions {
                crate::Permissions::<T>::insert(
                    id,
                    PermissionContract::<T> {
                        grantor: contract.grantor,
                        grantee: contract.grantee,
                        scope: contract.scope,
                        duration: contract.duration,
                        revocation: contract.revocation,
                        enforcement: contract.enforcement,
                        last_execution: contract.last_execution,
                        execution_count: contract.execution_count,
                        parent: contract.parent,
                        children: BoundedVec::truncate_from(
                            children.remove(&id).unwrap_or_default(),
                        ),
                        created_at: contract.created_at,
                    },
                );
            }

            Weight::zero()
        }
    }
}
