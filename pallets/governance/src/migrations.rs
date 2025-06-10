use polkadot_sdk::frame_support::{
    migrations::VersionedMigration, sp_runtime::traits::Get, traits::UncheckedOnRuntimeUpgrade,
    weights::Weight,
};

use crate::{Config, Pallet};

pub mod v3 {
    use pallet_permission0_api::{CuratorPermissions, Permission0CuratorApi, PermissionDuration};
    use polkadot_sdk::{
        frame_system::RawOrigin,
        sp_tracing::{info, warn},
    };

    use crate::proposal::{GlobalParamsData, ProposalData};

    use super::*;

    pub type Migration<T, W> = VersionedMigration<2, 5, MigrateToV3<T>, Pallet<T>, W>;
    pub struct MigrateToV3<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::{
            frame_support::{storage_alias, DebugNoBound, Identity},
            polkadot_sdk_frame::prelude::BlockNumberFor,
            sp_core::ConstU32,
            sp_runtime::{BoundedVec, Percent},
        };
        use scale_info::TypeInfo;

        use crate::{
            proposal::{ProposalId, ProposalStatus},
            AccountIdOf, BalanceOf,
        };

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct Proposal<T: crate::Config> {
            pub id: ProposalId,
            pub proposer: AccountIdOf<T>,
            pub expiration_block: BlockNumberFor<T>,
            pub data: ProposalData<T>,
            pub status: ProposalStatus<T>,
            pub metadata: BoundedVec<u8, ConstU32<256>>,
            pub proposal_cost: BalanceOf<T>,
            pub creation_block: BlockNumberFor<T>,
        }

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub enum ProposalData<T: crate::Config> {
            GlobalParams(GlobalParamsData<T>),
            GlobalCustom,
            Emission {
                recycling_percentage: Percent,
                treasury_percentage: Percent,
                incentives_ratio: Percent,
            },
            TransferDaoTreasury {
                account: AccountIdOf<T>,
                amount: BalanceOf<T>,
            },
        }

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub struct GlobalParamsData<T: crate::Config> {
            pub min_name_length: u16,
            pub max_name_length: u16,
            pub max_allowed_agents: u16,
            pub max_allowed_weights: u16,
            pub min_stake_per_weight: BalanceOf<T>,
            pub min_weight_control_fee: u8,
            pub min_staking_fee: u8,
            pub dividends_participation_weight: Percent,
            pub proposal_cost: BalanceOf<T>,
        }

        #[storage_alias]
        pub type Proposals<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, ProposalId, Proposal<T>>;

        #[storage_alias]
        pub type Curators<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, AccountIdOf<T>, ()>;
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV3<T> {
        fn on_runtime_upgrade() -> Weight {
            for (id, proposal) in old_storage::Proposals::iter() {
                let new_data = match proposal.data {
                    old_storage::ProposalData::GlobalParams(old_storage::GlobalParamsData {
                        min_name_length,
                        max_name_length,
                        min_weight_control_fee,
                        min_staking_fee,
                        dividends_participation_weight,
                        proposal_cost,
                        ..
                    }) => ProposalData::GlobalParams(GlobalParamsData {
                        min_name_length,
                        max_name_length,
                        min_weight_control_fee,
                        min_staking_fee,
                        dividends_participation_weight,
                        namespace_pricing_config:
                            <T as pallet_torus0::Config>::DefaultNamespacePricingConfig::get(),
                        proposal_cost,
                    }),
                    old_storage::ProposalData::GlobalCustom => ProposalData::GlobalCustom,
                    old_storage::ProposalData::Emission {
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                    } => ProposalData::Emission {
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                    },
                    old_storage::ProposalData::TransferDaoTreasury { account, amount } => {
                        ProposalData::TransferDaoTreasury { account, amount }
                    }
                };

                let new_proposal = crate::proposal::Proposal {
                    id: proposal.id,
                    proposer: proposal.proposer,
                    expiration_block: proposal.expiration_block,
                    data: new_data,
                    status: proposal.status,
                    metadata: proposal.metadata,
                    proposal_cost: proposal.proposal_cost,
                    creation_block: proposal.creation_block,
                };

                crate::Proposals::<T>::set(id, Some(new_proposal));
            }

            for (curator, _) in old_storage::Curators::<T>::iter() {
                let res = <<T as Config>::Permission0>::grant_curator_permission(
                    RawOrigin::Root.into(),
                    curator.clone(),
                    CuratorPermissions::all(),
                    None,
                    PermissionDuration::Indefinite,
                    pallet_permission0_api::RevocationTerms::RevocableByGrantor,
                );

                match res {
                    Ok(perm_id) => info!("migrated curator {curator:?} to permission0: {perm_id}"),
                    Err(err) => {
                        warn!("Could not migrate curator {curator:?} to permission0: {err:?}");
                    }
                }
            }

            Weight::zero()
        }
    }
}

pub mod v4 {
    use pallet_permission0_api::{CuratorPermissions, Permission0CuratorApi, PermissionDuration};
    use polkadot_sdk::{
        frame_system::RawOrigin,
        sp_tracing::{info, warn},
    };

    use super::*;

    pub type Migration<T, W> = VersionedMigration<3, 4, MigrateToV4<T>, Pallet<T>, W>;
    pub struct MigrateToV4<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use polkadot_sdk::frame_support::{storage_alias, Identity};

        use crate::AccountIdOf;

        #[storage_alias]
        pub type Curators<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, AccountIdOf<T>, ()>;
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV4<T> {
        fn on_runtime_upgrade() -> Weight {
            for (curator, _) in old_storage::Curators::<T>::iter() {
                let res = <<T as Config>::Permission0>::grant_curator_permission(
                    RawOrigin::Root.into(),
                    curator.clone(),
                    CuratorPermissions::all(),
                    None,
                    PermissionDuration::Indefinite,
                    pallet_permission0_api::RevocationTerms::RevocableByGrantor,
                );

                match res {
                    Ok(perm_id) => info!("migrated curator {curator:?} to permission0: {perm_id}"),
                    Err(err) => {
                        warn!("Could not migrate curator {curator:?} to permission0: {err:?}");
                    }
                }
            }

            Weight::zero()
        }
    }
}

pub mod v5 {
    use crate::proposal::{GlobalParamsData, ProposalData};

    use super::*;

    pub type Migration<T, W> = VersionedMigration<4, 5, MigrateToV5<T>, Pallet<T>, W>;
    pub struct MigrateToV5<T>(core::marker::PhantomData<T>);

    mod old_storage {
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::{
            frame_support::{storage_alias, DebugNoBound, Identity},
            polkadot_sdk_frame::prelude::BlockNumberFor,
            sp_core::ConstU32,
            sp_runtime::{BoundedVec, Percent},
        };
        use scale_info::TypeInfo;

        use crate::{
            proposal::{ProposalId, ProposalStatus},
            AccountIdOf, BalanceOf,
        };

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct Proposal<T: crate::Config> {
            pub id: ProposalId,
            pub proposer: AccountIdOf<T>,
            pub expiration_block: BlockNumberFor<T>,
            pub data: ProposalData<T>,
            pub status: ProposalStatus<T>,
            pub metadata: BoundedVec<u8, ConstU32<256>>,
            pub proposal_cost: BalanceOf<T>,
            pub creation_block: BlockNumberFor<T>,
        }

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub enum ProposalData<T: crate::Config> {
            GlobalParams(GlobalParamsData<T>),
            GlobalCustom,
            Emission {
                recycling_percentage: Percent,
                treasury_percentage: Percent,
                incentives_ratio: Percent,
            },
            TransferDaoTreasury {
                account: AccountIdOf<T>,
                amount: BalanceOf<T>,
            },
        }

        #[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub struct GlobalParamsData<T: crate::Config> {
            pub min_name_length: u16,
            pub max_name_length: u16,
            pub max_allowed_agents: u16,
            pub min_weight_control_fee: u8,
            pub min_staking_fee: u8,
            pub dividends_participation_weight: Percent,
            pub proposal_cost: BalanceOf<T>,
        }

        #[storage_alias]
        pub type Proposals<T: crate::Config> =
            StorageMap<crate::Pallet<T>, Identity, ProposalId, Proposal<T>>;
    }

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV5<T> {
        fn on_runtime_upgrade() -> Weight {
            for (id, proposal) in old_storage::Proposals::iter() {
                let new_data = match proposal.data {
                    old_storage::ProposalData::GlobalParams(old_storage::GlobalParamsData {
                        min_name_length,
                        max_name_length,
                        min_weight_control_fee,
                        min_staking_fee,
                        dividends_participation_weight,
                        proposal_cost,
                        ..
                    }) => ProposalData::GlobalParams(GlobalParamsData {
                        min_name_length,
                        max_name_length,
                        min_weight_control_fee,
                        min_staking_fee,
                        dividends_participation_weight,
                        namespace_pricing_config:
                            <T as pallet_torus0::Config>::DefaultNamespacePricingConfig::get(),
                        proposal_cost,
                    }),
                    old_storage::ProposalData::GlobalCustom => ProposalData::GlobalCustom,
                    old_storage::ProposalData::Emission {
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                    } => ProposalData::Emission {
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                    },
                    old_storage::ProposalData::TransferDaoTreasury { account, amount } => {
                        ProposalData::TransferDaoTreasury { account, amount }
                    }
                };

                let new_proposal = crate::proposal::Proposal {
                    id: proposal.id,
                    proposer: proposal.proposer,
                    expiration_block: proposal.expiration_block,
                    data: new_data,
                    status: proposal.status,
                    metadata: proposal.metadata,
                    proposal_cost: proposal.proposal_cost,
                    creation_block: proposal.creation_block,
                };

                crate::Proposals::<T>::set(id, Some(new_proposal));
            }

            Weight::zero()
        }
    }
}
