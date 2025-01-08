// use crate::*;
// use frame_support::{pallet_prelude::ValueQuery, traits::StorageVersion, Blake2_128Concat};
// use sp_runtime::Percent;

// use frame_system::Config as SystemConfig;

pub mod next {
    use crate::{
        application::{AgentApplication, ApplicationStatus},
        config::GovernanceConfiguration,
        proposal::{Proposal, ProposalStatus, UnrewardedProposal},
        AccountIdOf, Config, GlobalGovernanceConfig, Pallet,
    };
    use polkadot_sdk::{
        frame_support::{
            migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
        },
        polkadot_sdk_frame::prelude::BlockNumberFor,
        sp_core::{Get, U256},
        sp_runtime::traits::AccountIdConversion,
    };

    // use super::*;

    mod old_storage {
        use crate::proposal::{ProposalData, ProposalId};
        use crate::{AccountIdOf, BalanceOf};
        use codec::{Decode, Encode, MaxEncodedLen};
        use polkadot_sdk::frame_support::{storage_alias, Identity};
        use polkadot_sdk::sp_core::ConstU32;
        use polkadot_sdk::sp_runtime::{BoundedBTreeMap, BoundedBTreeSet, BoundedVec, Percent};
        use scale_info::TypeInfo;

        use super::*;

        type Block = u64;
        type BlockAmount = u64;

        #[derive(TypeInfo, Decode, Encode, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct AgentApplication<T: Config> {
            pub id: u32,
            pub payer_key: AccountIdOf<T>,
            pub agent_key: AccountIdOf<T>,
            pub data: BoundedVec<u8, T::MaxApplicationDataLength>,
            pub cost: BalanceOf<T>,
            pub expires_at: Block,
            pub action: crate::application::ApplicationAction,
            pub status: ApplicationStatus,
        }

        #[derive(Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
        pub enum ApplicationStatus {
            Open,
            Resolved { accepted: bool },
            Expired,
        }

        #[storage_alias]
        pub type AgentApplications<T: Config> =
            StorageMap<Pallet<T>, Identity, u32, AgentApplication<T>>;

        #[derive(Clone, TypeInfo, Decode, Encode, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct Proposal<T: Config> {
            pub id: ProposalId,
            pub proposer: AccountIdOf<T>,
            pub expiration_block: Block,
            pub data: ProposalData<T>,
            pub status: ProposalStatus<T>,
            pub metadata: BoundedVec<u8, ConstU32<256>>,
            pub proposal_cost: BalanceOf<T>,
            pub creation_block: Block,
        }

        #[derive(Clone, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub enum ProposalStatus<T: Config> {
            Open {
                votes_for: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
                votes_against: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
                stake_for: BalanceOf<T>,
                stake_against: BalanceOf<T>,
            },
            Accepted {
                block: Block,
                stake_for: BalanceOf<T>,
                stake_against: BalanceOf<T>,
            },
            Refused {
                block: Block,
                stake_for: BalanceOf<T>,
                stake_against: BalanceOf<T>,
            },
            Expired,
        }

        #[storage_alias]
        pub type Proposals<T: Config> = StorageMap<Pallet<T>, Identity, ProposalId, Proposal<T>>;

        #[derive(TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
        #[scale_info(skip_type_params(T))]
        pub struct UnrewardedProposal<T: Config> {
            pub block: Block,
            pub votes_for: BoundedBTreeMap<AccountIdOf<T>, BalanceOf<T>, ConstU32<{ u32::MAX }>>,
            pub votes_against:
                BoundedBTreeMap<AccountIdOf<T>, BalanceOf<T>, ConstU32<{ u32::MAX }>>,
        }

        #[storage_alias]
        pub type UnrewardedProposals<T: Config> =
            StorageMap<Pallet<T>, Identity, ProposalId, UnrewardedProposal<T>>;

        #[derive(Clone, TypeInfo, Decode, Encode, PartialEq, Eq, MaxEncodedLen)]
        #[scale_info(skip_type_params(T))]
        pub struct GovernanceConfiguration<T: Config> {
            pub proposal_cost: BalanceOf<T>,
            pub proposal_expiration: BlockAmount,
            pub agent_application_cost: BalanceOf<T>,
            pub agent_application_expiration: BlockAmount,
            pub proposal_reward_treasury_allocation: Percent,
            pub max_proposal_reward_treasury_allocation: BalanceOf<T>,
            pub proposal_reward_interval: BlockAmount,
        }

        #[storage_alias]
        pub type GlobalGovernanceConfig<T: Config> =
            StorageValue<Pallet<T>, GovernanceConfiguration<T>>;
    }

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToNext<T>, Pallet<T>, W>;
    pub struct MigrateToNext<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToNext<T> {
        fn on_runtime_upgrade() -> Weight {
            let resolver_id: AccountIdOf<T> =
                <T as Config>::PalletId::get().into_account_truncating();

            for (id, app) in old_storage::AgentApplications::<T>::iter() {
                let Ok(expires_at) = app.expires_at.try_into() else {
                    polkadot_sdk::sp_tracing::error!(
                        "failed to migrate application {id}, expires_at is invalid"
                    );
                    continue;
                };

                crate::AgentApplications::<T>::set(
                    id,
                    Some(AgentApplication {
                        id,
                        payer_key: app.payer_key,
                        agent_key: app.agent_key,
                        data: app.data,
                        cost: app.cost,
                        expires_at,
                        action: app.action,
                        status: match app.status {
                            old_storage::ApplicationStatus::Open => ApplicationStatus::Open,
                            old_storage::ApplicationStatus::Resolved { accepted } => {
                                ApplicationStatus::Resolved {
                                    accepted,
                                    resolved_by: resolver_id.clone(),
                                }
                            }
                            old_storage::ApplicationStatus::Expired => ApplicationStatus::Expired,
                        },
                    }),
                );

                polkadot_sdk::sp_tracing::info!("migrated application {id}");
            }

            for (id, proposal) in old_storage::Proposals::<T>::iter() {
                let Ok(expiration_block) =
                    BlockNumberFor::<T>::try_from(U256::from(proposal.expiration_block))
                else {
                    polkadot_sdk::sp_tracing::error!(
                        "failed to migrate proposal {id}, expiration block is invalid"
                    );
                    continue;
                };

                let Ok(creation_block) =
                    BlockNumberFor::<T>::try_from(U256::from(proposal.creation_block))
                else {
                    polkadot_sdk::sp_tracing::error!(
                        "failed to migrate proposal {id}, creation block is invalid"
                    );
                    continue;
                };

                crate::Proposals::<T>::set(
                    id,
                    Some(Proposal {
                        id,
                        proposer: proposal.proposer,
                        expiration_block,
                        data: proposal.data,
                        status: match proposal.status {
                            old_storage::ProposalStatus::Open {
                                votes_for,
                                votes_against,
                                stake_for,
                                stake_against,
                            } => ProposalStatus::Open {
                                votes_for,
                                votes_against,
                                stake_for,
                                stake_against,
                            },
                            old_storage::ProposalStatus::Accepted {
                                block,
                                stake_for,
                                stake_against,
                            } => ProposalStatus::Accepted {
                                block: if let Ok(block) =
                                    BlockNumberFor::<T>::try_from(U256::from(block))
                                {
                                    polkadot_sdk::sp_tracing::error!(
                                        "failed to migrate proposal {id}, acceptance block is invalid"
                                    );
                                    block
                                } else {
                                    continue;
                                },
                                stake_for,
                                stake_against,
                            },
                            old_storage::ProposalStatus::Refused {
                                block,
                                stake_for,
                                stake_against,
                            } => ProposalStatus::Refused {
                                block: if let Ok(block) =
                                    BlockNumberFor::<T>::try_from(U256::from(block))
                                {
                                    polkadot_sdk::sp_tracing::error!(
                                        "failed to migrate proposal {id}, refusal block is invalid"
                                    );
                                    block
                                } else {
                                    continue;
                                },
                                stake_for,
                                stake_against,
                            },
                            old_storage::ProposalStatus::Expired => ProposalStatus::Expired,
                        },
                        metadata: proposal.metadata,
                        proposal_cost: proposal.proposal_cost,
                        creation_block,
                    }),
                );

                polkadot_sdk::sp_tracing::info!("migrated proposal {id}");
            }

            for (id, proposal) in old_storage::UnrewardedProposals::<T>::iter() {
                let Ok(block) = BlockNumberFor::<T>::try_from(U256::from(proposal.block)) else {
                    polkadot_sdk::sp_tracing::error!(
                        "failed to migrate unrewarded proposal {id}, block is invalid"
                    );
                    continue;
                };

                crate::UnrewardedProposals::<T>::set(
                    id,
                    Some(UnrewardedProposal {
                        block,
                        votes_for: proposal.votes_for,
                        votes_against: proposal.votes_against,
                    }),
                );

                polkadot_sdk::sp_tracing::info!("migrated unrewarded proposal {id}");
            }

            if let Some(governance_config) = old_storage::GlobalGovernanceConfig::<T>::get() {
                loop {
                    GlobalGovernanceConfig::<T>::set(GovernanceConfiguration {
                        proposal_cost: governance_config.proposal_cost,
                        proposal_expiration: if let Ok(block) = BlockNumberFor::<T>::try_from(
                            U256::from(governance_config.proposal_expiration),
                        ) {
                            block
                        } else {
                            polkadot_sdk::sp_tracing::error!(
                              "failed to global governance config, proposal expiration block is invalid"
                            );
                            break;
                        },
                        agent_application_cost: governance_config.agent_application_cost,
                        agent_application_expiration: if let Ok(block) =
                            BlockNumberFor::<T>::try_from(U256::from(
                                governance_config.agent_application_expiration,
                            )) {
                            polkadot_sdk::sp_tracing::error!(
                              "failed to global governance config, agent application expiration block is invalid"
                            );
                            block
                        } else {
                            break;
                        },
                        proposal_reward_treasury_allocation: governance_config
                            .proposal_reward_treasury_allocation,
                        max_proposal_reward_treasury_allocation: governance_config
                            .max_proposal_reward_treasury_allocation,
                        proposal_reward_interval: if let Ok(block) = BlockNumberFor::<T>::try_from(
                            U256::from(governance_config.proposal_reward_interval),
                        ) {
                            polkadot_sdk::sp_tracing::error!(
                              "failed to global governance config, proposal reward interview is invalid"
                            );
                            block
                        } else {
                            break;
                        },
                    });
                    polkadot_sdk::sp_tracing::info!("migrated global governance config");
                }
            }

            Weight::zero()
        }
    }
}
