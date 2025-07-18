#![cfg_attr(not(feature = "std"), no_std)]

pub mod application;
pub mod config;
pub mod ext;
pub mod migrations;
pub mod proposal;
pub mod roles;
pub mod voting;
pub mod whitelist;

pub mod benchmarking;
pub mod weights;

pub(crate) use ext::*;
use frame::prelude::ensure_root;
pub use pallet::*;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::{ValueQuery, *},
        sp_runtime::Percent,
        traits::Currency,
        Identity, PalletId,
    },
    frame_system::pallet_prelude::{ensure_signed, BlockNumberFor, OriginFor},
    polkadot_sdk_frame::{
        traits::AccountIdConversion,
        {self as frame},
    },
    sp_std::vec::Vec,
};

use crate::{
    application::AgentApplication,
    config::GovernanceConfiguration,
    proposal::{Proposal, ProposalId, UnrewardedProposal},
};

#[frame::pallet]
pub mod pallet {
    #![allow(clippy::too_many_arguments)]

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);

    use pallet_permission0_api::{CuratorPermissions, Permission0Api, Permission0CuratorApi};
    use polkadot_sdk::sp_core::ConstBool;
    use proposal::GlobalParamsData;
    use weights::WeightInfo;

    use super::*;

    /// Map of past and present proposals indexed by their incrementing ID.
    #[pallet::storage]
    pub type Proposals<T: Config> = StorageMap<_, Identity, ProposalId, Proposal<T>>;

    /// Queue of proposals to be rewarded after closing.
    #[pallet::storage]
    pub type UnrewardedProposals<T: Config> =
        StorageMap<_, Identity, ProposalId, UnrewardedProposal<T>>;

    /// List of keys that are NOT delegating their voting power. By default, all
    /// keys delegate their voting power.
    #[pallet::storage]
    pub type NotDelegatingVotingPower<T: Config> =
        StorageValue<_, BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>, ValueQuery>;

    /// Global governance configuration files.
    #[pallet::storage]
    pub type GlobalGovernanceConfig<T: Config> =
        StorageValue<_, GovernanceConfiguration<T>, ValueQuery>;

    // This has to be different than DefaultKey, so we are not conflicting in tests.
    #[pallet::type_value]
    pub fn DefaultDaoTreasuryAddress<T: Config>() -> AccountIdOf<T> {
        <T as Config>::PalletId::get().into_account_truncating()
    }

    /// The treasury address to which the treasury emission percentages and
    /// other funds go to. A proposal can be created withdrawing the funds to a
    /// key.
    #[pallet::storage]
    pub type DaoTreasuryAddress<T: Config> =
        StorageValue<_, AccountIdOf<T>, ValueQuery, DefaultDaoTreasuryAddress<T>>;

    /// A map of agent applications, past and present.
    #[pallet::storage]
    pub type AgentApplications<T: Config> = StorageMap<_, Identity, u32, AgentApplication<T>>;

    /// List of whitelisted keys. Keys listed here are allowed to register
    /// agents.
    #[pallet::storage]
    pub type Whitelist<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, ()>;

    /// List of allocator keys, which are the default validators on the network.
    #[pallet::storage]
    pub type Allocators<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, ()>;

    /// Fee taken from emission distribution and deposited into
    /// [`DaoTreasuryAddress`].
    #[pallet::storage]
    pub type TreasuryEmissionFee<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultTreasuryEmissionFee>;

    /// Determines if new agents can be registered on the chain.
    #[pallet::storage]
    pub type AgentsFrozen<T: Config> = StorageValue<_, bool, ValueQuery, ConstBool<false>>;

    /// Determines if new namespaces can be created on the chain.
    #[pallet::storage]
    pub type NamespacesFrozen<T: Config> = StorageValue<_, bool, ValueQuery, ConstBool<false>>;

    #[pallet::config]
    pub trait Config:
        polkadot_sdk::frame_system::Config + pallet_torus0::Config + pallet_emission0::Config
    {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type PalletId: Get<PalletId>;

        #[pallet::constant]
        type MinApplicationDataLength: Get<u32>;

        #[pallet::constant]
        type MaxApplicationDataLength: Get<u32>;

        #[pallet::constant]
        type ApplicationExpiration: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type MaxPenaltyPercentage: Get<Percent>;

        #[pallet::constant]
        type DefaultTreasuryEmissionFee: Get<Percent>;

        #[pallet::constant]
        type DefaultProposalCost: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultProposalExpiration: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type DefaultAgentApplicationCost: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultAgentApplicationExpiration: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type DefaultProposalRewardTreasuryAllocation: Get<Percent>;

        #[pallet::constant]
        type DefaultMaxProposalRewardTreasuryAllocation: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultProposalRewardInterval: Get<BlockNumberFor<Self>>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;

        type Permission0: Permission0Api<OriginFor<Self>>
            + Permission0CuratorApi<Self::AccountId, OriginFor<Self>, BlockNumberFor<Self>>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            application::resolve_expired_applications::<T>(block_number);

            proposal::tick_proposals::<T>(block_number);
            proposal::tick_proposal_rewards::<T>(block_number);

            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Adds a new allocator to the list. Only available for the root key.
        #[pallet::call_index(2)]
        #[pallet::weight((<T as Config>::WeightInfo::add_allocator(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_allocator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::add_allocator::<T>(key)
        }

        /// Removes an existing allocator from the list. Only available for the
        /// root key.
        #[pallet::call_index(3)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_allocator(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_allocator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::remove_allocator::<T>(key)
        }

        /// Forcefully adds a new agent to the whitelist. Only available for the
        /// root key or curators.
        #[pallet::call_index(4)]
        #[pallet::weight((<T as Config>::WeightInfo::add_to_whitelist(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_to_whitelist(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            <T as Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::WHITELIST_MANAGE,
            )?;
            whitelist::add_to_whitelist::<T>(key)
        }

        /// Forcefully removes an agent from the whitelist. Only available for
        /// the root key or curators.
        #[pallet::call_index(5)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_from_whitelist(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_from_whitelist(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            <T as Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::WHITELIST_MANAGE,
            )?;
            whitelist::remove_from_whitelist::<T>(key)
        }

        /// Accepts an agent application. Only available for the root key or
        /// curators.
        #[pallet::call_index(6)]
        #[pallet::weight((<T as Config>::WeightInfo::accept_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn accept_application(origin: OriginFor<T>, application_id: u32) -> DispatchResult {
            <T as Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::APPLICATION_REVIEW,
            )?;
            application::accept_application::<T>(application_id)
        }

        /// Denies an agent application. Only available for the root key or
        /// curators.
        #[pallet::call_index(7)]
        #[pallet::weight((<T as Config>::WeightInfo::deny_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn deny_application(origin: OriginFor<T>, application_id: u32) -> DispatchResult {
            <T as Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::APPLICATION_REVIEW,
            )?;
            application::deny_application::<T>(application_id)
        }

        /// Sets a penalty factor to the given agent emissions. Only available
        /// for the root key or curators.
        #[pallet::call_index(8)]
        #[pallet::weight((<T as Config>::WeightInfo::penalize_agent(), DispatchClass::Normal, Pays::Yes))]
        pub fn penalize_agent(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            percentage: u8,
        ) -> DispatchResult {
            roles::penalize_agent::<T>(origin, agent_key, percentage)
        }

        /// Submits a new agent application on behalf of a given key.
        #[pallet::call_index(9)]
        #[pallet::weight((<T as Config>::WeightInfo::submit_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn submit_application(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            metadata: Vec<u8>,
            removing: bool,
        ) -> DispatchResult {
            let payer = ensure_signed(origin)?;
            application::submit_application::<T>(payer, agent_key, metadata, removing)
        }

        /// Creates a new global parameters proposal.
        #[pallet::call_index(10)]
        #[pallet::weight((<T as Config>::WeightInfo::add_global_params_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_global_params_proposal(
            origin: OriginFor<T>,
            data: GlobalParamsData<T>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_global_params_proposal::<T>(proposer, data, metadata)
        }

        /// Creates a new custom global proposal.
        #[pallet::call_index(11)]
        #[pallet::weight((<T as Config>::WeightInfo::add_global_custom_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_global_custom_proposal(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_global_custom_proposal::<T>(proposer, metadata)
        }

        /// Creates a proposal moving funds from the treasury account to the
        /// given key.
        #[pallet::call_index(12)]
        #[pallet::weight((<T as Config>::WeightInfo::add_dao_treasury_transfer_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_dao_treasury_transfer_proposal(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            destination_key: AccountIdOf<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_dao_treasury_transfer_proposal::<T>(
                proposer,
                value,
                destination_key,
                data,
            )
        }

        /// Casts a vote for an open proposal.
        #[pallet::call_index(13)]
        #[pallet::weight((<T as Config>::WeightInfo::vote_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn vote_proposal(
            origin: OriginFor<T>,
            proposal_id: u64,
            agree: bool,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            voting::add_vote::<T>(voter, proposal_id, agree)
        }

        /// Removes a casted vote for an open proposal.
        #[pallet::call_index(14)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_vote_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_vote_proposal(origin: OriginFor<T>, proposal_id: u64) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            voting::remove_vote::<T>(voter, proposal_id)
        }

        /// Enables vote power delegation.
        #[pallet::call_index(15)]
        #[pallet::weight((<T as Config>::WeightInfo::enable_vote_delegation(), DispatchClass::Normal, Pays::Yes))]
        pub fn enable_vote_delegation(origin: OriginFor<T>) -> DispatchResult {
            let delegator = ensure_signed(origin)?;
            voting::enable_delegation::<T>(delegator)
        }

        /// Disables vote power delegation.
        #[pallet::call_index(16)]
        #[pallet::weight((<T as Config>::WeightInfo::disable_vote_delegation(), DispatchClass::Normal, Pays::Yes))]
        pub fn disable_vote_delegation(origin: OriginFor<T>) -> DispatchResult {
            let delegator = ensure_signed(origin)?;
            voting::disable_delegation::<T>(delegator)
        }

        /// Creates a new emission percentage proposal.
        #[pallet::call_index(17)]
        #[pallet::weight((<T as Config>::WeightInfo::add_emission_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_emission_proposal(
            origin: OriginFor<T>,
            recycling_percentage: Percent,
            treasury_percentage: Percent,
            incentives_ratio: Percent,
            data: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_emission_proposal::<T>(
                proposer,
                recycling_percentage,
                treasury_percentage,
                incentives_ratio,
                data,
            )
        }

        /// Forcefully sets emission percentages. Only available for the root
        /// key.
        #[pallet::call_index(18)]
        #[pallet::weight((<T as Config>::WeightInfo::add_emission_proposal(), DispatchClass::Normal, Pays::No))]
        pub fn set_emission_params(
            origin: OriginFor<T>,
            recycling_percentage: Percent,
            treasury_percentage: Percent,
        ) -> DispatchResult {
            // ensure root
            ensure_root(origin)?;

            pallet_emission0::EmissionRecyclingPercentage::<T>::set(recycling_percentage);
            crate::TreasuryEmissionFee::<T>::set(treasury_percentage);

            Ok(())
        }

        #[pallet::call_index(19)]
        #[pallet::weight((<T as Config>::WeightInfo::toggle_agent_freezing(), DispatchClass::Normal, Pays::No))]
        pub fn toggle_agent_freezing(origin: OriginFor<T>) -> DispatchResult {
            let curator = <T as pallet::Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::AGENT_FREEZING_TOGGLING,
            )?;

            let new_state = !crate::AgentsFrozen::<T>::get();
            AgentsFrozen::<T>::set(new_state);

            crate::Pallet::<T>::deposit_event(crate::Event::AgentFreezingToggled {
                curator,
                new_state,
            });

            Ok(())
        }

        #[pallet::call_index(20)]
        #[pallet::weight((<T as Config>::WeightInfo::toggle_namespace_freezing(), DispatchClass::Normal, Pays::No))]
        pub fn toggle_namespace_freezing(origin: OriginFor<T>) -> DispatchResult {
            let curator = <T as pallet::Config>::Permission0::ensure_curator_permission(
                origin,
                CuratorPermissions::NAMESPACE_FREEZING_TOGGLING,
            )?;

            let new_state = !crate::NamespacesFrozen::<T>::get();
            NamespacesFrozen::<T>::set(new_state);

            crate::Pallet::<T>::deposit_event(crate::Event::NamespaceFreezingToggled {
                curator,
                new_state,
            });

            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new proposal has been created.
        ProposalCreated(ProposalId),
        /// A proposal has been accepted.
        ProposalAccepted(ProposalId),
        /// A proposal has been refused.
        ProposalRefused(ProposalId),
        /// A proposal has expired.
        ProposalExpired(ProposalId),
        /// A vote has been cast on a proposal.
        ProposalVoted(u64, T::AccountId, bool),
        /// A vote has been unregistered from a proposal.
        ProposalVoteUnregistered(u64, T::AccountId),
        /// An agent account has been added to the whitelist.
        WhitelistAdded(T::AccountId),
        /// An agent account has been removed from the whitelist.
        WhitelistRemoved(T::AccountId),
        /// A new application has been created.
        ApplicationCreated(u32),
        /// An application has been accepted.
        ApplicationAccepted(u32),
        /// An application has been denied.
        ApplicationDenied(u32),
        /// An application has expired.
        ApplicationExpired(u32),
        /// A penalty was applied to an agent.
        PenaltyApplied {
            curator: T::AccountId,
            agent: T::AccountId,
            penalty: Percent,
        },
        /// The agent freezing feature was toggled by a curator.
        AgentFreezingToggled {
            curator: T::AccountId,
            new_state: bool,
        },
        /// The namespace freezing feature was toggled by a curator.
        NamespaceFreezingToggled {
            curator: T::AccountId,
            new_state: bool,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The proposal is already finished. Do not retry.
        ProposalIsFinished,
        /// Invalid parameters were provided to the finalization process.
        InvalidProposalFinalizationParameters,
        /// Invalid parameters were provided to the voting process.
        InvalidProposalVotingParameters,
        /// Negative proposal cost when setting global or subnet governance
        /// configuration.
        InvalidProposalCost,
        /// Negative expiration when setting global or subnet governance
        /// configuration.
        InvalidProposalExpiration,
        /// Key doesn't have enough tokens to create a proposal.
        NotEnoughBalanceToPropose,
        /// Proposal data is empty.
        ProposalDataTooSmall,
        /// Proposal data is bigger than 256 characters.
        ProposalDataTooLarge,
        /// The staked module is already delegating for 2 ^ 32 keys.
        ModuleDelegatingForMaxStakers,
        /// Proposal with given id doesn't exist.
        ProposalNotFound,
        /// Proposal was either accepted, refused or expired and cannot accept
        /// votes.
        ProposalClosed,
        /// Proposal data isn't composed by valid UTF-8 characters.
        InvalidProposalData,
        /// Invalid value given when transforming a u64 into T::Currency.
        InvalidCurrencyConversionValue,
        /// Dao Treasury doesn't have enough funds to be transferred.
        InsufficientDaoTreasuryFunds,
        /// Key has already voted on given Proposal.
        AlreadyVoted,
        /// Key hasn't voted on given Proposal.
        NotVoted,
        /// Key doesn't have enough stake to vote.
        InsufficientStake,
        /// The voter is delegating its voting power to their staked modules.
        /// Disable voting power delegation.
        VoterIsDelegatingVotingPower,
        /// An internal error occurred, probably relating to the size of the
        /// bounded sets.
        InternalError,
        /// The application is not in a pending state.
        ApplicationNotOpen,
        /// The application key is already used in another application.
        ApplicationKeyAlreadyUsed,
        /// The account doesn't have enough balance to submit an application.
        NotEnoughBalanceToApply,
        /// The operation can only be performed by the curator.
        NotCurator,
        /// The application with the given ID was not found.
        ApplicationNotFound,
        /// The account is already whitelisted and cannot be added again.
        AlreadyWhitelisted,
        /// The account is not whitelisted and cannot be removed from the
        /// whitelist.
        NotWhitelisted,
        /// Failed to convert the given value to a balance.
        CouldNotConvertToBalance,
        /// The application data provided does not meet the length requirement
        InvalidApplicationDataLength,
        /// The key is already a curator.
        AlreadyCurator,
        /// The key is already an allocator.
        AlreadyAllocator,
        /// The key is not an allocator.
        NotAllocator,
        /// Agent not found
        AgentNotFound,
        /// Invalid agent penalty percentage
        InvalidPenaltyPercentage,
        /// Invalid minimum name length in proposal
        InvalidMinNameLength,
        /// Invalid maximum name length in proposal
        InvalidMaxNameLength,
        /// Invalid maximum allowed weights in proposal
        InvalidMaxAllowedWeights,
        /// Invalid minimum weight control fee in proposal
        InvalidMinWeightControlFee,
        /// Invalid minimum staking fee in proposal
        InvalidMinStakingFee,
        /// Invalid params given to Emission proposal
        InvalidEmissionProposalData,
    }
}

impl<T: Config> pallet_governance_api::GovernanceApi<T::AccountId> for Pallet<T> {
    fn dao_treasury_address() -> T::AccountId {
        DaoTreasuryAddress::<T>::get()
    }

    fn treasury_emission_fee() -> Percent {
        TreasuryEmissionFee::<T>::get()
    }

    fn is_whitelisted(key: &T::AccountId) -> bool {
        whitelist::is_whitelisted::<T>(key)
    }

    fn ensure_allocator(key: &T::AccountId) -> DispatchResult {
        crate::roles::ensure_allocator::<T>(key)
    }

    fn get_allocators() -> impl Iterator<Item = T::AccountId> {
        Allocators::<T>::iter_keys()
    }

    fn set_allocator(key: &T::AccountId) {
        Allocators::<T>::insert(key, ());
    }

    fn can_create_namespace(key: &T::AccountId) -> bool {
        !NamespacesFrozen::<T>::get() || Self::is_whitelisted(key)
    }

    fn can_register_agent(key: &T::AccountId) -> bool {
        !AgentsFrozen::<T>::get() || Self::is_whitelisted(key)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn force_set_whitelisted(key: &T::AccountId) {
        Whitelist::<T>::insert(key, ());
    }
}
